//! Sprites, as s&box batches them (`SpriteBatchSceneObject`: particles'
//! sprite renderers and `SpriteRenderer` components): a custom scene object
//! whose command list fills a buffer of `SpriteData`, runs two compute
//! shaders over it and draws it in one instanced draw of `sprite_ps`.
//! The compute shaders run here on the CPU, over the buffers' bytes as
//! the device keeps them (gpu.rs), in the order the command list issues
//! them: `sprite/sprite_cs` (each sprite's distance from the camera, a
//! velocity-facing sprite turned, a fast one's motion-blur copies, its
//! "splots", appended) and `sort_cs` (a bitonic sort of their order by
//! distance). What the draw recorded is read back as a batch in the order
//! its shader reads it (`GetSprite`): render/sprites.rs draws it.
use std::sync::Mutex;

use crate::gpu::{self, Val};

/// `SpriteData` (SpriteBatchSceneObject.cs, and the shaders' struct):
/// 37 words, packed as a structured buffer packs them.
pub const WORDS: usize = 37;
pub const STRIDE: usize = WORDS * 4;
pub mod at {
    pub const POSITION: usize = 0;
    pub const ROTATION: usize = 3;
    pub const SCALE: usize = 6;
    pub const TINT: usize = 8;
    pub const OVERLAY: usize = 9;
    pub const TEXTURE: usize = 10;
    pub const FLAGS: usize = 11;
    pub const BILLBOARD: usize = 12;
    pub const FOG_CUTOUT: usize = 13;
    pub const LIGHTING: usize = 14;
    pub const DEPTH_FEATHER: usize = 15;
    pub const SAMPLER: usize = 16;
    pub const SPLOTS: usize = 17;
    pub const SEQUENCE: usize = 18;
    pub const SEQUENCE_TIME: usize = 19;
    pub const ROTATION_OFFSET: usize = 20;
    pub const MOTION_BLUR: usize = 21;
    pub const VELOCITY: usize = 25;
    pub const SHEET: usize = 28;
    pub const OFFSET: usize = 32;
    pub const CAMERA_FADE: usize = 34;
    pub const NORMAL_TEXTURE: usize = 35;
    pub const REFRACTION: usize = 36;
}
/// SpriteFlags
pub const FLIP_X: u32 = 0x2;
pub const FLIP_Y: u32 = 0x4;
pub const TEXT: u32 = 0x10;

/// The camera a custom object is rendered for (Graphics' view while managed
/// renders it): where it is and its axes, world space (x forward, z up).
#[derive(Clone, Copy, Debug)]
pub struct Eye {
    pub position: [f32; 3],
    pub forward: [f32; 3],
    pub right: [f32; 3],
    pub up: [f32; 3],
    /// pitch, yaw, roll in degrees, as managed turned it
    pub angles: [f32; 3],
}

static EYE: Mutex<Option<Eye>> = Mutex::new(None);

/// The camera custom objects render for from here (None: none).
pub fn set_eye(eye: Option<Eye>) { *EYE.lock().unwrap() = eye; }
pub fn eye() -> Option<Eye> { *EYE.lock().unwrap() }

fn dot(a: [f32; 3], b: [f32; 3]) -> f32 { a[0] * b[0] + a[1] * b[1] + a[2] * b[2] }

fn word(r: &[u8], i: usize) -> u32 { u32::from_le_bytes(r[i * 4..i * 4 + 4].try_into().unwrap()) }
fn float(r: &[u8], i: usize) -> f32 { f32::from_bits(word(r, i)) }
fn put(r: &mut [u8], i: usize, v: u32) { r[i * 4..i * 4 + 4].copy_from_slice(&v.to_le_bytes()) }
fn put_f(r: &mut [u8], i: usize, v: f32) { put(r, i, v.to_bits()) }
fn vec3(r: &[u8], i: usize) -> [f32; 3] { [float(r, i), float(r, i + 1), float(r, i + 2)] }

fn unpack(c: u32) -> [f32; 4] { [0, 8, 16, 24].map(|s| ((c >> s) & 0xff) as f32 / 255.0) }
fn pack(c: [f32; 4]) -> u32 { c.iter().enumerate().fold(0, |a, (k, v)| a | (((v * 255.0) as u32 & 0xff) << (k * 8))) }

