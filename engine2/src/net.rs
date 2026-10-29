//! Connections: what Glue.Networking asks of Steam's networking sockets,
//! over a transport of our own. Listen sockets, connections, poll groups and
//! messages keep the handles and the message layout managed reads; the
//! bytes travel over TCP, one frame each:
//!
//!   u32 length (of what follows, little-endian), u8 kind, body
//!   Hello   1: u16 version, i32 virtual port, u64 identity  (opens, client to server)
//!   Welcome 2: u16 version, u64 identity                     (accepted, server to client)
//!   Message 3: i32 send flags, the bytes
//!   Close   4: i32 reason, UTF-8 text
//!
//! Every message is delivered in order, reliable or not: TCP's. A relay
//! carries the same frames.
//!
//! Managed calls from two threads: sends and receives from its networking
//! thread, everything else from the main one. Connection changes are queued
//! as they happen and told to managed in `run_callbacks`, on the main
//! thread, as Steam's callbacks are.
use std::collections::{HashMap, HashSet, VecDeque};
use std::io::{self, BufReader, BufWriter, Read, Write};
use std::net::{Shutdown, SocketAddr, TcpListener, TcpStream, ToSocketAddrs};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::time::Duration;

pub const VERSION: u16 = 1;
const HELLO: u8 = 1;
const WELCOME: u8 = 2;
const MESSAGE: u8 = 3;
const CLOSE: u8 = 4;
/// The largest message managed allows (Networking.Bootstrap's RecvMaxMessageSize).
pub const MAX_FRAME: usize = 64 * 1024 * 1024;

/// ESteamNetworkingConnectionState, as managed's ConnectionStatus reads it.
pub const STATE_NONE: i32 = 0;
pub const STATE_CONNECTING: i32 = 1;
pub const STATE_CONNECTED: i32 = 3;
pub const STATE_CLOSED_BY_PEER: i32 = 4;
pub const STATE_PROBLEM: i32 = 5;

/// ESteamNetConnectionEnd: the connection ended in a way we can't name.
pub const END_MISC_GENERIC: i32 = 5001;
/// The remote host didn't answer, or refused.
pub const END_MISC_TIMEOUT: i32 = 5003;

/// The managed side's half: what Steam's callbacks tell it.
pub trait Managed {
    fn should_accept(&self, socket: u32, conn: u32) -> bool;
    fn on_socket_connection(&self, socket: u32, conn: u32);
    fn on_socket_disconnection(&self, socket: u32, conn: u32);
    fn on_disconnection(&self, conn: u32, reason: i32, text: &str);
}

/// SteamNetworkingMessage_t as managed reads it (SteamNetworkMessage.cs):
/// 208 bytes, the peer identity a 136-byte blob.
#[repr(C)]
pub struct Message {
    pub data: *mut u8,
    pub size: i32,
    pub conn: u32,
    pub identity_type: i32,
    pub identity_size: i32,
    pub identity: u64,
    pub identity_padding: [i32; 30],
    pub connection_data: u64,
    pub time_received: i64,
    pub message_number: i64,
    pub free_data: usize,
    pub release: usize,
    pub channel: i32,
    pub flags: i32,
    pub user_data: i64,
}
const _: () = assert!(core::mem::size_of::<Message>() == 208);

/// A message and the bytes it points at, owned together: the message first,
/// so a pointer to one is a pointer to the other.
#[repr(C)]
struct Owned {
    msg: Message,
    bytes: Vec<u8>,
}

/// A new message of `bytes`, for managed: freed by `release`.
pub fn message(conn: u32, flags: i32, identity: u64, bytes: Vec<u8>) -> *mut Message {
    let mut o = Box::new(Owned {
        msg: Message {
            data: core::ptr::null_mut(),
            size: bytes.len() as i32,
            conn,
            identity_type: if identity != 0 { 16 } else { 0 }, // k_ESteamNetworkingIdentityType_SteamID
            identity_size: if identity != 0 { 8 } else { 0 },
            identity,
            identity_padding: [0; 30],
            connection_data: 0,
            time_received: 0,
            message_number: 0,
            free_data: 0,
            release: release_fn as extern "C" fn(*mut Message) as usize,
            channel: 0,
            flags,
            user_data: 0,
        },
        bytes,
    });
    o.msg.data = o.bytes.as_mut_ptr();
    Box::into_raw(o) as *mut Message
}

