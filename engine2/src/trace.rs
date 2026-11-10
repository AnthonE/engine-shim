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

/// A name a game asked a model or a graph for that it doesn't have (a bone,
/// a parameter, a sequence), said once each: what a stand-in for it
/// (standin.rs) would still have to answer.
pub fn asked(whose: &str, kind: &str, name: &str) {
    static SEEN: OnceLock<Mutex<std::collections::HashSet<String>>> = OnceLock::new();
    let line = format!("asked: {whose}: {kind} {name}: none");
    if SEEN.get_or_init(Default::default).lock().unwrap().insert(line.clone()) { note(&line); }
}

pub fn note(s: &str) {
    let mut w = out().lock().unwrap();
    let _ = writeln!(w, "{s}");
    let _ = w.flush();
}

/// ENGINE2_FRAME_TIMES: where a windowed client's frames go, said every 60
/// frames: each stage's average, and what whole frames took. It only reads
/// the clock (RENDER_PROFILE waits for the GPU at each stage), so the frames
/// it times are the frames a player gets.
struct FrameTimes {
    last: std::time::Instant,
    stages: Vec<(&'static str, f64)>,
    this_frame: f64,
    frames: Vec<f64>,
}

fn frame_times() -> Option<&'static Mutex<FrameTimes>> {
    static T: OnceLock<Option<Mutex<FrameTimes>>> = OnceLock::new();
    T.get_or_init(|| std::env::var_os("ENGINE2_FRAME_TIMES").map(|_| Mutex::new(FrameTimes { last: std::time::Instant::now(), stages: Vec::new(), this_frame: 0.0, frames: Vec::new() }))).as_ref()
}

/// The time since the last mark was spent on `stage`.
pub fn mark(stage: &'static str) {
    let Some(t) = frame_times() else { return };
    let mut t = t.lock().unwrap();
    let ms = t.last.elapsed().as_secs_f64() * 1000.0;
    t.last = std::time::Instant::now();
    t.this_frame += ms;
    match t.stages.iter_mut().find(|s| s.0 == stage) {
        Some(s) => s.1 += ms,
        None => t.stages.push((stage, ms)),
    }
}

pub fn frame_done() {
    let Some(t) = frame_times() else { return };
    let mut t = t.lock().unwrap();
    let whole = std::mem::take(&mut t.this_frame);
    t.frames.push(whole);
    if t.frames.len() < 60 { return; }
    let n = t.frames.len() as f64;
    let stages: Vec<String> = t.stages.iter().map(|(s, ms)| format!("{s} {:.1}", ms / n)).collect();
    t.frames.sort_by(f64::total_cmp);
    let at = |p: f64| t.frames[((n - 1.0) * p) as usize];
    note(&format!("frame times: {:.1} ms mean, {:.1} p50, {:.1} p95, {:.1} max ({})", t.frames.iter().sum::<f64>() / n, at(0.5), at(0.95), at(1.0), stages.join(", ")));
    t.frames.clear();
    t.stages.clear();
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