/// A compute shader dispatched (RenderTools.Compute: its material's mode,
/// the attributes it reads, threads in x, y, z), run here if it's one of
/// the sprite batch's; any other is said once and skipped.
pub fn dispatch(shader: &str, attrs: usize, threads: [i32; 3]) {
    let values = { let d = gpu::device(); gpu::flatten(&d, attrs) };
    let get = |n: &str| values.get(&crate::string_token(n));
    let int = |n: &str| match get(n) { Some(Val::Int(v)) => *v, Some(Val::Combo(v)) => *v as i32, Some(Val::Bool(b)) => *b as i32, Some(Val::Float(f)) => *f as i32, _ => 0 };
    let buffer = |n: &str| match get(n) { Some(Val::Buffer(b)) => *b, _ => 0 };
    if shader.contains("sprite_cs") {
        let camera = match get("CameraPosition") { Some(Val::Vec3(v)) => *v, Some(Val::Vec4(v)) => [v[0], v[1], v[2]], _ => eye().map(|e| e.position).unwrap_or_default() };
        sprite_cs(buffer("Sprites"), buffer("SpriteBufferOut"), buffer("DistanceBuffer"), buffer("AtomicCounter"), int("SpriteCount").max(0) as usize, threads[0].max(0) as usize, camera, eye());
    } else if shader.contains("line_cs") {
        crate::lines::dispatch(attrs);
    } else if shader.contains("sort_cs") {
        sort_cs(buffer("SortBuffer"), buffer("DistanceBuffer"), int("Count").max(0) as usize, int("Block").max(0) as usize, int("Dim").max(0) as usize, int("D_CLEAR") != 0, threads);
    } else {
        static SAID: Mutex<Option<std::collections::HashSet<String>>> = Mutex::new(None);
        if SAID.lock().unwrap().get_or_insert_with(Default::default).insert(shader.to_string()) {
            crate::trace::note(&format!("compute: {shader} isn't run here ({}x{}x{} threads)", threads[0], threads[1], threads[2]));
        }
    }
}