extern "C" fn release_fn(m: *mut Message) {
    unsafe { release(m) }
}

/// # Safety
/// `m` came from `message` and is released once.
pub unsafe fn release(m: *mut Message) {
    if !m.is_null() {
        drop(unsafe { Box::from_raw(m as *mut Owned) });
    }
}

/// # Safety
/// `m` came from `message` and has not been released.
pub unsafe fn message_bytes<'a>(m: *mut Message) -> &'a [u8] {
    let m = unsafe { &*m };
    if m.data.is_null() || m.size <= 0 { return &[]; }
    unsafe { core::slice::from_raw_parts(m.data, m.size as usize) }
}

// ---- frames

fn frame(kind: u8, body: &[u8]) -> Vec<u8> {
    let mut f = Vec::with_capacity(5 + body.len());
    f.extend_from_slice(&(body.len() as u32 + 1).to_le_bytes());
    f.push(kind);
    f.extend_from_slice(body);
    f
}

fn hello(vport: i32, identity: u64) -> Vec<u8> {
    let mut b = VERSION.to_le_bytes().to_vec();
    b.extend_from_slice(&vport.to_le_bytes());
    b.extend_from_slice(&identity.to_le_bytes());
    frame(HELLO, &b)
}

fn welcome(identity: u64) -> Vec<u8> {
    let mut b = VERSION.to_le_bytes().to_vec();
    b.extend_from_slice(&identity.to_le_bytes());
    frame(WELCOME, &b)
}

fn data(flags: i32, bytes: &[u8]) -> Vec<u8> {
    let mut b = flags.to_le_bytes().to_vec();
    b.extend_from_slice(bytes);
    frame(MESSAGE, &b)
}

fn close(reason: i32, text: &str) -> Vec<u8> {
    let mut b = reason.to_le_bytes().to_vec();
    b.extend_from_slice(text.as_bytes());
    frame(CLOSE, &b)
}

fn read_frame(r: &mut impl Read) -> io::Result<(u8, Vec<u8>)> {
    let mut len = [0u8; 4];
    r.read_exact(&mut len)?;
    let len = u32::from_le_bytes(len) as usize;
    if len == 0 || len > MAX_FRAME + 5 {
        return Err(io::Error::new(io::ErrorKind::InvalidData, format!("a frame of {len} bytes")));
    }
    let mut body = vec![0u8; len];
    r.read_exact(&mut body)?;
    let kind = body.remove(0);
    Ok((kind, body))
}

fn le_i32(b: &[u8], at: usize) -> i32 { b.get(at..at + 4).map(|s| i32::from_le_bytes(s.try_into().unwrap())).unwrap_or(0) }
fn le_u16(b: &[u8], at: usize) -> u16 { b.get(at..at + 2).map(|s| u16::from_le_bytes(s.try_into().unwrap())).unwrap_or(0) }
fn le_u64(b: &[u8], at: usize) -> u64 { b.get(at..at + 8).map(|s| u64::from_le_bytes(s.try_into().unwrap())).unwrap_or(0) }

// ---- the state

struct Listen {
    addr: Option<SocketAddr>,
    stop: Arc<AtomicBool>,
    conns: HashSet<u32>,
}

struct Conn {
    /// The listen socket it came in on (0: one we opened).
    socket: u32,
    state: i32,
    peer: u64,
    addr: String,
    out: Option<Sender<Vec<u8>>>,
    stream: Option<TcpStream>,
    inbox: VecDeque<*mut Message>,
    group: u32,
}

unsafe impl Send for Conn {}

