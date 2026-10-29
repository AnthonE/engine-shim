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

macro_rules! flag {
    ($store:ident, $set:ident, $is:ident) => {
        static $store: AtomicI32 = AtomicI32::new(0);
        pub unsafe fn $set(_self: usize, v: i32) { $store.store(v, Ordering::Relaxed) }
        pub unsafe fn $is(_self: usize) -> i32 { $store.load(Ordering::Relaxed) }
    };
}
flag!(DEDICATED, CMtrlSystm2ppSys_SetDedicatedServer, CMtrlSystm2ppSys_IsDedicatedServer);
flag!(TOOLS, CMtrlSystm2ppSys_SetInToolsMode, CMtrlSystm2ppSys_IsInToolsMode);
flag!(TEST, CMtrlSystm2ppSys_SetInTestMode, CMtrlSystm2ppSys_IsInTestMode);
flag!(STANDALONE, CMtrlSystm2ppSys_SetInStandaloneApp, CMtrlSystm2ppSys_IsStandaloneApp);

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
