//! Which slots are called, and in what order: each slot's first call is
//! written as it happens (`ENGINE2_TRACE`, a file, or stderr), and every
//! slot's count when the process exits.
use std::fs::File;
use std::io::Write;
use std::sync::atomic::{AtomicU32, AtomicUsize, Ordering};
use std::sync::{Mutex, OnceLock};

use crate::slots::{NAMES, SLOTS};

static COUNTS: [AtomicU32; SLOTS] = [const { AtomicU32::new(0) }; SLOTS];
static FIRSTS: AtomicUsize = AtomicUsize::new(0);
static OUT: OnceLock<Mutex<Box<dyn Write + Send>>> = OnceLock::new();

fn out() -> &'static Mutex<Box<dyn Write + Send>> {
    OUT.get_or_init(|| {
        let w: Box<dyn Write + Send> = match std::env::var("ENGINE2_TRACE") {
            Ok(p) => Box::new(File::create(p).expect("ENGINE2_TRACE: can't write there")),
            Err(_) => Box::new(std::io::stderr()),
        };
        Mutex::new(w)
    })
}

pub fn note(s: &str) {
    let mut w = out().lock().unwrap();
    let _ = writeln!(w, "{s}");
    let _ = w.flush();
}

extern "C" fn at_exit() {
    let mut w = out().lock().unwrap();
    let called: Vec<usize> = (0..SLOTS).filter(|&i| COUNTS[i].load(Ordering::Relaxed) > 0).collect();
    let _ = writeln!(w, "# exit: {} of {} slots called", called.len(), SLOTS);
    for i in called {
        let _ = writeln!(w, "# count {:>4} {:>9} {}", i, COUNTS[i].load(Ordering::Relaxed), NAMES[i]);
    }
    let _ = w.flush();
}

extern "C" {
    fn atexit(f: extern "C" fn()) -> i32;
}

pub fn start() {
    unsafe { atexit(at_exit) };
}

#[inline]
pub fn hit(i: usize) {
    if COUNTS[i].fetch_add(1, Ordering::Relaxed) == 0 {
        let n = FIRSTS.fetch_add(1, Ordering::Relaxed) + 1;
        note(&format!("first {n:>4}: slot {i:>4} {}", NAMES[i]));
    }
}

/// Slot 0: native's error callback, which managed reads as a function.
pub extern "C" fn debug_error(msg: *const core::ffi::c_char) {
    let s = if msg.is_null() { String::new() } else { unsafe { core::ffi::CStr::from_ptr(msg) }.to_string_lossy().into_owned() };
    note(&format!("Debug_Error: {s}"));
}

/// A stub's pointer: a fresh block of zeroed memory each call, so every
/// object a game makes is a different one, until a slot has made OBJECTS;
/// then that slot's last, again. Never freed: a stub engine lives a while.
const OBJECT_BYTES: usize = 1024;
const OBJECTS: u32 = 4096;
static LAST: [AtomicUsize; SLOTS] = [const { AtomicUsize::new(0) }; SLOTS];
pub fn object(i: usize) -> usize {
    if COUNTS[i].load(Ordering::Relaxed) > OBJECTS {
        let p = LAST[i].load(Ordering::Relaxed);
        if p != 0 {
            return p;
        }
    }
    let p = Box::into_raw(Box::new([0u64; OBJECT_BYTES / 8])) as usize;
    LAST[i].store(p, Ordering::Relaxed);
    p
}

/// One object per interface name, the same each time it is asked for.
pub fn interface(name: &str) -> usize {
    static ALL: Mutex<Vec<(String, usize)>> = Mutex::new(Vec::new());
    let mut all = ALL.lock().unwrap();
    if let Some((_, p)) = all.iter().find(|(n, _)| n == name) {
        return *p;
    }
    let p = Box::into_raw(Box::new([0u64; OBJECT_BYTES / 8])) as usize;
    all.push((name.to_string(), p));
    p
}