impl Conn {
    fn send(&self, f: Vec<u8>) -> bool {
        self.out.as_ref().is_some_and(|o| o.send(f).is_ok())
    }
    fn shut(&mut self) {
        self.out = None; // the writer ends once it has written what it has
        if let Some(s) = self.stream.take() { let _ = s.shutdown(Shutdown::Read); }
    }
}

enum Event {
    /// A client said hello on a listen socket: managed decides.
    Incoming(u32, u32),
    /// The other end went, or never came.
    Ended(u32, i32, String),
}

#[derive(Default)]
struct State {
    next: u32,
    listens: HashMap<u32, Listen>,
    conns: HashMap<u32, Conn>,
    groups: HashMap<u32, Vec<u32>>,
    events: VecDeque<Event>,
}

impl State {
    fn id(&mut self) -> u32 {
        self.next += 1;
        self.next
    }
}

/// One process's networking. `identity` is ours, as peers see it.
pub struct Net {
    state: Arc<Mutex<State>>,
    identity: u64,
}

impl Net {
    pub fn new(identity: u64) -> Net {
        Net { state: Arc::default(), identity }
    }

    pub fn identity(&self) -> u64 { self.identity }

    /// A listen socket on `port` (all interfaces; 0 picks one): its handle,
    /// 0 if the port can't be had.
    pub fn listen_ip(&self, port: u16) -> u32 {
        let l = match TcpListener::bind(("0.0.0.0", port)) {
            Ok(l) => l,
            Err(e) => {
                crate::trace::note(&format!("net: can't listen on {port}: {e}"));
                return 0;
            }
        };
        let addr = l.local_addr().ok();
        let _ = l.set_nonblocking(true);
        let stop = Arc::new(AtomicBool::new(false));
        let id = {
            let mut s = self.state.lock().unwrap();
            let id = s.id();
            s.listens.insert(id, Listen { addr, stop: stop.clone(), conns: HashSet::new() });
            id
        };
        let state = self.state.clone();
        std::thread::Builder::new().name(format!("net listen {id}")).spawn(move || {
            while !stop.load(Ordering::Relaxed) {
                match l.accept() {
                    Ok((stream, from)) => accept(&state, id, stream, from),
                    Err(e) if e.kind() == io::ErrorKind::WouldBlock => std::thread::sleep(Duration::from_millis(5)),
                    Err(_) => std::thread::sleep(Duration::from_millis(50)),
                }
            }
        }).expect("net: a thread");
        crate::trace::note(&format!("net: listening on {}", addr.map(|a| a.to_string()).unwrap_or_default()));
        id
    }

    /// A listen socket peers reach by our identity, not an address: it has
    /// no address of its own; a relay brings it connections.
    pub fn listen_id(&self, _vport: i32) -> u32 {
        let mut s = self.state.lock().unwrap();
        let id = s.id();
        s.listens.insert(id, Listen { addr: None, stop: Arc::new(AtomicBool::new(true)), conns: HashSet::new() });
        id
    }

    pub fn socket_address(&self, socket: u32) -> String {
        let s = self.state.lock().unwrap();
        s.listens.get(&socket).and_then(|l| l.addr).map(|a| a.to_string()).unwrap_or_default()
    }

    pub fn close_socket(&self, socket: u32) {
        let mut s = self.state.lock().unwrap();
        let Some(l) = s.listens.remove(&socket) else { return };
        l.stop.store(true, Ordering::Relaxed);
        for c in l.conns {
            if let Some(mut c) = s.conns.remove(&c) {
                c.send(close(END_MISC_GENERIC, "server closed"));
                c.shut();
                free_inbox(&mut c);
            }
        }
    }

