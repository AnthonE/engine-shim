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

/// The game's folder (where s&box's `game` is), for finding its files.
pub fn game_root() -> String {
    let root = ROOT.lock().unwrap().clone();
    if root.is_empty() { ".".into() } else { root }
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
// tools/compile.sh writes it; one compiled by s&box (a package downloaded
// from sbox.game) is Source 2's container, its JSON the DATA block.

thread_local! {
    /// The last JSON handed back, kept until the next call on this thread
    /// (managed copies it into a string straight away).
    static LAST_JSON: std::cell::RefCell<CString> = std::cell::RefCell::new(CString::default());
}

/// The JSON in a compiled resource: the file itself, or its container's DATA block.
fn resource_json(bytes: &[u8]) -> Option<&[u8]> {
    fn json(b: &[u8]) -> Option<&[u8]> {
        let start = b.iter().position(|c| !c.is_ascii_whitespace())?;
        let end = b.iter().rposition(|&c| c != 0 && !c.is_ascii_whitespace())? + 1;
        matches!(b[start], b'{' | b'[').then(|| &b[start..end])
    }
    json(bytes).or_else(|| json(resource_block(bytes, b"DATA")?))
}

/// The block of a compiled resource's container by its tag (DATA, a
/// GameResource's binary blob...): none in plain JSON or anything else.
fn resource_block<'a>(bytes: &'a [u8], tag: &[u8; 4]) -> Option<&'a [u8]> {
    let u32_at = |at: usize| bytes.get(at..at + 4).map(|x| u32::from_le_bytes(x.try_into().unwrap()) as usize);
    // the header: the file's size, a 12-byte header, a version, then the block list
    if bytes.get(4..6) != Some(&12u16.to_le_bytes()[..]) || u32_at(0)? > bytes.len() { return None; }
    let (list, count) = (8 + u32_at(8)?, u32_at(12)?);
    (0..count.min(64)).find_map(|i| {
        let at = list + 12 * i;
        if bytes.get(at..at + 4)? != tag { return None; }
        let start = at + 4 + u32_at(at + 4)?;
        bytes.get(start..start + u32_at(at + 8)?)
    })
}

fn json_returned(json: &[u8]) -> usize {
    let Ok(s) = CString::new(json) else { return 0 };
    LAST_JSON.with(|l| {
        *l.borrow_mut() = s;
        l.borrow().as_ptr() as usize
    })
}

pub unsafe fn EngineGlue_ReadCompiledResourceFileJson(data: usize, len: i32) -> usize {
    if data == 0 || len <= 0 { return 0; }
    let bytes = unsafe { core::slice::from_raw_parts(data as *const u8, len as usize) };
    resource_json(bytes).map(json_returned).unwrap_or(0)
}

/// As above, the file found by the engine's file system.
pub unsafe fn EngineGlue_ReadCompiledResourceFileJsonFromFilesystem(name: usize) -> usize {
    let name = unsafe { read(name) };
    crate::fs::read_compiled(&name).and_then(|b| resource_json(&b).map(json_returned)).unwrap_or(0)
}

/// A named block inside a compiled resource, where it lies in managed's
/// buffer (which managed copies from before letting go).
pub unsafe fn EngineGlue_ReadCompiledResourceFileBlock(name: usize, data: usize, len: i32, size: usize) -> usize {
    if size != 0 { unsafe { *(size as *mut i32) = 0 }; }
    if data == 0 || len <= 0 { return 0; }
    let name = unsafe { read(name) };
    let Ok(tag) = <[u8; 4]>::try_from(name.as_bytes().get(..4).unwrap_or_default()) else { return 0 };
    let bytes = unsafe { core::slice::from_raw_parts(data as *const u8, len as usize) };
    let Some(block) = resource_block(bytes, &tag) else { return 0 };
    if size != 0 { unsafe { *(size as *mut i32) = block.len() as i32 }; }
    block.as_ptr() as usize
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
    add_shape_of(body, g, &[], settings)
}

