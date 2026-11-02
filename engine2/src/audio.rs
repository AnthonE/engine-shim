//! Sound: s&box mixes in C# (Sandbox.Audio.MixingThread), and native gives
//! it what to mix with and where to send it.
//!
//! - **The device.** Stereo at 44.1 kHz. A thread of ours asks managed for
//!   each 512 samples (`MixOneBuffer`) as a sound card would, at the pace
//!   it plays them; managed mixes its voices and sends the mix back
//!   (`SendOutput`). Where it goes: a WAV file at `ENGINE2_AUDIO_CAPTURE`,
//!   else nowhere yet (the client's window will bring a sound card).
//! - **Sounds** (`CSfxTable`): a compiled sound (`.vsnd_c`: a header in its
//!   DATA block, the samples after the resource) or, in a project, its
//!   source (`.wav`, `.mp3`), decoded once into floats.
//! - **Mixers** (`CAudioMixer`): a voice's place in its sound, read 512
//!   samples at a time at a pitch (linear between samples), looping where
//!   the sound says, waiting out a delay, stopping at an end.
//! - **Mix buffers** (`CAudioMixBuffer`, `CAudioMixDeviceBuffers`): 512
//!   floats each, managed's to fill through their pointer, mixed here.
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Mutex, MutexGuard, OnceLock};
use std::time::{Duration, Instant};

pub const RATE: u32 = 44_100;
pub const CHANNELS: usize = 2;
/// MIX_BUFFER_SIZE: what managed mixes at a time.
pub const SAMPLES: usize = 512;

// ---- sounds

/// vsound_format_t (managed's SoundFormat).
pub const PCM16: i32 = 0;
pub const PCM8: i32 = 1;
pub const MP3: i32 = 2;

#[derive(Default)]
pub struct Sound {
    pub name: String,
    pub rate: u32,
    pub channels: usize,
    /// the format it came in (vsound_format_t)
    pub format: i32,
    /// interleaved, -1 to 1: `frames * channels`
    pub data: Vec<f32>,
    /// the frame a loop goes back to (none: it doesn't loop)
    pub loop_start: Option<u32>,
    /// the frame a loop ends at (none: the sound's end)
    pub loop_end: Option<u32>,
    pub failed: bool,
}

impl Sound {
    pub fn frames(&self) -> usize {
        if self.channels == 0 { 0 } else { self.data.len() / self.channels }
    }

    pub fn duration(&self) -> f32 {
        if self.rate == 0 { 0.0 } else { self.frames() as f32 / self.rate as f32 }
    }
}

fn i16s(b: &[u8]) -> impl Iterator<Item = f32> + '_ {
    b.chunks_exact(2).map(|c| i16::from_le_bytes([c[0], c[1]]) as f32 / 32768.0)
}

fn u8s(b: &[u8]) -> impl Iterator<Item = f32> + '_ {
    b.iter().map(|&x| (x as f32 - 128.0) / 128.0)
}

/// PCM or MP3 bytes as floats.
fn decode(format: i32, bytes: &[u8]) -> Result<Vec<f32>, String> {
    match format {
        PCM16 => Ok(i16s(bytes).collect()),
        PCM8 => Ok(u8s(bytes).collect()),
        MP3 => mp3(bytes).map(|s| s.data),
        f => Err(format!("format {f} isn't decoded here")),
    }
}

/// A compiled sound: its DATA block's header (rate, format, channels, loop
/// start, frames, duration; the samples' size at 28), the samples after
/// the resource's blocks (where its first word says it ends).
pub fn vsnd(name: &str, bytes: &[u8]) -> Result<Sound, String> {
    let r = crate::assets::resource::Resource::parse(bytes.to_vec())?;
    let d = r.block("DATA").ok_or("no DATA")?;
    if d.len() < 32 { return Err("DATA too short".into()); }
    let u32_at = |at: usize| u32::from_le_bytes(d[at..at + 4].try_into().unwrap());
    let rate = u16::from_le_bytes([d[0], d[1]]) as u32;
    let (format, channels) = (d[2] as i32, d[3] as usize);
    let loop_start = i32::from_le_bytes(d[4..8].try_into().unwrap());
    let frames = u32_at(8);
    let size = u32_at(28) as usize;
    let start = u32::from_le_bytes(bytes.get(..4).ok_or("empty")?.try_into().unwrap()) as usize;
    let samples = bytes.get(start..(start + size).min(bytes.len())).ok_or("no samples")?;
    let mut s = match format {
        MP3 => mp3(samples)?,
        _ => Sound { rate, channels: channels.max(1), data: decode(format, samples)?, ..Default::default() },
    };
    s.name = name.to_string();
    s.format = format;
    if format != MP3 { s.data.truncate(frames as usize * s.channels); }
    s.loop_start = (loop_start >= 0).then_some(loop_start as u32);
    Ok(s)
}