    /// A connection to `address` ("host:port"), opened in the background:
    /// connecting until the server welcomes it. What's sent meanwhile waits.
    pub fn connect(&self, address: &str, vport: i32) -> u32 {
        let (out, queued) = channel();
        let id = {
            let mut s = self.state.lock().unwrap();
            let id = s.id();
            s.conns.insert(id, Conn { socket: 0, state: STATE_CONNECTING, peer: 0, addr: address.to_string(), out: Some(out), stream: None, inbox: VecDeque::new(), group: 0 });
            id
        };
        let state = self.state.clone();
        let me = self.identity;
        let address = address.to_string();
        std::thread::Builder::new().name(format!("net conn {id}")).spawn(move || {
            let stream = address
                .to_socket_addrs()
                .ok()
                .and_then(|mut a| a.find_map(|a| TcpStream::connect_timeout(&a, Duration::from_secs(10)).ok()));
            let Some(stream) = stream else {
                ended(&state, id, END_MISC_TIMEOUT, &format!("couldn't reach {address}"));
                return;
            };
            let _ = stream.set_nodelay(true);
            if (&stream).write_all(&hello(vport, me)).is_err() || !writer(&stream, queued) {
                ended(&state, id, END_MISC_GENERIC, &format!("lost {address} on the way"));
                return;
            }
            {
                let mut s = state.lock().unwrap();
                let Some(c) = s.conns.get_mut(&id) else { return }; // closed while connecting
                c.stream = stream.try_clone().ok();
            }
            read_loop(&state, id, BufReader::new(stream));
        }).expect("net: a thread");
        id
    }

    /// Managed's answer to an incoming connection, given in `run_callbacks`.
    fn accept(&self, conn: u32) -> bool {
        let mut s = self.state.lock().unwrap();
        let Some(c) = s.conns.get_mut(&conn) else { return false };
        if c.state != STATE_CONNECTING { return false; }
        c.state = STATE_CONNECTED;
        c.send(welcome(self.identity))
    }

    pub fn close_connection(&self, conn: u32, reason: i32, text: &str) {
        let mut s = self.state.lock().unwrap();
        let Some(mut c) = s.conns.remove(&conn) else { return };
        c.send(close(reason, text));
        c.shut();
        free_inbox(&mut c);
        if let Some(l) = s.listens.get_mut(&c.socket) { l.conns.remove(&conn); }
        if let Some(g) = s.groups.get_mut(&c.group) { g.retain(|&x| x != conn); }
    }

    /// Steam's EResult: OK, or no such connection.
    pub fn send(&self, conn: u32, flags: i32, bytes: &[u8]) -> i64 {
        let s = self.state.lock().unwrap();
        match s.conns.get(&conn) {
            Some(c) if c.send(data(flags, bytes)) => 1,
            _ => 3, // k_EResultNoConnection
        }
    }

    pub fn state(&self, conn: u32) -> i32 {
        self.state.lock().unwrap().conns.get(&conn).map(|c| c.state).unwrap_or(STATE_NONE)
    }

    pub fn peer(&self, conn: u32) -> u64 {
        self.state.lock().unwrap().conns.get(&conn).map(|c| c.peer).unwrap_or(0)
    }

    pub fn description(&self, conn: u32) -> String {
        let s = self.state.lock().unwrap();
        s.conns.get(&conn).map(|c| format!("#{conn} {} ({})", c.addr, c.peer)).unwrap_or_default()
    }

    pub fn create_group(&self) -> u32 {
        let mut s = self.state.lock().unwrap();
        let id = s.id();
        s.groups.insert(id, Vec::new());
        id
    }

    pub fn destroy_group(&self, group: u32) {
        let mut s = self.state.lock().unwrap();
        for c in s.groups.remove(&group).unwrap_or_default() {
            if let Some(c) = s.conns.get_mut(&c) { c.group = 0; }
        }
    }

    pub fn set_group(&self, conn: u32, group: u32) {
        let mut s = self.state.lock().unwrap();
        let Some(old) = s.conns.get(&conn).map(|c| c.group) else { return };
        if let Some(g) = s.groups.get_mut(&old) { g.retain(|&x| x != conn); }
        let group = if s.groups.contains_key(&group) { group } else { 0 };
        if let Some(g) = s.groups.get_mut(&group) { g.push(conn); }
        s.conns.get_mut(&conn).unwrap().group = group;
    }