/// `sprite_cs`: each sprite copied out with its distance from the camera
/// (squared, for sorting), turned to face along its velocity on screen if
/// it asks to be, and its motion blur's copies appended after all the
/// sprites, a group of 64 at a time as the atomic counter hands them out.
fn sprite_cs(input: usize, output: usize, distances: usize, counter: usize, count: usize, threads: usize, camera: [f32; 3], eye: Option<Eye>) {
    let mut d = gpu::device();
    let Some(src) = d.buffers.get(&input).map(|b| b.data.clone()) else { return };
    let mut out = d.buffers.get(&output).map(|b| b.data.clone()).unwrap_or_default();
    let mut dist = d.buffers.get(&distances).map(|b| b.data.clone()).unwrap_or_default();
    let mut atomic = d.buffers.get(&counter).and_then(|b| b.data.get(0..4).map(|w| u32::from_le_bytes(w.try_into().unwrap()))).unwrap_or(0);
    let distance = |p: [f32; 3]| { let v = [p[0] - camera[0], p[1] - camera[1], p[2] - camera[2]]; dot(v, v) };
    let set_dist = |dist: &mut Vec<u8>, i: usize, v: f32| if i * 4 + 4 <= dist.len() { dist[i * 4..i * 4 + 4].copy_from_slice(&v.to_le_bytes()) };
    let set_out = |out: &mut Vec<u8>, i: usize, r: &[u8]| if (i + 1) * STRIDE <= out.len() { out[i * STRIDE..(i + 1) * STRIDE].copy_from_slice(r) };
    let groups = threads.div_ceil(64);
    for g in 0..groups {
        let mut sprites: Vec<(usize, Vec<u8>)> = Vec::new();
        for i in g * 64..(g + 1) * 64 {
            if i >= count || (i + 1) * STRIDE > src.len() {
                set_dist(&mut dist, i, f32::MAX);
                continue;
            }
            let mut s = src[i * STRIDE..(i + 1) * STRIDE].to_vec();
            let offset = float(&s, at::ROTATION_OFFSET);
            if offset > -1.0 {
                // the velocity on screen: its angle from up towards right, in degrees
                if let Some(e) = eye {
                    let v = vec3(&s, at::VELOCITY);
                    let (x, y) = (dot(v, e.right), dot(v, e.up));
                    let len = (x * x + y * y).sqrt();
                    let (x, y) = if len > 0.0 { (x / len, y / len) } else { (f32::NAN, f32::NAN) };
                    let roll = float(&s, at::ROTATION + 2) + x.atan2(y).to_degrees() + offset;
                    if roll.is_finite() { put_f(&mut s, at::ROTATION + 2, roll); }
                }
            }
            set_dist(&mut dist, i, distance(vec3(&s, at::POSITION)));
            set_out(&mut out, i, &s);
            sprites.push((i, s));
        }
        // the group's copies, written after every sprite at what the counter gives it
        let base = atomic as usize;
        let total: usize = sprites.iter().map(|(_, s)| splots(s)).sum();
        atomic += total as u32;
        let mut write = count + base;
        for (_, s) in &sprites {
            let n = word(s, at::SPLOTS) as i32;
            if n <= 0 { continue; }
            let blur = [float(s, at::MOTION_BLUR), float(s, at::MOTION_BLUR + 1), float(s, at::MOTION_BLUR + 2), float(s, at::MOTION_BLUR + 3)];
            let leading = blur[0] > 1.0;
            let mut tint = unpack(word(s, at::TINT));
            let overlay = unpack(word(s, at::OVERLAY));
            if overlay[3] > 0.0 { for k in 0..3 { tint[k] += (overlay[k] - tint[k]) * overlay[3]; } }
            tint[3] *= blur[3];
            let velocity = vec3(s, at::VELOCITY);
            let step = velocity.map(|v| v * blur[2] * 0.002);
            let position = vec3(s, at::POSITION);
            for f in 0..n {
                tint[3] = (tint[3] * blur[3]).clamp(0.001, 1.0);
                let mut b = s.clone();
                put(&mut b, at::TINT, pack(tint));
                let moved = step.map(|v| v * f as f32);
                for (sign, wanted) in [(1.0f32, leading), (-1.0, true)] {
                    if !wanted { continue; }
                    let p = [position[0] + sign * moved[0], position[1] + sign * moved[1], position[2] + sign * moved[2]];
                    for k in 0..3 { put_f(&mut b, at::POSITION + k, p[k]); }
                    set_dist(&mut dist, write, distance(p));
                    set_out(&mut out, write, &b);
                    write += 1;
                }
            }
        }
    }
    if let Some(b) = d.buffers.get_mut(&output) { b.data = out; }
    if let Some(b) = d.buffers.get_mut(&distances) { b.data = dist; }
    if let Some(b) = d.buffers.get_mut(&counter) { if b.data.len() >= 4 { b.data[0..4].copy_from_slice(&atomic.to_le_bytes()); } }
}

/// How many copies a sprite's motion blur makes (its splots, twice over
/// with a leading trail).
fn splots(s: &[u8]) -> usize {
    let n = word(s, at::SPLOTS) as i32;
    if n <= 0 { return 0; }
    n as usize * if float(s, at::MOTION_BLUR) > 1.0 { 2 } else { 1 }
}

/// `sort_cs`: with D_CLEAR, the order made 0, 1, 2... and every distance
/// the farthest; otherwise one step of a bitonic sort (`Block` apart, in
/// runs of `Dim`), ascending by distance.
fn sort_cs(order: usize, distances: usize, count: usize, block: usize, dim: usize, clear: bool, threads: [i32; 3]) {
    const MAX_DIM_THREADS: usize = 256 * 1024;
    let mut d = gpu::device();
    let Some(mut lut) = d.buffers.get(&order).map(|b| b.data.clone()) else { return };
    let Some(mut dist) = d.buffers.get(&distances).map(|b| b.data.clone()) else { return };
    let n = count.min(lut.len() / 4).min(dist.len() / 4);
    let u = |b: &[u8], i: usize| u32::from_le_bytes(b[i * 4..i * 4 + 4].try_into().unwrap());
    let f = |b: &[u8], i: usize| f32::from_le_bytes(b[i * 4..i * 4 + 4].try_into().unwrap());
    let all = (threads[0].max(1) as usize).min(MAX_DIM_THREADS) * threads[1].max(1) as usize;
    if clear {
        for i in 0..n.min(all.max(n)) {
            lut[i * 4..i * 4 + 4].copy_from_slice(&(i as u32).to_le_bytes());
            dist[i * 4..i * 4 + 4].copy_from_slice(&f32::MAX.to_le_bytes());
        }
        if let Some(b) = d.buffers.get_mut(&distances) { b.data = dist; }
    } else if block > 0 {
        for i in 0..n {
            let j = i ^ block;
            if j >= n || j < i { continue; }
            let (a, b) = (u(&lut, i) as usize, u(&lut, j) as usize);
            let (da, db) = (if a < dist.len() / 4 { f(&dist, a) } else { f32::MAX }, if b < dist.len() / 4 { f(&dist, b) } else { f32::MAX });
            let ascending = i & dim == 0;
            if (da - db) * if ascending { 1.0 } else { -1.0 } > 0.0 {
                lut[i * 4..i * 4 + 4].copy_from_slice(&(b as u32).to_le_bytes());
                lut[j * 4..j * 4 + 4].copy_from_slice(&(a as u32).to_le_bytes());
            }
        }
    }
    if let Some(b) = d.buffers.get_mut(&order) { b.data = lut; }
}