/// A WAV file (PCM 8, 16, 24 or 32-bit, or float), its `smpl` loop if it has one.
pub fn wav(bytes: &[u8]) -> Result<Sound, String> {
    if bytes.get(..4) != Some(b"RIFF") || bytes.get(8..12) != Some(b"WAVE") { return Err("not a WAV".into()); }
    let (mut fmt, mut data, mut looped) = (None, None, None);
    let mut at = 12;
    while at + 8 <= bytes.len() {
        let id = &bytes[at..at + 4];
        let n = u32::from_le_bytes(bytes[at + 4..at + 8].try_into().unwrap()) as usize;
        let body = &bytes[at + 8..(at + 8 + n).min(bytes.len())];
        match id {
            b"fmt " if body.len() >= 16 => fmt = Some(body),
            b"data" => data = Some(body),
            b"smpl" if body.len() >= 36 + 24 && u32::from_le_bytes(body[28..32].try_into().unwrap()) > 0 => {
                let l = &body[36..];
                looped = Some((u32::from_le_bytes(l[8..12].try_into().unwrap()), u32::from_le_bytes(l[12..16].try_into().unwrap())));
            }
            _ => {}
        }
        at += 8 + n + (n & 1);
    }
    let (fmt, data) = (fmt.ok_or("no fmt")?, data.ok_or("no data")?);
    let tag = u16::from_le_bytes([fmt[0], fmt[1]]);
    let channels = u16::from_le_bytes([fmt[2], fmt[3]]).max(1) as usize;
    let rate = u32::from_le_bytes(fmt[4..8].try_into().unwrap());
    let bits = u16::from_le_bytes([fmt[14], fmt[15]]);
    // WAVE_FORMAT_EXTENSIBLE: the real tag is its sub-format's first word
    let tag = if tag == 0xfffe && fmt.len() >= 26 { u16::from_le_bytes([fmt[24], fmt[25]]) } else { tag };
    let samples: Vec<f32> = match (tag, bits) {
        (1, 8) => u8s(data).collect(),
        (1, 16) => i16s(data).collect(),
        (1, 24) => data.chunks_exact(3).map(|c| (i32::from_le_bytes([0, c[0], c[1], c[2]]) >> 8) as f32 / 8_388_608.0).collect(),
        (1, 32) => data.chunks_exact(4).map(|c| i32::from_le_bytes(c.try_into().unwrap()) as f32 / 2_147_483_648.0).collect(),
        (3, 32) => data.chunks_exact(4).map(|c| f32::from_le_bytes(c.try_into().unwrap())).collect(),
        _ => return Err(format!("WAV format {tag}, {bits} bits, isn't read here")),
    };
    Ok(Sound { rate, channels, format: if bits == 8 { PCM8 } else { PCM16 }, data: samples, loop_start: looped.map(|l| l.0), loop_end: looped.map(|l| l.1 + 1), ..Default::default() })
}

/// An MP3: stereo as it decodes, kept mono if it's mono.
pub fn mp3(bytes: &[u8]) -> Result<Sound, String> {
    let (header, samples) = puremp3::read_mp3(bytes).map_err(|e| format!("mp3: {e:?}"))?;
    let mono = matches!(header.channels, puremp3::Channels::Mono);
    let data: Vec<f32> = if mono { samples.map(|(l, _)| l).collect() } else { samples.flat_map(|(l, r)| [l, r]).collect() };
    Ok(Sound { rate: header.sample_rate.hz(), channels: if mono { 1 } else { 2 }, format: MP3, data, ..Default::default() })
}

/// A sound by its game path (`sounds/x.vsnd`): compiled, or its source.
pub fn load(name: &str) -> Sound {
    let base = name.trim_end_matches("_c");
    let stem = base.strip_suffix(".vsnd").unwrap_or(base);
    let found = if let Some(b) = crate::fs::read_compiled(&format!("{stem}.vsnd")) {
        vsnd(name, &b)
    } else if let Some(b) = crate::fs::read(&format!("{stem}.wav")) {
        wav(&b)
    } else if let Some(b) = crate::fs::read(&format!("{stem}.mp3")) {
        mp3(&b)
    } else {
        Err("not found".into())
    };
    match found {
        Ok(mut s) => { s.name = name.to_string(); s }
        Err(e) => {
            crate::trace::note(&format!("audio: {name}: {e}"));
            Sound { name: name.to_string(), failed: true, ..Default::default() }
        }
    }
}

fn sounds() -> MutexGuard<'static, HashMap<usize, Arc<Sound>>> {
    static S: OnceLock<Mutex<HashMap<usize, Arc<Sound>>>> = OnceLock::new();
    S.get_or_init(Default::default).lock().unwrap_or_else(|e| e.into_inner())
}

/// A sound: loaded once by name, the same object after.
pub fn precache(name: &str) -> usize {
    static BY_NAME: OnceLock<Mutex<HashMap<String, usize>>> = OnceLock::new();
    let key = crate::fs::normal(name);
    let mut by_name = BY_NAME.get_or_init(Default::default).lock().unwrap();
    if let Some(&p) = by_name.get(&key) { return p; }
    let p = add(load(name));
    by_name.insert(key, p);
    p
}

/// A sound made from managed's PCM (CreateSound): `bytes` of `format`.
pub fn create(name: &str, channels: i32, rate: i32, format: i32, frames: i32, loop_start: i32, loop_end: i32, bytes: &[u8]) -> usize {
    let mut s = match decode(format, bytes) {
        Ok(data) => Sound { name: name.to_string(), rate: rate.max(1) as u32, channels: channels.max(1) as usize, format, data, ..Default::default() },
        Err(e) => { crate::trace::note(&format!("audio: {name}: {e}")); Sound { name: name.to_string(), failed: true, ..Default::default() } }
    };
    if format != MP3 && frames > 0 { s.data.truncate(frames as usize * s.channels); }
    s.loop_start = (loop_start >= 0).then_some(loop_start as u32);
    s.loop_end = (loop_end > 0).then_some(loop_end as u32);
    add(s)
}