/// A shape whose triangles (or a height field's samples) have materials.
fn add_shape_of(body: usize, g: Geometry, materials: &[u8], settings: usize) -> i32 {
    let s = unsafe { phys::read::<phys::ShapeSettings>(settings) };
    let shape = crate::handles::object();
    if !phys::add_shape(body, shape, &g, &s, materials) { return 0; }
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
pub unsafe fn IPhysicsBody_AddHeightFieldShape(body: usize, heights: usize, materials: usize, x: i32, y: i32, spacing: f32, scale: f32, _n: i32, settings: usize) -> i32 {
    if heights == 0 || x < 2 || y < 2 { return 0; }
    let h = unsafe { core::slice::from_raw_parts(heights as *const u16, (x * y) as usize) }.to_vec();
    let m = if materials == 0 { &[][..] } else { unsafe { core::slice::from_raw_parts(materials as *const u8, (x * y) as usize) } };
    add_shape_of(body, Geometry::Height(h, x as usize, y as usize, spacing, scale), m, settings)
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
/// What its shapes weigh for their volume, kilograms per cubic metre.
pub unsafe fn IPhysicsBody_GetDensity(body: usize) -> f32 { phys::body_density(body) }
pub unsafe fn IPhysicsBody_GetClosestPoint(body: usize, p: usize) -> S_Vector3 { phys::v3(phys::closest_point(body, unsafe { phys::read_vec(p) })) }
pub unsafe fn IPhysicsBody_SetMaterialIndex(body: usize, name: S_Sandbox_StringToken) { phys::set_body_surface(body, surface_named(name)) }
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

// surfaces (surfaces.rs): the controller is the one list of them

use crate::surfaces;

/// The surface a StringToken names: none (-1, the default) for a name no
/// surface has.
fn surface_named(name: S_Sandbox_StringToken) -> i32 {
    if name.f_Value == 0 { -1 } else { surfaces::by_hash(name.f_Value).unwrap_or(-1) }
}

static SURFACE_CONTROLLER: u64 = 0;

pub unsafe fn g_pPhysicsSystem_GetSurfacePropertyController() -> usize { &SURFACE_CONTROLLER as *const u64 as usize }
pub unsafe fn PhysSrfcPrprtyCn_GetSurfacePropCount(_c: usize) -> i32 { surfaces::count() }
pub unsafe fn PhysSrfcPrprtyCn_GetSurfaceProperty(_c: usize, i: i32) -> usize { surfaces::object(i) }
pub unsafe fn PhysSrfcPrprtyCn_AddProperty(_c: usize, name: usize, base: usize, description: usize) -> usize {
    surfaces::add(&unsafe { read(name) }, &unsafe { read(base) }, &unsafe { read(description) })
}
/// A surface's values: the shapes made of it take them at once.
pub unsafe fn CPhysSrfcPrprts_UpdatePhysics(p: usize, friction: f32, elasticity: f32, density: f32, rolling_resistance: f32, bounce_threshold: f32) {
    let props = surfaces::Props { friction, elasticity, density, rolling_resistance, bounce_threshold };
    if surfaces::update(p, props).is_some() { phys::surfaces_changed(); }
}
pub unsafe fn g_pPhysicsSystem_UpdateSurfaceProperties(_p: usize) { phys::surfaces_changed() }
pub unsafe fn Get__CPhysSrfcPrprts_m_name(p: usize) -> usize { surfaces::name_of(p) }
pub unsafe fn Get__CPhysSrfcPrprts_m_description(p: usize) -> usize { surfaces::description(p) }

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
pub unsafe fn IPhysicsShape_SetSurfaceIndex(shape: usize, index: i32, part: i32) { phys::set_surface(shape, part.max(0), index) }
pub unsafe fn IPhysicsShape_SetMaterialIndex(shape: usize, name: S_Sandbox_StringToken) { phys::set_surface(shape, -1, surface_named(name)) }
pub unsafe fn IPhysicsShape_GetMaterialName(shape: usize) -> usize { surfaces::name(phys::surface(shape)) }
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
pub unsafe fn IPhysicsShape_GetFriction(shape: usize) -> f32 { phys::friction(shape) }
pub unsafe fn IPhysicsShape_SetLocalVelocity(shape: usize, v: usize) {
    let v = unsafe { phys::read_vec(v) };
    phys::with_shape_mut(shape, false, |s, _| s.local_velocity = v);
}
pub unsafe fn IPhysicsShape_GetLocalVelocity(shape: usize) -> S_Vector3 { phys::v3(phys::with_shape(shape, |s, _| s.local_velocity).unwrap_or_default()) }
pub unsafe fn IPhysicsShape_SetElasticity(shape: usize, e: f32) { phys::set_elasticity(shape, e) }
pub unsafe fn IPhysicsShape_SetRollingResistance(shape: usize, r: f32) { phys::set_rolling_resistance(shape, r) }
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

// ---- models made at runtime: models.rs. Their collision (an editor mesh's,
// a procedural prop's) is what physics reads back and makes shapes of.

use crate::models::{self, Hull, Mesh, Obj};
use crate::slots::S_System_Guid;

pub unsafe fn CModelBuilder_Create() -> usize { models::make(Obj::Builder { physics: 0 }) }
pub unsafe fn CModelBuilder_DeleteThis(_builder: usize) {}
pub unsafe fn CModelBuilder_SetPhysics_Runtime(builder: usize, bodies: usize) {
    if let Some(Obj::Builder { physics }) = models::models().objs.get_mut(&builder) { *physics = bodies; }
}
pub unsafe fn CModelBuilder_CreateRuntimeModel(builder: usize, _name: usize, _strong: i32) -> usize { models::build(builder) }

/// A model by name, from its compiled file (models.rs).
pub unsafe fn Glue_Resources_GetModel(name: usize, _guid: S_System_Guid) -> usize { models::load(&unsafe { read(name) }) }
pub unsafe fn Glue_Resources_GetModelResourceName(model: usize) -> usize { cstr(&models::name_of(model).unwrap_or_default()) }

// the file system: fs.rs

pub unsafe fn g_pFllFlSystm_AddSymLink(path: usize, _path_id: usize, target: usize) { crate::fs::add_link(&unsafe { read(path) }, &unsafe { read(target) }) }
pub unsafe fn g_pFllFlSystm_RemoveSymLink(path: usize, _path_id: usize) { crate::fs::remove_link(&unsafe { read(path) }) }
pub unsafe fn g_pFllFlSystm_GetSymLink(path: usize, _path_id: usize) -> usize { cstr(&crate::fs::link(&unsafe { read(path) }).unwrap_or_default()) }
pub unsafe fn g_pFllFlSystm_AddProjectPath(ident: usize, path: usize) { crate::fs::add_project(&unsafe { read(ident) }, &unsafe { read(path) }) }
pub unsafe fn g_pFllFlSystm_AddCloudPath(ident: usize, path: usize) { crate::fs::add_cloud(&unsafe { read(ident) }, &unsafe { read(path) }) }
pub unsafe fn g_pFllFlSystm_ResetProjectPaths(cloud: i32) { crate::fs::reset_projects(cloud != 0) }

/// A model is itself: managed caches models by this.
pub unsafe fn CModel_GetBindingPtr(model: usize) -> usize { model }
pub unsafe fn CModel_GetPhysicsContainer(model: usize) -> usize { models::models().physics_of(model) }

/// A polygon's triangles: indices written to `out` (room for `max`), how many.
pub unsafe fn MeshGlue_TriangulatePolygon(points: usize, n: i32, out: usize, max: i32) -> i32 {
    let t = models::triangulate(&unsafe { phys::read_points(points, n) });
    let k = t.len().min(max.max(0) as usize);
    if out != 0 {
        for (i, &v) in t.iter().take(k).enumerate() { unsafe { core::ptr::write_unaligned((out as *mut i32).add(i), v as i32) }; }
    }
    k as i32
}

// a builder's bodies (CPhysBodyDescArray), and a model's (CPhysicsData)

pub unsafe fn CPhysBdyDscrry_Create(n: i32, joints: i32) -> usize { models::new_bodies(n, joints) }
pub unsafe fn CPhysBdyDscrry_DeleteThis(_bodies: usize) {}
pub unsafe fn CPhysBdyDscrry_Get(bodies: usize, i: i32) -> usize {
    models::models().bodies(bodies).and_then(|b| b.parts.get(i as usize).copied()).unwrap_or(0)
}
pub unsafe fn CPhysBdyDscrry_GetJoint(bodies: usize, i: i32) -> usize {
    models::models().bodies(bodies).and_then(|b| b.joints.get(i as usize).copied()).unwrap_or(0)
}
pub unsafe fn CPhysBdyDscrry_AddSurface(bodies: usize, surface: i32) {
    if let Some(b) = models::models().bodies(bodies) { b.surfaces.push(surface); }
}

pub unsafe fn CPhysicsData_GetBindingPtr(physics: usize) -> usize { physics }
/// The surfaces a model's shapes name by index: a built one's, or a model
/// file's, found by name (none yet: null, which managed reads as "default").
pub unsafe fn CPhysicsData_GetSurfacePropertiesCount(physics: usize) -> i32 {
    models::models().bodies(physics).map(|b| b.surfaces.len().max(b.surface_hashes.len()) as i32).unwrap_or(0)
}
pub unsafe fn CPhysicsData_GetSurfaceProperties(physics: usize, i: i32) -> usize {
    let Ok(i) = usize::try_from(i) else { return 0 };
    let index = models::models().bodies(physics).and_then(|b| match b.surfaces.get(i) {
        Some(&s) => Some(s),
        None => b.surface_hashes.get(i).and_then(|&h| surfaces::by_hash(h)),
    });
    index.map(surfaces::object).unwrap_or(0)
}
pub unsafe fn CPhysicsData_GetPartCount(physics: usize) -> i32 {
    models::models().bodies(physics).map(|b| b.parts.len() as i32).unwrap_or(0)
}
pub unsafe fn CPhysicsData_GetPart(physics: usize, i: i32) -> usize { unsafe { CPhysBdyDscrry_Get(physics, i) } }
pub unsafe fn CPhysicsData_GetJointCount(physics: usize) -> i32 {
    models::models().bodies(physics).map(|b| b.joints.len() as i32).unwrap_or(0)
}
pub unsafe fn CPhysicsData_GetJoint(physics: usize, i: i32) -> usize { unsafe { CPhysBdyDscrry_GetJoint(physics, i) } }
pub unsafe fn CPhysicsData_GetGuid(_physics: usize) -> S_System_Guid { unsafe { core::mem::zeroed() } }
/// A model file's parts follow its bones: their names and bind poses.
pub unsafe fn CPhysicsData_GetBoneCount(physics: usize) -> i32 {
    models::models().bodies(physics).map(|b| b.bones.len() as i32).unwrap_or(0)
}
pub unsafe fn CPhysicsData_GetBoneName(physics: usize, i: i32) -> usize {
    cstr(&models::models().bodies(physics).and_then(|b| b.bones.get(i as usize).map(|x| x.0.clone())).unwrap_or_default())
}
pub unsafe fn CPhysicsData_GetBindPose(physics: usize, i: i32) -> S_Transform {
    let p = models::models().bodies(physics).and_then(|b| b.bones.get(i as usize).map(|x| x.1)).unwrap_or(rapier3d::math::Pose::IDENTITY);
    S_Transform { f_Position: phys::v3(p.translation), f_Scale: phys::v3(rapier3d::math::Vector::ONE), f_Rotation: phys::rot(p.rotation) }
}

// a body's description (CPhysBodyDesc), which is also the part it becomes (VPhysXBodyPart_t)

fn part<R: Default>(p: usize, f: impl FnOnce(&mut models::Part) -> R) -> R {
    models::models().part(p).map(f).unwrap_or_default()
}

/// A shape of a part: a new object, its pointer listed on the part.
fn add_to_part(p: usize, o: Obj, list: impl FnOnce(&mut models::Part) -> &mut Vec<usize>) {
    let ptr = models::make(o);
    part(p, |x| list(x).push(ptr));
}

pub unsafe fn CPhysBodyDesc_SetBoneName(p: usize, name: usize) {
    let name = unsafe { read(name) };
    part(p, |x| x.bone = name);
}
pub unsafe fn CPhysBodyDesc_SetSurface(p: usize, surface: S_Sandbox_StringToken) {
    let token: u32 = unsafe { core::mem::transmute(surface) };
    part(p, |x| x.surface = token);
}
pub unsafe fn CPhysBodyDesc_SetBindPose(p: usize, tx: usize) {
    let pose = unsafe { phys::read_pose(tx) };
    part(p, |x| x.bind_pose = pose);
}
pub unsafe fn CPhysBodyDesc_AddSphere(p: usize, sphere: usize) {
    let s = unsafe { core::ptr::read_unaligned(sphere as *const S_Sandbox_Sphere) };
    add_to_part(p, Obj::Sphere(phys::vec(s.f_Center), s.f_Radius), |x| &mut x.spheres);
}
pub unsafe fn CPhysBodyDesc_AddCapsule(p: usize, capsule: usize) {
    let c = unsafe { core::ptr::read_unaligned(capsule as *const S_Capsule) };
    add_to_part(p, Obj::Capsule(phys::vec(c.f_CenterA), phys::vec(c.f_CenterB), c.f_Radius), |x| &mut x.capsules);
}

/// The hull of `points` placed by `tx`, added to the part.
fn add_hull(p: usize, points: Vec<rapier3d::math::Vector>, tx: rapier3d::math::Pose) {
    let placed: Vec<_> = points.into_iter().map(|v| tx.transform_point(v)).collect();
    if let Some(h) = Hull::of(&placed) { add_to_part(p, Obj::Hull(h), |x| &mut x.hulls); }
}

pub unsafe fn CPhysBodyDesc_AddBox(p: usize, extents: usize, tx: usize) {
    let e = unsafe { phys::read_vec(extents) }.abs();
    let corners = (0..8).map(|i| rapier3d::math::Vector::new(if i & 1 == 0 { -e.x } else { e.x }, if i & 2 == 0 { -e.y } else { e.y }, if i & 4 == 0 { -e.z } else { e.z })).collect();
    add_hull(p, corners, unsafe { phys::read_pose(tx) });
}
pub unsafe fn CPhysBodyDesc_AddHull(p: usize, points: usize, n: i32, tx: usize) {
    add_hull(p, unsafe { phys::read_points(points, n) }, unsafe { phys::read_pose(tx) });
}
/// As AddHull: the simplification asked for is not done.
pub unsafe fn CPhysBodyDesc_AddHull_1(p: usize, points: usize, n: i32, tx: usize, _degrees: f32, _inches: f32, _faces: i32, _edges: i32, _verts: i32, _algo: i32) {
    add_hull(p, unsafe { phys::read_points(points, n) }, unsafe { phys::read_pose(tx) });
}
pub unsafe fn CPhysBodyDesc_AddMesh(p: usize, vertices: usize, nv: u32, indices: usize, ni: u32, materials: usize) {
    let triangles = unsafe { read_triangles(indices, ni as i32) };
    let materials = if materials == 0 { Vec::new() } else { unsafe { core::slice::from_raw_parts(materials as *const u8, triangles.len()) }.to_vec() };
    let mesh = Mesh { vertices: unsafe { phys::read_points(vertices, nv as i32) }, triangles, materials };
    add_to_part(p, Obj::Mesh(mesh), |x| &mut x.meshes);
}

pub unsafe fn Get__CPhysBodyDesc_m_flMass(p: usize) -> f32 { part(p, |x| x.mass) }
pub unsafe fn Set__CPhysBodyDesc_m_flMass(p: usize, v: f32) { part(p, |x| x.mass = v) }
pub unsafe fn Get__CPhysBodyDesc_m_flInertiaScale(p: usize) -> f32 { part(p, |x| x.inertia_scale) }
pub unsafe fn Set__CPhysBodyDesc_m_flInertiaScale(p: usize, v: f32) { part(p, |x| x.inertia_scale = v) }
pub unsafe fn Get__CPhysBodyDesc_m_flLinearDamping(p: usize) -> f32 { part(p, |x| x.linear_damping) }
pub unsafe fn Set__CPhysBodyDesc_m_flLinearDamping(p: usize, v: f32) { part(p, |x| x.linear_damping = v) }
pub unsafe fn Get__CPhysBodyDesc_m_flAngularDamping(p: usize) -> f32 { part(p, |x| x.angular_damping) }
pub unsafe fn Set__CPhysBodyDesc_m_flAngularDamping(p: usize, v: f32) { part(p, |x| x.angular_damping = v) }

pub unsafe fn VPhysXBodyPart_t_GetSphereCount(p: usize) -> i32 { part(p, |x| x.spheres.len() as i32) }
pub unsafe fn VPhysXBodyPart_t_GetSphere(p: usize, i: i32) -> usize { part(p, |x| x.spheres.get(i as usize).copied().unwrap_or(0)) }
pub unsafe fn VPhysXBodyPart_t_GetCapsuleCount(p: usize) -> i32 { part(p, |x| x.capsules.len() as i32) }
pub unsafe fn VPhysXBodyPart_t_GetCapsule(p: usize, i: i32) -> usize { part(p, |x| x.capsules.get(i as usize).copied().unwrap_or(0)) }
pub unsafe fn VPhysXBodyPart_t_GetHullCount(p: usize) -> i32 { part(p, |x| x.hulls.len() as i32) }
pub unsafe fn VPhysXBodyPart_t_GetHull(p: usize, i: i32) -> usize { part(p, |x| x.hulls.get(i as usize).copied().unwrap_or(0)) }
pub unsafe fn VPhysXBodyPart_t_GetMeshCount(p: usize) -> i32 { part(p, |x| x.meshes.len() as i32) }
pub unsafe fn VPhysXBodyPart_t_GetMesh(p: usize, i: i32) -> usize { part(p, |x| x.meshes.get(i as usize).copied().unwrap_or(0)) }
pub unsafe fn Get__VPhysXBodyPart_t_m_nFlags(p: usize) -> u32 { part(p, |x| x.flags) }
pub unsafe fn Set__VPhysXBodyPart_t_m_nFlags(p: usize, v: u32) { part(p, |x| x.flags = v) }
pub unsafe fn Get__VPhysXBodyPart_t_m_flMass(p: usize) -> f32 { part(p, |x| x.mass) }
pub unsafe fn Set__VPhysXBodyPart_t_m_flMass(p: usize, v: f32) { part(p, |x| x.mass = v) }
pub unsafe fn Get__VPhysXBodyPart_t_m_flInertiaScale(p: usize) -> f32 { part(p, |x| x.inertia_scale) }
pub unsafe fn Set__VPhysXBodyPart_t_m_flInertiaScale(p: usize, v: f32) { part(p, |x| x.inertia_scale = v) }
pub unsafe fn Get__VPhysXBodyPart_t_m_flLinearDamping(p: usize) -> f32 { part(p, |x| x.linear_damping) }
pub unsafe fn Set__VPhysXBodyPart_t_m_flLinearDamping(p: usize, v: f32) { part(p, |x| x.linear_damping = v) }
pub unsafe fn Get__VPhysXBodyPart_t_m_flAngularDamping(p: usize) -> f32 { part(p, |x| x.angular_damping) }
pub unsafe fn Set__VPhysXBodyPart_t_m_flAngularDamping(p: usize, v: f32) { part(p, |x| x.angular_damping = v) }
pub unsafe fn Get__VPhysXBodyPart_t_m_flGravityScale(p: usize) -> f32 { part(p, |x| x.gravity_scale) }
pub unsafe fn Set__VPhysXBodyPart_t_m_flGravityScale(p: usize, v: f32) { part(p, |x| x.gravity_scale = v) }
pub unsafe fn Get__VPhysXBodyPart_t_m_bOverrideMassCenter(p: usize) -> i32 { part(p, |x| x.override_mass_center as i32) }
pub unsafe fn Set__VPhysXBodyPart_t_m_bOverrideMassCenter(p: usize, v: i32) { part(p, |x| x.override_mass_center = v != 0) }
pub unsafe fn Get__VPhysXBodyPart_t_m_vMassCenterOverride(p: usize) -> S_Vector3 { phys::v3(part(p, |x| x.mass_center)) }
pub unsafe fn Set__VPhysXBodyPart_t_m_vMassCenterOverride(p: usize, v: usize) {
    let v = unsafe { phys::read_vec(v) };
    part(p, |x| x.mass_center = v)
}

// a part's shapes: spheres, capsules, hulls, meshes

pub unsafe fn Get__RnSphereDesc_t_m_Sphere(p: usize) -> S_Sandbox_Sphere {
    match models::models().objs.get(&p) {
        Some(Obj::Sphere(c, r)) => S_Sandbox_Sphere { f_Center: phys::v3(*c), f_Radius: *r },
        _ => S_Sandbox_Sphere { f_Center: phys::v3(rapier3d::math::Vector::ZERO), f_Radius: 0.0 },
    }
}
pub unsafe fn Get__RnCapsuleDesc_t_m_Capsule(p: usize) -> S_Capsule {
    match models::models().objs.get(&p) {
        Some(Obj::Capsule(a, b, r)) => S_Capsule { f_CenterA: phys::v3(*a), f_CenterB: phys::v3(*b), f_Radius: *r },
        _ => S_Capsule { f_CenterA: phys::v3(rapier3d::math::Vector::ZERO), f_CenterB: phys::v3(rapier3d::math::Vector::ZERO), f_Radius: 0.0 },
    }
}
/// A hull's (a mesh's) description is the hull (the mesh).
pub unsafe fn RnHullDesc_t_GetHull(p: usize) -> usize { p }
pub unsafe fn RnMeshDesc_t_GetMesh(p: usize) -> usize { p }

fn hull<R: Default>(p: usize, f: impl FnOnce(&Hull) -> R) -> R { models::models().hull(p).map(f).unwrap_or_default() }
fn mesh<R: Default>(p: usize, f: impl FnOnce(&Mesh) -> R) -> R { models::models().mesh(p).map(f).unwrap_or_default() }
fn bbox((lo, hi): (rapier3d::math::Vector, rapier3d::math::Vector)) -> S_BBox { S_BBox { f_Mins: phys::v3(lo), f_Maxs: phys::v3(hi) } }

pub unsafe fn RnHull_t_GetVertexCount(p: usize) -> i32 { hull(p, |h| h.points.len() as i32) }
pub unsafe fn RnHull_t_GetEdgeCount(p: usize) -> i32 { hull(p, |h| h.edges.len() as i32) }
pub unsafe fn RnHull_t_GetVertex(p: usize, i: i32) -> S_Vector3 { phys::v3(hull(p, |h| h.points.get(i as usize).copied().unwrap_or_default())) }
pub unsafe fn RnHull_t_GetEdgeVertex(p: usize, i: i32, a: usize, b: usize) {
    let (va, vb) = hull(p, |h| h.edges.get(i as usize).map(|&(x, y)| (h.points[x as usize], h.points[y as usize])).unwrap_or_default());
    if a != 0 { unsafe { core::ptr::write_unaligned(a as *mut S_Vector3, phys::v3(va)) }; }
    if b != 0 { unsafe { core::ptr::write_unaligned(b as *mut S_Vector3, phys::v3(vb)) }; }
}
pub unsafe fn RnHull_t_GetBbox(p: usize) -> S_BBox { bbox(hull(p, |h| h.bounds())) }
pub unsafe fn RnHull_t_GetCentroid(p: usize) -> S_Vector3 {
    phys::v3(hull(p, |h| h.points.iter().copied().sum::<rapier3d::math::Vector>() / h.points.len().max(1) as f32))
}
pub unsafe fn RnHull_t_GetMassCenter(p: usize) -> S_Vector3 { unsafe { RnHull_t_GetCentroid(p) } }

pub unsafe fn RnMesh_t_GetTriangleCount(p: usize) -> i32 { mesh(p, |m| m.triangles.len() as i32) }
pub unsafe fn RnMesh_t_GetVertexCount(p: usize) -> i32 { mesh(p, |m| m.vertices.len() as i32) }
pub unsafe fn RnMesh_t_GetVertex(p: usize, i: i32) -> S_Vector3 { phys::v3(mesh(p, |m| m.vertices.get(i as usize).copied().unwrap_or_default())) }
pub unsafe fn RnMesh_t_GetBbox(p: usize) -> S_BBox { bbox(mesh(p, |m| m.bounds())) }
/// How many materials a mesh's triangles name: one more than the highest.
pub unsafe fn RnMesh_t_GetMaterialCount(p: usize) -> i32 { mesh(p, |m| m.materials.iter().max().map(|&x| x as i32 + 1).unwrap_or(0)) }
pub unsafe fn RnMesh_t_GetTriangleMaterial(p: usize, i: i32) -> i32 { mesh(p, |m| m.materials.get(i as usize).map(|&x| x as i32).unwrap_or(0)) }
pub unsafe fn RnMesh_t_GetTriangleIndices(p: usize, i: i32, a: usize, b: usize, c: usize) {
    let t = mesh(p, |m| m.triangles.get(i as usize).copied().unwrap_or_default());
    for (out, v) in [(a, t[0]), (b, t[1]), (c, t[2])] {
        if out != 0 { unsafe { core::ptr::write_unaligned(out as *mut i32, v as i32) }; }
    }
}
pub unsafe fn RnMesh_t_GetTriangle(p: usize, i: i32, a: usize, b: usize, c: usize) {
    let t = mesh(p, |m| m.triangles.get(i as usize).map(|t| t.map(|k| m.vertices.get(k as usize).copied().unwrap_or_default())).unwrap_or_default());
    for (out, v) in [(a, t[0]), (b, t[1]), (c, t[2])] {
        if out != 0 { unsafe { core::ptr::write_unaligned(out as *mut S_Vector3, phys::v3(v)) }; }
    }
}

// a body given a model's hull or mesh, placed (and scaled) by a transform

/// A transform as where it puts things, and how it scales their points
/// first (no scale, or a zero one: none).
unsafe fn placed(tx: usize, points: Vec<rapier3d::math::Vector>) -> (rapier3d::math::Pose, Vec<rapier3d::math::Vector>) {
    if tx == 0 { return (rapier3d::math::Pose::IDENTITY, points); }
    let t = unsafe { core::ptr::read_unaligned(tx as *const S_Transform) };
    let s = phys::vec(t.f_Scale);
    let s = rapier3d::math::Vector::new(if s.x == 0.0 { 1.0 } else { s.x }, if s.y == 0.0 { 1.0 } else { s.y }, if s.z == 0.0 { 1.0 } else { s.z });
    let pose = rapier3d::math::Pose::from_parts(phys::vec(t.f_Position), phys::quat(t.f_Rotation));
    (pose, if s == rapier3d::math::Vector::ONE { points } else { points.into_iter().map(|p| p * s).collect() })
}

pub unsafe fn IPhysicsBody_AddHullShape_1(body: usize, h: usize, tx: usize, settings: usize) -> i32 {
    let (at, points) = unsafe { placed(tx, hull(h, |h| h.points.clone())) };
    add_shape(body, Geometry::Hull(at, points), settings)
}
/// A mesh as its hull.
pub unsafe fn IPhysicsBody_AddHullShape_2(body: usize, m: usize, tx: usize, settings: usize) -> i32 {
    let (at, points) = unsafe { placed(tx, mesh(m, |m| m.vertices.clone())) };
    add_shape(body, Geometry::Hull(at, points), settings)
}
pub unsafe fn IPhysicsBody_AddMeshShape_1(body: usize, m: usize, tx: usize, _materials: i32, settings: usize) -> i32 {
    let (vertices, triangles, materials) = mesh(m, |m| (m.vertices.clone(), m.triangles.clone(), m.materials.clone()));
    let (at, vertices) = unsafe { placed(tx, vertices) };
    add_shape_of(body, Geometry::Mesh(at, vertices, triangles), &materials, settings)
}

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

// ---- what a model file holds besides its collision (assets/model.rs): its
// bones, meshes and body groups, the hitboxes and attachments its meshes
// carry, its sequences. A model built at runtime has none of these yet.

use crate::animation::{self, Xf};
use crate::assets::model::{HitBox, HitBoxSet, ModelFile};
use crate::models::Piece;
use crate::slots::S_Color32;

fn file(model: usize) -> Option<&'static ModelFile> { models::file(model) }

/// A string for managed to read, kept once however often it's asked for.
fn interned(s: &str) -> usize {
    static ALL: Mutex<Option<std::collections::HashMap<String, CString>>> = Mutex::new(None);
    let mut all = ALL.lock().unwrap();
    let all = all.get_or_insert_with(Default::default);
    all.entry(s.to_string()).or_insert_with(|| CString::new(s.replace('\0', "")).unwrap()).as_ptr() as usize
}

fn ptr(s: &CStr) -> usize { s.as_ptr() as usize }

/// A value written where managed's out pointer points (nothing if null).
unsafe fn out<T>(p: usize, v: T) {
    if p != 0 { unsafe { core::ptr::write_unaligned(p as *mut T, v) }; }
}

fn at<T>(v: &[T], i: i32) -> Option<&T> {
    usize::try_from(i).ok().and_then(|i| v.get(i))
}

pub unsafe fn CModel_IsStrongHandleLoaded(_model: usize) -> i32 { 1 }
pub unsafe fn CModel_GetModelName(model: usize) -> usize { interned(&models::name_of(model).unwrap_or_default()) }
pub unsafe fn CModel_GetBaseModelName(model: usize) -> usize { unsafe { CModel_GetModelName(model) } }
/// No model here inherits from another.
pub unsafe fn CModel_GetBaseModel(_model: usize) -> usize { 0 }
pub unsafe fn CModel_IsTranslucent(_model: usize) -> i32 { 0 }
pub unsafe fn CModel_IsTranslucentTwoPass(_model: usize) -> i32 { 0 }
pub unsafe fn CModel_HasPhysics(model: usize) -> i32 {
    let mut m = models::models();
    let physics = m.physics_of(model);
    m.bodies(physics).is_some_and(|b| !b.parts.is_empty()) as i32
}
/// Model data (KeyValues3) isn't made here yet: none.
pub unsafe fn CModel_FindModelSubKey(_model: usize, _name: usize) -> usize { 0 }
pub unsafe fn CModel_GetAnimationGraph(_model: usize) -> usize { 0 }
pub unsafe fn CModel_MeshTrace(_model: usize, _input: usize, _output: usize) -> i32 { 0 }

// bones

pub unsafe fn CModel_NumBones(model: usize) -> i32 { file(model).map(|f| f.bones.len() as i32).unwrap_or(0) }
pub unsafe fn CModel_FindBoneIndex(model: usize, name: usize) -> i32 { file(model).map(|f| f.bone(&unsafe { read(name) })).unwrap_or(-1) }
pub unsafe fn CModel_boneName(model: usize, i: i32) -> usize {
    file(model).and_then(|f| at(&f.bones, i)).map(|b| ptr(&b.name)).unwrap_or(ptr(c""))
}
pub unsafe fn CModel_boneParent(model: usize, i: i32) -> i32 { file(model).and_then(|f| at(&f.bones, i)).map(|b| b.parent).unwrap_or(-1) }
pub unsafe fn CModel_boneFlags(model: usize, i: i32) -> i64 { file(model).and_then(|f| at(&f.bones, i)).map(|b| b.flags).unwrap_or(0) }
/// A bone in bind pose, in the model's space.
pub unsafe fn CModel_GetBoneTransform(model: usize, i: i32) -> S_Transform {
    file(model).and_then(|f| at(&f.bones, i)).map(|b| Xf::of(b.model).managed()).unwrap_or(Xf::IDENTITY.managed())
}
pub unsafe fn CModel_bonePosParentSpace(model: usize, i: i32) -> S_Vector3 {
    phys::v3(file(model).and_then(|f| at(&f.bones, i)).map(|b| b.pos).unwrap_or_default())
}
pub unsafe fn CModel_boneRotParentSpace(model: usize, i: i32) -> S_Rotation {
    phys::rot(file(model).and_then(|f| at(&f.bones, i)).map(|b| b.rot).unwrap_or(rapier3d::math::Rotation::IDENTITY))
}
pub unsafe fn CModel_HasSkinnedMeshes(model: usize) -> i32 { file(model).is_some_and(|f| !f.bones.is_empty()) as i32 }
pub unsafe fn CModel_HasAnimationDrivenFlexes(_model: usize) -> i32 { 0 }
pub unsafe fn CModel_NumFlexControllers(_model: usize) -> i32 { 0 }
pub unsafe fn CModel_GetVisemeIndex(_model: usize, _viseme: usize) -> i32 { -1 }

// sequences and animations: their names

pub unsafe fn CModel_GetNumAnim(model: usize) -> i32 { file(model).map(|f| f.animations.len() as i32).unwrap_or(0) }
pub unsafe fn CModel_GetAnimationName(model: usize, i: i32) -> usize {
    file(model).and_then(|f| at(&f.animations, i)).map(|a| ptr(a)).unwrap_or(ptr(c""))
}
pub unsafe fn CModel_GetSequenceNames(model: usize, names: usize) {
    let Some(f) = file(model) else { return };
    if let Some((_, v)) = STRINGS.lock().unwrap().iter_mut().find(|(p, _)| *p == names) {
        v.extend(f.sequences.iter().cloned());
    }
}

// CUtlVector<CUtlString>: a list of names managed asks native to fill

static STRINGS: Mutex<Vec<(usize, Vec<CString>)>> = Mutex::new(Vec::new());

pub unsafe fn CtlVctrCtlStrng_Create(_grow: i32, _capacity: i32) -> usize {
    let p = crate::handles::object();
    STRINGS.lock().unwrap().push((p, Vec::new()));
    p
}
pub unsafe fn CtlVctrCtlStrng_DeleteThis(v: usize) { STRINGS.lock().unwrap().retain(|(p, _)| *p != v) }
pub unsafe fn CtlVctrCtlStrng_Count(v: usize) -> i32 {
    STRINGS.lock().unwrap().iter().find(|(p, _)| *p == v).map(|(_, x)| x.len() as i32).unwrap_or(0)
}
pub unsafe fn CtlVctrCtlStrng_SetCount(v: usize, n: i32) {
    if let Some((_, x)) = STRINGS.lock().unwrap().iter_mut().find(|(p, _)| *p == v) { x.resize(n.max(0) as usize, CString::default()); }
}
pub unsafe fn CtlVctrCtlStrng_Element(v: usize, i: i32) -> usize {
    let all = STRINGS.lock().unwrap();
    // the list's own string, alive until managed deletes the list
    all.iter().find(|(p, _)| *p == v).and_then(|(_, x)| at(x, i)).map(|s| ptr(s)).unwrap_or(ptr(c""))
}

// meshes, their LODs and the groups that switch them

pub unsafe fn CModel_GetNumMeshes(model: usize) -> i32 { file(model).map(|f| f.meshes.len() as i32).unwrap_or(0) }
pub unsafe fn CModel_HasSceneObjects(model: usize) -> i32 { file(model).is_some_and(|f| f.meshes.iter().any(|m| !m.objects.is_empty())) as i32 }
pub unsafe fn CModel_GetMeshName(model: usize, mesh: i32) -> usize {
    file(model).and_then(|f| at(&f.meshes, mesh)).map(|m| ptr(&m.name)).unwrap_or(ptr(c""))
}
pub unsafe fn CModel_GetNumSceneObjects(model: usize, mesh: i32) -> i32 {
    file(model).and_then(|f| at(&f.meshes, mesh)).map(|m| m.objects.len() as i32).unwrap_or(0)
}
pub unsafe fn CModel_GetNumDrawCalls(model: usize, mesh: i32, object: i32) -> i32 {
    file(model).and_then(|f| at(&f.meshes, mesh)).and_then(|m| at(&m.objects, object)).map(|o| o.draws.len() as i32).unwrap_or(0)
}
/// Materials aren't loaded on a server: none.
pub unsafe fn CModel_GetMaterial(_model: usize, _mesh: i32, _object: i32, _draw: i32) -> usize { 0 }
pub unsafe fn CModel_GetDrawCallMaterial(_model: usize, _mesh: i32, _draw: i32) -> usize { 0 }
pub unsafe fn CModel_GetMaterialIndexCount(_model: usize) -> i32 { 0 }
pub unsafe fn CModel_GetMaterialByIndex(_model: usize, _i: i32) -> usize { 0 }
pub unsafe fn CModel_GetMaterialInGroup(_model: usize, _group: i32, _i: i32) -> usize { 0 }
pub unsafe fn CModel_GetNumMaterialsInGroup(_model: usize, _group: i32) -> i32 { 0 }
pub unsafe fn CModel_GetNumMaterialGroups(model: usize) -> i32 { file(model).map(|f| f.material_groups.len() as i32).unwrap_or(0) }
pub unsafe fn CModel_GetMaterialGroupName(model: usize, i: i32) -> usize {
    file(model).and_then(|f| at(&f.material_groups, i)).map(|g| ptr(g)).unwrap_or(ptr(c""))
}
pub unsafe fn CModel_GetMaterialGroupIndex(model: usize, token: S_Sandbox_StringToken) -> i32 {
    file(model).and_then(|f| f.material_groups.iter().position(|g| crate::handles::murmur2(&g.to_string_lossy()) == token.f_Value)).map(|i| i as i32).unwrap_or(-1)
}
pub unsafe fn CModel_GetTotalMeshCounts(model: usize, vertices: usize, triangles: usize, draws: usize) {
    let (v, t, d) = file(model).map(|f| f.meshes.iter().map(|m| m.counts()).fold((0, 0, 0), |a, c| (a.0 + c.0, a.1 + c.1, a.2 + c.2))).unwrap_or_default();
    unsafe { out(vertices, v); out(triangles, t); out(draws, d); }
}
pub unsafe fn CModel_GetMeshInfo(model: usize, mesh: i32, vertices: usize, triangles: usize, draws: usize, lod_mask: usize, translucency: usize, min: usize, max: usize) {
    let f = file(model);
    let m = f.and_then(|f| at(&f.meshes, mesh));
    let (v, t, d) = m.map(|m| m.counts()).unwrap_or_default();
    let (lo, hi) = m.and_then(|m| m.bounds()).unwrap_or_default();
    let lod = f.and_then(|f| at(&f.lod_masks, mesh)).copied().unwrap_or(0) as i32;
    unsafe { out(vertices, v); out(triangles, t); out(draws, d); out(lod_mask, lod); out(translucency, 0i32); out(min, phys::v3(lo)); out(max, phys::v3(hi)); }
}
/// A draw call by its index across the mesh's scene objects.
pub unsafe fn CModel_GetDrawCallInfo(model: usize, mesh: i32, draw: i32, vertices: usize, indices: usize, primitive: usize, uv_density: usize, instances: usize, alpha: usize, start: usize, base: usize) {
    let d = file(model).and_then(|f| at(&f.meshes, mesh)).and_then(|m| m.objects.iter().flat_map(|o| &o.draws).nth(draw.max(0) as usize));
    unsafe {
        out(vertices, d.map(|d| d.vertices).unwrap_or(0));
        out(indices, d.map(|d| d.indices).unwrap_or(0));
        out(primitive, d.map(|d| d.primitive).unwrap_or(0));
        out(uv_density, d.map(|d| d.uv_density).unwrap_or(0.0));
        out(instances, d.map(|d| d.instances).unwrap_or(0));
        out(alpha, 0i32);
        out(start, d.map(|d| d.start_index).unwrap_or(0));
        out(base, d.map(|d| d.base_vertex).unwrap_or(0));
    }
}
pub unsafe fn CModel_ComputeMaxLODLevelUsedByModel(model: usize) -> i32 {
    let all = file(model).map(|f| f.lod_masks.iter().fold(0u64, |a, m| a | m)).unwrap_or(0);
    if all == 0 { 0 } else { 63 - all.leading_zeros() as i32 }
}
pub unsafe fn CModel_LODLevelForScreenSize(_model: usize, _width: f32, _scale: f32) -> i32 { 0 }
pub unsafe fn CModel_GetLODSwitchDistanceCount(model: usize) -> i32 { file(model).map(|f| f.lod_switch.len() as i32).unwrap_or(0) }
pub unsafe fn CModel_GetLODSwitchDistance(model: usize, i: i32) -> f32 { file(model).and_then(|f| at(&f.lod_switch, i)).copied().unwrap_or(0.0) }

pub unsafe fn CModel_GetMeshBounds(model: usize) -> S_BBox { bbox(file(model).and_then(ModelFile::render_bounds).unwrap_or_default()) }
pub unsafe fn CModel_GetModelRenderBounds(model: usize) -> S_BBox { unsafe { CModel_GetMeshBounds(model) } }
/// What the model's collision covers: its parts' shapes, each in its bone's bind pose.
pub unsafe fn CModel_GetPhysicsBounds(model: usize) -> S_BBox {
    let mut m = models::models();
    let physics = m.physics_of(model);
    let Some(b) = m.bodies(physics).cloned() else { return bbox(Default::default()) };
    let mut points = Vec::new();
    for &p in &b.parts {
        let Some(part) = m.part(p).cloned() else { continue };
        let pose = b.bones.iter().find(|(n, _)| *n == part.bone).map(|x| x.1).unwrap_or(part.bind_pose);
        for &s in &part.spheres { if let Some(Obj::Sphere(c, r)) = m.objs.get(&s) { points.extend([*c - rapier3d::math::Vector::splat(*r), *c + rapier3d::math::Vector::splat(*r)].map(|x| pose * x)); } }
        for &s in &part.capsules { if let Some(Obj::Capsule(a, c, r)) = m.objs.get(&s) { let r = rapier3d::math::Vector::splat(*r); points.extend([*a - r, *a + r, *c - r, *c + r].map(|x| pose * x)); } }
        for &h in &part.hulls { if let Some(h) = m.hull(h) { points.extend(h.points.iter().map(|&x| pose * x)); } }
        for &h in &part.meshes { if let Some(h) = m.mesh(h) { points.extend(h.vertices.iter().map(|&x| pose * x)); } }
    }
    if points.is_empty() { return bbox(Default::default()); }
    bbox(points.iter().fold((rapier3d::math::Vector::splat(f32::MAX), rapier3d::math::Vector::splat(f32::MIN)), |(lo, hi), &p| (lo.min(p), hi.max(p))))
}

// body groups: a group's choices are mesh groups (`<group>_@<n>`)

pub unsafe fn CModel_GetNumMeshGroups(model: usize) -> i32 { file(model).map(|f| f.mesh_groups.len() as i32).unwrap_or(0) }
pub unsafe fn CModel_GetDefaultMeshGroupMask(model: usize) -> u64 { file(model).map(|f| f.default_mask).unwrap_or(0) }
pub unsafe fn CModel_GetMeshGroupMaskForMesh(model: usize, mesh: i32) -> u64 { file(model).and_then(|f| at(&f.mesh_group_masks, mesh)).copied().unwrap_or(0) }
pub unsafe fn CModel_GetNumBodyParts(model: usize) -> i32 { file(model).map(|f| f.body_parts.len() as i32).unwrap_or(0) }
pub unsafe fn CModel_GetBodyPartForName(model: usize, name: usize) -> i32 {
    let n = unsafe { read(name) };
    file(model).and_then(|f| f.body_parts.iter().position(|p| p.name.to_bytes().eq_ignore_ascii_case(n.as_bytes()))).map(|i| i as i32).unwrap_or(-1)
}
pub unsafe fn CModel_GetBodyPartName(model: usize, part: i32) -> usize {
    file(model).and_then(|f| at(&f.body_parts, part)).map(|p| ptr(&p.name)).unwrap_or(ptr(c""))
}
pub unsafe fn CModel_GetNumBodyPartMeshes(model: usize, part: i32) -> i32 {
    file(model).and_then(|f| at(&f.body_parts, part)).map(|p| p.choices.len() as i32).unwrap_or(0)
}
pub unsafe fn CModel_GetBodyPartMask(model: usize, part: i32) -> u64 {
    file(model).and_then(|f| at(&f.body_parts, part)).map(|p| p.choices.iter().fold(0, |a, c| a | c.1)).unwrap_or(0)
}
pub unsafe fn CModel_GetBodyPartMeshMask(model: usize, part: i32, choice: i32) -> u64 {
    file(model).and_then(|f| at(&f.body_parts, part)).and_then(|p| at(&p.choices, choice)).map(|c| c.1).unwrap_or(0)
}
pub unsafe fn CModel_GetBodyPartMeshName(model: usize, part: i32, choice: i32) -> usize {
    file(model).and_then(|f| at(&f.body_parts, part)).and_then(|p| at(&p.choices, choice)).map(|c| ptr(&c.0)).unwrap_or(ptr(c""))
}
/// The choice of a body group that a mesh group mask selects.
pub unsafe fn CModel_FindMeshIndexForMask(model: usize, part: i32, mask: u64) -> i32 {
    file(model).and_then(|f| at(&f.body_parts, part)).and_then(|p| p.choices.iter().position(|c| c.1 & mask != 0)).map(|i| i as i32).unwrap_or(-1)
}

// hitboxes (CHitBoxSet, CHitBox): the model's own, by their addresses

fn hitbox_set(p: usize) -> Option<&'static HitBoxSet> {
    match models::piece(p) { Some(Piece::HitBoxSet(s)) => Some(s), _ => None }
}
fn hitbox(p: usize) -> Option<&'static HitBox> {
    match models::piece(p) { Some(Piece::HitBox(h)) => Some(h), _ => None }
}

