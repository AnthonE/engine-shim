//! What native objects remember: each property a `Set…` stored, for its
//! `Get…` (a field accessor's `Set__…`/`Get__…` too), by object and by the
//! property's name. gen/ pairs them by name and by type and writes the calls;
//! a property never set reads as zeroes, as the stub did.
use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

fn store() -> &'static Mutex<HashMap<(usize, u32), Vec<u8>>> {
    static S: OnceLock<Mutex<HashMap<(usize, u32), Vec<u8>>>> = OnceLock::new();
    S.get_or_init(Default::default)
}

/// `obj`'s property `key` is now `v`.
pub fn set<T: Copy>(obj: usize, key: u32, v: T) {
    let bytes = unsafe { core::slice::from_raw_parts(&v as *const T as *const u8, core::mem::size_of::<T>()) }.to_vec();
    store().lock().unwrap().insert((obj, key), bytes);
}

/// As `set`, the value read through the pointer managed passed it by.
///
/// # Safety
/// `p` is null or points at a `T`.
pub unsafe fn set_from<T: Copy>(obj: usize, key: u32, p: usize) {
    if p != 0 {
        set(obj, key, unsafe { core::ptr::read_unaligned(p as *const T) });
    }
}

/// `obj`'s property `key`: what was last set, or zeroes.
pub fn get<T: Copy>(obj: usize, key: u32) -> T {
    let s = store().lock().unwrap();
    match s.get(&(obj, key)) {
        Some(b) if b.len() == core::mem::size_of::<T>() => unsafe { core::ptr::read_unaligned(b.as_ptr() as *const T) },
        _ => unsafe { core::mem::zeroed() },
    }
}

/// As `get`, written through the pointer managed passed for it.
///
/// # Safety
/// `out` is null or points at room for a `T`.
pub unsafe fn get_into<T: Copy>(obj: usize, key: u32, out: usize) {
    if out != 0 {
        unsafe { core::ptr::write_unaligned(out as *mut T, get::<T>(obj, key)) };
    }
}

#[cfg(test)]
mod tests {
    #[repr(C)]
    #[derive(Clone, Copy, PartialEq, Debug)]
    struct V3 { x: f32, y: f32, z: f32 }

    #[test]
    fn a_property_is_what_was_set() {
        let (a, b) = (0x1000usize, 0x2000usize);
        assert_eq!(super::get::<f32>(a, 7), 0.0);
        super::set(a, 7, 2.5f32);
        assert_eq!(super::get::<f32>(a, 7), 2.5);
        assert_eq!(super::get::<f32>(b, 7), 0.0, "per object");
        let v = V3 { x: 1.0, y: 2.0, z: 3.0 };
        unsafe { super::set_from::<V3>(a, 8, &v as *const V3 as usize) };
        assert_eq!(super::get::<V3>(a, 8), v);
        let mut out = V3 { x: 0.0, y: 0.0, z: 0.0 };
        unsafe { super::get_into::<V3>(a, 8, &mut out as *mut V3 as usize) };
        assert_eq!(out, v);
        // a different size under the same key reads as unset, never as garbage
        assert_eq!(super::get::<u64>(a, 8), 0);
    }
}