    /// Up to `max` messages received on `conn`, oldest first.
    pub fn receive(&self, conn: u32, max: usize) -> Vec<*mut Message> {
        let mut s = self.state.lock().unwrap();
        let Some(c) = s.conns.get_mut(&conn) else { return Vec::new() };
        let n = c.inbox.len().min(max);
        c.inbox.drain(..n).collect()
    }

    /// Up to `max` messages received on the connections in `group`.
    pub fn receive_group(&self, group: u32, max: usize) -> Vec<*mut Message> {
        let mut s = self.state.lock().unwrap();
        let members = s.groups.get(&group).cloned().unwrap_or_default();
        let mut got = Vec::new();
        for m in members {
            if got.len() >= max { break; }
            if let Some(c) = s.conns.get_mut(&m) {
                let n = c.inbox.len().min(max - got.len());
                got.extend(c.inbox.drain(..n));
            }
        }
        got
    }

    /// What happened since last time, told to managed: connections to accept
    /// or refuse, and the ones that ended. On the main thread.
    pub fn run_callbacks(&self, managed: &dyn Managed) {
        loop {
            let e = self.state.lock().unwrap().events.pop_front();
            let Some(e) = e else { break };
            match e {
                Event::Incoming(socket, conn) => {
                    if self.state(conn) != STATE_CONNECTING { continue; }
                    if managed.should_accept(socket, conn) && self.accept(conn) {
                        managed.on_socket_connection(socket, conn);
                    } else {
                        self.close_connection(conn, END_MISC_GENERIC, "not accepted");
                    }
                }
                Event::Ended(conn, reason, text) => {
                    let socket = {
                        let s = self.state.lock().unwrap();
                        match s.conns.get(&conn) {
                            Some(c) => c.socket,
                            None => continue, // closed here meanwhile
                        }
                    };
                    if socket != 0 {
                        managed.on_socket_disconnection(socket, conn);
                    } else {
                        managed.on_disconnection(conn, reason, &text);
                    }
                    // managed closes it in answer; if not, it's still gone
                    self.close_connection(conn, reason, &text);
                }
            }
        }
    }
}

fn free_inbox(c: &mut Conn) {
    for m in c.inbox.drain(..) {
        unsafe { release(m) };
    }
}

fn ended(state: &Mutex<State>, conn: u32, reason: i32, text: &str) {
    let mut s = state.lock().unwrap();
    let Some(c) = s.conns.get_mut(&conn) else { return };
    if c.state == STATE_CLOSED_BY_PEER || c.state == STATE_PROBLEM { return; }
    c.state = if reason == END_MISC_TIMEOUT { STATE_PROBLEM } else { STATE_CLOSED_BY_PEER };
    s.events.push_back(Event::Ended(conn, reason, text.to_string()));
}

/// A thread writing what's sent on `rx` to `stream`, in order; it ends when
/// the sender is dropped, or the stream breaks.
fn writer(stream: &TcpStream, rx: Receiver<Vec<u8>>) -> bool {
    let Ok(w) = stream.try_clone() else { return false };
    std::thread::Builder::new().name("net write".into()).spawn(move || {
        let mut w = BufWriter::new(w);
        while let Ok(f) = rx.recv() {
            if w.write_all(&f).is_err() { return; }
            // more queued: write them together
            while let Ok(f) = rx.try_recv() {
                if w.write_all(&f).is_err() { return; }
            }
            if w.flush().is_err() { return; }
        }
        let _ = w.flush();
        let _ = w.get_ref().shutdown(Shutdown::Write);
    }).is_ok()
}