pub unsafe fn CModel_GetHitboxSetCount(model: usize, mesh: i32) -> i32 { file(model).map(|f| f.hitbox_sets(mesh).len() as i32).unwrap_or(0) }
pub unsafe fn CModel_GetHitboxSetByIndex(model: usize, i: i32, mesh: i32) -> usize {
    file(model).and_then(|f| at(f.hitbox_sets(mesh), i)).map(|s| s as *const HitBoxSet as usize).unwrap_or(0)
}
pub unsafe fn CModel_FindHitboxSetIndexByName(model: usize, name: usize, mesh: i32) -> i32 {
    let n = unsafe { read(name) };
    file(model).and_then(|f| f.hitbox_sets(mesh).iter().position(|s| s.name.to_bytes().eq_ignore_ascii_case(n.as_bytes()))).map(|i| i as i32).unwrap_or(-1)
}
pub unsafe fn CModel_FindHitboxSetByName(model: usize, name: usize, mesh: i32) -> usize {
    let i = unsafe { CModel_FindHitboxSetIndexByName(model, name, mesh) };
    unsafe { CModel_GetHitboxSetByIndex(model, i, mesh) }
}
pub unsafe fn CModel_GetBoneIndexForHitbox(model: usize, h: usize) -> i32 {
    match (file(model), hitbox(h)) {
        (Some(f), Some(h)) => f.bone(&h.bone.to_string_lossy()),
        _ => -1,
    }
}