/// A batch of sprites as its draw would read them: in drawing order (the
/// sorted ones farthest first), each its 37 words.
#[derive(Clone, Debug, Default)]
pub struct Batch {
    pub records: Vec<u8>,
    /// D_BLEND: added onto what's behind, rather than blended over it
    pub additive: bool,
    /// D_OPAQUE: writes depth
    pub opaque: bool,
}

impl Batch {
    pub fn len(&self) -> usize { self.records.len() / STRIDE }
    pub fn is_empty(&self) -> bool { self.records.is_empty() }
    pub fn record(&self, i: usize) -> &[u8] { &self.records[i * STRIDE..(i + 1) * STRIDE] }
    pub fn word(&self, i: usize, w: usize) -> u32 { word(self.record(i), w) }
}

/// A recorded draw of `sprite_ps`, read as its vertex shader reads it: the
/// instance'th sprite is `Sprites[SortLUT[SpriteCount - 1 - instance]]`
/// once sorted (the nearest last), else the instance'th.
pub fn batch(draw: &gpu::Draw) -> Option<Batch> {
    if !crate::camera::material_shader(draw.material).is_some_and(|s| s.contains("sprite_ps")) { return None; }
    let get = |n: &str| draw.attrs.get(&crate::string_token(n));
    let int = |n: &str| match get(n) { Some(Val::Int(v)) => *v, Some(Val::Combo(v)) => *v as i32, Some(Val::Bool(b)) => *b as i32, _ => 0 };
    let bytes = |n: &str| match get(n) { Some(Val::Buffer(b)) => draw.buffers.get(b).cloned(), _ => None };
    let sprites = bytes("Sprites")?;
    let lut = bytes("SortLUT");
    let count = int("SpriteCount").max(0) as usize;
    let instances = (draw.instances.max(0) as usize).min(count.max(draw.instances.max(0) as usize));
    let sorted = int("IsSorted") == 1;
    let mut records = Vec::with_capacity(instances * STRIDE);
    for i in 0..instances {
        let k = if sorted {
            let at = count.checked_sub(1 + i)?;
            lut.as_ref().and_then(|l| l.get(at * 4..at * 4 + 4)).map(|w| u32::from_le_bytes(w.try_into().unwrap()) as usize)?
        } else { i };
        if let Some(r) = sprites.get(k * STRIDE..(k + 1) * STRIDE) { records.extend_from_slice(r); }
    }
    Some(Batch { records, additive: int("D_BLEND") == 1, opaque: int("D_OPAQUE") == 1 })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sprite(position: [f32; 3]) -> Vec<u8> {
        let mut s = vec![0u8; STRIDE];
        for k in 0..3 { put_f(&mut s, at::POSITION + k, position[k]); }
        put_f(&mut s, at::ROTATION_OFFSET, -1.0);
        put(&mut s, at::TINT, 0xff80_8080);
        s
    }

    fn floats(b: usize) -> Vec<f32> { gpu::device().buffers[&b].data.chunks_exact(4).map(|w| f32::from_le_bytes(w.try_into().unwrap())).collect() }
    fn words(b: usize) -> Vec<u32> { gpu::device().buffers[&b].data.chunks_exact(4).map(|w| u32::from_le_bytes(w.try_into().unwrap())).collect() }

    #[test]
    fn sprites_are_sorted_farthest_first_and_blurred_into_copies() {
        // three sprites along x, the camera at 0: the middle one moving, blurred into 2 copies
        let size = 8;
        let (input, output) = (gpu::new_buffer(0, size, STRIDE as i32, "Sprites"), gpu::new_buffer(0, size, STRIDE as i32, "SpriteBufferOut"));
        let (order, distances, counter) = (gpu::new_buffer(0, size, 4, "sort"), gpu::new_buffer(0, size, 4, "distance"), gpu::new_buffer(0, 1, 4, "atomic"));
        let mut data = Vec::new();
        for x in [100.0, 300.0, 200.0] { data.extend(sprite([x, 0.0, 0.0])); }
        put(&mut data[STRIDE..2 * STRIDE], at::SPLOTS, 2);
        put_f(&mut data[STRIDE..2 * STRIDE], at::MOTION_BLUR, 1.0);
        put_f(&mut data[STRIDE..2 * STRIDE], at::MOTION_BLUR + 2, 1.0);
        put_f(&mut data[STRIDE..2 * STRIDE], at::MOTION_BLUR + 3, 0.5);
        put_f(&mut data[STRIDE..2 * STRIDE], at::VELOCITY, 1000.0);
        gpu::write_buffer(input, &data, 0);

        let attrs = gpu::new_attrs();
        let set = |n: &str, v: Val| gpu::set(attrs, crate::string_token(n), v);
        set("SortBuffer", Val::Buffer(order));
        set("DistanceBuffer", Val::Buffer(distances));
        set("Count", Val::Int(size));
        set("D_CLEAR", Val::Combo(1));
        dispatch("shaders/sort_cs.shader", attrs, [size, 1, 1]);
        assert_eq!(words(order), (0..8).collect::<Vec<u32>>());
        set("Sprites", Val::Buffer(input));
        set("SpriteBufferOut", Val::Buffer(output));
        set("SpriteCount", Val::Int(3));
        set("AtomicCounter", Val::Buffer(counter));
        set("CameraPosition", Val::Vec3([0.0; 3]));
        dispatch("shaders/sprite/sprite_cs.shader", attrs, [3, 1, 1]);
        assert_eq!(words(counter)[0], 2, "the moving sprite's two copies");
        let dist = floats(distances);
        assert_eq!(&dist[0..3], &[10000.0, 90000.0, 40000.0]);
        // its copies trail behind it: 300 - 1000 * 0.002 * f
        assert_eq!(&dist[3..5], &[90000.0, 298.0 * 298.0]);
        assert_eq!(dist[5], f32::MAX);
        set("D_CLEAR", Val::Combo(0));
        let mut dim = 2;
        while dim <= size as usize {
            set("Dim", Val::Int(dim as i32));
            let mut block = dim >> 1;
            while block > 0 { set("Block", Val::Int(block as i32)); dispatch("shaders/sort_cs.shader", attrs, [size, 1, 1]); block >>= 1; }
            dim <<= 1;
        }
        // nearest first, the unused slots last; the moving sprite and its first copy tie
        let w = words(order);
        assert_eq!(&w[0..3], &[0, 2, 4]);
        assert!(w[3..5] == [1, 3] || w[3..5] == [3, 1]);

        // the draw reads them back farthest first
        let ctx = gpu::new_context();
        let draw_attrs = gpu::new_attrs();
        let dset = |n: &str, v: Val| gpu::set(draw_attrs, crate::string_token(n), v);
        dset("Sprites", Val::Buffer(output));
        dset("SortLUT", Val::Buffer(order));
        dset("IsSorted", Val::Int(1));
        dset("SpriteCount", Val::Int(5));
        dset("D_BLEND", Val::Combo(1));
        let material = crate::camera::material("sprite batch test");
        crate::camera::set_material_shader(material, "shaders/sprite/sprite_ps.shader");
        gpu::set_state(ctx, draw_attrs, material);
        gpu::record(ctx, 5, 0, 6, 5, 0, true);
        let c = gpu::take_context(ctx).unwrap();
        let b = batch(&c.draws[0]).unwrap();
        assert_eq!(b.len(), 5);
        assert!(b.additive && !b.opaque);
        let xs: Vec<f32> = (0..5).map(|i| f32::from_bits(b.word(i, at::POSITION))).collect();
        assert_eq!(xs, vec![300.0, 300.0, 298.0, 200.0, 100.0]);
        // each copy fainter: the blur's opacity, once before them and again each copy
        assert_eq!(unpack(b.word(2, at::TINT))[3], 31.0 / 255.0, "the second copy: 1 x 0.5 x 0.5 x 0.5");
    }
}
