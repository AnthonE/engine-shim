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
    crate::elo::TICK.fetch_add(1, Ordering::Relaxed);
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

// ---- LZ4: what s&box compresses its packets and saved data with, in the C
// library's block and frame formats (lz4_flex). HC compresses as the fast
// mode does: valid LZ4, a little larger.

unsafe fn bytes<'a>(p: usize, n: i32) -> &'a [u8] {
    if p == 0 || n <= 0 { &[] } else { unsafe { core::slice::from_raw_parts(p as *const u8, n as usize) } }
}
unsafe fn bytes_mut<'a>(p: usize, n: i32) -> &'a mut [u8] {
    if p == 0 || n <= 0 { &mut [] } else { unsafe { core::slice::from_raw_parts_mut(p as *mut u8, n as usize) } }
}

pub unsafe fn LZ4Glue_CompressBound(n: i32) -> i32 { lz4_flex::block::get_maximum_output_size(n.max(0) as usize) as i32 }
/// The compressed size, or 0: LZ4's own failure.
pub unsafe fn LZ4Glue_Compress(src: usize, dst: usize, n: i32, cap: i32) -> i32 {
    lz4_flex::block::compress_into(unsafe { bytes(src, n) }, unsafe { bytes_mut(dst, cap) }).map_or(0, |n| n as i32)
}
pub unsafe fn LZ4Glue_CompressHC(src: usize, dst: usize, n: i32, cap: i32, _level: i32) -> i32 {
    unsafe { LZ4Glue_Compress(src, dst, n, cap) }
}
/// The decompressed size, or negative.
pub unsafe fn LZ4Glue_Decompress(src: usize, dst: usize, n: i32, cap: i32) -> i32 {
    lz4_flex::block::decompress_into(unsafe { bytes(src, n) }, unsafe { bytes_mut(dst, cap) }).map_or(-1, |n| n as i32)
}