pub unsafe fn CHitBoxSet_numhitboxes(set: usize) -> i32 { hitbox_set(set).map(|s| s.boxes.len() as i32).unwrap_or(0) }
pub unsafe fn CHitBoxSet_pHitbox(set: usize, i: i32) -> usize { hitbox_set(set).and_then(|s| at(&s.boxes, i)).map(|h| h as *const HitBox as usize).unwrap_or(0) }
pub unsafe fn Get__CHitBoxSet_m_name(set: usize) -> usize { hitbox_set(set).map(|s| ptr(&s.name)).unwrap_or(ptr(c"")) }
pub unsafe fn Get__CHitBoxSet_m_SourceFilename(set: usize) -> usize { hitbox_set(set).map(|s| ptr(&s.source)).unwrap_or(ptr(c"")) }

pub unsafe fn CHitBox_GetTag(h: usize, i: i32) -> usize { hitbox(h).and_then(|h| at(&h.tags, i)).map(|t| ptr(t)).unwrap_or(ptr(c"")) }
pub unsafe fn Get__CHitBox_m_vMinBounds(h: usize) -> S_Vector3 { phys::v3(hitbox(h).map(|h| h.min).unwrap_or_default()) }
pub unsafe fn Get__CHitBox_m_vMaxBounds(h: usize) -> S_Vector3 { phys::v3(hitbox(h).map(|h| h.max).unwrap_or_default()) }
pub unsafe fn Get__CHitBox_m_name(h: usize) -> usize { hitbox(h).map(|h| ptr(&h.name)).unwrap_or(ptr(c"")) }
pub unsafe fn Get__CHitBox_m_sSurfaceProperty(h: usize) -> usize { hitbox(h).map(|h| ptr(&h.surface)).unwrap_or(ptr(c"")) }
pub unsafe fn Get__CHitBox_m_sBoneName(h: usize) -> usize { hitbox(h).map(|h| ptr(&h.bone)).unwrap_or(ptr(c"")) }
pub unsafe fn Get__CHitBox_m_flShapeRadius(h: usize) -> f32 { hitbox(h).map(|h| h.radius).unwrap_or(0.0) }
pub unsafe fn Get__CHitBox_m_nBoneNameHash(h: usize) -> u32 { hitbox(h).map(|h| h.bone_hash).unwrap_or(0) }
pub unsafe fn Get__CHitBox_m_cRenderColor(h: usize) -> S_Color32 {
    let [r, g, b, a] = hitbox(h).map(|h| h.color).unwrap_or_default();
    S_Color32 { f_r: r, f_g: g, f_b: b, f_a: a }
}
pub unsafe fn Get__CHitBox_m_nHitBoxIndex(h: usize) -> u16 { hitbox(h).map(|h| h.index).unwrap_or(0) }
pub unsafe fn Get__CHitBox_m_nShapeType(h: usize) -> u8 { hitbox(h).map(|h| h.shape).unwrap_or(0) }
pub unsafe fn Get__CHitBox_m_bTranslationOnly(h: usize) -> i32 { hitbox(h).is_some_and(|h| h.translation_only) as i32 }
pub unsafe fn Get__CHitBox_m_bVisible(h: usize) -> i32 { hitbox(h).is_some_and(|h| h.visible) as i32 }
pub unsafe fn Get__CHitBox_m_bForcedTransform(_h: usize) -> i32 { 0 }
pub unsafe fn Get__CHitBox_m_bSelected(_h: usize) -> i32 { 0 }