/// A client on a listen socket: its hello, then it waits for managed.
fn accept(state: &Arc<Mutex<State>>, socket: u32, stream: TcpStream, from: SocketAddr) {
    let _ = stream.set_nonblocking(false);
    let _ = stream.set_nodelay(true);
    let state = state.clone();
    std::thread::Builder::new().name("net accept".into()).spawn(move || {
        let mut r = BufReader::new(match stream.try_clone() { Ok(s) => s, Err(_) => return });
        let _ = stream.set_read_timeout(Some(Duration::from_secs(10)));
        let Ok((HELLO, body)) = read_frame(&mut r) else { return };
        if le_u16(&body, 0) != VERSION {
            let _ = (&stream).write_all(&close(END_MISC_GENERIC, "engine-shim: a different protocol version"));
            return;
        }
        let _ = stream.set_read_timeout(None);
        let (out, queued) = channel();
        if !writer(&stream, queued) { return; }
        let id = {
            let mut s = state.lock().unwrap();
            if !s.listens.contains_key(&socket) { return; }
            let id = s.id();
            s.conns.insert(id, Conn { socket, state: STATE_CONNECTING, peer: le_u64(&body, 6), addr: from.to_string(), out: Some(out), stream: stream.try_clone().ok(), inbox: VecDeque::new(), group: 0 });
            s.listens.get_mut(&socket).unwrap().conns.insert(id);
            s.events.push_back(Event::Incoming(socket, id));
            id
        };
        read_loop(&state, id, r);
    }).expect("net: a thread");
}