pub fn add(s: Sound) -> usize {
    let p = crate::handles::object();
    sounds().insert(p, Arc::new(s));
    p
}

pub fn sound(p: usize) -> Option<Arc<Sound>> {
    sounds().get(&p).cloned()
}

// ---- mixers: a voice's place in its sound

pub struct Mixer {
    pub sound: Arc<Sound>,
    pub sfx: usize,
    /// in the sound's frames
    pub position: f64,
    pub end: Option<u32>,
    pub looping: bool,
    /// output samples still to wait before it starts
    pub delay: i64,
    pub time_scale: f32,
}

fn mixers() -> MutexGuard<'static, HashMap<usize, Mixer>> {
    static M: OnceLock<Mutex<HashMap<usize, Mixer>>> = OnceLock::new();
    M.get_or_init(Default::default).lock().unwrap_or_else(|e| e.into_inner())
}

/// A mixer of a sound (none if it can't play).
pub fn new_mixer(sfx: usize) -> usize {
    let Some(sound) = sound(sfx).filter(|s| !s.failed && s.frames() > 0) else { return 0 };
    let looping = sound.loop_start.is_some();
    let p = crate::handles::object();
    mixers().insert(p, Mixer { sound, sfx, position: 0.0, end: None, looping, delay: 0, time_scale: 1.0 });
    p
}

pub fn with_mixer<T>(p: usize, f: impl FnOnce(&mut Mixer) -> T) -> Option<T> {
    mixers().get_mut(&p).map(f)
}

pub fn drop_mixer(p: usize) {
    mixers().remove(&p);
}

impl Mixer {
    /// The frame it stops at (or loops back from).
    fn last(&self) -> f64 {
        let n = self.sound.frames() as u32;
        let end = if self.looping { self.sound.loop_end.unwrap_or(n) } else { n };
        self.end.map_or(end, |e| e.min(end)).min(n) as f64
    }

    pub fn finished(&self) -> bool {
        !self.looping && self.position >= self.last()
    }

    /// The next 512 samples into `out` (one buffer per channel, as many as
    /// the sound has), at `pitch` (1: as recorded, at its own rate).
    pub fn read(&mut self, pitch: f32, out: &mut [&mut [f32; SAMPLES]]) {
        let s = self.sound.clone();
        if self.position == 0.0 && self.delay == 0 {
            crate::trace::note(&format!("audio: {} read from its start, at pitch {pitch} ({} frames at {} Hz, {} channels)", s.name, s.frames(), s.rate, s.channels));
        }
        let ch = s.channels;
        let step = (pitch.max(0.0) * self.time_scale.max(0.0)) as f64 * s.rate as f64 / RATE as f64;
        let last = self.last();
        let back = s.loop_start.unwrap_or(0) as f64;
        for i in 0..SAMPLES {
            if self.delay > 0 {
                self.delay -= 1;
                for b in out.iter_mut() { b[i] = 0.0; }
                continue;
            }
            if self.position >= last {
                if self.looping && last > back { self.position = back + (self.position - last) % (last - back); } else {
                    for b in out.iter_mut() { b[i] = 0.0; }
                    continue;
                }
            }
            let f = self.position.floor();
            let (a, t) = (f as usize, (self.position - f) as f32);
            let next = if a + 1 < last as usize { a + 1 } else if self.looping { back as usize } else { a };
            for (c, b) in out.iter_mut().enumerate().take(ch) {
                let x0 = s.data.get(a * ch + c).copied().unwrap_or(0.0);
                let x1 = s.data.get(next * ch + c).copied().unwrap_or(x0);
                b[i] = x0 + (x1 - x0) * t;
            }
            self.position += step;
        }
    }
}

// ---- mix buffers

#[repr(C)]
pub struct MixBuf(pub [f32; SAMPLES]);

/// CAudioMixDeviceBuffers: a mix buffer per channel, side by side.
pub struct DeviceBuffers {
    pub channels: usize,
    pub bufs: Box<[MixBuf]>,
}

pub fn new_buffer() -> usize {
    Box::into_raw(Box::new(MixBuf([0.0; SAMPLES]))) as usize
}

/// # Safety
/// `p` came from `new_buffer`, and is freed once.
pub unsafe fn free_buffer(p: usize) {
    if p != 0 { drop(unsafe { Box::from_raw(p as *mut MixBuf) }) }
}

/// # Safety
/// `p` is a mix buffer (its own, or one of a device buffers').
pub unsafe fn buffer<'a>(p: usize) -> Option<&'a mut [f32; SAMPLES]> {
    if p == 0 { None } else { Some(unsafe { &mut (*(p as *mut MixBuf)).0 }) }
}

pub fn new_device_buffers(channels: i32) -> usize {
    let channels = channels.clamp(1, 8) as usize;
    let bufs = (0..channels).map(|_| MixBuf([0.0; SAMPLES])).collect();
    Box::into_raw(Box::new(DeviceBuffers { channels, bufs })) as usize
}