// attachments (CAttachment)

fn attachment(p: usize) -> Option<&'static crate::assets::model::Attachment> {
    match models::piece(p) { Some(Piece::Attachment(a)) => Some(a), _ => None }
}

pub unsafe fn CModel_GetNumAttachments(model: usize) -> i32 { file(model).map(|f| f.attachments().len() as i32).unwrap_or(0) }
pub unsafe fn CModel_GetAttachment(model: usize, i: i32) -> usize {
    file(model).and_then(|f| at(f.attachments(), i)).map(|a| a as *const _ as usize).unwrap_or(0)
}
pub unsafe fn CModel_GetAttachmentNameFromIndex(model: usize, i: i32) -> usize {
    file(model).and_then(|f| at(f.attachments(), i)).map(|a| ptr(&a.name)).unwrap_or(ptr(c""))
}
/// An attachment by its name's token, in the model's space, its bones in bind pose.
pub unsafe fn CModel_GetAttachmentTransform(model: usize, name: S_Sandbox_StringToken, tx: usize) -> i32 {
    let Some(f) = file(model) else { return 0 };
    let Some(a) = f.attachments().iter().find(|a| crate::handles::murmur2(&a.name.to_string_lossy()) == name.f_Value) else { return 0 };
    unsafe { out(tx, Xf::of(f.attachment_pose(a)).managed()) };
    1
}
pub unsafe fn Get__CAttachment_m_name(a: usize) -> usize { attachment(a).map(|a| ptr(&a.name)).unwrap_or(ptr(c"")) }
pub unsafe fn Get__CAttachment_m_nInfluences(a: usize) -> u8 { attachment(a).map(|a| a.influences.len() as u8).unwrap_or(0) }
pub unsafe fn Get__CAttachment_m_bIgnoreRotation(a: usize) -> i32 { attachment(a).is_some_and(|a| a.ignore_rotation) as i32 }
pub unsafe fn CAttachment_GetInfluenceName(a: usize, i: i32) -> usize {
    attachment(a).and_then(|a| at(&a.influences, i)).map(|x| ptr(&x.0)).unwrap_or(ptr(c""))
}
pub unsafe fn CAttachment_GetInfluenceOffset(a: usize, i: i32) -> S_Vector3 { phys::v3(attachment(a).and_then(|a| at(&a.influences, i)).map(|x| x.1).unwrap_or_default()) }
pub unsafe fn CAttachment_GetInfluenceRotation(a: usize, i: i32) -> S_Rotation {
    phys::rot(attachment(a).and_then(|a| at(&a.influences, i)).map(|x| x.2).unwrap_or(rapier3d::math::Rotation::IDENTITY))
}

