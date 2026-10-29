//! Slots implemented by hand. A `pub unsafe fn` here named as a slot (the
//! managed field's name, slots.rs's NAMES) replaces that slot's stub; gen/
//! picks it up, so its arguments are the slot's, in order, typed as
//! slots.rs types them.
#![allow(non_snake_case)]

use std::ffi::{CStr, CString};
use std::sync::atomic::{AtomicI32, AtomicU32, Ordering};
use std::sync::Mutex;

/// A string for managed to read: native keeps it (leaked, as the engine's
/// own static strings are).
fn cstr(s: &str) -> usize {
    CString::new(s).unwrap().into_raw() as usize
}

/// A string managed handed over (UTF-8, NUL-terminated).
unsafe fn read(p: usize) -> String {
    if p == 0 { return String::new(); }
    unsafe { CStr::from_ptr(p as *const core::ffi::c_char) }.to_string_lossy().into_owned()
}

// ---- the process

static ROOT: Mutex<String> = Mutex::new(String::new());

pub unsafe fn global_Plat_SetCurrentDirectory(dir: usize) {
    *ROOT.lock().unwrap() = unsafe { read(dir) };
}

pub unsafe fn global_GetGameRootFolder() -> usize {
    let root = ROOT.lock().unwrap().clone();
    cstr(if root.is_empty() { "." } else { &root })
}

/// The engine's first start: the command line and the app system. Nothing
/// to start yet.
pub unsafe fn global_SourceEnginePreInit(_command_line: usize, _app_system: usize) -> i32 { 1 }

/// The engine's start proper.
pub unsafe fn global_SourceEngineInit(_app_system: usize) -> i32 { 1 }

// ---- the app system (CMaterialSystem2AppSystemDict): what kind of process
// this is, set by managed before the engine starts and read back after

// written out, not by a macro: gen/ finds these by their names in this
// file. Dedicated takes a value; the other three only switch on.
static DEDICATED: AtomicI32 = AtomicI32::new(0);
pub unsafe fn CMtrlSystm2ppSys_SetDedicatedServer(_self: usize, v: i32) { DEDICATED.store(v, Ordering::Relaxed) }
pub unsafe fn CMtrlSystm2ppSys_IsDedicatedServer(_self: usize) -> i32 { DEDICATED.load(Ordering::Relaxed) }
static TOOLS: AtomicI32 = AtomicI32::new(0);
pub unsafe fn CMtrlSystm2ppSys_SetInToolsMode(_self: usize) { TOOLS.store(1, Ordering::Relaxed) }
pub unsafe fn CMtrlSystm2ppSys_IsInToolsMode(_self: usize) -> i32 { TOOLS.load(Ordering::Relaxed) }
static TEST: AtomicI32 = AtomicI32::new(0);
pub unsafe fn CMtrlSystm2ppSys_SetInTestMode(_self: usize) { TEST.store(1, Ordering::Relaxed) }
pub unsafe fn CMtrlSystm2ppSys_IsInTestMode(_self: usize) -> i32 { TEST.load(Ordering::Relaxed) }
static STANDALONE: AtomicI32 = AtomicI32::new(0);
pub unsafe fn CMtrlSystm2ppSys_SetInStandaloneApp(_self: usize) { STANDALONE.store(1, Ordering::Relaxed) }
pub unsafe fn CMtrlSystm2ppSys_IsStandaloneApp(_self: usize) -> i32 { STANDALONE.load(Ordering::Relaxed) }

static APP_ID: AtomicU32 = AtomicU32::new(0);
pub unsafe fn CMtrlSystm2ppSys_SetSteamAppId(_self: usize, id: u32) { APP_ID.store(id, Ordering::Relaxed) }
pub unsafe fn CMtrlSystm2ppSys_IsConsoleApp(_self: usize) -> i32 { DEDICATED.load(Ordering::Relaxed) }
pub unsafe fn CMtrlSystm2ppSys_Init(_self: usize) -> i32 { 1 }
pub unsafe fn CMtrlSystm2ppSys_InitWithoutMaterialSystem(_self: usize) -> i32 { 1 }
pub unsafe fn CMtrlSystm2ppSys_InitFinishSetupMaterialSystem(_self: usize) -> i32 { 1 }
pub unsafe fn CMtrlSystm2ppSys_GetContentPath(_self: usize) -> usize { global_GetGameRootFolder() }