/// # Safety
/// `p` came from `new_device_buffers`, and is freed once.
pub unsafe fn free_device_buffers(p: usize) {
    if p != 0 { drop(unsafe { Box::from_raw(p as *mut DeviceBuffers) }) }
}

/// # Safety
/// `p` is a device buffers.
pub unsafe fn device_buffers<'a>(p: usize) -> Option<&'a mut DeviceBuffers> {
    if p == 0 { None } else { Some(unsafe { &mut *(p as *mut DeviceBuffers) }) }
}

/// A buffer's peak, and its mean level.
pub fn levels(b: &[f32]) -> (f32, f32) {
    let peak = b.iter().fold(0f32, |m, x| m.max(x.abs()));
    (peak, b.iter().map(|x| x.abs()).sum::<f32>() / b.len().max(1) as f32)
}

/// `into` plus `from`, scaled from `a` to `b` across the buffer.
pub fn mix_ramp(into: &mut [f32; SAMPLES], from: &[f32; SAMPLES], a: f32, b: f32) {
    for i in 0..SAMPLES {
        into[i] += from[i] * (a + (b - a) * i as f32 / SAMPLES as f32);
    }
}

pub fn ramp(into: &mut [f32; SAMPLES], a: f32, b: f32) {
    for (i, x) in into.iter_mut().enumerate() {
        *x *= a + (b - a) * i as f32 / SAMPLES as f32;
    }
}

/// `ENGINE2_AUDIO_DEBUG`: each stage's peak, now and then.
pub fn debug(stage: &str, b: &[f32]) {
    static ON: OnceLock<bool> = OnceLock::new();
    if !*ON.get_or_init(|| std::env::var("ENGINE2_AUDIO_DEBUG").is_ok()) { return; }
    static SEEN: Mutex<Option<HashMap<String, (u64, f32)>>> = Mutex::new(None);
    let mut g = SEEN.lock().unwrap();
    let e = g.get_or_insert_with(Default::default).entry(stage.to_string()).or_insert((0, 0.0));
    e.0 += 1;
    e.1 = e.1.max(levels(b).0);
    if e.0 % 200 == 0 { crate::trace::note(&format!("audio debug: {stage} x{} peak {:.4}", e.0, e.1)); e.1 = 0.0; }
}

// ---- Steam Audio's effects, as managed calls them: simpler, ours

/// An effect's state between buffers: the gains it last reached, so a
/// change of gain glides across a buffer rather than clicking at its edge.
#[derive(Default)]
pub struct Effect {
    gains: [f32; 2],
    started: bool,
}

pub fn new_effect() -> usize {
    Box::into_raw(Box::<Effect>::default()) as usize
}

/// # Safety
/// `p` came from `new_effect`, and is freed once.
pub unsafe fn free_effect(p: usize) {
    if p != 0 { drop(unsafe { Box::from_raw(p as *mut Effect) }) }
}

/// # Safety
/// `p` is an effect.
pub unsafe fn effect<'a>(p: usize) -> Option<&'a mut Effect> {
    if p == 0 { None } else { Some(unsafe { &mut *(p as *mut Effect) }) }
}

/// The direct path (CDirectEffect): distance, air and what's in the way, as
/// one gain (Steam Audio applies the three bands as an EQ; here their
/// mean), from the last buffer's gain to this one's across it.
pub fn direct(e: &mut Effect, distance: f32, air: [f32; 3], occlusion: f32, transmission: [f32; 3], input: &DeviceBuffers, output: &mut DeviceBuffers) {
    let band = |i: usize| air[i] * (occlusion + (1.0 - occlusion) * transmission[i]);
    let gain = (distance * (band(0) + band(1) + band(2)) / 3.0).max(0.0);
    debug("direct in", &input.bufs[0].0);
    debug("direct gain", &[gain]);
    let from = if e.started { e.gains[0] } else { gain };
    e.gains[0] = gain;
    e.started = true;
    for (o, i) in output.bufs.iter_mut().zip(input.bufs.iter()) {
        o.0.fill(0.0);
        mix_ramp(&mut o.0, &i.0, from, gain);
    }
}

/// Where a sound comes from (CBinauralEffect), as managed's own fallback
/// (BinauralEffect, `snd_steamaudio 0`) places it: each side's gain from
/// how far to that side and how far ahead (in the listener's space, x
/// ahead, y left), to the 0.8th power, blended with the sound as it came;
/// Steam Audio's HRTF in its place.
pub fn binaural(e: &mut Effect, direction: [f32; 3], blend: f32, input: &DeviceBuffers, output: &mut DeviceBuffers) {
    let n = input.channels.max(1);
    let mut mono = [0f32; SAMPLES];
    for b in input.bufs.iter() { for (m, x) in mono.iter_mut().zip(b.0.iter()) { *m += x / n as f32; } }
    let len = (direction[0] * direction[0] + direction[1] * direction[1] + direction[2] * direction[2]).sqrt().max(1e-6);
    let (ahead, left) = (direction[0] / len, direction[1] / len);
    let ahead = if ahead < 0.0 { ahead * -0.5 } else { ahead };
    let side = |l: f32| ((l.clamp(0.0, 1.0) + ahead).powf(0.8) * blend.clamp(0.0, 1.0)).clamp(0.0001, 1.0);
    let targets = [side(left), side(-left)];
    for (c, o) in output.bufs.iter_mut().enumerate().take(2) {
        o.0.fill(0.0);
        // as it came, unspatialized, for the part not blended
        if blend < 1.0 { mix_ramp(&mut o.0, &input.bufs[c.min(input.channels - 1)].0, 1.0 - blend.max(0.0), 1.0 - blend.max(0.0)); }
        let from = if e.started { e.gains[c] } else { targets[c] };
        // managed's smoothing: a tenth of the way each buffer
        let to = from + (targets[c] - from) * 0.1;
        e.gains[c] = to;
        mix_ramp(&mut o.0, &mono, from, to);
    }
    e.started = true;
    debug("binaural in", &mono);
    debug("binaural out", &output.bufs[0].0);
}