// ---- models in the scene (SceneModel, CSceneAnimatableObject): their
// bones, animation.rs. Every model scene object is one.

/// A scene object showing `model`, at the transform managed passes, in the
/// world it names.
pub unsafe fn g_pMeshSystem_CreateSceneObject(model: usize, transform: usize, _desc: usize, _flags: i64, _type_flags: i64, world: usize, _x: i32) -> i32 {
    let obj = crate::handles::object();
    unsafe { crate::state::set_from::<S_Transform>(obj, crate::slots::PROP_Transform, transform) };
    animation::create(obj, model);
    crate::handles::register(obj, "SceneObject", &[model, world])
}
pub unsafe fn g_pMeshSystem_ChangeModel(obj: usize, model: usize) { animation::change_model(obj, model) }
pub unsafe fn CSceneObject_GetModelHandle(obj: usize) -> usize { animation::model_of(obj) }
pub unsafe fn CScnnmtblbjct_GetModelHandle(obj: usize) -> usize { animation::model_of(obj) }
pub unsafe fn CSceneObject_GetCTransform(obj: usize) -> S_Transform { crate::state::get::<S_Transform>(obj, crate::slots::PROP_Transform) }
pub unsafe fn CScnnmtblbjct_GetCTransform(obj: usize) -> S_Transform { crate::state::get::<S_Transform>(obj, crate::slots::PROP_Transform) }

fn bone_of(obj: usize, name: usize) -> i32 {
    file(animation::model_of(obj)).map(|f| f.bone(&unsafe { read(name) })).unwrap_or(-1)
}
/// A bone's transform from one of the pose's arrays (zeroes for none: managed's "no such bone").
fn bone(obj: usize, i: i32, which: impl FnOnce(&animation::Posed) -> &Vec<Xf>) -> S_Transform {
    animation::with(obj, |p| at(which(p), i).map(Xf::managed)).flatten().unwrap_or(unsafe { core::mem::zeroed() })
}
/// `n` transforms from one of the pose's arrays into managed's buffer.
unsafe fn bones_into(obj: usize, n: i32, dest: usize, which: impl FnOnce(&animation::Posed) -> &Vec<Xf>) {
    if dest == 0 { return; }
    animation::with(obj, |p| {
        for (k, x) in which(p).iter().take(n.max(0) as usize).enumerate() {
            unsafe { core::ptr::write_unaligned((dest as *mut S_Transform).add(k), x.managed()) };
        }
    });
}

