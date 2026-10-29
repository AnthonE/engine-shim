//! The native library s&box's managed engine loads (`libengine2.so`). The
//! managed side calls one export, `igen_engine`, with a table to fill: a
//! pointer for every native function, in the order Facepunch's InteropGen
//! writes (slots.rs, generated from it by gen/), and the managed callbacks
//! native may call back into.
//!
//! Every slot is a stub for now: it does nothing, returns zeroes, and
//! records that it was called (trace.rs). Running a game against it says
//! which slots a game needs, in the order it needs them.
mod impls;
mod slots;
mod trace;

use std::sync::atomic::{AtomicPtr, Ordering};

/// The managed callbacks managed hands over (logging, handles, the frame).
pub static MANAGED: AtomicPtr<*const ()> = AtomicPtr::new(core::ptr::null_mut());

/// # Safety
/// Called once by the managed side with tables of the sizes slots.rs names.
#[no_mangle]
pub unsafe extern "C" fn igen_engine(hash: i32, managed: *mut *const (), native: *mut usize, _struct_sizes: *const i32) {
    trace::start();
    if hash != slots::HASH {
        trace::note(&format!("igen_engine: managed's interop hash is {hash}, these stubs were generated for {}: regenerate", slots::HASH));
    }
    MANAGED.store(managed, Ordering::SeqCst);
    let table = slots::table();
    unsafe { core::ptr::copy_nonoverlapping(table.as_ptr(), native, slots::SLOTS) };
    trace::note(&format!("igen_engine: {} slots filled, {} managed callbacks taken", slots::SLOTS, slots::MANAGED));
}