// ---- the frame

/// One engine frame: true to keep running. The engine paces the loop; the
/// stub at 60 Hz, as a dedicated server's default tick would.
pub unsafe fn global_SourceEngineFrame(_app_system: usize, _time: f64, _previous: f64) -> i32 {
    std::thread::sleep(std::time::Duration::from_micros(16_667));
    1
}

// ---- images

/// Bytes a pixel takes in an ImageFormat (Source's numbering), or for a
/// block-compressed format the bytes a 4x4 block takes, negated.
fn format_bytes(format: i64) -> i32 {
    match format {
        0 | 1 | 11 | 12 | 15 | 41..=45 | 51 | 52 | 56 | 57 | 62 | 63..=67 | 80..=82 => 4, // the 8888s, BGRA1010102, D24S8s, D32
        2 | 3 | 46 | 47 => 3,
        4 | 16 | 17 | 18 | 20 | 49 | 53 | 59 | 60 | 83 => 2, // 565, 5551, 4444, 16-bit depth, RG1616's halves: no
        5 | 8 | 48 => 1,
        6 => 2,
        24 | 25 | 50 | 58 => 8,
        27 | 86 => 4,
        28 => 12,
        29 | 91 => 16,
        13 | 19 | 87 | 88 => -8, // DXT1, DXT1_ONEBITALPHA, ETC2 RGB, R11
        14 | 78 | 84 | 85 | 89 | 90 => -16, // DXT3, DXT5, BC6H, BC7, ETC2 RGBA, RG11
        _ => 4,
    }
}

fn level_bytes(w: i32, h: i32, d: i32, format: i64) -> i32 {
    let b = format_bytes(format);
    if b > 0 { w.max(1) * h.max(1) * d.max(1) * b } else { ((w.max(1) + 3) / 4) * ((h.max(1) + 3) / 4) * d.max(1) * -b }
}

fn mem_required(w: i32, h: i32, d: i32, mips: i32, format: i64) -> i32 {
    (0..mips.max(1)).map(|l| level_bytes(w >> l, h >> l, d >> l, format)).sum()
}

/// (width, height, depth, format, with every mip down to 1x1)
pub unsafe fn ImageLoader_GetMemRequired(w: i32, h: i32, d: i32, format: i64, mipmap: i32) -> i32 {
    let mips = if mipmap != 0 { 32 - (w.max(h).max(1) as u32).leading_zeros() as i32 } else { 1 };
    mem_required(w, h, d, mips, format)
}

/// (width, height, depth, mip levels, format)
pub unsafe fn ImageLoader_GetMemRequired_1(w: i32, h: i32, d: i32, mips: i32, format: i64) -> i32 {
    mem_required(w, h, d, mips, format)
}

// ---- sound: a device with nowhere to play, as a server's is

pub unsafe fn g_pAudioDevice_Name() -> usize { cstr("null") }
pub unsafe fn g_pAudioDevice_ChannelCount() -> u32 { 2 }
pub unsafe fn g_pAudioDevice_MixChannelCount() -> u32 { 2 }
pub unsafe fn g_pAudioDevice_BitsPerSample() -> u32 { 16 }
pub unsafe fn g_pAudioDevice_BytesPerSample() -> u32 { 2 }
pub unsafe fn g_pAudioDevice_SampleRate() -> u32 { 44_100 }

/// What the engine would show in a box: on a server, a line on stderr.
pub unsafe fn global_Plat_MessageBox(title: usize, text: usize) {
    let (t, x) = unsafe { (read(title), read(text)) };
    eprintln!("engine2: message box: {t}: {x}");
    crate::trace::note(&format!("Plat_MessageBox: {t}: {x}"));
}

/// The engine ends the process when it can't go on (a failed start
/// included): so does this.
pub unsafe fn global_Plat_ExitProcess(code: i32) {
    crate::trace::note(&format!("Plat_ExitProcess({code})"));
    std::process::exit(code);
}