/// Frames in, until the stream ends: messages to the inbox, a welcome
/// connects, a close or a broken stream ends it.
fn read_loop(state: &Arc<Mutex<State>>, conn: u32, mut r: BufReader<TcpStream>) {
    loop {
        match read_frame(&mut r) {
            Ok((MESSAGE, body)) => {
                let flags = le_i32(&body, 0);
                let mut s = state.lock().unwrap();
                let Some(c) = s.conns.get_mut(&conn) else { return };
                let m = message(conn, flags, c.peer, body[4.min(body.len())..].to_vec());
                c.inbox.push_back(m);
            }
            Ok((WELCOME, body)) => {
                let mut s = state.lock().unwrap();
                let Some(c) = s.conns.get_mut(&conn) else { return };
                if c.state == STATE_CONNECTING {
                    c.state = STATE_CONNECTED;
                    c.peer = le_u64(&body, 2);
                }
            }
            Ok((CLOSE, body)) => {
                let text = String::from_utf8_lossy(body.get(4..).unwrap_or_default()).into_owned();
                ended(state, conn, le_i32(&body, 0), &text);
                return;
            }
            Ok(_) => {}
            Err(e) => {
                ended(state, conn, END_MISC_GENERIC, &format!("connection lost: {e}"));
                return;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;

    #[derive(Default)]
    struct Log(Mutex<Vec<String>>, AtomicBool);

    impl Managed for Log {
        fn should_accept(&self, s: u32, c: u32) -> bool {
            self.0.lock().unwrap().push(format!("accept? {s} {c}"));
            !self.1.load(Ordering::Relaxed)
        }
        fn on_socket_connection(&self, s: u32, c: u32) { self.0.lock().unwrap().push(format!("connected {s} {c}")) }
        fn on_socket_disconnection(&self, s: u32, c: u32) { self.0.lock().unwrap().push(format!("gone {s} {c}")) }
        fn on_disconnection(&self, c: u32, r: i32, t: &str) { self.0.lock().unwrap().push(format!("ended {c} {r} {t}")) }
    }

    impl Log {
        fn take(&self) -> Vec<String> { std::mem::take(&mut self.0.lock().unwrap()) }
    }

    /// Callbacks, round and round, until `done` says so.
    fn until(nets: &[(&Net, &Log)], mut done: impl FnMut() -> bool) {
        let t = Instant::now();
        while !done() {
            for (n, l) in nets { n.run_callbacks(*l); }
            assert!(t.elapsed() < Duration::from_secs(5), "timed out");
            std::thread::sleep(Duration::from_millis(2));
        }
    }

    fn texts(ms: Vec<*mut Message>) -> Vec<String> {
        ms.into_iter().map(|m| unsafe {
            let s = String::from_utf8(message_bytes(m).to_vec()).unwrap();
            release(m);
            s
        }).collect()
    }

    #[test]
    fn message_layout_is_managed_s() {
        let m = message(7, 8, 42, b"hi".to_vec());
        let base = m as usize;
        let r = unsafe { &*m };
        assert_eq!(&r.size as *const _ as usize - base, 8);
        assert_eq!(&r.conn as *const _ as usize - base, 12);
        assert_eq!(&r.identity as *const _ as usize - base, 24);
        assert_eq!(&r.connection_data as *const _ as usize - base, 152);
        assert_eq!(&r.free_data as *const _ as usize - base, 176);
        assert_eq!(&r.channel as *const _ as usize - base, 192);
        assert_eq!(&r.user_data as *const _ as usize - base, 200);
        assert_eq!(unsafe { message_bytes(m) }, b"hi");
        unsafe { release(m) };
    }

    #[test]
    fn a_client_connects_talks_and_leaves() {
        let (server, slog) = (Net::new(1000), Log::default());
        let (client, clog) = (Net::new(2000), Log::default());
        let socket = server.listen_ip(0);
        assert_ne!(socket, 0);
        let addr = server.socket_address(socket).replace("0.0.0.0", "127.0.0.1");
        let group = server.create_group();

        let c = client.connect(&addr, 0);
        // sent while connecting: it goes once the server takes it, in order
        assert_eq!(client.send(c, 8, b"early"), 1);
        until(&[(&server, &slog), (&client, &clog)], || client.state(c) == STATE_CONNECTED);
        let log = slog.take();
        assert_eq!(log.len(), 2, "{log:?}");
        let sc: u32 = log[1].rsplit(' ').next().unwrap().parse().unwrap();
        assert_eq!(log[1], format!("connected {socket} {sc}"));
        assert_eq!(server.peer(sc), 2000);
        assert_eq!(client.peer(c), 1000);
        server.set_group(sc, group);

        client.send(c, 8, b"one");
        client.send(c, 0, b"two");
        let mut got = Vec::new();
        until(&[], || { got.extend(texts(server.receive_group(group, 16))); got.len() == 3 });
        assert_eq!(got, ["early", "one", "two"]);

        assert_eq!(server.send(sc, 8, b"back"), 1);
        let mut got = Vec::new();
        until(&[], || { got.extend(texts(client.receive(c, 16))); !got.is_empty() });
        assert_eq!(got, ["back"]);

        client.close_connection(c, 1000, "bye");
        until(&[(&server, &slog)], || server.state(sc) == STATE_NONE);
        assert_eq!(slog.take(), [format!("gone {socket} {sc}")]);
        assert_eq!(client.send(c, 8, b"late"), 3);
        server.close_socket(socket);
    }

    #[test]
    fn a_refused_client_is_told() {
        let (server, slog) = (Net::new(1), Log::default());
        slog.1.store(true, Ordering::Relaxed);
        let (client, clog) = (Net::new(2), Log::default());
        let socket = server.listen_ip(0);
        let addr = server.socket_address(socket).replace("0.0.0.0", "127.0.0.1");
        let c = client.connect(&addr, 0);
        until(&[(&server, &slog), (&client, &clog)], || client.state(c) == STATE_NONE);
        assert_eq!(clog.take(), [format!("ended {c} {END_MISC_GENERIC} not accepted")]);
        server.close_socket(socket);
    }

    #[test]
    fn nobody_there() {
        let (client, clog) = (Net::new(2), Log::default());
        // a port just freed: nothing listens
        let port = TcpListener::bind("127.0.0.1:0").unwrap().local_addr().unwrap().port();
        let c = client.connect(&format!("127.0.0.1:{port}"), 0);
        until(&[(&client, &clog)], || client.state(c) == STATE_NONE);
        let log = clog.take();
        assert!(log[0].starts_with(&format!("ended {c} {END_MISC_TIMEOUT} couldn't reach")), "{log:?}");
    }
}