// ---- the device: a thread asking managed for each buffer, and what's sent

/// Buffers mixed, and the loudest of the last second's.
pub static MIXED: AtomicU64 = AtomicU64::new(0);
static RUNNING: AtomicBool = AtomicBool::new(false);

/// What the mix is sent to.
struct Output {
    capture: Option<Wav>,
    /// the last second's peak, and when it was said
    peak: f32,
    said: u64,
}

fn output() -> MutexGuard<'static, Output> {
    static O: OnceLock<Mutex<Output>> = OnceLock::new();
    O.get_or_init(|| {
        let capture = std::env::var("ENGINE2_AUDIO_CAPTURE").ok().filter(|p| !p.is_empty()).and_then(|p| match Wav::create(&p) {
            Ok(w) => { crate::trace::note(&format!("audio: the mix is captured to {p}")); Some(w) }
            Err(e) => { crate::trace::note(&format!("audio: can't capture to {p}: {e}")); None }
        });
        Mutex::new(Output { capture, peak: 0.0, said: 0 })
    }).lock().unwrap_or_else(|e| e.into_inner())
}

/// Managed's mix, sent (SendOutput): `channels` buffers of 512.
pub fn send(buffers: &DeviceBuffers) {
    let n = MIXED.fetch_add(1, Ordering::Relaxed) + 1;
    let mut o = output();
    let mut interleaved = Vec::with_capacity(SAMPLES * CHANNELS);
    for i in 0..SAMPLES {
        for c in 0..CHANNELS {
            // mono to both sides; past two, the first two
            let b = &buffers.bufs[c.min(buffers.channels - 1)].0;
            interleaved.push(b[i]);
        }
    }
    o.peak = o.peak.max(levels(&interleaved).0);
    debug("sent", &interleaved);
    if let Some(w) = &mut o.capture { w.write(&interleaved); }
    #[cfg(feature = "render")]
    card::play(&interleaved);
    // a line a second while anything sounds
    let per_second = (RATE as u64).div_ceil(SAMPLES as u64);
    if n - o.said >= per_second {
        if o.peak > 0.0 { crate::trace::note(&format!("audio: {n} buffers mixed, the last second's peak {:.3}", o.peak)); }
        o.peak = 0.0;
        o.said = n;
    }
}

/// The mixing thread, once: managed's MixOneBuffer every 512 samples' time.
pub fn start() {
    if RUNNING.swap(true, Ordering::AcqRel) { return; }
    let f = crate::managed(crate::slots::M_SandboxAudio_MixingThread_MixOneBuffer);
    if f == 0 { RUNNING.store(false, Ordering::Release); return; }
    let mix: extern "C" fn() = unsafe { std::mem::transmute(f) };
    let period = Duration::from_secs_f64(SAMPLES as f64 / RATE as f64);
    std::thread::Builder::new().name("audio mix".into()).spawn(move || {
        crate::trace::note("audio: mixing, 512 samples at a time at 44.1 kHz");
        let mut next = Instant::now();
        let (mut spent, mut calls, mut dropped) = (Duration::ZERO, 0u32, 0u32);
        loop {
            let began = Instant::now();
            mix();
            spent += began.elapsed();
            calls += 1;
            // every ten seconds' worth: how long managed takes for each (11.6 ms is all a buffer has)
            if calls == 861 {
                crate::trace::note(&format!("audio: managed mixed {calls} buffers at {:.2} ms each; {dropped} times behind", spent.as_secs_f64() * 1000.0 / calls as f64));
                (spent, calls, dropped) = (Duration::ZERO, 0, 0);
            }
            next += period;
            let now = Instant::now();
            // behind by more than a few buffers: catch up from now, as a card would drop
            if now > next + period * 4 { next = now; dropped += 1; }
            if next > now { std::thread::sleep(next - now); }
        }
    }).expect("audio: a thread");
}

// ---- a sound card (a client's): the mix through a short queue its callback
// drains, at the card's own rate if it isn't 44.1 kHz

#[cfg(feature = "render")]
mod card {
    use super::{CHANNELS, RATE};
    use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
    use std::collections::VecDeque;
    use std::sync::{Arc, Mutex, OnceLock};

    /// What's queued for the card: interleaved stereo at 44.1 kHz.
    type Queue = Arc<Mutex<VecDeque<f32>>>;
    /// The most a queue holds: 150 ms; past it the oldest goes (the mix ran ahead).
    const MOST: usize = RATE as usize * CHANNELS * 15 / 100;