// ---- rendering: the empty device, the one Facepunch's engine runs a
// server on (Graphics.Initialize skips the GPU's work for it)

const RENDER_DEVICE_API_EMPTY: i64 = 2;
pub unsafe fn g_pRenderDevice_GetRenderDeviceAPI() -> i64 { RENDER_DEVICE_API_EMPTY }

// ---- compiled resources: a game's JSON resources (scenes, prefabs, its own
// GameResource types) are read as `<name>_c`. Ours is the JSON itself, as
// tools/compile.sh writes it; Source 2's own container (a header and named
// blocks) is not read yet.

thread_local! {
    /// The last JSON handed back, kept until the next call on this thread
    /// (managed copies it into a string straight away).
    static LAST_JSON: std::cell::RefCell<CString> = std::cell::RefCell::new(CString::default());
}

pub unsafe fn EngineGlue_ReadCompiledResourceFileJson(data: usize, len: i32) -> usize {
    if data == 0 || len <= 0 { return 0; }
    let bytes = unsafe { core::slice::from_raw_parts(data as *const u8, len as usize) };
    let start = bytes.iter().position(|b| !b.is_ascii_whitespace()).unwrap_or(bytes.len());
    if !matches!(bytes.get(start), Some(b'{') | Some(b'[')) {
        crate::trace::note("ReadCompiledResourceFileJson: a Source 2 container, not plain JSON: not read yet");
        return 0;
    }
    let Ok(s) = CString::new(&bytes[start..]) else { return 0 };
    LAST_JSON.with(|l| {
        *l.borrow_mut() = s;
        l.borrow().as_ptr() as usize
    })
}

/// A named binary block inside a compiled resource: plain JSON has none.
pub unsafe fn EngineGlue_ReadCompiledResourceFileBlock(_name: usize, _data: usize, _len: i32, size: usize) -> usize {
    if size != 0 { unsafe { *(size as *mut i32) = 0 }; }
    0
}

/// A string returned to managed, kept until the next on this thread
/// (managed copies it straight away, as SafeReturnString's callers do).
fn returned(s: String) -> usize {
    thread_local! {
        static LAST: std::cell::RefCell<CString> = std::cell::RefCell::new(CString::default());
    }
    let s = CString::new(s.replace('\0', "")).unwrap_or_default();
    LAST.with(|l| {
        *l.borrow_mut() = s;
        l.borrow().as_ptr() as usize
    })
}

// ---- connections (Glue.Networking): net.rs, over our own transport

use crate::net::{self, Net};
use crate::slots::{
    S_Sandbox_Network_HSteamListenSocket as Socket, S_Sandbox_Network_HSteamNetConnection as Conn,
    S_Sandbox_Network_HSteamNetPollGroup as Group, S_Steamworks_Data_HAuthTicket as Ticket,
};

fn net() -> &'static Net {
    static NET: std::sync::OnceLock<Net> = std::sync::OnceLock::new();
    NET.get_or_init(|| Net::new(crate::steam::identity()))
}

/// SandboxNetwork.SteamNetwork's callbacks, as net.rs tells them.
struct SteamNetwork;

impl net::Managed for SteamNetwork {
    fn should_accept(&self, socket: u32, conn: u32) -> bool {
        let f = crate::managed(crate::slots::M_SandboxNetwork_SteamNetwork_ShouldAcceptConnection);
        if f == 0 { return false; }
        let f: extern "C" fn(Socket, Conn) -> i32 = unsafe { std::mem::transmute(f) };
        f(Socket { f_Id: socket }, Conn { f_Id: conn }) != 0
    }
    fn on_socket_connection(&self, socket: u32, conn: u32) {
        let f = crate::managed(crate::slots::M_SandboxNetwork_SteamNetwork_OnSocketConnection);
        if f == 0 { return; }
        let f: extern "C" fn(Socket, Conn) = unsafe { std::mem::transmute(f) };
        f(Socket { f_Id: socket }, Conn { f_Id: conn })
    }
    fn on_socket_disconnection(&self, socket: u32, conn: u32) {
        let f = crate::managed(crate::slots::M_SandboxNetwork_SteamNetwork_OnSocketDisconnection);
        if f == 0 { return; }
        let f: extern "C" fn(Socket, Conn) = unsafe { std::mem::transmute(f) };
        f(Socket { f_Id: socket }, Conn { f_Id: conn })
    }
    fn on_disconnection(&self, conn: u32, reason: i32, text: &str) {
        let f = crate::managed(crate::slots::M_SandboxNetwork_SteamNetwork_OnDisconnection);
        if f == 0 { return; }
        let f: extern "C" fn(Conn, i32, *const core::ffi::c_char) = unsafe { std::mem::transmute(f) };
        let text = CString::new(text.replace('\0', "")).unwrap_or_default();
        f(Conn { f_Id: conn }, reason, text.as_ptr())
    }
}

