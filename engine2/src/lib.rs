//! The native library s&box's managed engine loads (`libengine2.so`). The
//! managed side calls one export, `igen_engine`, with a table to fill: a
//! pointer for every native function, in the order Facepunch's InteropGen
//! writes (slots.rs, generated from it by gen/), and the managed callbacks
//! native may call back into.
//!
//! Every slot is a stub for now: it does nothing, returns zeroes, and
//! records that it was called (trace.rs). Running a game against it says
//! which slots a game needs, in the order it needs them.
#[allow(dead_code)]
mod elo_overlay;
mod elo;
mod handles;
mod impls;
mod animation;
mod assets;
mod fs;
mod models;
mod net;
mod physics;
pub mod relay;
mod siwe;
mod state;
mod slots;
mod steam;
mod surfaces;
mod trace;

use std::sync::OnceLock;

/// The managed callbacks managed hands over (logging, handles, the frame),
/// copied out: managed's array is pinned only for the call that gives it.
pub static MANAGED: OnceLock<Vec<usize>> = OnceLock::new();

/// Managed callback `i` (slots.rs's M_* constants), or null before igen_engine.
pub fn managed(i: usize) -> usize {
    MANAGED.get().and_then(|m| m.get(i).copied()).unwrap_or(0)
}

/// # Safety
/// Called once by the managed side with tables of the sizes slots.rs names.
#[no_mangle]
pub unsafe extern "C" fn igen_engine(hash: i32, managed: *mut *const (), native: *mut usize, _struct_sizes: *const i32) {
    trace::start();
    if hash != slots::HASH {
        trace::note(&format!("igen_engine: managed's interop hash is {hash}, these stubs were generated for {}: regenerate", slots::HASH));
    }
    let _ = MANAGED.set(unsafe { core::slice::from_raw_parts(managed as *const usize, slots::MANAGED) }.to_vec());
    let table = slots::table();
    unsafe { core::ptr::copy_nonoverlapping(table.as_ptr(), native, slots::SLOTS) };
    trace::note(&format!("igen_engine: {} slots filled, {} managed callbacks taken", slots::SLOTS, slots::MANAGED));
}

/// Source's module interface: the managed side loads a few engine modules
/// by name (the shader compiler, `vfx_vulkan`; the file system) and asks
/// each for an interface. Their methods are slots in the table above, so
/// an interface is only an object to call them on; this library stands in
/// for those modules too (spike.sh installs it under their names).
///
/// # Safety
/// `name` is a NUL-terminated string, as Source's CreateInterface takes.
#[no_mangle]
pub unsafe extern "C" fn CreateInterface(name: *const core::ffi::c_char, return_code: *mut i32) -> usize {
    let n = if name.is_null() { String::new() } else { unsafe { core::ffi::CStr::from_ptr(name) }.to_string_lossy().into_owned() };
    trace::note(&format!("CreateInterface: {n}"));
    if !return_code.is_null() {
        unsafe { *return_code = 0 };
    }
    trace::interface(&n)
}
