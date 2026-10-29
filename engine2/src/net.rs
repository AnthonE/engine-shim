//! Connections: what Glue.Networking asks of Steam's networking sockets,
//! over a transport of our own. Listen sockets, connections, poll groups and
//! messages keep the handles and the message layout managed reads; the
//! bytes travel over QUIC, through WebTransport, as Gates' shards speak it
//! (its docs/NETCODE.md §2), so a browser can join one day.
//!
//! A connection is one WebTransport session. The player opens one
//! bidirectional stream, and it carries the handshake and every reliable
//! message, in order, as frames:
//!
//!   u32 length (of what follows, little-endian), u8 kind, body
//!   Hello   1: u16 version, i32 virtual port, u64 identity  (opens, client to server)
//!   Welcome 2: u16 version, u64 identity, the nonce          (accepted, server to client)
//!   Message 3: i32 send flags, the bytes
//!   Close   4: i32 reason, UTF-8 text
//!
//! A message sent without Steam's reliable flag goes as a datagram (kind,
//! flags, the bytes; no length): it may be lost, and nothing waits for it.
//! One too big for the path rides the stream instead. The welcome's nonce
//! is the one the player's sign-in proves (steam.rs).
//!
//! Certificates (NETCODE.md §2.2's posture): a server uses
//! `ENGINE2_TLS_CERT`/`ENGINE2_TLS_KEY` (PEM), or makes itself a
//! self-signed one and prints its hash. A player checks the server against
//! the system's roots, or against the hash `ENGINE2_CERT_HASH` pins, and
//! takes any certificate only on loopback: the sign-in has no channel
//! binding, so a relay nobody checked could sign in as the player.
//!
//! Managed calls from two threads: sends and receives from its networking
//! thread, everything else from the main one. The I/O runs on a small tokio
//! runtime of its own. Connection changes are queued as they happen and
//! told to managed in `run_callbacks`, on the main thread, as Steam's
//! callbacks are.
use std::collections::{HashMap, HashSet, VecDeque};
use std::net::SocketAddr;
use std::sync::{Arc, Mutex, OnceLock};
use std::time::Duration;

use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};
use tokio::sync::Notify;
use wtransport::endpoint::IncomingSession;
use wtransport::tls::{Sha256Digest, Sha256DigestFmt};
use wtransport::config::IpBindConfig;
use wtransport::{ClientConfig, Connection, Endpoint, Identity, RecvStream, SendStream, ServerConfig};

pub const VERSION: u16 = 1;
const HELLO: u8 = 1;
const WELCOME: u8 = 2;
const MESSAGE: u8 = 3;
const CLOSE: u8 = 4;
/// The largest message managed allows (Networking.Bootstrap's RecvMaxMessageSize).
pub const MAX_FRAME: usize = 64 * 1024 * 1024;
/// k_nSteamNetworkingSend_Reliable
pub const SEND_RELIABLE: i32 = 8;

/// NETCODE.md §2.2: the server keeps connections alive; a silent one ends.
const KEEP_ALIVE: Duration = Duration::from_secs(10);
const IDLE_TIMEOUT: Duration = Duration::from_secs(30);
/// How long a player has to connect and say hello.
const HANDSHAKE: Duration = Duration::from_secs(10);
/// How long a close waits for the other end to have read it.
const LINGER: Duration = Duration::from_secs(2);

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

fn welcome(identity: u64, nonce: &str) -> Vec<u8> {
    let mut b = VERSION.to_le_bytes().to_vec();
    b.extend_from_slice(&identity.to_le_bytes());
    b.extend_from_slice(nonce.as_bytes());
    frame(WELCOME, &b)
}

/// A message's body: its flags, then its bytes.
fn message_body(flags: i32, bytes: &[u8]) -> Vec<u8> {
    let mut b = flags.to_le_bytes().to_vec();
    b.extend_from_slice(bytes);
    b
}

fn close(reason: i32, text: &str) -> Vec<u8> {
    let mut b = reason.to_le_bytes().to_vec();
    b.extend_from_slice(text.as_bytes());
    frame(CLOSE, &b)
}

async fn read_frame(r: &mut RecvStream) -> Result<(u8, Vec<u8>), String> {
    let mut len = [0u8; 4];
    r.read_exact(&mut len).await.map_err(|e| e.to_string())?;
    let len = u32::from_le_bytes(len) as usize;
    if len == 0 || len > MAX_FRAME + 5 {
        return Err(format!("a frame of {len} bytes"));
    }
    let mut body = vec![0u8; len];
    r.read_exact(&mut body).await.map_err(|e| e.to_string())?;
    let kind = body.remove(0);
    Ok((kind, body))
}