/// k_ESteamNetworkingAvailability_Current: nothing to wait for.
const AVAILABLE: i64 = 100;

pub unsafe fn Glue_Networking_RunCallbacks() { net().run_callbacks(&SteamNetwork) }
pub unsafe fn Glue_Networking_GetAuthenticationStatus(_debug: usize) -> i64 { AVAILABLE }
pub unsafe fn Glue_Networking_GetRelayNetworkStatus(_debug: usize) -> i64 { AVAILABLE }
pub unsafe fn Glue_Networking_CreateSocket(vport: i32) -> Socket { Socket { f_Id: net().listen_id(vport) } }
pub unsafe fn Glue_Networking_CreateIpBasedSocket(port: i32, _hide_address: i32) -> Socket {
    Socket { f_Id: net().listen_ip(port.clamp(0, 65535) as u16) }
}
pub unsafe fn Glue_Networking_CloseSocket(socket: Socket) { net().close_socket(socket.f_Id) }
pub unsafe fn Glue_Networking_GetSocketAddress(socket: Socket) -> usize { returned(net().socket_address(socket.f_Id)) }
pub unsafe fn Glue_Networking_GetIdentity() -> usize { returned(net().identity().to_string()) }

pub unsafe fn Glue_Networking_CreatePollGroup() -> Group { Group { f_Id: net().create_group() } }
pub unsafe fn Glue_Networking_DestroyPollGroup(group: Group) { net().destroy_group(group.f_Id) }
pub unsafe fn Glue_Networking_SetPollGroup(conn: Conn, group: Group) { net().set_group(conn.f_Id, group.f_Id) }

/// Messages into managed's array of pointers: how many.
unsafe fn hand_over(got: Vec<*mut net::Message>, out: usize) -> i32 {
    for (i, m) in got.iter().enumerate() {
        unsafe { *(out as *mut usize).add(i) = *m as usize };
    }
    got.len() as i32
}

pub unsafe fn Glue_Networking_GetPollGroupMessages(group: Group, out: usize, max: i32) -> i32 {
    if out == 0 || max <= 0 { return 0; }
    unsafe { hand_over(net().receive_group(group.f_Id, max as usize), out) }
}
pub unsafe fn Glue_Networking_GetConnectionMessages(conn: Conn, out: usize, max: i32) -> i32 {
    if out == 0 || max <= 0 { return 0; }
    unsafe { hand_over(net().receive(conn.f_Id, max as usize), out) }
}

/// A peer by identity alone: Steam's relay finds it. Ours is to come.
pub unsafe fn Glue_Networking_ConnectToSteamId(id: u64, vport: i32) -> Conn {
    crate::trace::note(&format!("net: no relay yet to reach {id}:{vport}"));
    Conn { f_Id: 0 }
}
pub unsafe fn Glue_Networking_ConnectToIpAddress(address: usize) -> Conn {
    Conn { f_Id: net().connect(&unsafe { read(address) }, 0) }
}
pub unsafe fn Glue_Networking_CloseConnection(conn: Conn, reason: i32, text: usize) {
    net().close_connection(conn.f_Id, reason, &unsafe { read(text) })
}