    struct Card { queue: Queue, _stream: cpal::Stream }
    // cpal's stream is kept only to keep it playing; nothing else touches it
    unsafe impl Send for Card {}
    unsafe impl Sync for Card {}

    fn card() -> Option<&'static Card> {
        static C: OnceLock<Option<Card>> = OnceLock::new();
        C.get_or_init(|| {
            if std::env::var("ENGINE2_AUDIO_DEVICE").is_ok_and(|v| v == "none") { return None; }
            open().map_err(|e| crate::trace::note(&format!("audio: no sound card: {e}"))).ok()
        }).as_ref()
    }

    fn open() -> Result<Card, String> {
        let host = cpal::default_host();
        let device = host.default_output_device().ok_or("no output device")?;
        let config = device.default_output_config().map_err(|e| e.to_string())?;
        let (rate, channels) = (config.sample_rate().0, config.channels() as usize);
        let queue: Queue = Arc::new(Mutex::new(VecDeque::with_capacity(MOST)));
        let q = queue.clone();
        let mut r = Resampler::new(rate, channels);
        let mut fill = move |out: &mut [f32]| r.fill(&mut q.lock().unwrap(), out);
        let err = |e| crate::trace::note(&format!("audio: the sound card: {e}"));
        let stream = match config.sample_format() {
            cpal::SampleFormat::F32 => device.build_output_stream(&config.into(), move |o: &mut [f32], _| fill(o), err, None),
            cpal::SampleFormat::I16 => {
                let mut buf = Vec::new();
                device.build_output_stream(&config.into(), move |o: &mut [i16], _| {
                    buf.resize(o.len(), 0.0);
                    fill(&mut buf);
                    for (d, s) in o.iter_mut().zip(&buf) { *d = (s.clamp(-1.0, 1.0) * 32767.0) as i16; }
                }, err, None)
            }
            f => return Err(format!("a sample format of {f:?}")),
        }.map_err(|e| e.to_string())?;
        stream.play().map_err(|e| e.to_string())?;
        crate::trace::note(&format!("audio: playing on {} ({rate} Hz, {channels} channels)", device.name().unwrap_or_default()));
        Ok(Card { queue, _stream: stream })
    }

    /// Our 44.1 kHz stereo to a card's rate and channels: between our frames,
    /// linearly; silence when the queue runs dry.
    pub struct Resampler {
        step: f64,
        at: f64,
        channels: usize,
        last: [f32; CHANNELS],
    }

    impl Resampler {
        pub fn new(rate: u32, channels: usize) -> Resampler {
            Resampler { step: RATE as f64 / rate.max(1) as f64, at: 1.0, channels: channels.max(1), last: [0.0; CHANNELS] }
        }

        pub fn fill(&mut self, q: &mut VecDeque<f32>, out: &mut [f32]) {
            for frame in out.chunks_mut(self.channels) {
                while self.at >= 1.0 {
                    self.at -= 1.0;
                    if q.len() >= CHANNELS { for c in 0..CHANNELS { self.last[c] = q.pop_front().unwrap(); } } else { self.last = [0.0; CHANNELS]; }
                }
                let next = [q.front().copied().unwrap_or(self.last[0]), q.get(1).copied().unwrap_or(self.last[1])];
                for (c, x) in frame.iter_mut().enumerate() {
                    let k = c.min(CHANNELS - 1);
                    *x = self.last[k] + (next[k] - self.last[k]) * self.at as f32;
                }
                self.at += self.step;
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn the_mix_reaches_a_card_at_its_rate() {
            // a ramp, left rising and right falling, at 44.1 kHz
            let mut q: VecDeque<f32> = (0..441).flat_map(|i| [i as f32 / 441.0, -(i as f32) / 441.0]).collect();
            // a 44.1 kHz stereo card: as it is
            let mut r = Resampler::new(RATE, 2);
            let mut out = vec![0f32; 20];
            r.fill(&mut q, &mut out);
            assert_eq!(&out[..6], &[0.0, 0.0, 1.0 / 441.0, -1.0 / 441.0, 2.0 / 441.0, -2.0 / 441.0]);
            // a 48 kHz one: each of its frames 0.91875 of ours on, between them
            let mut q: VecDeque<f32> = (0..441).flat_map(|i| [i as f32, 0.0]).collect();
            let mut r = Resampler::new(48_000, 2);
            let mut out = vec![0f32; 480 * 2];
            r.fill(&mut q, &mut out);
            assert!((out[2 * 100] - 100.0 * 44_100.0 / 48_000.0).abs() < 1e-3, "{}", out[200]);
            // a 5.1 card: our left and right, the rest the right again
            let mut q: VecDeque<f32> = [0.5, -0.5].into();
            let mut r = Resampler::new(RATE, 6);
            let mut out = vec![9f32; 12];
            r.fill(&mut q, &mut out);
            assert_eq!(&out[..6], &[0.5, -0.5, -0.5, -0.5, -0.5, -0.5]);
            // and dry, silence
            assert!(out[6..].iter().all(|&x| x == 0.0));
        }
    }

    /// The mix, to the card (opened the first time: a player's, never a server's).
    pub fn play(interleaved: &[f32]) {
        let Some(c) = card() else { return };
        let mut q = c.queue.lock().unwrap();
        q.extend(interleaved.iter().copied());
        let over = q.len().saturating_sub(MOST);
        q.drain(..over - over % CHANNELS);
    }
}

// ---- a WAV file, written as it goes (its sizes fixed up each write)

pub struct Wav {
    file: std::fs::File,
    frames: u32,
}

impl Wav {
    pub fn create(path: &str) -> std::io::Result<Wav> {
        let mut w = Wav { file: std::fs::File::create(path)?, frames: 0 };
        w.header()?;
        Ok(w)
    }

    fn header(&mut self) -> std::io::Result<()> {
        use std::io::{Seek, SeekFrom, Write};
        let data = self.frames * CHANNELS as u32 * 2;
        let mut h = Vec::with_capacity(44);
        h.extend_from_slice(b"RIFF");
        h.extend_from_slice(&(36 + data).to_le_bytes());
        h.extend_from_slice(b"WAVEfmt ");
        h.extend_from_slice(&16u32.to_le_bytes());
        h.extend_from_slice(&1u16.to_le_bytes());
        h.extend_from_slice(&(CHANNELS as u16).to_le_bytes());
        h.extend_from_slice(&RATE.to_le_bytes());
        h.extend_from_slice(&(RATE * CHANNELS as u32 * 2).to_le_bytes());
        h.extend_from_slice(&(CHANNELS as u16 * 2).to_le_bytes());
        h.extend_from_slice(&16u16.to_le_bytes());
        h.extend_from_slice(b"data");
        h.extend_from_slice(&data.to_le_bytes());
        self.file.seek(SeekFrom::Start(0))?;
        self.file.write_all(&h)?;
        self.file.seek(SeekFrom::End(0)).map(|_| ())
    }

    pub fn write(&mut self, interleaved: &[f32]) {
        use std::io::Write;
        let bytes: Vec<u8> = interleaved.iter().flat_map(|x| ((x.clamp(-1.0, 1.0) * 32767.0) as i16).to_le_bytes()).collect();
        if self.file.write_all(&bytes).is_ok() {
            self.frames += (interleaved.len() / CHANNELS) as u32;
            let _ = self.header();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pcm16_wav(channels: u16, rate: u32, samples: &[i16]) -> Vec<u8> {
        let data: Vec<u8> = samples.iter().flat_map(|s| s.to_le_bytes()).collect();
        let mut b = Vec::new();
        b.extend_from_slice(b"RIFF");
        b.extend_from_slice(&(36 + data.len() as u32).to_le_bytes());
        b.extend_from_slice(b"WAVEfmt ");
        b.extend_from_slice(&16u32.to_le_bytes());
        b.extend_from_slice(&1u16.to_le_bytes());
        b.extend_from_slice(&channels.to_le_bytes());
        b.extend_from_slice(&rate.to_le_bytes());
        b.extend_from_slice(&(rate * channels as u32 * 2).to_le_bytes());
        b.extend_from_slice(&(channels * 2).to_le_bytes());
        b.extend_from_slice(&16u16.to_le_bytes());
        b.extend_from_slice(b"data");
        b.extend_from_slice(&(data.len() as u32).to_le_bytes());
        b.extend_from_slice(&data);
        b
    }

    fn mixer_of(s: Sound) -> Mixer {
        Mixer { sound: Arc::new(s), sfx: 0, position: 0.0, end: None, looping: false, delay: 0, time_scale: 1.0 }
    }

    #[test]
    fn a_wav_reads_and_plays_at_its_pitch() {
        // a ramp of 1,000 frames, stereo, the right side negated
        let samples: Vec<i16> = (0..1000).flat_map(|i| [i as i16 * 16, -(i as i16 * 16)]).collect();
        let s = wav(&pcm16_wav(2, 44_100, &samples)).unwrap();
        assert_eq!((s.channels, s.rate, s.frames()), (2, 44_100, 1000));
        assert!((s.duration() - 1000.0 / 44_100.0).abs() < 1e-6);
        let mut m = mixer_of(s);
        let (mut l, mut r) = ([0f32; SAMPLES], [0f32; SAMPLES]);
        m.read(1.0, &mut [&mut l, &mut r]);
        assert_eq!(l[100], 1600.0 / 32768.0);
        assert_eq!(r[100], -l[100]);
        // an octave up: twice as far through it each sample, and done sooner
        m.position = 0.0;
        m.read(2.0, &mut [&mut l, &mut r]);
        assert_eq!(l[100], 3200.0 / 32768.0);
        // 1,000 frames two at a time: the last at sample 499, silence after
        assert!(m.finished());
        assert_ne!(l[499], 0.0);
        assert!(l[500..].iter().all(|&x| x == 0.0), "silence after it ends");
    }

    #[test]
    fn a_sound_at_half_the_rate_is_read_at_half_the_step() {
        // 22,050 Hz, mono: each output sample half a frame on, between them
        let s = Sound { rate: 22_050, channels: 1, data: vec![0.0, 1.0, 0.0, 1.0], ..Default::default() };
        let mut m = mixer_of(s);
        let mut b = [0f32; SAMPLES];
        m.read(1.0, &mut [&mut b]);
        assert_eq!(&b[..4], &[0.0, 0.5, 1.0, 0.5]);
    }

    #[test]
    fn loops_delays_and_ends() {
        let s = Sound { rate: RATE, channels: 1, data: (0..10).map(|i| i as f32).collect(), loop_start: Some(4), ..Default::default() };
        let mut m = mixer_of(s);
        m.looping = true;
        m.delay = 3;
        let mut b = [0f32; SAMPLES];
        m.read(1.0, &mut [&mut b]);
        // three samples' wait, then 0..9, then 4..9 again and again
        assert_eq!(&b[..20], &[0.0, 0.0, 0.0, 0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 4.0]);
        assert!(!m.finished(), "a loop never finishes");
        // looping off, and an end set: it stops there
        m.looping = false;
        m.position = 0.0;
        m.end = Some(5);
        m.read(1.0, &mut [&mut b]);
        assert_eq!(&b[..7], &[0.0, 1.0, 2.0, 3.0, 4.0, 0.0, 0.0]);
        assert!(m.finished());
    }

    #[test]
    fn mix_buffers_mix() {
        let (a, b) = (new_buffer(), new_buffer());
        let (x, y) = unsafe { (buffer(a).unwrap(), buffer(b).unwrap()) };
        x.fill(0.5);
        y.fill(1.0);
        mix_ramp(x, y, 0.0, 1.0);
        assert_eq!(x[0], 0.5);
        assert!((x[256] - 1.0).abs() < 1e-6);
        ramp(y, 1.0, 0.0);
        assert!((y[256] - 0.5).abs() < 1e-6);
        assert_eq!(levels(&[0.5, -1.0, 0.0, 0.5]), (1.0, 0.5));
        let d = new_device_buffers(2);
        assert_eq!(unsafe { device_buffers(d) }.unwrap().bufs.len(), 2);
        unsafe { free_buffer(a); free_buffer(b); free_device_buffers(d); }
    }

    #[test]
    fn a_compiled_sound_reads() {
        // s&box's download cache, if this machine has one
        let Ok(dir) = std::env::var("ENGINE2_SAMPLE_ASSETS") else { return };
        let dir = std::path::Path::new(&dir).join("weapons/rust_python/sounds");
        let Some(file) = std::fs::read_dir(&dir).ok().and_then(|d| d.filter_map(|e| e.ok()).map(|e| e.path()).find(|p| p.to_string_lossy().contains("python_dryfire."))) else { return };
        let s = vsnd("python_dryfire.vsnd", &std::fs::read(file).unwrap()).unwrap();
        assert_eq!((s.rate, s.channels, s.format, s.frames(), s.loop_start), (44_100, 1, PCM16, 15_151, None));
        assert!((s.duration() - 0.3436).abs() < 1e-3);
        let (peak, mean) = levels(&s.data);
        eprintln!("python_dryfire: peak {peak:.3}, mean {mean:.4}");
        assert!(peak > 0.1 && peak <= 1.0, "a click you can hear");
    }

    #[test]
    fn effects_attenuate_and_place() {
        let input = unsafe { device_buffers(new_device_buffers(2)) }.unwrap();
        let output = unsafe { device_buffers(new_device_buffers(2)) }.unwrap();
        for b in input.bufs.iter_mut() { b.0.fill(1.0); }
        let mut e = Effect::default();
        // half the distance gain, open air, nothing in the way
        direct(&mut e, 0.5, [1.0; 3], 1.0, [1.0; 3], input, output);
        assert!(output.bufs[0].0.iter().all(|&x| (x - 0.5).abs() < 1e-6));
        // fully behind a wall that lets a fifth through: from 0.5 to 0.1 across the buffer
        direct(&mut e, 0.5, [1.0; 3], 0.0, [0.2; 3], input, output);
        assert!((output.bufs[0].0[0] - 0.5).abs() < 1e-6 && (output.bufs[0].0[511] - 0.1).abs() < 2e-3);

        // straight to the left: the left side louder, the right near silent
        let mut b = Effect::default();
        binaural(&mut b, [0.0, 1.0, 0.0], 1.0, input, output);
        assert!(output.bufs[0].0[256] > 0.9 && output.bufs[1].0[256] < 0.01, "{} {}", output.bufs[0].0[256], output.bufs[1].0[256]);
        // straight ahead: both sides alike
        let mut c = Effect::default();
        binaural(&mut c, [1.0, 0.0, 0.0], 1.0, input, output);
        assert!((output.bufs[0].0[256] - output.bufs[1].0[256]).abs() < 1e-6 && output.bufs[0].0[256] > 0.9);
    }

    #[test]
    fn a_wav_written_reads_back() {
        let path = std::env::temp_dir().join(format!("engine2-audio-{}.wav", std::process::id()));
        let mut w = Wav::create(path.to_str().unwrap()).unwrap();
        w.write(&[0.5, -0.5, 0.25, -0.25]);
        drop(w);
        let s = wav(&std::fs::read(&path).unwrap()).unwrap();
        assert_eq!((s.channels, s.rate, s.frames()), (2, RATE, 2));
        assert!((s.data[0] - 0.5).abs() < 1e-3 && (s.data[3] + 0.25).abs() < 1e-3);
        let _ = std::fs::remove_file(path);
    }
}