fn le_i32(b: &[u8], at: usize) -> i32 { b.get(at..at + 4).map(|s| i32::from_le_bytes(s.try_into().unwrap())).unwrap_or(0) }
fn le_u16(b: &[u8], at: usize) -> u16 { b.get(at..at + 2).map(|s| u16::from_le_bytes(s.try_into().unwrap())).unwrap_or(0) }
fn le_u64(b: &[u8], at: usize) -> u64 { b.get(at..at + 8).map(|s| u64::from_le_bytes(s.try_into().unwrap())).unwrap_or(0) }

/// The I/O's runtime: two threads, made on first use.
fn rt() -> &'static tokio::runtime::Runtime {
    static RT: OnceLock<tokio::runtime::Runtime> = OnceLock::new();
    RT.get_or_init(|| {
        tokio::runtime::Builder::new_multi_thread().worker_threads(2).thread_name("engine2 net").enable_all().build().expect("net: a runtime")
    })
}

// ---- the state

/// What a connection's writer sends.
enum Out {
    /// On the stream, in order.
    Reliable(Vec<u8>),
    /// A message's body, as a datagram if it fits.
    Unreliable(Vec<u8>),
    /// The last frame: then the connection closes.
    Close(Vec<u8>),
}

struct Listen {
    addr: Option<SocketAddr>,
    /// Stops the accept loop, which then lets the endpoint go.
    stop: Option<Arc<Notify>>,
    conns: HashSet<u32>,
}

struct Conn {
    /// The listen socket it came in on (0: one we opened).
    socket: u32,
    state: i32,
    peer: u64,
    addr: String,
    out: Option<UnboundedSender<Out>>,
    inbox: VecDeque<*mut Message>,
    group: u32,
    /// The nonce the server welcomed us with: what our sign-in proves.
    challenge: String,
}

unsafe impl Send for Conn {}