pub unsafe fn Glue_Networking_AllocateMessage(conn: Conn, size: i32, flags: i32) -> usize {
    net::message(conn.f_Id, flags, 0, vec![0; size.max(0) as usize]) as usize
}
pub unsafe fn Glue_Networking_GetMessageDataBuffer(message: usize) -> usize {
    if message == 0 { return 0; }
    unsafe { (*(message as *const net::Message)).data as usize }
}
pub unsafe fn Glue_Networking_SendMessage(conn: Conn, data: usize, len: i32, flags: i32) -> i64 {
    let bytes = if data == 0 || len <= 0 { &[][..] } else { unsafe { core::slice::from_raw_parts(data as *const u8, len as usize) } };
    net().send(conn.f_Id, flags, bytes)
}
/// Messages from AllocateMessage, sent and released: the send takes them.
pub unsafe fn Glue_Networking_SendMessages(messages: usize, count: i32) {
    if messages == 0 { return; }
    for i in 0..count.max(0) as usize {
        let m = unsafe { *(messages as *const *mut net::Message).add(i) };
        if m.is_null() { continue; }
        let (conn, flags) = unsafe { ((*m).conn, (*m).flags) };
        net().send(conn, flags, unsafe { net::message_bytes(m) });
        unsafe { net::release(m) };
    }
}

pub unsafe fn Glue_Networking_GetConnectionState(conn: Conn) -> i32 { net().state(conn.f_Id) }
pub unsafe fn Glue_Networking_GetConnectionDescription(conn: Conn) -> usize { returned(net().description(conn.f_Id)) }
pub unsafe fn Glue_Networking_GetConnectionSteamId(conn: Conn) -> u64 { net().peer(conn.f_Id) }

/// ISteamNetworkingSockets::ReleaseMessage: a received message, done with.
pub unsafe fn StmNtwrkngSckts_ReleaseMessage(_self: usize, message: usize) {
    unsafe { net::release(message as *mut net::Message) }
}

// ---- Steam's game server and user, and their callbacks: steam.rs

/// Steamworks.Dispatch.OnClientCallback, for each callback queued.
fn dispatch(server: bool) {
    let f = crate::managed(crate::slots::M_Steamworks_Dispatch_OnClientCallback);
    if f == 0 { return; }
    let f: extern "C" fn(i32, *const u8, i32, i32) = unsafe { std::mem::transmute(f) };
    crate::steam::run_callbacks(server, |kind, data, size, server| f(kind, data, size, server as i32));
}

/// The engine loop's: the server's callbacks, then connections'.
pub unsafe fn globalSteam_SteamGameServer_RunCallbacks() {
    dispatch(true);
    net().run_callbacks(&SteamNetwork);
}
pub unsafe fn globalSteam_SteamAPI_RunCallbacks() {
    dispatch(false);
    net().run_callbacks(&SteamNetwork);
}
pub unsafe fn globalSteam_SteamGameServer_GetSteamID() -> u64 { crate::steam::identity() }

pub unsafe fn ISteamGameServer_LogOnAnonymous(_self: usize) { crate::steam::log_on() }
pub unsafe fn ISteamGameServer_LogOn(_self: usize, _token: usize) { crate::steam::log_on() }
pub unsafe fn ISteamGameServer_LogOff(_self: usize) { crate::steam::log_off() }
pub unsafe fn ISteamGameServer_BLoggedOn(_self: usize) -> i32 { crate::steam::logged_on() as i32 }

/// A ticket for us into managed's buffer (1024 bytes, as the .def asks for).
unsafe fn auth_ticket(buffer: usize, length: usize) -> Ticket {
    let t = crate::steam::ticket();
    let n = t.len().min(1024);
    if buffer != 0 { unsafe { core::ptr::copy_nonoverlapping(t.as_ptr(), buffer as *mut u8, n) }; }
    if length != 0 { unsafe { *(length as *mut u32) = n as u32 }; }
    Ticket { f_Value: crate::steam::next_ticket_handle() }
}

unsafe fn begin_auth(player: u64, ticket: usize, len: i32, server: bool) -> i64 {
    let t = if ticket == 0 || len <= 0 { &[][..] } else { unsafe { core::slice::from_raw_parts(ticket as *const u8, len as usize) } };
    crate::steam::begin_auth(player, t, server)
}