/// The most a frame of `n` bytes can take: a header, a size per 64 KiB
/// block, the blocks (stored, at worst), the end mark.
pub unsafe fn LZ4Glue_CompressFrameBound(n: i32) -> i32 {
    let n = n.max(0) as usize;
    (19 + 4 * (n / 65_536 + 1) + lz4_flex::block::get_maximum_output_size(n) + 4) as i32
}
/// A frame carrying its content size (DecompressFrame reads it back), or 0.
pub unsafe fn LZ4Glue_CompressFrame(src: usize, dst: usize, n: i32, cap: i32, _level: i32) -> i32 {
    use lz4_flex::frame::{BlockSize, FrameEncoder, FrameInfo};
    use std::io::Write;
    let input = unsafe { bytes(src, n) };
    let info = FrameInfo::new().block_size(BlockSize::Max64KB).content_size(Some(input.len() as u64));
    let mut e = FrameEncoder::with_frame_info(info, Vec::with_capacity(input.len() / 2 + 64));
    let Some(frame) = e.write_all(input).ok().and_then(|_| e.finish().ok()) else { return 0 };
    let out = unsafe { bytes_mut(dst, cap) };
    if frame.len() > out.len() { return 0; }
    out[..frame.len()].copy_from_slice(&frame);
    frame.len() as i32
}
/// The size a frame's header says it holds: 0 if it doesn't say, -1 if
/// it's not a frame.
pub unsafe fn LZ4Glue_GetFrameContentSize(src: usize, n: i32) -> i64 {
    let b = unsafe { bytes(src, n) };
    if b.len() < 7 || b[..4] != 0x184D_2204u32.to_le_bytes() { return -1; }
    if b[4] & 0x08 == 0 { return 0; }
    b.get(6..14).map_or(-1, |c| u64::from_le_bytes(c.try_into().unwrap()) as i64)
}
/// The decompressed size, or negative (not a frame, or more than `cap`).
pub unsafe fn LZ4Glue_DecompressFrame(src: usize, dst: usize, n: i32, cap: i32) -> i32 {
    use std::io::Read;
    let mut d = lz4_flex::frame::FrameDecoder::new(unsafe { bytes(src, n) });
    let out = unsafe { bytes_mut(dst, cap) };
    let mut at = 0;
    loop {
        match d.read(&mut out[at..]) {
            Ok(0) if at < out.len() => return at as i32,
            Ok(0) => {
                // full: anything left over is too much
                let mut more = [0u8; 1];
                return if matches!(d.read(&mut more), Ok(0)) { at as i32 } else { -1 };
            }
            Ok(k) => at += k,
            Err(_) => return -1,
        }
    }
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
/// A socket players reach by our identity: through the relay at
/// `ENGINE2_RELAY`, proven with our host key (`ENGINE2_HOST_KEY`). Without
/// both, it waits for connections nothing can bring.
pub unsafe fn Glue_Networking_CreateSocket(vport: i32) -> Socket {
    let relay = std::env::var("ENGINE2_RELAY").ok().filter(|r| !r.is_empty());
    let f_Id = match (relay, crate::steam::host_key()) {
        (Some(relay), Some(key)) if crate::siwe::id_of(&crate::siwe::address_of_key(key)) == net().identity() => net().listen_relay(vport, &relay, key.clone()),
        (Some(_), _) => {
            crate::trace::note("net: ENGINE2_RELAY needs our identity to be our host key's (ENGINE2_HOST_KEY, and no ENGINE2_IDENTITY)");
            net().listen_id(vport)
        }
        _ => net().listen_id(vport),
    };
    Socket { f_Id }
}
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

/// A peer by identity alone, through the relay at `ENGINE2_RELAY`.
pub unsafe fn Glue_Networking_ConnectToSteamId(id: u64, vport: i32) -> Conn {
    match std::env::var("ENGINE2_RELAY").ok().filter(|r| !r.is_empty()) {
        Some(relay) => Conn { f_Id: net().connect_via(&relay, id, vport) },
        None => {
            crate::trace::note(&format!("net: no ENGINE2_RELAY to reach {id}:{vport} through"));
            Conn { f_Id: 0 }
        }
    }
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

/// Logged on, a server is open for business: its status answers, and on a
/// title that sells copies, the roster is swept (elo.rs).
fn open_for_business() {
    crate::steam::log_on();
    crate::elo::announce();
    crate::elo::serve_status(|| net().players(), crate::steam::online);
    crate::elo::sweep(crate::steam::roster, |player| net().kick(player, KICK_NO_COPY, "you no longer hold a copy of this game"));
}

/// k_ESteamNetConnectionEnd_App_Min: an end the application names.
const KICK_NO_COPY: i32 = 1000;

pub unsafe fn ISteamGameServer_LogOnAnonymous(_self: usize) { open_for_business() }
pub unsafe fn ISteamGameServer_LogOn(_self: usize, _token: usize) { open_for_business() }

// what the server list shows: the Elo Pros shard row (elo.rs)
pub unsafe fn ISteamGameServer_SetServerName(_self: usize, name: usize) {
    let name = unsafe { read(name) };
    crate::elo::describe(|s| s.name = name)
}
pub unsafe fn ISteamGameServer_SetMapName(_self: usize, map: usize) {
    let map = unsafe { read(map) };
    crate::elo::describe(|s| s.map = map)
}
pub unsafe fn ISteamGameServer_SetMaxPlayerCount(_self: usize, max: i32) { crate::elo::describe(|s| s.max_players = max) }
pub unsafe fn ISteamGameServer_LogOff(_self: usize) { crate::steam::log_off() }
pub unsafe fn ISteamGameServer_BLoggedOn(_self: usize) -> i32 { crate::steam::logged_on() as i32 }

/// A ticket for us into managed's buffer (1024 bytes, as the .def asks for):
/// a sign-in for the server we joined, over the nonce it sent.
unsafe fn auth_ticket(buffer: usize, length: usize) -> Ticket {
    let t = crate::steam::ticket(net().challenge());
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

// ---- physics: physics.rs, Rapier where s&box's native engine has Box3D.
// Worlds, bodies and shapes are handle objects of their own (made here,
// registered with managed, freed when gone); a pointer is one of them.

use crate::physics::{self as phys, Geometry};
use crate::slots::{S_BBox, S_Capsule, S_PhysicsTrace_Result, S_Rotation, S_Sandbox_Sphere, S_Sandbox_StringToken, S_Transform, S_Vector3};

fn freed(gone: Vec<usize>) {
    for p in gone { crate::handles::free(p); }
}

fn b(v: i32) -> bool { v != 0 }

pub unsafe fn g_pPhysicsSystem_NumWorlds() -> i32 { phys::world_count() }

pub unsafe fn g_pPhysicsSystem_CreateWorld() -> i32 {
    let w = crate::handles::object();
    phys::create_world(w);
    crate::handles::register(w, "PhysicsWorld", &[])
}

pub unsafe fn g_pPhysicsSystem_DestroyWorld(world: usize) { freed(phys::destroy_world(world)) }

// the world

fn add_body(world: usize, s: &phys::BodySettings) -> i32 {
    let body = crate::handles::object();
    phys::add_body(world, body, s);
    crate::handles::register(body, "PhysicsBody", &[world])
}

pub unsafe fn IPhysicsWorld_AddBody(world: usize) -> i32 { add_body(world, &phys::BodySettings::default()) }
pub unsafe fn IPhysicsWorld_AddBody_1(world: usize, settings: usize) -> i32 {
    add_body(world, &unsafe { phys::read::<phys::BodySettings>(settings) })
}
pub unsafe fn IPhysicsWorld_RemoveBody(_world: usize, body: usize) { freed(phys::remove_body(body)) }
pub unsafe fn IPhysicsWorld_GetWorldReferenceBody(world: usize) -> i32 { crate::handles::of(phys::reference_body(world)) }
pub unsafe fn IPhysicsWorld_SetWorldReferenceBody(world: usize, body: usize) { phys::set_reference_body(world, body) }
pub unsafe fn IPhysicsWorld_SetGravity(world: usize, g: usize) { phys::set_gravity(world, unsafe { phys::read_vec(g) }) }
pub unsafe fn IPhysicsWorld_GetGravity(world: usize) -> S_Vector3 { phys::v3(phys::gravity(world)) }
pub unsafe fn IPhysicsWorld_SetSimulation(world: usize, mode: i64) { phys::set_simulation(world, mode) }
pub unsafe fn IPhysicsWorld_GetSimulation(world: usize) -> i64 { phys::simulation(world) }
pub unsafe fn IPhysicsWorld_EnableSleeping(world: usize) { phys::set_sleeping(world, true) }
pub unsafe fn IPhysicsWorld_DisableSleeping(world: usize) { phys::set_sleeping(world, false) }
pub unsafe fn IPhysicsWorld_IsSleepingEnabled(world: usize) -> i32 { phys::sleeping(world) as i32 }
pub unsafe fn IPhysicsWorld_SetMaximumLinearSpeed(world: usize, speed: f32) { phys::set_max_speed(world, speed) }
pub unsafe fn IPhysicsWorld_SetCollisionRulesFromJson(world: usize, json: usize) { phys::set_rules(world, &unsafe { read(json) }) }
pub unsafe fn IPhysicsWorld_ManagedObject(world: usize) -> i32 { crate::handles::of(world) }

/// One step, then each body that moved is told to managed
/// (PhysicsEngine.OnActive): where it is, how fast, whether it fell asleep;
/// and each joint that broke (PhysicsEngine.OnPhysicsJointBreak).
pub unsafe fn IPhysicsWorld_StepSimulation(world: usize, dt: f32, steps: i32) {
    let stepped = phys::step(world, dt, steps);
    let f = crate::managed(crate::slots::M_SandboxPhysics_PhysicsEngine_OnActive);
    if f != 0 {
        let on_active: extern "C" fn(i32, S_Transform, S_Vector3, S_Vector3, i32, i32) = unsafe { std::mem::transmute(f) };
        for a in stepped.active {
            let h = crate::handles::of(a.body);
            if h > 0 { on_active(h, a.transform, phys::v3(a.velocity), phys::v3(a.angular_velocity), a.fell_asleep as i32, 0); }
        }
    }
    let f = crate::managed(crate::slots::M_SandboxPhysics_PhysicsEngine_OnPhysicsJointBreak);
    if f != 0 {
        let on_break: extern "C" fn(i32) = unsafe { std::mem::transmute(f) };
        for j in stepped.broken {
            let h = crate::handles::of(j);
            if h > 0 { on_break(h); }
        }
    }
}

/// What touched what since last asked, handed to managed's delegate.
pub unsafe fn IPhysicsWorld_ProcessIntersections(world: usize, f: usize) {
    let events = phys::take_intersections(world);
    if f == 0 || events.is_empty() { return; }
    let deliver: extern "C" fn(*const phys::Intersection, i32) = unsafe { std::mem::transmute(f) };
    deliver(events.as_ptr(), events.len() as i32);
}

// queries: the shapes in a sphere, a box, the hull of some points

static QUERIES: Mutex<Vec<(usize, Vec<usize>)>> = Mutex::new(Vec::new());

fn query_into(result: usize, found: Vec<usize>) {
    let mut q = QUERIES.lock().unwrap();
    match q.iter_mut().find(|(p, _)| *p == result) {
        Some((_, v)) => v.extend(found),
        None => q.push((result, found)),
    }
}

/// Managed's sets to include (1 static, 2 dynamic, 4 keyframed) as the ones a trace leaves out.
fn left_out(sets: u16) -> u16 { !sets & 7 }

pub unsafe fn CQueryResult_Create() -> usize {
    let p = crate::handles::object();
    QUERIES.lock().unwrap().push((p, Vec::new()));
    p
}
pub unsafe fn CQueryResult_DeleteThis(result: usize) { QUERIES.lock().unwrap().retain(|(p, _)| *p != result) }
pub unsafe fn CQueryResult_RemoveAll(result: usize) {
    if let Some((_, v)) = QUERIES.lock().unwrap().iter_mut().find(|(p, _)| *p == result) { v.clear(); }
}
pub unsafe fn CQueryResult_Count(result: usize) -> i32 {
    QUERIES.lock().unwrap().iter().find(|(p, _)| *p == result).map(|(_, v)| v.len() as i32).unwrap_or(0)
}
pub unsafe fn CQueryResult_Element(result: usize, i: i32) -> i32 {
    let shape = QUERIES.lock().unwrap().iter().find(|(p, _)| *p == result).and_then(|(_, v)| v.get(i as usize).copied()).unwrap_or(0);
    crate::handles::of(shape)
}

pub unsafe fn IPhysicsWorld_Query(world: usize, result: usize, center: usize, radius: f32, sets: u16) {
    let at = rapier3d::math::Pose::from_translation(unsafe { phys::read_vec(center) });
    query_into(result, phys::query(world, rapier3d::parry::shape::SharedShape::ball(radius.max(0.0)), at, left_out(sets)));
}
pub unsafe fn IPhysicsWorld_Query_1(world: usize, result: usize, bounds: usize, sets: u16) {
    let Some(b) = (bounds != 0).then(|| unsafe { core::ptr::read_unaligned(bounds as *const S_BBox) }) else { return };
    let (mins, maxs) = (phys::vec(b.f_Mins), phys::vec(b.f_Maxs));
    let half = ((maxs - mins) * 0.5).abs().max(rapier3d::math::Vector::splat(1e-3));
    let at = rapier3d::math::Pose::from_translation((mins + maxs) * 0.5);
    query_into(result, phys::query(world, rapier3d::parry::shape::SharedShape::cuboid(half.x, half.y, half.z), at, left_out(sets)));
}
pub unsafe fn IPhysicsWorld_Query_2(world: usize, result: usize, points: usize, n: i32, sets: u16) {
    let Some(hull) = rapier3d::parry::shape::SharedShape::convex_hull(&unsafe { phys::read_points(points, n) }) else { return };
    query_into(result, phys::query(world, hull, rapier3d::math::Pose::IDENTITY, left_out(sets)));
}

// bodies

pub unsafe fn IPhysicsBody_SetGravityScale(body: usize, s: f32) { phys::set_gravity_scale(body, s) }
pub unsafe fn IPhysicsBody_GetGravityScale(body: usize) -> f32 { phys::with_body(body, |b, _| b.gravity_scale).unwrap_or(1.0) }
pub unsafe fn IPhysicsBody_IsGravityEnabled(body: usize) -> i32 { phys::with_body(body, |b, _| b.gravity).unwrap_or(true) as i32 }
pub unsafe fn IPhysicsBody_EnableGravity(body: usize, on: i32) { phys::set_gravity_enabled(body, b(on)) }
pub unsafe fn IPhysicsBody_SetMass(body: usize, m: f32) { phys::set_mass(body, (m > 0.0).then_some(m)) }
pub unsafe fn IPhysicsBody_GetMass(body: usize) -> f32 { phys::with_body(body, |_, rb| rb.mass()).unwrap_or(0.0) }
pub unsafe fn IPhysicsBody_GetMassCenter(body: usize) -> S_Vector3 { phys::v3(phys::with_body(body, |_, rb| rb.center_of_mass()).unwrap_or_default()) }
pub unsafe fn IPhysicsBody_GetLocalMassCenter(body: usize) -> S_Vector3 { phys::v3(phys::with_body(body, |_, rb| rb.local_center_of_mass()).unwrap_or_default()) }
pub unsafe fn IPhysicsBody_SetLocalMassCenter(body: usize, c: usize) { phys::set_local_mass_center(body, unsafe { phys::read_vec(c) }) }
pub unsafe fn IPhysicsBody_SetOverrideMassCenter(body: usize, on: i32) { phys::set_override_mass_center(body, b(on)) }
pub unsafe fn IPhysicsBody_GetOverrideMassCenter(body: usize) -> i32 { phys::with_body(body, |b, _| b.mass_center.is_some()).unwrap_or(false) as i32 }
pub unsafe fn IPhysicsBody_SetPosition(body: usize, v: usize) { phys::set_transform(body, Some(unsafe { phys::read_vec(v) }), None) }
pub unsafe fn IPhysicsBody_GetPosition(body: usize) -> S_Vector3 { phys::v3(phys::with_body(body, |_, rb| rb.translation()).unwrap_or_default()) }
pub unsafe fn IPhysicsBody_SetOrientation(body: usize, q: usize) { phys::set_transform(body, None, Some(unsafe { phys::read_rot(q) })) }
pub unsafe fn IPhysicsBody_GetOrientation(body: usize) -> S_Rotation { phys::rot(phys::with_body(body, |_, rb| *rb.rotation()).unwrap_or_default()) }
pub unsafe fn IPhysicsBody_SetTransform(body: usize, v: usize, q: usize) {
    phys::set_transform(body, Some(unsafe { phys::read_vec(v) }), Some(unsafe { phys::read_rot(q) }))
}
pub unsafe fn IPhysicsBody_GetTransform(body: usize) -> S_Transform {
    let p = phys::with_body(body, |_, rb| *rb.position()).unwrap_or(rapier3d::math::Pose::IDENTITY);
    S_Transform { f_Position: phys::v3(p.translation), f_Scale: phys::v3(rapier3d::math::Vector::ONE), f_Rotation: phys::rot(p.rotation) }
}
pub unsafe fn IPhysicsBody_SetLinearVelocity(body: usize, v: usize) {
    let v = unsafe { phys::read_vec(v) };
    phys::with_body_mut(body, |_, rb| rb.set_linvel(v, true));
}
pub unsafe fn IPhysicsBody_GetLinearVelocity(body: usize) -> S_Vector3 { phys::v3(phys::with_body(body, |_, rb| rb.linvel()).unwrap_or_default()) }
pub unsafe fn IPhysicsBody_GetVelocityAtPoint(body: usize, p: usize) -> S_Vector3 {
    let p = unsafe { phys::read_vec(p) };
    phys::v3(phys::with_body(body, |_, rb| rb.velocity_at_point(p)).unwrap_or_default())
}
pub unsafe fn IPhysicsBody_AddLinearVelocity(body: usize, v: usize) {
    let v = unsafe { phys::read_vec(v) };
    phys::with_body_mut(body, |_, rb| rb.set_linvel(rb.linvel() + v, true));
}
pub unsafe fn IPhysicsBody_SetAngularVelocity(body: usize, v: usize) {
    let v = unsafe { phys::read_vec(v) };
    phys::with_body_mut(body, |_, rb| rb.set_angvel(v, true));
}
pub unsafe fn IPhysicsBody_GetAngularVelocity(body: usize) -> S_Vector3 { phys::v3(phys::with_body(body, |_, rb| rb.angvel()).unwrap_or_default()) }
pub unsafe fn IPhysicsBody_Wake(body: usize) { phys::set_sleeping_body(body, false) }
pub unsafe fn IPhysicsBody_Sleep(body: usize) { phys::set_sleeping_body(body, true) }
pub unsafe fn IPhysicsBody_IsSleeping(body: usize) -> i32 { phys::with_body(body, |_, rb| rb.is_sleeping()).unwrap_or(false) as i32 }
pub unsafe fn IPhysicsBody_EnableAutoSleeping(body: usize) { phys::set_auto_sleep(body, true) }
pub unsafe fn IPhysicsBody_DisableAutoSleeping(body: usize) { phys::set_auto_sleep(body, false) }
pub unsafe fn IPhysicsBody_SetSleepThreshold(body: usize, t: f32) { phys::set_sleep_threshold(body, t) }
pub unsafe fn IPhysicsBody_GetSleepThreshold(body: usize) -> f32 { phys::with_body(body, |b, _| b.sleep_threshold).unwrap_or(0.0) }
pub unsafe fn IPhysicsBody_EnableTouchEvents(body: usize) { phys::set_touch_events(body, true) }
pub unsafe fn IPhysicsBody_DisableTouchEvents(body: usize) { phys::set_touch_events(body, false) }
pub unsafe fn IPhysicsBody_IsTouchEventEnabled(body: usize) -> i32 { phys::with_body(body, |b, _| b.touch_events).unwrap_or(false) as i32 }
pub unsafe fn IPhysicsBody_GetType(body: usize) -> i64 { phys::body_kind(body) }
pub unsafe fn IPhysicsBody_SetType(body: usize, kind: i64) { phys::set_body_kind(body, kind) }
pub unsafe fn IPhysicsBody_GetShapeCount(body: usize) -> i32 { phys::shapes_of_body(body).len() as i32 }
pub unsafe fn IPhysicsBody_GetShape(body: usize, i: i32) -> i32 {
    crate::handles::of(phys::shapes_of_body(body).get(i as usize).copied().unwrap_or(0))
}

/// A new shape on `body`: its handle, or 0 if it can't be made.
fn add_shape(body: usize, g: Geometry, settings: usize) -> i32 {
    let s = unsafe { phys::read::<phys::ShapeSettings>(settings) };
    let shape = crate::handles::object();
    if !phys::add_shape(body, shape, &g, &s) { return 0; }
    crate::handles::register(shape, "PhysicsShape", &[body, phys::world_of_body(body)])
}

fn pose_of(position: usize, rotation: usize) -> rapier3d::math::Pose {
    rapier3d::math::Pose::from_parts(unsafe { phys::read_vec(position) }, unsafe { phys::read_rot(rotation) })
}

/// `n` indices as triangles.
unsafe fn read_triangles(p: usize, n: i32) -> Vec<[u32; 3]> {
    if p == 0 || n < 3 { return Vec::new(); }
    let s = unsafe { core::slice::from_raw_parts(p as *const u32, n as usize) };
    s.chunks_exact(3).map(|t| [t[0], t[1], t[2]]).collect()
}

pub unsafe fn IPhysicsBody_AddSphereShape(body: usize, center: usize, radius: f32, settings: usize) -> i32 {
    add_shape(body, Geometry::Sphere(unsafe { phys::read_vec(center) }, radius), settings)
}
pub unsafe fn IPhysicsBody_AddCapsuleShape(body: usize, a: usize, b: usize, radius: f32, settings: usize) -> i32 {
    add_shape(body, Geometry::Capsule(unsafe { phys::read_vec(a) }, unsafe { phys::read_vec(b) }, radius), settings)
}
pub unsafe fn IPhysicsBody_AddBoxShape(body: usize, position: usize, rotation: usize, extent: usize, settings: usize) -> i32 {
    add_shape(body, Geometry::Box(pose_of(position, rotation), unsafe { phys::read_vec(extent) }), settings)
}
pub unsafe fn IPhysicsBody_AddHullShape(body: usize, position: usize, rotation: usize, n: i32, points: usize, settings: usize) -> i32 {
    add_shape(body, Geometry::Hull(pose_of(position, rotation), unsafe { phys::read_points(points, n) }), settings)
}
pub unsafe fn IPhysicsBody_AddMeshShape(body: usize, nv: i32, vertices: usize, ni: i32, indices: usize, _materials: i32, settings: usize) -> i32 {
    let g = Geometry::Mesh(rapier3d::math::Pose::IDENTITY, unsafe { phys::read_points(vertices, nv) }, unsafe { read_triangles(indices, ni) });
    add_shape(body, g, settings)
}
pub unsafe fn IPhysicsBody_AddHeightFieldShape(body: usize, heights: usize, _materials: usize, x: i32, y: i32, spacing: f32, scale: f32, _n: i32, settings: usize) -> i32 {
    if heights == 0 || x < 2 || y < 2 { return 0; }
    let h = unsafe { core::slice::from_raw_parts(heights as *const u16, (x * y) as usize) }.to_vec();
    add_shape(body, Geometry::Height(h, x as usize, y as usize, spacing, scale), settings)
}
pub unsafe fn IPhysicsBody_RemoveShape(_body: usize, shape: usize) {
    if phys::remove_shape(shape) { crate::handles::free(shape); }
}
pub unsafe fn IPhysicsBody_PurgeShapes(body: usize) {
    for s in phys::shapes_of_body(body) {
        if phys::remove_shape(s) { crate::handles::free(s); }
    }
}
pub unsafe fn IPhysicsBody_ApplyLinearImpulse(body: usize, i: usize) {
    let i = unsafe { phys::read_vec(i) };
    phys::with_body_mut(body, |_, rb| rb.apply_impulse(i, true));
}
pub unsafe fn IPhysicsBody_ApplyLinearImpulseAtWorldSpace(body: usize, i: usize, at: usize) {
    let (i, at) = (unsafe { phys::read_vec(i) }, unsafe { phys::read_vec(at) });
    phys::with_body_mut(body, |_, rb| rb.apply_impulse_at_point(i, at, true));
}
pub unsafe fn IPhysicsBody_ApplyAngularImpulse(body: usize, i: usize) {
    let i = unsafe { phys::read_vec(i) };
    phys::with_body_mut(body, |_, rb| rb.apply_torque_impulse(i, true));
}
pub unsafe fn IPhysicsBody_ApplyForce(body: usize, f: usize) {
    let f = unsafe { phys::read_vec(f) };
    phys::with_body_mut(body, |_, rb| rb.add_force(f, true));
}
pub unsafe fn IPhysicsBody_ApplyForceAt(body: usize, f: usize, at: usize) {
    let (f, at) = (unsafe { phys::read_vec(f) }, unsafe { phys::read_vec(at) });
    phys::with_body_mut(body, |_, rb| rb.add_force_at_point(f, at, true));
}
pub unsafe fn IPhysicsBody_ApplyTorque(body: usize, t: usize) {
    let t = unsafe { phys::read_vec(t) };
    phys::with_body_mut(body, |_, rb| rb.add_torque(t, true));
}
pub unsafe fn IPhysicsBody_ClearForces(body: usize) { phys::with_body_mut(body, |_, rb| rb.reset_forces(false)); }
pub unsafe fn IPhysicsBody_ClearTorque(body: usize) { phys::with_body_mut(body, |_, rb| rb.reset_torques(false)); }
pub unsafe fn IPhysicsBody_Enable(body: usize) { phys::with_body_mut(body, |_, rb| rb.set_enabled(true)); }
pub unsafe fn IPhysicsBody_Disable(body: usize) { phys::with_body_mut(body, |_, rb| rb.set_enabled(false)); }
pub unsafe fn IPhysicsBody_IsEnabled(body: usize) -> i32 { phys::with_body(body, |_, rb| rb.is_enabled()).unwrap_or(false) as i32 }
pub unsafe fn IPhysicsBody_BuildMass(body: usize) { phys::build_mass(body) }
pub unsafe fn IPhysicsBody_SetMassUpdatesDeferred(body: usize, defer: i32) -> i32 { phys::defer_mass(body, b(defer)) as i32 }
pub unsafe fn IPhysicsBody_SetLinearDamping(body: usize, d: f32) { phys::with_body_mut(body, |_, rb| rb.set_linear_damping(d)); }
pub unsafe fn IPhysicsBody_GetLinearDamping(body: usize) -> f32 { phys::with_body(body, |_, rb| rb.linear_damping()).unwrap_or(0.0) }
pub unsafe fn IPhysicsBody_SetAngularDamping(body: usize, d: f32) { phys::with_body_mut(body, |_, rb| rb.set_angular_damping(d)); }
pub unsafe fn IPhysicsBody_GetAngularDamping(body: usize) -> f32 { phys::with_body(body, |_, rb| rb.angular_damping()).unwrap_or(0.0) }
pub unsafe fn IPhysicsBody_BuildBounds(body: usize) -> S_BBox { phys::body_bounds(body) }
/// Every shape weighs as water does, kilograms per cubic metre.
pub unsafe fn IPhysicsBody_GetDensity(_body: usize) -> f32 { 1000.0 }
pub unsafe fn IPhysicsBody_GetClosestPoint(body: usize, p: usize) -> S_Vector3 { phys::v3(phys::closest_point(body, unsafe { phys::read_vec(p) })) }
pub unsafe fn IPhysicsBody_SetMaterialIndex(_body: usize, _name: S_Sandbox_StringToken) {}
pub unsafe fn IPhysicsBody_SetTargetTransform(body: usize, position: usize, rotation: usize, dt: f32) {
    phys::set_target(body, pose_of(position, rotation), dt)
}
pub unsafe fn IPhysicsBody_CheckOverlap(body: usize, other: usize, at: usize) -> i32 {
    phys::penetration(body, other, unsafe { phys::read_pose(at) }).is_some() as i32
}
pub unsafe fn IPhysicsBody_ComputePenetration(body: usize, other: usize, at: usize, direction: usize, distance: usize) -> i32 {
    let Some((d, depth)) = phys::penetration(body, other, unsafe { phys::read_pose(at) }) else { return 0 };
    if direction != 0 { unsafe { core::ptr::write_unaligned(direction as *mut S_Vector3, phys::v3(d)) }; }
    if distance != 0 { unsafe { core::ptr::write_unaligned(distance as *mut f32, depth) }; }
    1
}
pub unsafe fn IPhysicsBody_GetLocalInertiaVector(body: usize) -> S_Vector3 { phys::v3(phys::principal_inertia(body).0) }
pub unsafe fn IPhysicsBody_GetLocalInertiaOrientation(body: usize) -> S_Rotation { phys::rot(phys::principal_inertia(body).1) }
pub unsafe fn IPhysicsBody_SetLocalInertia(body: usize, v: usize, q: usize) {
    phys::set_inertia(body, Some((unsafe { phys::read_vec(v) }, unsafe { phys::read_rot(q) })))
}
pub unsafe fn IPhysicsBody_ResetLocalInertia(body: usize) { phys::set_inertia(body, None) }
pub unsafe fn IPhysicsBody_ScaleInertia(body: usize, s: f32) { phys::scale_inertia(body, s) }
pub unsafe fn IPhysicsBody_ManagedObject(body: usize) -> i32 { crate::handles::of(body) }
pub unsafe fn IPhysicsBody_SetMotionLocks(body: usize, x: i32, y: i32, z: i32, pitch: i32, yaw: i32, roll: i32) {
    let locks = [x, y, z, pitch, yaw, roll].iter().enumerate().fold(0u32, |m, (i, &on)| if on != 0 { m | 1 << i } else { m });
    phys::set_motion_locks(body, locks)
}
pub unsafe fn IPhysicsBody_IsTouching(body: usize, other: usize, triggers: i32) -> i32 { phys::touching_body(body, other, b(triggers)) as i32 }
pub unsafe fn IPhysicsBody_IsTouching_1(body: usize, shape: usize, triggers: i32) -> i32 { phys::touching_shape(body, shape, b(triggers)) as i32 }
pub unsafe fn IPhysicsBody_SetBullet(body: usize, on: i32) { phys::with_body_mut(body, |_, rb| rb.enable_ccd(on != 0)); }

// shapes

pub unsafe fn IPhysicsShape_AddCollisionFunctionMask(shape: usize, m: u8) { phys::set_mask(shape, |was| was | m) }
pub unsafe fn IPhysicsShape_RemoveCollisionFunctionMask(shape: usize, m: u8) { phys::set_mask(shape, |was| was & !m) }
pub unsafe fn IPhysicsShape_GetCollisionFunctionMask(shape: usize) -> u8 { phys::with_shape(shape, |s, _| s.mask).unwrap_or(0) }
pub unsafe fn IPhysicsShape_HasTag(shape: usize, tag: u32) -> i32 { phys::shape_tags(shape).contains(&tag) as i32 }
pub unsafe fn IPhysicsShape_AddTag(shape: usize, tag: u32) -> i32 {
    phys::edit_tags(shape, |t| !t.contains(&tag) && { t.push(tag); true }) as i32
}
pub unsafe fn IPhysicsShape_RemoveTag(shape: usize, tag: u32) -> i32 {
    phys::edit_tags(shape, |t| {
        let n = t.len();
        t.retain(|&x| x != tag);
        t.len() != n
    }) as i32
}
pub unsafe fn IPhysicsShape_ClearTags(shape: usize) -> i32 {
    phys::edit_tags(shape, |t| {
        let had = !t.is_empty();
        t.clear();
        had
    }) as i32
}
pub unsafe fn IPhysicsShape_SetSurfaceIndex(shape: usize, part: i32, index: i32) {
    if part == 0 { phys::with_shape_mut(shape, false, |s, _| s.surface = index); }
}
pub unsafe fn IPhysicsShape_GetType(shape: usize) -> i64 { phys::shape_kind(shape) }
pub unsafe fn IPhysicsShape_UpdateMeshShape(shape: usize, nv: i32, vertices: usize, ni: i32, indices: usize) {
    phys::update_shape(shape, &Geometry::Mesh(rapier3d::math::Pose::IDENTITY, unsafe { phys::read_points(vertices, nv) }, unsafe { read_triangles(indices, ni) }))
}
pub unsafe fn IPhysicsShape_UpdateSphereShape(shape: usize, center: usize, radius: f32) {
    phys::update_shape(shape, &Geometry::Sphere(unsafe { phys::read_vec(center) }, radius))
}
pub unsafe fn IPhysicsShape_UpdateCapsuleShape(shape: usize, a: usize, b: usize, radius: f32) {
    phys::update_shape(shape, &Geometry::Capsule(unsafe { phys::read_vec(a) }, unsafe { phys::read_vec(b) }, radius))
}
pub unsafe fn IPhysicsShape_UpdateHullShape(shape: usize, position: usize, rotation: usize, n: i32, points: usize) {
    phys::update_shape(shape, &Geometry::Hull(pose_of(position, rotation), unsafe { phys::read_points(points, n) }))
}
pub unsafe fn IPhysicsShape_UpdateBoxShape(shape: usize, center: usize, rotation: usize, extents: usize) {
    phys::update_shape(shape, &Geometry::Box(pose_of(center, rotation), unsafe { phys::read_vec(extents) }))
}
pub unsafe fn IPhysicsShape_ManagedObject(shape: usize) -> i32 { crate::handles::of(shape) }
pub unsafe fn IPhysicsShape_SetTrigger(shape: usize, on: i32) { phys::set_trigger(shape, b(on)) }
pub unsafe fn IPhysicsShape_IsTrigger(shape: usize) -> i32 { phys::with_shape(shape, |s, _| s.trigger).unwrap_or(false) as i32 }
pub unsafe fn IPhysicsShape_GetTriangulation(shape: usize, vertices: usize, indices: usize) {
    let (v, i) = phys::triangulation(shape);
    vectors_set(vertices, v);
    indices_set(indices, i);
}
pub unsafe fn IPhysicsShape_AsSphere(shape: usize) -> S_Sandbox_Sphere { phys::as_sphere(shape) }
pub unsafe fn IPhysicsShape_AsCapsule(shape: usize) -> S_Capsule { phys::as_capsule(shape) }
pub unsafe fn IPhysicsShape_SetFriction(shape: usize, f: f32) { phys::set_friction(shape, f) }
pub unsafe fn IPhysicsShape_GetFriction(shape: usize) -> f32 { phys::with_shape(shape, |s, _| s.friction).unwrap_or(0.0) }
pub unsafe fn IPhysicsShape_SetLocalVelocity(shape: usize, v: usize) {
    let v = unsafe { phys::read_vec(v) };
    phys::with_shape_mut(shape, false, |s, _| s.local_velocity = v);
}
pub unsafe fn IPhysicsShape_GetLocalVelocity(shape: usize) -> S_Vector3 { phys::v3(phys::with_shape(shape, |s, _| s.local_velocity).unwrap_or_default()) }
pub unsafe fn IPhysicsShape_SetElasticity(shape: usize, e: f32) { phys::with_shape_mut(shape, false, |_, co| co.set_restitution(e.max(0.0))); }
pub unsafe fn IPhysicsShape_SetIgnoreTraces(shape: usize, on: i32) { phys::with_shape_mut(shape, false, |s, _| s.ignore_traces = on != 0); }
pub unsafe fn IPhysicsShape_SetHasNoMass(shape: usize, on: i32) { phys::with_shape_mut(shape, true, |s, _| s.no_mass = on != 0); }
pub unsafe fn IPhysicsShape_BuildBounds(shape: usize) -> S_BBox { phys::shape_bounds(shape, false) }
pub unsafe fn IPhysicsShape_LocalBounds(shape: usize) -> S_BBox { phys::shape_bounds(shape, true) }
pub unsafe fn IPhysicsShape_IsTouching(shape: usize, other: usize, triggers: i32) -> i32 { phys::shape_touching(shape, other, b(triggers)) as i32 }

// joints: physics/joints.rs. What a setter means can depend on the kind
// (an angular spring is a hinge's target, a weld's softness), as in
// managed's own 2D joints.

use crate::physics::joints::{self as joint, Drive, Kind};

fn add_joint(world: usize, kind: Kind, body1: usize, body2: usize, frame1: usize, frame2: usize) -> i32 {
    let j = crate::handles::object();
    let (f1, f2) = (unsafe { phys::read_pose(frame1) }, unsafe { phys::read_pose(frame2) });
    if !joint::add_joint(world, j, kind, body1, body2, f1, f2) { return 0; }
    crate::handles::register(j, "PhysicsJoint", &[world, body1, body2])
}

pub unsafe fn IPhysicsWorld_AddWeldJoint(w: usize, a: usize, b: usize, fa: usize, fb: usize) -> i32 { add_joint(w, Kind::Weld, a, b, fa, fb) }
pub unsafe fn IPhysicsWorld_AddSpringJoint(w: usize, a: usize, b: usize, fa: usize, fb: usize) -> i32 { add_joint(w, Kind::Spring, a, b, fa, fb) }
pub unsafe fn IPhysicsWorld_AddRevoluteJoint(w: usize, a: usize, b: usize, fa: usize, fb: usize) -> i32 { add_joint(w, Kind::Revolute, a, b, fa, fb) }
pub unsafe fn IPhysicsWorld_AddPrismaticJoint(w: usize, a: usize, b: usize, fa: usize, fb: usize) -> i32 { add_joint(w, Kind::Prismatic, a, b, fa, fb) }
pub unsafe fn IPhysicsWorld_AddSphericalJoint(w: usize, a: usize, b: usize, fa: usize, fb: usize) -> i32 { add_joint(w, Kind::Spherical, a, b, fa, fb) }
pub unsafe fn IPhysicsWorld_AddMotorJoint(w: usize, a: usize, b: usize, fa: usize, fb: usize) -> i32 { add_joint(w, Kind::Motor, a, b, fa, fb) }
pub unsafe fn IPhysicsWorld_AddWheelJoint(w: usize, a: usize, b: usize, fa: usize, fb: usize) -> i32 { add_joint(w, Kind::Wheel, a, b, fa, fb) }
pub unsafe fn IPhysicsWorld_AddParallelJoint(w: usize, a: usize, b: usize, fa: usize, fb: usize) -> i32 { add_joint(w, Kind::Parallel, a, b, fa, fb) }
pub unsafe fn IPhysicsWorld_AddFilterJoint(w: usize, a: usize, b: usize) -> i32 { add_joint(w, Kind::Filter, a, b, 0, 0) }
pub unsafe fn IPhysicsWorld_RemoveJoint(_world: usize, j: usize) {
    if joint::remove_joint(j) { crate::handles::free(j); }
}

fn kind(j: usize) -> Option<Kind> { joint::with_joint(j, |j| j.kind) }

/// A turn's angle about z, in radians (a hinge's target from a rotation).
fn yaw(q: rapier3d::math::Rotation) -> f32 { 2.0 * q.z.atan2(q.w) }

pub unsafe fn IPhysicsJoint_GetBody1(j: usize) -> i32 { crate::handles::of(joint::with_joint(j, |j| j.body1).unwrap_or(0)) }
pub unsafe fn IPhysicsJoint_GetBody2(j: usize) -> i32 { crate::handles::of(joint::with_joint(j, |j| j.body2).unwrap_or(0)) }

unsafe fn write_frame(f: rapier3d::math::Pose, position: usize, rotation: usize) {
    if position != 0 { unsafe { core::ptr::write_unaligned(position as *mut S_Vector3, phys::v3(f.translation)) }; }
    if rotation != 0 { unsafe { core::ptr::write_unaligned(rotation as *mut S_Rotation, phys::rot(f.rotation)) }; }
}
pub unsafe fn IPhysicsJoint_GetLocalFrameA(j: usize, position: usize, rotation: usize) {
    if let Some(f) = joint::with_joint(j, |j| j.frame1) { unsafe { write_frame(f, position, rotation) } }
}
pub unsafe fn IPhysicsJoint_GetLocalFrameB(j: usize, position: usize, rotation: usize) {
    if let Some(f) = joint::with_joint(j, |j| j.frame2) { unsafe { write_frame(f, position, rotation) } }
}
pub unsafe fn IPhysicsJoint_SetLocalFrameA(j: usize, position: usize, rotation: usize) {
    let f = pose_of(position, rotation);
    joint::edit_joint(j, |j| j.frame1 = f);
}
pub unsafe fn IPhysicsJoint_SetLocalFrameB(j: usize, position: usize, rotation: usize) {
    let f = pose_of(position, rotation);
    joint::edit_joint(j, |j| j.frame2 = f);
}
pub unsafe fn IPhysicsJoint_SetEnableCollision(j: usize, on: i32) { joint::edit_joint(j, |j| j.collide = on != 0); }
pub unsafe fn IPhysicsJoint_IsCollisionEnabled(j: usize) -> i32 { joint::with_joint(j, |j| j.collide).unwrap_or(false) as i32 }
pub unsafe fn IPhysicsJoint_GetType(j: usize) -> i64 { kind(j).map(Kind::native).unwrap_or(0) }

/// A slider's: frequency, damping, target. A weld's or a spring's: PhysicsSpring.
pub unsafe fn IPhysicsJoint_SetLinearSpring(j: usize, v: usize) {
    let v = unsafe { phys::read_vec(v) };
    joint::edit_joint(j, |j| {
        j.linear_spring = v;
        if j.kind == Kind::Prismatic { j.drive = Drive::Spring(v.z, v.x, v.y); }
    });
}
pub unsafe fn IPhysicsJoint_GetLinearSpring(j: usize) -> S_Vector3 { phys::v3(joint::with_joint(j, |j| j.linear_spring).unwrap_or_default()) }
/// A hinge's: target angle, frequency, damping. A weld's: PhysicsSpring.
pub unsafe fn IPhysicsJoint_SetAngularSpring(j: usize, v: usize) {
    let v = unsafe { phys::read_vec(v) };
    joint::edit_joint(j, |j| {
        j.angular_spring = v;
        if j.kind == Kind::Revolute { j.drive = Drive::Spring(v.x, v.y, v.z); }
    });
}
pub unsafe fn IPhysicsJoint_GetAngularSpring(j: usize) -> S_Vector3 { phys::v3(joint::with_joint(j, |j| j.angular_spring).unwrap_or_default()) }
pub unsafe fn IPhysicsJoint_SetAngularMotor(j: usize, speed: f32, most: f32) {
    joint::edit_joint(j, |j| if j.kind == Kind::Revolute { j.drive = Drive::Velocity(speed, most) });
}
pub unsafe fn IPhysicsJoint_SetLinearMotor(j: usize, speed: f32, most: f32) {
    joint::edit_joint(j, |j| if j.kind == Kind::Prismatic { j.drive = Drive::Velocity(speed, most) });
}
/// A spring's shortest, a hinge's lowest angle (radians), a slider's least travel.
pub unsafe fn IPhysicsJoint_SetMinLength(j: usize, v: f32) {
    joint::edit_joint(j, |j| {
        j.min = v;
        j.limited = if j.kind == Kind::Revolute { j.max > v } else { true };
    });
}
pub unsafe fn IPhysicsJoint_GetMinLength(j: usize) -> f32 { joint::with_joint(j, |j| j.min).unwrap_or(0.0) }
pub unsafe fn IPhysicsJoint_SetMaxLength(j: usize, v: f32) {
    joint::edit_joint(j, |j| {
        j.max = v;
        j.limited = if j.kind == Kind::Revolute { v > j.min } else { true };
    });
}
pub unsafe fn IPhysicsJoint_GetMaxLength(j: usize) -> f32 { joint::with_joint(j, |j| j.max).unwrap_or(0.0) }
pub unsafe fn IPhysicsJoint_SetMinForce(j: usize, v: f32) { joint::edit_joint(j, |j| j.min_force = v); }
pub unsafe fn IPhysicsJoint_GetMinForce(j: usize) -> f32 { joint::with_joint(j, |j| j.min_force).unwrap_or(f32::MIN) }
pub unsafe fn IPhysicsJoint_SetMaxForce(j: usize, v: f32) { joint::edit_joint(j, |j| j.max_force = v); }
pub unsafe fn IPhysicsJoint_GetMaxForce(j: usize) -> f32 { joint::with_joint(j, |j| j.max_force).unwrap_or(f32::MAX) }
pub unsafe fn IPhysicsJoint_SetFriction(j: usize, mu: f32) { joint::edit_joint(j, |j| j.drive = Drive::Friction(mu)); }
pub unsafe fn IPhysicsJoint_SetSwingLimit(j: usize, degrees: f32) { joint::edit_joint(j, |j| j.swing = degrees); }
pub unsafe fn IPhysicsJoint_SetSwingLimitEnabled(j: usize, on: i32) { joint::edit_joint(j, |j| j.swing_limited = on != 0); }
pub unsafe fn IPhysicsJoint_SetTwistLimits(j: usize, lower: f32, upper: f32) {
    joint::edit_joint(j, |j| {
        j.min = lower.to_radians();
        j.max = upper.to_radians();
    });
}
pub unsafe fn IPhysicsJoint_SetTwistLimitEnabled(j: usize, on: i32) {
    joint::edit_joint(j, |j| if j.kind == Kind::Spherical { j.twist_limited = on != 0 } else { j.limited = on != 0 });
}
pub unsafe fn IPhysicsJoint_SetLinearLimits(j: usize, lower: f32, upper: f32) {
    joint::edit_joint(j, |j| {
        j.min = lower;
        j.max = upper;
    });
}
pub unsafe fn IPhysicsJoint_SetLinearLimitEnabled(j: usize, on: i32) { joint::edit_joint(j, |j| j.limited = on != 0); }
pub unsafe fn IPhysicsJoint_GetAngle(j: usize) -> f32 { joint::joint_angle(j, rapier3d::math::Vector::Z) }
pub unsafe fn IPhysicsJoint_GetLinearImpulse(j: usize) -> f32 { joint::joint_forces(j).0 }
pub unsafe fn IPhysicsJoint_GetAngularImpulse(j: usize) -> f32 { joint::joint_forces(j).1 }
pub unsafe fn IPhysicsJoint_GetMaxLinearImpulse(j: usize) -> f32 { joint::with_joint(j, |j| j.max_linear).unwrap_or(f32::MAX) }
pub unsafe fn IPhysicsJoint_GetMaxAngularImpulse(j: usize) -> f32 { joint::with_joint(j, |j| j.max_angular).unwrap_or(f32::MAX) }
/// A break threshold; nothing (zero) is unbreakable.
pub unsafe fn IPhysicsJoint_SetMaxLinearImpulse(j: usize, v: f32) { joint::edit_joint(j, |j| j.max_linear = if v > 0.0 { v } else { f32::MAX }); }
pub unsafe fn IPhysicsJoint_SetMaxAngularImpulse(j: usize, v: f32) { joint::edit_joint(j, |j| j.max_angular = if v > 0.0 { v } else { f32::MAX }); }
pub unsafe fn IPhysicsJoint_SetMotorVelocity(j: usize, v: usize, most: f32) {
    let v = unsafe { phys::read_vec(v) };
    joint::edit_joint(j, |j| match j.kind {
        Kind::Spherical => {
            j.ball_target = None;
            j.ball_motor = Some((v, most));
        }
        Kind::Revolute => j.drive = Drive::Velocity(v.z, most),
        _ => {}
    });
}
pub unsafe fn IPhysicsJoint_SetTargetRotation(j: usize, q: usize, hz: f32, zeta: f32) {
    let q = unsafe { phys::read_rot(q) };
    joint::edit_joint(j, |j| match j.kind {
        Kind::Spherical => {
            j.ball_motor = None;
            j.ball_target = Some((q, hz, zeta));
        }
        Kind::Revolute => j.drive = Drive::Spring(yaw(q), hz, zeta),
        _ => {}
    });
}

// the motor joint (ControlJoint)

pub unsafe fn IPhysicsJoint_Motor_SetLinearVelocity(j: usize, v: usize) {
    let v = unsafe { phys::read_vec(v) };
    joint::edit_joint(j, |j| j.motor.linear_velocity = v);
}
pub unsafe fn IPhysicsJoint_Motor_SetAngularVelocity(j: usize, v: usize) {
    let v = unsafe { phys::read_vec(v) };
    joint::edit_joint(j, |j| j.motor.angular_velocity = v);
}
pub unsafe fn IPhysicsJoint_Motor_SetMaxVelocityForce(j: usize, v: f32) { joint::edit_joint(j, |j| j.motor.max_velocity_force = v); }
pub unsafe fn IPhysicsJoint_Motor_SetMaxVelocityTorque(j: usize, v: f32) { joint::edit_joint(j, |j| j.motor.max_velocity_torque = v); }
pub unsafe fn IPhysicsJoint_Motor_SetLinearHertz(j: usize, v: f32) { joint::edit_joint(j, |j| j.motor.linear_hertz = v); }
pub unsafe fn IPhysicsJoint_Motor_SetLinearDampingRatio(j: usize, v: f32) { joint::edit_joint(j, |j| j.motor.linear_damping = v); }
pub unsafe fn IPhysicsJoint_Motor_SetAngularHertz(j: usize, v: f32) { joint::edit_joint(j, |j| j.motor.angular_hertz = v); }
pub unsafe fn IPhysicsJoint_Motor_SetAngularDampingRatio(j: usize, v: f32) { joint::edit_joint(j, |j| j.motor.angular_damping = v); }
pub unsafe fn IPhysicsJoint_Motor_SetMaxSpringForce(j: usize, v: f32) { joint::edit_joint(j, |j| j.motor.max_spring_force = v); }
pub unsafe fn IPhysicsJoint_Motor_SetMaxSpringTorque(j: usize, v: f32) { joint::edit_joint(j, |j| j.motor.max_spring_torque = v); }
fn motor(j: usize) -> joint::MotorJoint { joint::with_joint(j, |j| j.motor).unwrap_or_default() }
pub unsafe fn IPhysicsJoint_Motor_GetLinearVelocity(j: usize) -> S_Vector3 { phys::v3(motor(j).linear_velocity) }
pub unsafe fn IPhysicsJoint_Motor_GetAngularVelocity(j: usize) -> S_Vector3 { phys::v3(motor(j).angular_velocity) }
pub unsafe fn IPhysicsJoint_Motor_GetMaxVelocityForce(j: usize) -> f32 { motor(j).max_velocity_force }
pub unsafe fn IPhysicsJoint_Motor_GetMaxVelocityTorque(j: usize) -> f32 { motor(j).max_velocity_torque }
pub unsafe fn IPhysicsJoint_Motor_GetLinearHertz(j: usize) -> f32 { motor(j).linear_hertz }
pub unsafe fn IPhysicsJoint_Motor_GetLinearDampingRatio(j: usize) -> f32 { motor(j).linear_damping }
pub unsafe fn IPhysicsJoint_Motor_GetAngularHertz(j: usize) -> f32 { motor(j).angular_hertz }
pub unsafe fn IPhysicsJoint_Motor_GetAngularDampingRatio(j: usize) -> f32 { motor(j).angular_damping }
pub unsafe fn IPhysicsJoint_Motor_GetMaxSpringForce(j: usize) -> f32 { motor(j).max_spring_force }
pub unsafe fn IPhysicsJoint_Motor_GetMaxSpringTorque(j: usize) -> f32 { motor(j).max_spring_torque }

// the wheel joint: suspension along the frame's x, steering about it, spin about z

fn wheel(j: usize) -> joint::Wheel { joint::with_joint(j, |j| j.wheel).unwrap_or_default() }
pub unsafe fn IPhysicsJoint_Wheel_EnableSuspension(j: usize, on: i32) { joint::edit_joint(j, |j| j.wheel.suspension = on != 0); }
pub unsafe fn IPhysicsJoint_Wheel_IsSuspensionEnabled(j: usize) -> i32 { wheel(j).suspension as i32 }
pub unsafe fn IPhysicsJoint_Wheel_SetSuspensionHertz(j: usize, v: f32) { joint::edit_joint(j, |j| j.wheel.suspension_hertz = v); }
pub unsafe fn IPhysicsJoint_Wheel_GetSuspensionHertz(j: usize) -> f32 { wheel(j).suspension_hertz }
pub unsafe fn IPhysicsJoint_Wheel_SetSuspensionDampingRatio(j: usize, v: f32) { joint::edit_joint(j, |j| j.wheel.suspension_damping = v); }
pub unsafe fn IPhysicsJoint_Wheel_GetSuspensionDampingRatio(j: usize) -> f32 { wheel(j).suspension_damping }
pub unsafe fn IPhysicsJoint_Wheel_EnableSuspensionLimit(j: usize, on: i32) { joint::edit_joint(j, |j| j.wheel.suspension_limited = on != 0); }
pub unsafe fn IPhysicsJoint_Wheel_IsSuspensionLimitEnabled(j: usize) -> i32 { wheel(j).suspension_limited as i32 }
pub unsafe fn IPhysicsJoint_Wheel_GetLowerSuspensionLimit(j: usize) -> f32 { wheel(j).lower }
pub unsafe fn IPhysicsJoint_Wheel_GetUpperSuspensionLimit(j: usize) -> f32 { wheel(j).upper }
pub unsafe fn IPhysicsJoint_Wheel_SetSuspensionLimits(j: usize, lower: f32, upper: f32) {
    joint::edit_joint(j, |j| {
        j.wheel.lower = lower;
        j.wheel.upper = upper;
    });
}
pub unsafe fn IPhysicsJoint_Wheel_EnableSpinMotor(j: usize, on: i32) { joint::edit_joint(j, |j| j.wheel.spin = on != 0); }
pub unsafe fn IPhysicsJoint_Wheel_IsSpinMotorEnabled(j: usize) -> i32 { wheel(j).spin as i32 }
pub unsafe fn IPhysicsJoint_Wheel_SetSpinMotorSpeed(j: usize, v: f32) { joint::edit_joint(j, |j| j.wheel.spin_speed = v); }
pub unsafe fn IPhysicsJoint_Wheel_GetSpinMotorSpeed(j: usize) -> f32 { wheel(j).spin_speed }
pub unsafe fn IPhysicsJoint_Wheel_SetMaxSpinTorque(j: usize, v: f32) { joint::edit_joint(j, |j| j.wheel.spin_torque = v); }
pub unsafe fn IPhysicsJoint_Wheel_GetMaxSpinTorque(j: usize) -> f32 { wheel(j).spin_torque }
pub unsafe fn IPhysicsJoint_Wheel_EnableSteering(j: usize, on: i32) { joint::edit_joint(j, |j| j.wheel.steering = on != 0); }
pub unsafe fn IPhysicsJoint_Wheel_IsSteeringEnabled(j: usize) -> i32 { wheel(j).steering as i32 }
pub unsafe fn IPhysicsJoint_Wheel_SetSteeringHertz(j: usize, v: f32) { joint::edit_joint(j, |j| j.wheel.steering_hertz = v); }
pub unsafe fn IPhysicsJoint_Wheel_GetSteeringHertz(j: usize) -> f32 { wheel(j).steering_hertz }
pub unsafe fn IPhysicsJoint_Wheel_SetSteeringDampingRatio(j: usize, v: f32) { joint::edit_joint(j, |j| j.wheel.steering_damping = v); }
pub unsafe fn IPhysicsJoint_Wheel_GetSteeringDampingRatio(j: usize) -> f32 { wheel(j).steering_damping }
pub unsafe fn IPhysicsJoint_Wheel_SetMaxSteeringTorque(j: usize, v: f32) { joint::edit_joint(j, |j| j.wheel.steering_torque = v); }
pub unsafe fn IPhysicsJoint_Wheel_GetMaxSteeringTorque(j: usize) -> f32 { wheel(j).steering_torque }
pub unsafe fn IPhysicsJoint_Wheel_EnableSteeringLimit(j: usize, on: i32) { joint::edit_joint(j, |j| j.wheel.steering_limited = on != 0); }
pub unsafe fn IPhysicsJoint_Wheel_IsSteeringLimitEnabled(j: usize) -> i32 { wheel(j).steering_limited as i32 }
pub unsafe fn IPhysicsJoint_Wheel_GetLowerSteeringLimit(j: usize) -> f32 { wheel(j).steer_lower }
pub unsafe fn IPhysicsJoint_Wheel_GetUpperSteeringLimit(j: usize) -> f32 { wheel(j).steer_upper }
pub unsafe fn IPhysicsJoint_Wheel_SetSteeringLimits(j: usize, lower: f32, upper: f32) {
    joint::edit_joint(j, |j| {
        j.wheel.steer_lower = lower;
        j.wheel.steer_upper = upper;
    });
}
pub unsafe fn IPhysicsJoint_Wheel_SetTargetSteeringAngle(j: usize, v: f32) { joint::edit_joint(j, |j| j.wheel.target_steer = v); }
pub unsafe fn IPhysicsJoint_Wheel_GetTargetSteeringAngle(j: usize) -> f32 { wheel(j).target_steer }
pub unsafe fn IPhysicsJoint_Wheel_GetSpinSpeed(j: usize) -> f32 { joint::joint_spin(j, rapier3d::math::Vector::Z) }
pub unsafe fn IPhysicsJoint_Wheel_GetSteeringAngle(j: usize) -> f32 { joint::joint_angle(j, rapier3d::math::Vector::X) }
pub unsafe fn IPhysicsJoint_Wheel_GetSpinTorque(j: usize) -> f32 { joint::joint_forces(j).1 }
pub unsafe fn IPhysicsJoint_Wheel_GetSteeringTorque(j: usize) -> f32 { joint::joint_forces(j).1 }

// the parallel joint (UprightJoint): keeps the frames' z together

pub unsafe fn IPhysicsJoint_Parallel_SetHertz(j: usize, v: f32) { joint::edit_joint(j, |j| j.parallel.0 = v); }
pub unsafe fn IPhysicsJoint_Parallel_GetHertz(j: usize) -> f32 { joint::with_joint(j, |j| j.parallel.0).unwrap_or(0.0) }
pub unsafe fn IPhysicsJoint_Parallel_SetDampingRatio(j: usize, v: f32) { joint::edit_joint(j, |j| j.parallel.1 = v); }
pub unsafe fn IPhysicsJoint_Parallel_GetDampingRatio(j: usize) -> f32 { joint::with_joint(j, |j| j.parallel.1).unwrap_or(0.0) }
pub unsafe fn IPhysicsJoint_Parallel_SetMaxTorque(j: usize, v: f32) { joint::edit_joint(j, |j| j.parallel.2 = v); }
pub unsafe fn IPhysicsJoint_Parallel_GetMaxTorque(j: usize) -> f32 { joint::with_joint(j, |j| j.parallel.2).unwrap_or(0.0) }

// traces

pub unsafe fn PhysicsTrace_Trace(request: usize) -> S_PhysicsTrace_Result {
    let r = unsafe { core::ptr::read_unaligned(request as *const phys::TraceRequest) };
    phys::trace(&r, false)[0]
}

static TRACE_RESULTS: Mutex<Vec<(usize, Vec<S_PhysicsTrace_Result>)>> = Mutex::new(Vec::new());

pub unsafe fn PhysicsTrace_TraceAll(request: usize, results: usize) {
    let r = unsafe { core::ptr::read_unaligned(request as *const phys::TraceRequest) };
    let hits = phys::trace(&r, true);
    let mut all = TRACE_RESULTS.lock().unwrap();
    match all.iter_mut().find(|(p, _)| *p == results) {
        Some((_, v)) => v.extend(hits),
        None => all.push((results, hits)),
    }
}

fn against(request: usize, target: rapier3d::parry::shape::SharedShape, centre: rapier3d::math::Vector, tx: usize) -> S_PhysicsTrace_Result {
    let r = unsafe { core::ptr::read_unaligned(request as *const phys::TraceRequest) };
    let at = unsafe { phys::read_pose(tx) } * rapier3d::math::Pose::from_translation(centre);
    phys::trace_against(&r, target, at)
}

pub unsafe fn PhysicsTrace_TraceAgainstCapsule(request: usize, capsule: usize, tx: usize) -> S_PhysicsTrace_Result {
    let c = unsafe { core::ptr::read_unaligned(capsule as *const S_Capsule) };
    let shape = rapier3d::parry::shape::SharedShape::capsule(phys::vec(c.f_CenterA), phys::vec(c.f_CenterB), c.f_Radius.max(0.0));
    against(request, shape, rapier3d::math::Vector::ZERO, tx)
}
pub unsafe fn PhysicsTrace_TraceAgainstBBox(request: usize, bbox: usize, tx: usize) -> S_PhysicsTrace_Result {
    let b = unsafe { core::ptr::read_unaligned(bbox as *const S_BBox) };
    let (mins, maxs) = (phys::vec(b.f_Mins), phys::vec(b.f_Maxs));
    let half = ((maxs - mins) * 0.5).abs().max(rapier3d::math::Vector::splat(1e-3));
    against(request, rapier3d::parry::shape::SharedShape::cuboid(half.x, half.y, half.z), (mins + maxs) * 0.5, tx)
}
pub unsafe fn PhysicsTrace_TraceAgainstSphere(request: usize, sphere: usize, tx: usize) -> S_PhysicsTrace_Result {
    let s = unsafe { core::ptr::read_unaligned(sphere as *const S_Sandbox_Sphere) };
    against(request, rapier3d::parry::shape::SharedShape::ball(s.f_Radius.max(0.0)), phys::vec(s.f_Center), tx)
}

// CUtlVector<PhysicsTrace.Result>, as TraceAll fills it

pub unsafe fn CtlVctrPhyscsTrc_Result_Create(_grow: i32, _capacity: i32) -> usize {
    let p = crate::handles::object();
    TRACE_RESULTS.lock().unwrap().push((p, Vec::new()));
    p
}
pub unsafe fn CtlVctrPhyscsTrc_Result_DeleteThis(v: usize) { TRACE_RESULTS.lock().unwrap().retain(|(p, _)| *p != v) }
pub unsafe fn CtlVctrPhyscsTrc_Result_Count(v: usize) -> i32 {
    TRACE_RESULTS.lock().unwrap().iter().find(|(p, _)| *p == v).map(|(_, r)| r.len() as i32).unwrap_or(0)
}
pub unsafe fn CtlVctrPhyscsTrc_Result_Element(v: usize, i: i32) -> S_PhysicsTrace_Result {
    let all = TRACE_RESULTS.lock().unwrap();
    match all.iter().find(|(p, _)| *p == v).and_then(|(_, r)| r.get(i as usize)) {
        Some(r) => *r,
        None => unsafe { core::mem::zeroed() },
    }
}
pub unsafe fn CtlVctrPhyscsTrc_Result_RemoveAll(v: usize) {
    if let Some((_, r)) = TRACE_RESULTS.lock().unwrap().iter_mut().find(|(p, _)| *p == v) { r.clear(); }
}

// CUtlVector<Vector> and CUtlVector<uint32>: managed's buffers that native
// fills (a shape's triangles) or reads

static VECTORS: Mutex<Vec<(usize, Vec<S_Vector3>)>> = Mutex::new(Vec::new());
static INDICES: Mutex<Vec<(usize, Vec<u32>)>> = Mutex::new(Vec::new());

fn vectors_set(v: usize, to: Vec<rapier3d::math::Vector>) {
    let to: Vec<S_Vector3> = to.into_iter().map(phys::v3).collect();
    let mut all = VECTORS.lock().unwrap();
    match all.iter_mut().find(|(p, _)| *p == v) {
        Some((_, x)) => *x = to,
        None => all.push((v, to)),
    }
}

fn indices_set(v: usize, to: Vec<u32>) {
    let mut all = INDICES.lock().unwrap();
    match all.iter_mut().find(|(p, _)| *p == v) {
        Some((_, x)) => *x = to,
        None => all.push((v, to)),
    }
}

pub unsafe fn CtlVctrVctr_Create(_grow: i32, _capacity: i32) -> usize {
    let p = crate::handles::object();
    VECTORS.lock().unwrap().push((p, Vec::new()));
    p
}
pub unsafe fn CtlVctrVctr_DeleteThis(v: usize) { VECTORS.lock().unwrap().retain(|(p, _)| *p != v) }
pub unsafe fn CtlVctrVctr_Count(v: usize) -> i32 {
    VECTORS.lock().unwrap().iter().find(|(p, _)| *p == v).map(|(_, x)| x.len() as i32).unwrap_or(0)
}
pub unsafe fn CtlVctrVctr_SetCount(v: usize, n: i32) {
    if let Some((_, x)) = VECTORS.lock().unwrap().iter_mut().find(|(p, _)| *p == v) {
        x.resize(n.max(0) as usize, phys::v3(rapier3d::math::Vector::ZERO));
    }
}
pub unsafe fn CtlVctrVctr_Element(v: usize, i: i32) -> S_Vector3 {
    VECTORS.lock().unwrap().iter().find(|(p, _)| *p == v).and_then(|(_, x)| x.get(i as usize).copied()).unwrap_or(phys::v3(rapier3d::math::Vector::ZERO))
}
pub unsafe fn CtlVctrnt32_Create(_grow: i32, _capacity: i32) -> usize {
    let p = crate::handles::object();
    INDICES.lock().unwrap().push((p, Vec::new()));
    p
}
pub unsafe fn CtlVctrnt32_DeleteThis(v: usize) { INDICES.lock().unwrap().retain(|(p, _)| *p != v) }
pub unsafe fn CtlVctrnt32_Count(v: usize) -> i32 {
    INDICES.lock().unwrap().iter().find(|(p, _)| *p == v).map(|(_, x)| x.len() as i32).unwrap_or(0)
}
pub unsafe fn CtlVctrnt32_SetCount(v: usize, n: i32) {
    if let Some((_, x)) = INDICES.lock().unwrap().iter_mut().find(|(p, _)| *p == v) { x.resize(n.max(0) as usize, 0); }
}
pub unsafe fn CtlVctrnt32_Element(v: usize, i: i32) -> u32 {
    INDICES.lock().unwrap().iter().find(|(p, _)| *p == v).and_then(|(_, x)| x.get(i as usize).copied()).unwrap_or(0)
}

#[cfg(test)]
mod tests {
    //! The slots as managed calls them: a dedicated server logs on, listens,
    //! takes a player, checks their ticket, and they talk.
    #[test]
    fn lz4_round_trips_blocks_and_frames() {
        unsafe {
            let data: Vec<u8> = (0..200_000u32).map(|i| (i % 251) as u8 ^ (i / 1000) as u8).collect();
            let n = data.len() as i32;
            let mut c = vec![0u8; LZ4Glue_CompressBound(n) as usize];
            let k = LZ4Glue_Compress(data.as_ptr() as usize, c.as_mut_ptr() as usize, n, c.len() as i32);
            assert!(k > 0 && k < n);
            let mut d = vec![0u8; data.len()];
            assert_eq!(LZ4Glue_Decompress(c.as_ptr() as usize, d.as_mut_ptr() as usize, k, n), n);
            assert_eq!(d, data);
            // too small a destination fails, as LZ4's does
            assert_eq!(LZ4Glue_Compress(data.as_ptr() as usize, c.as_mut_ptr() as usize, n, 10), 0);
            assert!(LZ4Glue_Decompress(c.as_ptr() as usize, d.as_mut_ptr() as usize, k, 100) < 0);

            let mut f = vec![0u8; LZ4Glue_CompressFrameBound(n) as usize];
            let k = LZ4Glue_CompressFrame(data.as_ptr() as usize, f.as_mut_ptr() as usize, n, f.len() as i32, 1);
            assert!(k > 0);
            assert_eq!(LZ4Glue_GetFrameContentSize(f.as_ptr() as usize, k), n as i64);
            let mut d = vec![0u8; data.len()];
            assert_eq!(LZ4Glue_DecompressFrame(f.as_ptr() as usize, d.as_mut_ptr() as usize, k, n), n);
            assert_eq!(d, data);
            assert!(LZ4Glue_DecompressFrame(f.as_ptr() as usize, d.as_mut_ptr() as usize, k, n - 1) < 0);
            assert_eq!(LZ4Glue_GetFrameContentSize(data.as_ptr() as usize, 64), -1);
            // incompressible bytes still fit the bound
            let noise: Vec<u8> = (0..70_000u32).map(|i| (i.wrapping_mul(2_654_435_761) >> 13) as u8).collect();
            let mut f = vec![0u8; LZ4Glue_CompressFrameBound(noise.len() as i32) as usize];
            assert!(LZ4Glue_CompressFrame(noise.as_ptr() as usize, f.as_mut_ptr() as usize, noise.len() as i32, f.len() as i32, 1) > 0);
        }
    }

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
        m[M_Sandbox_HandleIndex_RegisterHandle] = register_handle as extern "C" fn(usize, u32) -> i32 as usize;
        m[M_Sandbox_HandleIndex_FreeHandle] = free_handle as extern "C" fn(i32) as usize;
        m[M_SandboxPhysics_PhysicsEngine_OnActive] = on_active as extern "C" fn(i32, S_Transform, S_Vector3, S_Vector3, i32, i32) as usize;
        let _ = crate::MANAGED.set(m);
    }

    // managed's HandleIndex, and what PhysicsEngine.OnActive heard

    static HANDLES: Mutex<Vec<(usize, u32, i32)>> = Mutex::new(Vec::new());
    static FREED: Mutex<Vec<i32>> = Mutex::new(Vec::new());
    static ACTIVE: Mutex<Vec<(i32, f32, bool)>> = Mutex::new(Vec::new());
    static TOUCHES: Mutex<Vec<(i32, i32, i32)>> = Mutex::new(Vec::new());

    extern "C" fn register_handle(ptr: usize, ty: u32) -> i32 {
        let mut h = HANDLES.lock().unwrap();
        let n = h.len() as i32 + 2;
        h.push((ptr, ty, n));
        n
    }
    extern "C" fn free_handle(h: i32) { FREED.lock().unwrap().push(h) }
    extern "C" fn on_active(h: i32, t: S_Transform, _v: S_Vector3, _w: S_Vector3, asleep: i32, _out: i32) {
        ACTIVE.lock().unwrap().push((h, t.f_Position.f__vec.f_Z, asleep != 0));
    }
    extern "C" fn on_intersections(e: *const phys::Intersection, n: i32) {
        let e = unsafe { core::slice::from_raw_parts(e, n as usize) };
        TOUCHES.lock().unwrap().extend(e.iter().map(|e| (e.reason, e.left.shape_handle, e.right.shape_handle)));
    }
    static IGNORE: AtomicI32 = AtomicI32::new(0);
    extern "C" fn trace_filter(shape: i32) -> u8 { (shape != IGNORE.load(Ordering::Relaxed)) as u8 }

    fn ptr_of(h: i32) -> usize {
        HANDLES.lock().unwrap().iter().find(|x| x.2 == h).map(|x| x.0).unwrap()
    }

    #[test]
    fn a_ball_dropped_through_the_slots() {
        managed();
        unsafe {
            // Scene.PhysicsWorld: a world, its gravity, its rules
            let world = ptr_of(g_pPhysicsSystem_CreateWorld());
            let g = phys::v3(rapier3d::math::Vector::new(0.0, 0.0, -850.0));
            IPhysicsWorld_SetGravity(world, &g as *const S_Vector3 as usize);
            assert_eq!(phys::vec(IPhysicsWorld_GetGravity(world)).z, -850.0);
            let rules = CString::new(r#"{"Defaults":{"solid":"Collide"},"Pairs":[]}"#).unwrap();
            IPhysicsWorld_SetCollisionRulesFromJson(world, rules.as_ptr() as usize);

            // a BoxCollider on a static object: the floor
            let floor = IPhysicsWorld_AddBody_1(world, &phys::BodySettings::default() as *const _ as usize);
            let at = phys::v3(rapier3d::math::Vector::ZERO);
            let turn = phys::rot(rapier3d::math::Rotation::IDENTITY);
            let half = phys::v3(rapier3d::math::Vector::new(256.0, 256.0, 8.0));
            let mut solid = phys::ShapeSettings { flags: 8 | 16 | 32, ..Default::default() };
            solid.tags[0] = crate::handles::murmur2("solid");
            let floor_shape = IPhysicsBody_AddBoxShape(ptr_of(floor), &at as *const _ as usize, &turn as *const _ as usize, &half as *const _ as usize, &solid as *const _ as usize);
            assert!(floor_shape > 0);
            assert_eq!(IPhysicsShape_GetType(ptr_of(floor_shape)), phys::SHAPE_HULL);

            // a Rigidbody with a SphereCollider, 100 above it
            let mut s = phys::BodySettings { body_type: phys::DYNAMIC as i32, ..Default::default() };
            s.position = phys::v3(rapier3d::math::Vector::new(0.0, 0.0, 100.0));
            let ball = IPhysicsWorld_AddBody_1(world, &s as *const _ as usize);
            let b = ptr_of(ball);
            assert_eq!(IPhysicsBody_GetType(b), phys::DYNAMIC);
            let ball_shape = IPhysicsBody_AddSphereShape(b, &at as *const _ as usize, 6.0, &solid as *const _ as usize);
            assert_eq!(IPhysicsBody_GetShapeCount(b), 1);
            assert_eq!(IPhysicsBody_GetShape(b, 0), ball_shape);
            assert!(IPhysicsBody_GetMass(b) > 0.0);
            IPhysicsBody_SetMass(b, 10.0);
            assert_eq!(IPhysicsBody_GetMass(b), 10.0);
            assert_eq!(phys::vec(IPhysicsBody_GetPosition(b)).z, 100.0);

            // ScenePhysicsSystem: steps, then the intersections
            for _ in 0..240 {
                IPhysicsWorld_StepSimulation(world, 1.0 / 60.0, 2);
                IPhysicsWorld_ProcessIntersections(world, on_intersections as extern "C" fn(*const phys::Intersection, i32) as usize);
            }
            let z = phys::vec(IPhysicsBody_GetTransform(b).f_Position).z;
            assert!((z - 14.0).abs() < 0.5, "at rest on the floor (8) by its radius (6): {z}");
            let heard: Vec<(i32, f32, bool)> = ACTIVE.lock().unwrap().iter().filter(|a| a.0 == ball).copied().collect();
            assert!(heard.len() > 20, "OnActive for the falling ball");
            assert!(heard.first().unwrap().1 < 100.0 && (heard.last().unwrap().1 - 14.0).abs() < 0.5);
            assert!(heard.iter().any(|a| a.2), "and that it fell asleep");
            assert!(!ACTIVE.lock().unwrap().iter().any(|a| a.0 == floor), "never the static floor");
            let touches = TOUCHES.lock().unwrap().clone();
            assert!(touches.iter().any(|&(r, a, c)| r == phys::TOUCH_BEGIN && [a, c].contains(&ball_shape) && [a, c].contains(&floor_shape)), "{touches:?}");

            // a trace down from above hits the ball, then (filtered past it) the floor
            let mut r = phys::TraceRequest {
                world,
                body: 0,
                start: phys::v3(rapier3d::math::Vector::new(0.0, 0.0, 200.0)),
                end: phys::v3(rapier3d::math::Vector::new(0.0, 0.0, -200.0)),
                tag_require: [0; 8],
                tag_any: [0; 8],
                tag_exclude: [0; 8],
                filter: 0,
                trigger_filter: 0,
                object_set_mask: 0,
                use_hit_position: 0,
                shape: phys::TraceShape { kind: 0, rotation: turn, mins: at, maxs: at, radius: [0.0; 2] },
            };
            let hit = PhysicsTrace_Trace(&r as *const _ as usize);
            assert_eq!((hit.f_PhysicsBodyHandle, hit.f_PhysicsShapeHandle), (ball, ball_shape));
            assert!((phys::vec(hit.f_HitPos).z - 20.0).abs() < 0.5);
            assert_eq!(hit.f_Tags[0], crate::handles::murmur2("solid"));
            IGNORE.store(ball_shape, Ordering::Relaxed);
            r.filter = trace_filter as extern "C" fn(i32) -> u8 as usize;
            let hit = PhysicsTrace_Trace(&r as *const _ as usize);
            assert_eq!(hit.f_PhysicsBodyHandle, floor);
            r.filter = 0;
            let all = CtlVctrPhyscsTrc_Result_Create(32, 32);
            PhysicsTrace_TraceAll(&r as *const _ as usize, all);
            assert_eq!(CtlVctrPhyscsTrc_Result_Count(all), 2);
            assert_eq!(CtlVctrPhyscsTrc_Result_Element(all, 1).f_PhysicsShapeHandle, floor_shape);

            // FindInPhysics: a sphere around the ball finds it (and the floor it rests on)
            let q = CQueryResult_Create();
            let centre = phys::v3(rapier3d::math::Vector::new(0.0, 0.0, 14.0));
            IPhysicsWorld_Query(world, q, &centre as *const _ as usize, 8.0, 7);
            let found: Vec<i32> = (0..CQueryResult_Count(q)).map(|i| CQueryResult_Element(q, i)).collect();
            assert!(found.contains(&ball_shape) && found.contains(&floor_shape), "{found:?}");
            CQueryResult_RemoveAll(q);
            IPhysicsWorld_Query(world, q, &centre as *const _ as usize, 8.0, 7 & !1);
            assert_eq!((0..CQueryResult_Count(q)).map(|i| CQueryResult_Element(q, i)).collect::<Vec<_>>(), vec![ball_shape]);

            // an impulse up wakes it and sends it flying
            let up = phys::v3(rapier3d::math::Vector::new(0.0, 0.0, 5000.0));
            IPhysicsBody_ApplyLinearImpulse(b, &up as *const _ as usize);
            assert_eq!(IPhysicsBody_IsSleeping(b), 0);
            IPhysicsWorld_StepSimulation(world, 1.0 / 60.0, 1);
            assert!(phys::vec(IPhysicsBody_GetLinearVelocity(b)).z > 400.0);

            // Rigidbody removed: the body and its shape go, and managed lets go of both
            IPhysicsWorld_RemoveBody(world, b);
            let freed = FREED.lock().unwrap().clone();
            assert!(freed.contains(&ball) && freed.contains(&ball_shape), "{freed:?}");
            assert_eq!(PhysicsTrace_Trace(&r as *const _ as usize).f_PhysicsBodyHandle, floor);
        }
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
            let port = read(Glue_Networking_GetSocketAddress(socket)).rsplit(':').next().unwrap().to_string();
            let addr = format!("127.0.0.1:{port}");
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