impl Conn {
    fn new(socket: u32, state: i32, peer: u64, addr: String, out: UnboundedSender<Out>) -> Conn {
        Conn { socket, state, peer, addr, out: Some(out), inbox: VecDeque::new(), group: 0, challenge: String::new() }
    }
    fn send(&self, o: Out) -> bool {
        self.out.as_ref().is_some_and(|out| out.send(o).is_ok())
    }
    /// The close frame, and then nothing: the writer ends after it.
    fn shut(&mut self, reason: i32, text: &str) {
        if let Some(out) = self.out.take() { let _ = out.send(Out::Close(close(reason, text))); }
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

    /// A listen socket on `port` (all interfaces, v4 and v6; 0 picks one):
    /// its handle, 0 if the port can't be had.
    pub fn listen_ip(&self, port: u16) -> u32 {
        // v4 and v6, or v4 alone on a host without v6
        let bind = |on: IpBindConfig| {
            let config = ServerConfig::builder()
                .with_bind_config(on, port)
                .with_identity(server_identity()?)
                .keep_alive_interval(Some(KEEP_ALIVE))
                .max_idle_timeout(Some(IDLE_TIMEOUT))
                .map_err(|e| e.to_string())?
                .build();
            let _rt = rt().enter();
            Endpoint::server(config).map_err(|e| e.to_string())
        };
        let endpoint = bind(IpBindConfig::InAddrAnyDual).or_else(|_| bind(IpBindConfig::InAddrAnyV4));
        let endpoint = match endpoint {
            Ok(e) => e,
            Err(e) => {
                crate::trace::note(&format!("net: can't listen on {port}: {e}"));
                return 0;
            }
        };
        let addr = endpoint.local_addr().ok();
        let stop = Arc::new(Notify::new());
        let id = {
            let mut s = self.state.lock().unwrap();
            let id = s.id();
            s.listens.insert(id, Listen { addr, stop: Some(stop.clone()), conns: HashSet::new() });
            id
        };
        let state = self.state.clone();
        rt().spawn(async move {
            loop {
                tokio::select! {
                    incoming = endpoint.accept() => { rt().spawn(accept(state.clone(), id, incoming)); }
                    _ = stop.notified() => break,
                }
            }
            // the loop is done with it: the endpoint goes now, never under accept
            drop(endpoint);
        });
        crate::trace::note(&format!("net: listening on {}", addr.map(|a| a.to_string()).unwrap_or_default()));
        id
    }

    /// A listen socket peers reach by our identity, not an address: it has
    /// no address of its own; a relay brings it connections.
    pub fn listen_id(&self, _vport: i32) -> u32 {
        let mut s = self.state.lock().unwrap();
        let id = s.id();
        s.listens.insert(id, Listen { addr: None, stop: None, conns: HashSet::new() });
        id
    }

    pub fn socket_address(&self, socket: u32) -> String {
        let s = self.state.lock().unwrap();
        s.listens.get(&socket).and_then(|l| l.addr).map(|a| a.to_string()).unwrap_or_default()
    }

    /// The port a listen socket has (0 if none).
    pub fn socket_port(&self, socket: u32) -> u16 {
        let s = self.state.lock().unwrap();
        s.listens.get(&socket).and_then(|l| l.addr).map(|a| a.port()).unwrap_or(0)
    }

    pub fn close_socket(&self, socket: u32) {
        let mut s = self.state.lock().unwrap();
        let Some(l) = s.listens.remove(&socket) else { return };
        if let Some(stop) = l.stop { stop.notify_one(); }
        for c in l.conns {
            if let Some(mut c) = s.conns.remove(&c) {
                c.shut(END_MISC_GENERIC, "server closed");
                free_inbox(&mut c);
            }
        }
    }

    /// A connection to `address` ("host:port"), opened in the background:
    /// connecting until the server welcomes it. What's sent meanwhile waits.
    pub fn connect(&self, address: &str, vport: i32) -> u32 {
        let (out, queued) = unbounded_channel();
        let id = {
            let mut s = self.state.lock().unwrap();
            let id = s.id();
            s.conns.insert(id, Conn::new(0, STATE_CONNECTING, 0, address.to_string(), out));
            id
        };
        rt().spawn(dial(self.state.clone(), id, address.to_string(), vport, self.identity, queued));
        id
    }

    /// Managed's answer to an incoming connection, given in `run_callbacks`.
    fn accept(&self, conn: u32) -> bool {
        let mut s = self.state.lock().unwrap();
        let Some(c) = s.conns.get_mut(&conn) else { return false };
        if c.state != STATE_CONNECTING { return false; }
        c.state = STATE_CONNECTED;
        c.send(Out::Reliable(welcome(self.identity, &crate::steam::challenge(c.peer))))
    }

    pub fn close_connection(&self, conn: u32, reason: i32, text: &str) {
        let mut s = self.state.lock().unwrap();
        let Some(mut c) = s.conns.remove(&conn) else { return };
        c.shut(reason, text);
        free_inbox(&mut c);
        if let Some(l) = s.listens.get_mut(&c.socket) { l.conns.remove(&conn); }
        if let Some(g) = s.groups.get_mut(&c.group) { g.retain(|&x| x != conn); }
    }

    /// Steam's EResult: OK, or no such connection.
    pub fn send(&self, conn: u32, flags: i32, bytes: &[u8]) -> i64 {
        let s = self.state.lock().unwrap();
        let body = message_body(flags, bytes);
        let o = if flags & SEND_RELIABLE != 0 { Out::Reliable(frame(MESSAGE, &body)) } else { Out::Unreliable(body) };
        match s.conns.get(&conn) {
            Some(c) if c.send(o) => 1,
            _ => 3, // k_EResultNoConnection
        }
    }
    /// Players on our listen sockets, past the door.
    pub fn players(&self) -> usize {
        self.state.lock().unwrap().conns.values().filter(|c| c.socket != 0 && c.state == STATE_CONNECTED).count()
    }

    /// Every connection on our listen sockets from `peer`, ended as if they
    /// had left: they hear `text` now, managed hears it in `run_callbacks`
    /// and closes its side (with its own words, which no longer reach them).
    pub fn kick(&self, peer: u64, reason: i32, text: &str) {
        let mut s = self.state.lock().unwrap();
        let theirs: Vec<u32> = s.conns.iter().filter(|(_, c)| c.socket != 0 && c.peer == peer && c.state == STATE_CONNECTED).map(|(id, _)| *id).collect();
        for id in theirs {
            let c = s.conns.get_mut(&id).unwrap();
            c.shut(reason, text);
            c.state = STATE_CLOSED_BY_PEER;
            s.events.push_back(Event::Ended(id, reason, text.to_string()));
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

    /// The server we joined last and the nonce it welcomed us with: what a
    /// ticket for it proves.
    pub fn challenge(&self) -> Option<(String, String)> {
        let s = self.state.lock().unwrap();
        s.conns
            .iter()
            .filter(|(_, c)| c.socket == 0 && c.state == STATE_CONNECTED && !c.challenge.is_empty())
            .max_by_key(|(id, _)| **id)
            .map(|(_, c)| (c.addr.clone(), c.challenge.clone()))
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

// ---- certificates

/// The server's certificate: `ENGINE2_TLS_CERT`/`ENGINE2_TLS_KEY`, or a
/// self-signed one, made once, whose hash players pin.
fn server_identity() -> Result<Identity, String> {
    static SELF_SIGNED: OnceLock<Result<Identity, String>> = OnceLock::new();
    if let (Ok(cert), Ok(key)) = (std::env::var("ENGINE2_TLS_CERT"), std::env::var("ENGINE2_TLS_KEY")) {
        return rt().block_on(Identity::load_pemfiles(&cert, &key)).map_err(|e| format!("{cert}, {key}: {e}"));
    }
    let made = SELF_SIGNED.get_or_init(|| {
        let id = Identity::self_signed(["localhost", "127.0.0.1", "::1"]).map_err(|e| e.to_string())?;
        let hash = id.certificate_chain().as_slice()[0].hash().fmt(Sha256DigestFmt::DottedHex);
        let line = format!("net: a self-signed certificate; players pin it with ENGINE2_CERT_HASH={hash}");
        eprintln!("engine2: {line}");
        crate::trace::note(&line);
        Ok(id)
    });
    made.as_ref().map(|i| i.clone_identity()).map_err(|e| e.clone())
}

/// How a player checks the server at `address`: the pinned hash, else
/// any certificate on loopback, else the system's roots.
fn client_config(address: &str, on: IpBindConfig) -> Result<ClientConfig, String> {
    let b = ClientConfig::builder().with_bind_config(on);
    let b = if let Ok(pin) = std::env::var("ENGINE2_CERT_HASH") {
        let digest = Sha256Digest::from_str_fmt(pin.trim(), Sha256DigestFmt::DottedHex).map_err(|_| format!("ENGINE2_CERT_HASH: not a sha256 as aa:bb:… ({pin})"))?;
        b.with_server_certificate_hashes([digest])
    } else if loopback(address) {
        b.with_no_cert_validation()
    } else {
        b.with_native_certs()
    };
    Ok(b.keep_alive_interval(Some(KEEP_ALIVE)).max_idle_timeout(Some(IDLE_TIMEOUT)).map_err(|e| e.to_string())?.build())
}

fn loopback(address: &str) -> bool {
    let host = address.rsplit_once(':').map(|(h, _)| h).unwrap_or(address).trim_matches(|c| c == '[' || c == ']');
    host == "localhost" || host.parse::<std::net::IpAddr>().is_ok_and(|ip| ip.is_loopback())
}

// ---- the I/O: a task per connection's stream, datagrams and writes

/// A player on a listen socket: their hello, then they wait for managed.
async fn accept(state: Arc<Mutex<State>>, socket: u32, incoming: IncomingSession) {
    let opened = tokio::time::timeout(HANDSHAKE, async {
        let session = incoming.await.map_err(|e| e.to_string())?;
        let from = session.remote_address();
        let conn = session.accept().await.map_err(|e| e.to_string())?;
        let (send, mut recv) = conn.accept_bi().await.map_err(|e| e.to_string())?;
        let (kind, body) = read_frame(&mut recv).await?;
        Ok::<_, String>((from, conn, send, recv, kind, body))
    });
    let (from, conn, mut send, recv, kind, body) = match opened.await {
        Ok(Ok(o)) => o,
        Ok(Err(e)) => return crate::trace::note(&format!("net: a player didn't get in: {e}")),
        Err(_) => return crate::trace::note("net: a player didn't say hello in time"),
    };
    if kind != HELLO || le_u16(&body, 0) != VERSION {
        let _ = send.write_all(&close(END_MISC_GENERIC, "engine-shim: a different protocol version")).await;
        let _ = send.finish().await;
        return;
    }
    let (out, queued) = unbounded_channel();
    let id = {
        let mut s = state.lock().unwrap();
        if !s.listens.contains_key(&socket) { return; }
        let id = s.id();
        s.conns.insert(id, Conn::new(socket, STATE_CONNECTING, le_u64(&body, 6), from.to_string(), out));
        s.listens.get_mut(&socket).unwrap().conns.insert(id);
        s.events.push_back(Event::Incoming(socket, id));
        id
    };
    run(state, id, conn, send, recv, queued).await;
}

/// A connection we open: dial, say hello, then as any other.
async fn dial(state: Arc<Mutex<State>>, id: u32, address: String, vport: i32, me: u64, queued: UnboundedReceiver<Out>) {
    let opened = tokio::time::timeout(HANDSHAKE, async {
        let endpoint = Endpoint::client(client_config(&address, IpBindConfig::InAddrAnyDual)?)
            .or_else(|_| Endpoint::client(client_config(&address, IpBindConfig::InAddrAnyV4)?).map_err(|e| e.to_string()))?;
        let conn = endpoint.connect(format!("https://{address}")).await.map_err(|e| e.to_string())?;
        let (mut send, recv) = conn.open_bi().await.map_err(|e| e.to_string())?.await.map_err(|e| e.to_string())?;
        send.write_all(&hello(vport, me)).await.map_err(|e| e.to_string())?;
        Ok::<_, String>((endpoint, conn, send, recv))
    });
    match opened.await {
        Ok(Ok((endpoint, conn, send, recv))) => {
            run(state, id, conn, send, recv, queued).await;
            drop(endpoint);
        }
        Ok(Err(e)) => ended(&state, id, END_MISC_TIMEOUT, &format!("couldn't reach {address}: {e}")),
        Err(_) => ended(&state, id, END_MISC_TIMEOUT, &format!("couldn't reach {address}: no answer")),
    }
}

/// A connection, open: its writes, its datagrams and its stream, until the
/// stream ends.
async fn run(state: Arc<Mutex<State>>, id: u32, conn: Connection, send: SendStream, mut recv: RecvStream, queued: UnboundedReceiver<Out>) {
    let writer = rt().spawn(write(conn.clone(), send, queued));
    let datagrams = rt().spawn(datagrams(state.clone(), id, conn.clone()));
    loop {
        match read_frame(&mut recv).await {
            Ok((MESSAGE, body)) => deliver(&state, id, &body),
            Ok((WELCOME, body)) => {
                let mut s = state.lock().unwrap();
                let Some(c) = s.conns.get_mut(&id) else { break };
                if c.state == STATE_CONNECTING {
                    c.state = STATE_CONNECTED;
                    c.peer = le_u64(&body, 2);
                    c.challenge = String::from_utf8_lossy(body.get(10..).unwrap_or_default()).into_owned();
                }
            }
            Ok((CLOSE, body)) => {
                let text = String::from_utf8_lossy(body.get(4..).unwrap_or_default()).into_owned();
                ended(&state, id, le_i32(&body, 0), &text);
                break;
            }
            Ok(_) => {}
            Err(e) => {
                ended(&state, id, END_MISC_GENERIC, &format!("connection lost: {e}"));
                break;
            }
        }
    }
    datagrams.abort();
    // the writer ends with the close frame, or when the connection goes
    let _ = writer.await;
    conn.close(0u32.into(), b"");
}

/// A message, from the stream or a datagram (flags, then bytes), into the inbox.
fn deliver(state: &Mutex<State>, id: u32, body: &[u8]) {
    let mut s = state.lock().unwrap();
    let Some(c) = s.conns.get_mut(&id) else { return };
    let m = message(id, le_i32(body, 0), c.peer, body[4.min(body.len())..].to_vec());
    c.inbox.push_back(m);
}

async fn datagrams(state: Arc<Mutex<State>>, id: u32, conn: Connection) {
    while let Ok(d) = conn.receive_datagram().await {
        if d.first() == Some(&MESSAGE) { deliver(&state, id, &d[1..]); }
    }
}

/// What's sent, in order: reliable frames on the stream, unreliable
/// messages as datagrams (or on the stream, too big for one), until the
/// close frame. A datagram the path can't take now is dropped, never waited
/// for (NETCODE.md §2.1).
async fn write(conn: Connection, mut send: SendStream, mut queued: UnboundedReceiver<Out>) {
    while let Some(o) = queued.recv().await {
        let written = match o {
            Out::Reliable(f) => send.write_all(&f).await.is_ok(),
            Out::Unreliable(body) => {
                let mut d = Vec::with_capacity(1 + body.len());
                d.push(MESSAGE);
                d.extend_from_slice(&body);
                if conn.max_datagram_size().is_some_and(|max| d.len() <= max) {
                    let _ = conn.send_datagram(d);
                    true
                } else {
                    send.write_all(&frame(MESSAGE, &body)).await.is_ok()
                }
            }
            Out::Close(f) => {
                if send.write_all(&f).await.is_ok() {
                    // read before the connection goes: the other end closes
                    // it once it has, or we do when the linger runs out
                    let _ = tokio::time::timeout(LINGER, async {
                        let _ = send.finish().await;
                        conn.closed().await
                    }).await;
                }
                conn.close(0u32.into(), b"");
                return;
            }
        };
        if !written { return; }
    }
    // dropped without a close: the connection is done with
    conn.close(0u32.into(), b"");
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicBool, Ordering};
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
            assert!(t.elapsed() < Duration::from_secs(20), "timed out");
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
    fn only_loopback_goes_unchecked() {
        for a in ["127.0.0.1:27015", "localhost:27015", "[::1]:27015", "127.4.5.6:1"] { assert!(loopback(a), "{a}"); }
        for a in ["10.0.0.2:27015", "shard.example:27015", "[2001:db8::1]:27015", "localhost.evil.example:1"] { assert!(!loopback(a), "{a}"); }
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
        let addr = format!("127.0.0.1:{}", server.socket_port(socket));
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
        // welcomed with a nonce of 32 hex digits, for the address dialled
        let (at, nonce) = client.challenge().unwrap();
        assert_eq!(at, addr);
        assert!(nonce.len() == 32 && nonce.bytes().all(|b| b.is_ascii_hexdigit()), "{nonce}");
        server.set_group(sc, group);

        client.send(c, SEND_RELIABLE, b"one");
        client.send(c, SEND_RELIABLE, b"two");
        let mut got = Vec::new();
        until(&[], || { got.extend(texts(server.receive_group(group, 16))); got.len() == 3 });
        assert_eq!(got, ["early", "one", "two"]);

        // unreliable: a datagram, and one too big for a datagram on the stream
        let big = "x".repeat(5000);
        client.send(c, 0, b"quick");
        client.send(c, 0, big.as_bytes());
        let mut got = Vec::new();
        until(&[], || { got.extend(texts(server.receive_group(group, 16))); got.len() == 2 });
        got.sort_by_key(|m| m.len());
        assert_eq!(got, ["quick".to_string(), big]);

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
    fn a_kicked_player_hears_why() {
        let (server, slog) = (Net::new(1), Log::default());
        let (client, clog) = (Net::new(77), Log::default());
        let socket = server.listen_ip(0);
        let c = client.connect(&format!("127.0.0.1:{}", server.socket_port(socket)), 0);
        until(&[(&server, &slog), (&client, &clog)], || client.state(c) == STATE_CONNECTED);
        assert_eq!(server.players(), 1);
        let sc: u32 = slog.take()[1].rsplit(' ').next().unwrap().parse().unwrap();
        server.kick(77, 1000, "no copy");
        until(&[(&server, &slog), (&client, &clog)], || client.state(c) == STATE_NONE && server.players() == 0);
        assert_eq!(slog.take(), [format!("gone {socket} {sc}")]);
        assert_eq!(clog.take(), [format!("ended {c} 1000 no copy")]);
        server.close_socket(socket);
    }

    #[test]
    fn a_refused_client_is_told() {
        let (server, slog) = (Net::new(1), Log::default());
        slog.1.store(true, Ordering::Relaxed);
        let (client, clog) = (Net::new(2), Log::default());
        let socket = server.listen_ip(0);
        let addr = format!("127.0.0.1:{}", server.socket_port(socket));
        let c = client.connect(&addr, 0);
        until(&[(&server, &slog), (&client, &clog)], || client.state(c) == STATE_NONE);
        assert_eq!(clog.take(), [format!("ended {c} {END_MISC_GENERIC} not accepted")]);
        server.close_socket(socket);
    }

    #[test]
    fn nobody_there() {
        let (client, clog) = (Net::new(2), Log::default());
        // a port just freed: nothing listens
        let port = std::net::UdpSocket::bind("127.0.0.1:0").unwrap().local_addr().unwrap().port();
        let c = client.connect(&format!("127.0.0.1:{port}"), 0);
        until(&[(&client, &clog)], || client.state(c) == STATE_NONE);
        let log = clog.take();
        assert!(log[0].starts_with(&format!("ended {c} {END_MISC_TIMEOUT} couldn't reach")), "{log:?}");
    }
}
