//! Handle objects: scene objects, worlds, physics bodies and the other
//! classes managed holds by an int. A native create makes the object and
//! calls managed's HandleIndex.RegisterHandle(pointer, type token), which
//! binds the managed object waiting for it (or makes one) and answers with
//! the handle; the create returns that handle. The type token is
//! StringToken's: MurmurHash2 of the type name, lowercased.
use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

use crate::slots::M_Sandbox_HandleIndex_RegisterHandle;

/// MurmurHash2 as Sandbox.System's StringExtensions.MurmurHash2 computes it
/// (seed 0x31415926, the ASCII bytes, little-endian words).
pub fn murmur2(s: &str) -> u32 {
    let bytes = s.to_ascii_lowercase().into_bytes();
    let m: u32 = 0x5bd1_e995;
    let mut h: u32 = 0x3141_5926 ^ bytes.len() as u32;
    let mut chunks = bytes.chunks_exact(4);
    for c in &mut chunks {
        let mut k = u32::from_le_bytes([c[0], c[1], c[2], c[3]]);
        k = k.wrapping_mul(m);
        k ^= k >> 24;
        k = k.wrapping_mul(m);
        h = h.wrapping_mul(m) ^ k;
    }
    let t = chunks.remainder();
    if t.len() >= 3 { h ^= (t[2] as u32) << 16; }
    if t.len() >= 2 { h ^= (t[1] as u32) << 8; }
    if !t.is_empty() { h ^= t[0] as u32; h = h.wrapping_mul(m); }
    h ^= h >> 13;
    h = h.wrapping_mul(m);
    h ^ (h >> 15)
}

/// An object made here: its handle, its type, and the objects it was made
/// with (a create's pointer arguments: the world a light is in).
struct Obj {
    handle: i32,
    ty: &'static str,
    links: Vec<usize>,
}

/// Every object made here, by its pointer.
fn index() -> &'static Mutex<HashMap<usize, Obj>> {
    static I: OnceLock<Mutex<HashMap<usize, Obj>>> = OnceLock::new();
    I.get_or_init(|| Mutex::new(HashMap::new()))
}

/// A new object of the handle type `ty`, made with `links`, registered: its
/// handle. Known before managed hears of it, because managed's init asks
/// the new object about itself (its world) inside the registering call.
pub fn create(slot: usize, ty: &'static str, links: &[usize]) -> i32 {
    let ptr = crate::trace::object(slot);
    index().lock().unwrap().insert(ptr, Obj { handle: 0, ty, links: links.iter().copied().filter(|&l| l != 0).collect() });
    let f = crate::managed(M_Sandbox_HandleIndex_RegisterHandle);
    if f == 0 { return 0; }
    let register: extern "C" fn(usize, u32) -> i32 = unsafe { std::mem::transmute(f) };
    let h = register(ptr, murmur2(ty));
    if let Some(o) = index().lock().unwrap().get_mut(&ptr) { o.handle = h; }
    h
}

/// The handle of an object made here, by its pointer (0 if none).
pub fn of(ptr: usize) -> i32 {
    index().lock().unwrap().get(&ptr).map(|o| o.handle).unwrap_or(0)
}

/// The handle of the object of type `ty` that `obj` was made with: a scene
/// object's world (0 if none).
pub fn related(obj: usize, ty: &str) -> i32 {
    let i = index().lock().unwrap();
    let Some(o) = i.get(&obj) else { return 0 };
    o.links.iter().filter_map(|l| i.get(l)).find(|l| l.ty == ty).map(|l| l.handle).unwrap_or(0)
}

#[cfg(test)]
mod tests {
    #[test]
    fn murmur2_matches_known_values() {
        // the empty string hashes the seed alone
        assert_eq!(super::murmur2(""), {
            let m: u32 = 0x5bd1_e995;
            let mut h: u32 = 0x3141_5926;
            h ^= h >> 13; h = h.wrapping_mul(m); h ^ (h >> 15)
        });
        assert_eq!(super::murmur2("SceneWorld"), super::murmur2("sceneworld"));
    }
}