pub unsafe fn ISteamGameServer_GetAuthSessionTicket(_self: usize, _target: u64, buffer: usize, length: usize) -> Ticket {
    unsafe { auth_ticket(buffer, length) }
}
pub unsafe fn ISteamGameServer_BeginAuthSession(_self: usize, player: u64, ticket: usize, len: i32) -> i64 {
    unsafe { begin_auth(player, ticket, len, true) }
}
pub unsafe fn ISteamGameServer_EndAuthSession(_self: usize, player: u64) { crate::steam::end_auth(player) }

pub unsafe fn ISteamUser_BLoggedOn(_self: usize) -> i32 { 1 }
pub unsafe fn ISteamUser_GetSteamID(_self: usize) -> u64 { crate::steam::identity() }
pub unsafe fn ISteamUser_GetAuthSessionTicket(_self: usize, _target: u64, buffer: usize, length: usize) -> Ticket {
    unsafe { auth_ticket(buffer, length) }
}
pub unsafe fn ISteamUser_BeginAuthSession(_self: usize, player: u64, ticket: usize, len: i32) -> i64 {
    unsafe { begin_auth(player, ticket, len, false) }
}
pub unsafe fn ISteamUser_EndAuthSession(_self: usize, player: u64) { crate::steam::end_auth(player) }

#[cfg(test)]
mod tests {
    //! The slots as managed calls them: a dedicated server logs on, listens,
    //! takes a player, checks their ticket, and they talk.
    use super::*;
    use std::sync::Mutex;

    static SEEN: Mutex<Vec<String>> = Mutex::new(Vec::new());
    fn seen(s: String) { SEEN.lock().unwrap().push(s) }

    extern "C" fn should_accept(_s: Socket, _c: Conn) -> i32 { 1 }
    extern "C" fn on_socket_connection(s: Socket, c: Conn) {
        seen(format!("connected {} {}", s.f_Id, c.f_Id));
    }
    extern "C" fn on_socket_disconnection(s: Socket, c: Conn) { seen(format!("gone {} {}", s.f_Id, c.f_Id)) }
    extern "C" fn on_disconnection(c: Conn, r: i32, _t: *const core::ffi::c_char) { seen(format!("ended {} {r}", c.f_Id)) }
    extern "C" fn on_client_callback(kind: i32, data: *const u8, size: i32, server: i32) {
        let d = unsafe { core::slice::from_raw_parts(data, size as usize) };
        if kind == crate::steam::VALIDATE_AUTH_TICKET_RESPONSE {
            seen(format!("auth {} {} {server}", u64::from_le_bytes(d[..8].try_into().unwrap()), i32::from_le_bytes(d[8..12].try_into().unwrap())));
        } else {
            seen(format!("callback {kind} {server}"));
        }
    }

    fn managed() {
        use crate::slots::*;
        let mut m = vec![0usize; crate::slots::MANAGED];
        m[M_SandboxNetwork_SteamNetwork_ShouldAcceptConnection] = should_accept as extern "C" fn(Socket, Conn) -> i32 as usize;
        m[M_SandboxNetwork_SteamNetwork_OnSocketConnection] = on_socket_connection as extern "C" fn(Socket, Conn) as usize;
        m[M_SandboxNetwork_SteamNetwork_OnSocketDisconnection] = on_socket_disconnection as extern "C" fn(Socket, Conn) as usize;
        m[M_SandboxNetwork_SteamNetwork_OnDisconnection] = on_disconnection as extern "C" fn(Conn, i32, *const core::ffi::c_char) as usize;
        m[M_Steamworks_Dispatch_OnClientCallback] = on_client_callback as extern "C" fn(i32, *const u8, i32, i32) as usize;
        let _ = crate::MANAGED.set(m);
    }

    /// The engine loop, until `done` sees what it waits for.
    fn frames(done: impl Fn(&[String]) -> bool) {
        let t = std::time::Instant::now();
        loop {
            unsafe { globalSteam_SteamGameServer_RunCallbacks() };
            if done(&SEEN.lock().unwrap()) { return; }
            assert!(t.elapsed().as_secs() < 5, "timed out: {:?}", SEEN.lock().unwrap());
            std::thread::sleep(std::time::Duration::from_millis(2));
        }
    }