pub unsafe fn CScnnmtblbjct_GetWorldSpaceRenderBoneTransform(obj: usize, i: i32) -> S_Transform { bone(obj, i, |p| &p.world) }
pub unsafe fn CScnnmtblbjct_GetWorldSpaceRenderBoneTransform_1(obj: usize, name: usize) -> S_Transform { bone(obj, bone_of(obj, name), |p| &p.world) }
pub unsafe fn CScnnmtblbjct_GetWorldSpaceRenderBonePreviousTransform(obj: usize, i: i32) -> S_Transform { bone(obj, i, |p| &p.previous) }
pub unsafe fn CScnnmtblbjct_GetWorldSpaceRenderBonePreviousTransform_1(obj: usize, name: usize) -> S_Transform { bone(obj, bone_of(obj, name), |p| &p.previous) }
pub unsafe fn CScnnmtblbjct_GetLocalSpaceRenderBoneTransform(obj: usize, i: i32) -> S_Transform { bone(obj, i, |p| &p.model_space) }
pub unsafe fn CScnnmtblbjct_GetLocalSpaceRenderBoneTransform_1(obj: usize, name: usize) -> S_Transform { bone(obj, bone_of(obj, name), |p| &p.model_space) }
pub unsafe fn CScnnmtblbjct_GetWorldSpaceAnimationTransform(obj: usize, i: i32) -> S_Transform { bone(obj, i, |p| &p.world) }
pub unsafe fn CScnnmtblbjct_GetParentSpaceBone(obj: usize, i: i32) -> S_Transform { bone(obj, i, |p| &p.local) }
pub unsafe fn CScnnmtblbjct_GetWorldSpaceRenderBoneTransforms(obj: usize, n: i32, dest: usize) { unsafe { bones_into(obj, n, dest, |p| &p.world) } }
pub unsafe fn CScnnmtblbjct_GetWorldSpaceRenderBonePreviousTransforms(obj: usize, n: i32, dest: usize) { unsafe { bones_into(obj, n, dest, |p| &p.previous) } }
pub unsafe fn CScnnmtblbjct_GetParentSpaceBones(obj: usize, n: i32, dest: usize) { unsafe { bones_into(obj, n, dest, |p| &p.local) } }
pub unsafe fn CScnnmtblbjct_SetWorldSpaceRenderBoneTransform(obj: usize, i: i32, tx: usize) {
    if tx == 0 { return; }
    let x = Xf::from_managed(&unsafe { core::ptr::read_unaligned(tx as *const S_Transform) });
    animation::with(obj, |p| if let Some(w) = usize::try_from(i).ok().and_then(|i| p.world.get_mut(i)) { *w = x; });
}
pub unsafe fn CScnnmtblbjct_SetParentSpaceBone(obj: usize, i: i32, tx: usize) {
    if tx == 0 { return; }
    let x = Xf::from_managed(&unsafe { core::ptr::read_unaligned(tx as *const S_Transform) });
    animation::with(obj, |p| if let Some(l) = usize::try_from(i).ok().and_then(|i| p.local.get_mut(i)) { *l = x; });
}
pub unsafe fn CScnnmtblbjct_GetClosestBoneIndex(obj: usize, point: usize) -> i32 {
    let at = unsafe { phys::read_vec(point) };
    animation::with(obj, |p| p.world.iter().enumerate().min_by(|a, b| (a.1.pos - at).length_squared().total_cmp(&(b.1.pos - at).length_squared())).map(|(i, _)| i as i32)).flatten().unwrap_or(-1)
}
pub unsafe fn CScnnmtblbjct_Update(obj: usize, delta: f32) { animation::update(obj, delta) }
pub unsafe fn Get__CScnnmtblbjct_m_flDeltaTime(obj: usize) -> f32 { animation::with(obj, |p| p.delta).unwrap_or(0.0) }
pub unsafe fn CScnnmtblbjct_SetBindPose(obj: usize) { animation::set_bind_pose(obj) }
pub unsafe fn CScnnmtblbjct_CalculateWorldSpaceBones(obj: usize) { animation::calculate(obj) }
pub unsafe fn CScnnmtblbjct_FinishUpdate(_obj: usize) {}
pub unsafe fn CScnnmtblbjct_MergeFrom(obj: usize, from: usize) { animation::merge_from(obj, from) }
pub unsafe fn CScnnmtblbjct_GetAnimGraph(_obj: usize) -> usize { 0 }

// sequences played straight (no animation graph runs here)

pub unsafe fn CScnnmtblbjct_SetSequence(obj: usize, name: usize) {
    let n = unsafe { read(name) };
    animation::with(obj, |p| p.play(&n));
}
pub unsafe fn CScnnmtblbjct_GetSequence(obj: usize) -> usize {
    interned(&animation::with(obj, |p| p.playing().map(|(s, _, _)| s.name.clone())).flatten().unwrap_or_default())
}
pub unsafe fn CScnnmtblbjct_GetSequenceDuration(obj: usize) -> f32 { animation::with(obj, |p| p.playing().map(|x| x.1)).flatten().unwrap_or(0.0) }
pub unsafe fn CScnnmtblbjct_GetSequenceCycle(obj: usize) -> f32 { animation::with(obj, |p| p.cycle).unwrap_or(0.0) }
pub unsafe fn CScnnmtblbjct_SetSequenceCycle(obj: usize, cycle: f32) { animation::with(obj, |p| p.cycle = cycle.clamp(0.0, 1.0)); }
pub unsafe fn CScnnmtblbjct_SetSequenceLooping(obj: usize, looping: i32) { animation::with(obj, |p| p.looping = Some(looping != 0)); }
pub unsafe fn CScnnmtblbjct_IsSequenceFinished(obj: usize) -> i32 { animation::with(obj, |p| p.finished()).unwrap_or(true) as i32 }
/// Sequences cut from one to the next here: blending is asked for, not done.
pub unsafe fn CScnnmtblbjct_SetSequenceBlending(_obj: usize, _blending: i32) {}
pub unsafe fn CScnnmtblbjct_GetPlaybackRate(obj: usize) -> f32 { animation::with(obj, |p| p.rate.unwrap_or(1.0)).unwrap_or(1.0) }
pub unsafe fn CScnnmtblbjct_SetPlaybackRate(obj: usize, rate: f32) { animation::with(obj, |p| p.rate = Some(rate)); }
/// A physics bone (a ragdoll's): the bone where physics has it, in the model's space.
pub unsafe fn CScnnmtblbjct_SetPhysicsBone(obj: usize, i: u16, tx: usize) {
    if tx == 0 { return; }
    let x = Xf::from_managed(&unsafe { core::ptr::read_unaligned(tx as *const S_Transform) });
    animation::with(obj, |p| { p.overrides.insert(i as usize, x); });
}
pub unsafe fn CScnnmtblbjct_ClearPhysicsBones(obj: usize) { animation::with(obj, |p| p.overrides.clear()); }
pub unsafe fn CScnnmtblbjct_HasPhysicsBones(obj: usize) -> i32 { animation::with(obj, |p| !p.overrides.is_empty()).unwrap_or(false) as i32 }
/// An attachment by its name's token, where the bones are now: in the
/// world, or in the model's space.
pub unsafe fn CScnnmtblbjct_SBox_GetAttachment(obj: usize, name: S_Sandbox_StringToken, world: i32, tx: usize) -> i32 {
    let model = animation::model_of(obj);
    let Some(f) = file(model) else { return 0 };
    let Some(a) = f.attachments().iter().find(|a| crate::handles::murmur2(&a.name.to_string_lossy()) == name.f_Value) else { return 0 };
    let Some((bone_name, offset, rot)) = a.influences.first() else { return 0 };
    let local = Xf::of(rapier3d::math::Pose::from_parts(*offset, *rot));
    let i = f.bone(&bone_name.to_string_lossy());
    let found = animation::with(obj, |p| {
        let space = if world != 0 { &p.world } else { &p.model_space };
        match usize::try_from(i).ok().and_then(|i| space.get(i)) {
            Some(b) => b.to_world(&local),
            None if world != 0 => Xf::from_managed(&crate::state::get::<S_Transform>(obj, crate::slots::PROP_Transform)).to_world(&local),
            None => local,
        }
    });
    let Some(x) = found else { return 0 };
    unsafe { out(tx, x.managed()) };
    1
}

#[cfg(test)]
mod tests {
    //! The slots as managed calls them: a dedicated server logs on, listens,
    //! takes a player, checks their ticket, and they talk.

    /// A container as s&box compiles a GameResource: the header, a block
    /// list, and the JSON in DATA (ended with a NUL).
    fn container(blocks: &[(&[u8; 4], &[u8])]) -> Vec<u8> {
        let list = 16;
        let mut data_at = list + 12 * blocks.len();
        let mut out = vec![0u8; data_at];
        out[4..6].copy_from_slice(&12u16.to_le_bytes());
        out[8..12].copy_from_slice(&8u32.to_le_bytes());
        out[12..16].copy_from_slice(&(blocks.len() as u32).to_le_bytes());
        for (i, (tag, body)) in blocks.iter().enumerate() {
            let at = list + 12 * i;
            out[at..at + 4].copy_from_slice(&tag[..]);
            out[at + 4..at + 8].copy_from_slice(&((data_at - (at + 4)) as u32).to_le_bytes());
            out[at + 8..at + 12].copy_from_slice(&(body.len() as u32).to_le_bytes());
            out.extend_from_slice(body);
            data_at += body.len();
        }
        let n = out.len() as u32;
        out[0..4].copy_from_slice(&n.to_le_bytes());
        out
    }

    #[test]
    fn a_compiled_resource_reads_as_its_json() {
        let read = |b: &[u8]| {
            let p = unsafe { EngineGlue_ReadCompiledResourceFileJson(b.as_ptr() as usize, b.len() as i32) };
            (p != 0).then(|| unsafe { CStr::from_ptr(p as *const core::ffi::c_char) }.to_str().unwrap().to_string())
        };
        // ours: the JSON itself
        assert_eq!(read(b"  {\"Volume\":1}\n").as_deref(), Some("{\"Volume\":1}"));
        // s&box's: its DATA block, past the references and the edit info
        let c = container(&[(b"RERL", b"\x0c\0\0\0refs"), (b"REDI", &[0u8; 20]), (b"DATA", b"{\"Volume\":2}\0"), (b"BLOB", b"\x01\x02\x03")]);
        assert_eq!(read(&c).as_deref(), Some("{\"Volume\":2}"));
        assert_eq!(read(b"\x03VKV not json"), None);
        // and a block of it by name, where it lies
        let mut size = 0i32;
        let name = c"BLOB";
        let p = unsafe { EngineGlue_ReadCompiledResourceFileBlock(name.as_ptr() as usize, c.as_ptr() as usize, c.len() as i32, &mut size as *mut i32 as usize) };
        assert_eq!(unsafe { core::slice::from_raw_parts(p as *const u8, size as usize) }, b"\x01\x02\x03");
        let p = unsafe { EngineGlue_ReadCompiledResourceFileBlock(c"MDAT".as_ptr() as usize, c.as_ptr() as usize, c.len() as i32, &mut size as *mut i32 as usize) };
        assert_eq!((p, size), (0, 0), "no such block");
    }

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

#[cfg(test)]
mod scene_models {
    //! A model in the scene as managed drives it: placed, posed, merged,
    //! its bones and hitboxes read back.
    use super::*;
    use crate::assets::model::{Bone, HitBox, HitBoxSet, ModelFile, RenderMesh};
    use rapier3d::math::{Pose, Rotation, Vector};