    unsafe fn send(conn: Conn, text: &str) {
        let m = unsafe { Glue_Networking_AllocateMessage(conn, text.len() as i32, 8) };
        let buf = unsafe { Glue_Networking_GetMessageDataBuffer(m) };
        unsafe { core::ptr::copy_nonoverlapping(text.as_ptr(), buf as *mut u8, text.len()) };
        let batch = [m];
        unsafe { Glue_Networking_SendMessages(batch.as_ptr() as usize, 1) };
    }

    #[test]
    fn a_dedicated_server_takes_a_player() {
        managed();
        unsafe {
            // DedicatedServer.LogOnToSteamInternalAsync
            assert_eq!(ISteamGameServer_BLoggedOn(0), 0);
            ISteamGameServer_LogOnAnonymous(0);
            assert_eq!(ISteamGameServer_BLoggedOn(0), 1);
            frames(|s| s.contains(&format!("callback {} 1", crate::steam::STEAM_SERVERS_CONNECTED)));

            // the IpListenSocket and its poll group
            let socket = Glue_Networking_CreateIpBasedSocket(0, 0);
            assert_ne!(socket.f_Id, 0);
            let addr = read(Glue_Networking_GetSocketAddress(socket)).replace("0.0.0.0", "127.0.0.1");
            let group = Glue_Networking_CreatePollGroup();

            // a player connects (IpConnection); the server takes them
            let a = CString::new(addr).unwrap();
            let player = Glue_Networking_ConnectToIpAddress(a.as_ptr() as usize);
            frames(|s| s.iter().any(|l| l.starts_with(&format!("connected {} ", socket.f_Id))));
            let line = SEEN.lock().unwrap().iter().find(|l| l.starts_with("connected")).unwrap().clone();
            let at_server = Conn { f_Id: line.rsplit(' ').next().unwrap().parse().unwrap() };
            Glue_Networking_SetPollGroup(at_server, group);
            while Glue_Networking_GetConnectionState(player) != net::STATE_CONNECTED { std::thread::sleep(std::time::Duration::from_millis(2)) }

            // their ticket, as AuthenticatedConnection gets it and SocketConnection checks it
            let me = Glue_Networking_GetConnectionSteamId(at_server);
            assert_eq!(me, crate::steam::identity());
            let mut ticket = [0u8; 1024];
            let mut len = 0u32;
            ISteamUser_GetAuthSessionTicket(0, 0, ticket.as_mut_ptr() as usize, &mut len as *mut u32 as usize);
            assert!(len > 0);
            assert_eq!(ISteamGameServer_BeginAuthSession(0, me, ticket.as_ptr() as usize, len as i32), crate::steam::BEGIN_AUTH_OK);
            frames(|s| s.contains(&format!("auth {me} 0 1")));

            // they talk
            send(player, "hello");
            let mut out = [0usize; 8];
            let mut n = 0;
            while n == 0 { n = Glue_Networking_GetPollGroupMessages(group, out.as_mut_ptr() as usize, 8); }
            assert_eq!(n, 1);
            assert_eq!(net::message_bytes(out[0] as *mut net::Message), b"hello");
            StmNtwrkngSckts_ReleaseMessage(0, out[0]);

            assert_eq!(Glue_Networking_SendMessage(at_server, b"welcome".as_ptr() as usize, 7, 8), 1);
            let mut n = 0;
            while n == 0 { n = Glue_Networking_GetConnectionMessages(player, out.as_mut_ptr() as usize, 8); }
            assert_eq!(net::message_bytes(out[0] as *mut net::Message), b"welcome");
            StmNtwrkngSckts_ReleaseMessage(0, out[0]);

            // the server kicks them: the player hears it
            let why = CString::new("kicked").unwrap();
            Glue_Networking_CloseConnection(at_server, 5000, why.as_ptr() as usize);
            frames(|s| s.contains(&format!("ended {} 5000", player.f_Id)));
            assert_eq!(Glue_Networking_GetConnectionState(player), net::STATE_NONE);
            ISteamGameServer_EndAuthSession(0, me);
            Glue_Networking_DestroyPollGroup(group);
            Glue_Networking_CloseSocket(socket);
        }
    }
}