    fn cs(s: &str) -> CString { CString::new(s).unwrap() }

    /// A root, and an arm 10 up it with a capsule hitbox along it.
    fn arm_model() -> usize {
        let bone = |name: &str, parent: i32, pos: Vector, model: Vector| Bone { name: cs(name), parent, pos, rot: Rotation::IDENTITY, flags: 0, model: Pose::from_translation(model) };
        let hit = HitBox {
            name: cs("arm"), surface: cs("flesh"), bone: cs("arm"), min: Vector::ZERO, max: Vector::new(0.0, 0.0, 8.0), radius: 2.0, shape: 2,
            bone_hash: 0, color: [0; 4], index: 0, translation_only: false, visible: false, tags: vec![cs("head")],
        };
        let mesh = RenderMesh { hitbox_sets: vec![HitBoxSet { name: cs("default"), source: cs(""), boxes: vec![hit] }], ..Default::default() };
        let file = ModelFile {
            bones: vec![bone("root", -1, Vector::ZERO, Vector::ZERO), bone("arm", 0, Vector::new(0.0, 0.0, 10.0), Vector::new(0.0, 0.0, 10.0))],
            meshes: vec![mesh],
            ..Default::default()
        };
        models::from_file(0, Some(file))
    }

    fn transform(pos: Vector, rot: Rotation) -> S_Transform { Xf { pos, rot, scale: Vector::ONE }.managed() }
    fn pos(t: S_Transform) -> Vector { phys::vec(t.f_Position) }
    fn near(a: Vector, b: Vector) -> bool { (a - b).length() < 1e-4 }

    #[test]
    fn a_scene_models_bones_follow_it_and_managed() {
        unsafe {
            let model = arm_model();
            let name = cs("ARM");
            assert_eq!(CModel_NumBones(model), 2);
            assert_eq!(CModel_FindBoneIndex(model, name.as_ptr() as usize), 1, "names are case blind");
            assert_eq!(CModel_boneParent(model, 1), 0);
            assert!(near(pos(CModel_GetBoneTransform(model, 1)), Vector::new(0.0, 0.0, 10.0)));

            // its hitbox, as ModelHitboxes reads it
            assert_eq!(CModel_GetHitboxSetCount(model, 0), 1);
            let set = CModel_GetHitboxSetByIndex(model, 0, 0);
            assert_eq!(read(Get__CHitBoxSet_m_name(set)), "default");
            assert_eq!(CHitBoxSet_numhitboxes(set), 1);
            let h = CHitBoxSet_pHitbox(set, 0);
            assert_eq!(read(Get__CHitBox_m_sBoneName(h)), "arm");
            assert_eq!(Get__CHitBox_m_nShapeType(h), 2);
            assert_eq!(Get__CHitBox_m_flShapeRadius(h), 2.0);
            assert_eq!(read(CHitBox_GetTag(h, 0)), "head");
            assert_eq!(read(CHitBox_GetTag(h, 1)), "");
            assert_eq!(CModel_GetBoneIndexForHitbox(model, h), 1);
            assert_eq!(CModel_GetHitboxSetByIndex(model, 1, 0), 0, "no second set");

            // placed at x 100, turned a quarter about z: the arm stands 10 over it
            let at = transform(Vector::new(100.0, 0.0, 0.0), Rotation::from_rotation_z(std::f32::consts::FRAC_PI_2));
            g_pMeshSystem_CreateSceneObject(model, &at as *const _ as usize, 0, 0, 0, 0, 0);
            let obj = crate::animation::find_by_model(model).unwrap();
            assert!(near(pos(CScnnmtblbjct_GetWorldSpaceRenderBoneTransform(obj, 1)), Vector::new(100.0, 0.0, 10.0)));
            assert!(near(pos(CScnnmtblbjct_GetWorldSpaceRenderBoneTransform_1(obj, name.as_ptr() as usize)), Vector::new(100.0, 0.0, 10.0)));
            assert_eq!(pos(CScnnmtblbjct_GetWorldSpaceRenderBoneTransform(obj, 7)), Vector::ZERO, "no such bone: zeroes");

            // managed moves it, and bends the arm (a procedural bone): parent space, then world
            let moved = transform(Vector::new(0.0, 50.0, 0.0), Rotation::IDENTITY);
            crate::state::set(obj, crate::slots::PROP_Transform, moved);
            CScnnmtblbjct_Update(obj, 0.016);
            let bent = transform(Vector::new(5.0, 0.0, 0.0), Rotation::IDENTITY);
            CScnnmtblbjct_SetParentSpaceBone(obj, 1, &bent as *const _ as usize);
            CScnnmtblbjct_CalculateWorldSpaceBones(obj);
            assert!(near(pos(CScnnmtblbjct_GetWorldSpaceRenderBoneTransform(obj, 1)), Vector::new(5.0, 50.0, 0.0)));
            assert!(near(pos(CScnnmtblbjct_GetWorldSpaceRenderBonePreviousTransform(obj, 1)), Vector::new(100.0, 0.0, 10.0)), "where it was");
            let mut parent_space = [transform(Vector::ZERO, Rotation::IDENTITY); 2];
            CScnnmtblbjct_GetParentSpaceBones(obj, 2, parent_space.as_mut_ptr() as usize);
            assert!(near(pos(parent_space[1]), Vector::new(5.0, 0.0, 0.0)));
            // the next update starts again from the animation's pose (bind, for now)
            CScnnmtblbjct_Update(obj, 0.016);
            CScnnmtblbjct_CalculateWorldSpaceBones(obj);
            assert!(near(pos(CScnnmtblbjct_GetWorldSpaceRenderBoneTransform(obj, 1)), Vector::new(0.0, 50.0, 10.0)));

            // a physics bone (a ragdoll's) wins, in the model's space
            let ragdoll = transform(Vector::new(0.0, 0.0, -3.0), Rotation::IDENTITY);
            CScnnmtblbjct_SetPhysicsBone(obj, 1, &ragdoll as *const _ as usize);
            assert_eq!(CScnnmtblbjct_HasPhysicsBones(obj), 1);
            CScnnmtblbjct_CalculateWorldSpaceBones(obj);
            assert!(near(pos(CScnnmtblbjct_GetWorldSpaceRenderBoneTransform(obj, 1)), Vector::new(0.0, 50.0, -3.0)));
            CScnnmtblbjct_ClearPhysicsBones(obj);
            let point = phys::v3(Vector::new(0.0, 50.0, 9.0));
            CScnnmtblbjct_CalculateWorldSpaceBones(obj);
            assert_eq!(CScnnmtblbjct_GetClosestBoneIndex(obj, &point as *const _ as usize), 1);

            // a second model merged onto the first takes its arm's pose
            let clothes = arm_model();
            g_pMeshSystem_CreateSceneObject(clothes, &moved as *const _ as usize, 0, 0, 0, 0, 0);
            let worn = crate::animation::find_by_model(clothes).unwrap();
            CScnnmtblbjct_SetParentSpaceBone(obj, 1, &bent as *const _ as usize);
            CScnnmtblbjct_CalculateWorldSpaceBones(obj);
            CScnnmtblbjct_MergeFrom(worn, obj);
            assert!(near(pos(CScnnmtblbjct_GetWorldSpaceRenderBoneTransform(worn, 1)), Vector::new(5.0, 50.0, 0.0)));
        }
    }

    #[test]
    fn a_downloaded_models_sequence_plays() {
        // s&box's own download cache, if this machine has one
        let Ok(dir) = std::env::var("ENGINE2_SAMPLE_ASSETS") else { return };
        let dir = std::path::Path::new(&dir).join("models/first_person");
        let Some(file) = std::fs::read_dir(&dir).ok().and_then(|d| d.filter_map(|e| e.ok()).map(|e| e.path()).find(|p| p.to_string_lossy().contains("v_first_person_arms_human."))) else { return };
        crate::fs::add_link("models/first_person/v_first_person_arms_human.vmdl_c", file.to_str().unwrap());
        unsafe {
            let name = cs("models/first_person/v_first_person_arms_human.vmdl");
            let model = Glue_Resources_GetModel(name.as_ptr() as usize, core::mem::zeroed());
            let at = transform(Vector::ZERO, Rotation::IDENTITY);
            g_pMeshSystem_CreateSceneObject(model, &at as *const _ as usize, 0, 0, 0, 0, 0);
            let obj = crate::animation::find_by_model(model).unwrap();
            let hand = cs("hand_R");
            let rest = pos(CScnnmtblbjct_GetWorldSpaceRenderBoneTransform_1(obj, hand.as_ptr() as usize));

            let seq = cs("Punching_Right_Hit_01");
            CScnnmtblbjct_SetSequence(obj, seq.as_ptr() as usize);
            assert_eq!(read(CScnnmtblbjct_GetSequence(obj)), "Punching_Right_Hit_01");
            let duration = CScnnmtblbjct_GetSequenceDuration(obj);
            assert!(duration > 0.1);
            CScnnmtblbjct_SetSequenceLooping(obj, 0);
            // through the punch a tenth at a time: the hand goes somewhere, and it ends
            let mut farthest = 0f32;
            for _ in 0..12 {
                CScnnmtblbjct_Update(obj, duration / 10.0);
                CScnnmtblbjct_CalculateWorldSpaceBones(obj);
                farthest = farthest.max((pos(CScnnmtblbjct_GetWorldSpaceRenderBoneTransform_1(obj, hand.as_ptr() as usize)) - rest).length());
            }
            assert!(farthest > 1.0, "the hand moved {farthest}");
            assert_eq!(CScnnmtblbjct_GetSequenceCycle(obj), 1.0);
            assert_eq!(CScnnmtblbjct_IsSequenceFinished(obj), 1);
            // looping, it comes round again
            CScnnmtblbjct_SetSequenceLooping(obj, 1);
            CScnnmtblbjct_SetSequenceCycle(obj, 0.9);
            CScnnmtblbjct_Update(obj, duration / 5.0);
            assert!((CScnnmtblbjct_GetSequenceCycle(obj) - 0.1).abs() < 1e-3);
            assert_eq!(CScnnmtblbjct_IsSequenceFinished(obj), 0);
        }
    }
}
