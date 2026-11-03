//! Lines, as s&box draws them (`SceneLineObject`: a `LineRenderer`, a beam,
//! a particle trail): a custom scene object whose command list uploads its
//! points, runs `line_cs` to turn them into a strip of vertices and the
//! indices that join them (with caps at either end), and draws them with
//! `line.shader`. `line_cs` runs here on the CPU, over the buffers' bytes
//! (gpu.rs), as the sprite batch's compute shaders do (sprites.rs); a draw
//! whose vertices it wrote is read back as a line for render/lines.rs.
use std::collections::HashSet;
use std::sync::Mutex;

use crate::gpu::{self, Val};

/// `LinePoint` (SceneLineObject.cs): an offset, position, normal, colour
/// (four floats), width and texture coordinate: 13 words.
const POINT: usize = 13;
/// `LineVertex`: position, normal, tangent, colour (four floats), uv: 15 floats.
pub const VERTEX: usize = 15;
pub const VERTEX_BYTES: usize = VERTEX * 4;

/// The vertex buffers line_cs has written: a draw from one of them is a line's.
static LINE_VERTICES: Mutex<Option<HashSet<usize>>> = Mutex::new(None);

type V3 = [f32; 3];
fn add(a: V3, b: V3) -> V3 { [a[0] + b[0], a[1] + b[1], a[2] + b[2]] }
fn sub(a: V3, b: V3) -> V3 { [a[0] - b[0], a[1] - b[1], a[2] - b[2]] }
fn mul(a: V3, s: f32) -> V3 { [a[0] * s, a[1] * s, a[2] * s] }
fn dot(a: V3, b: V3) -> f32 { a[0] * b[0] + a[1] * b[1] + a[2] * b[2] }
fn cross(a: V3, b: V3) -> V3 { [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]] }
fn length(a: V3) -> f32 { dot(a, a).sqrt() }
/// HLSL's normalize, but nothing for nothing (where it would give NaNs)
fn normalize(a: V3) -> V3 { let l = length(a); if l > 0.0 { mul(a, 1.0 / l) } else { [0.0; 3] } }

#[derive(Clone, Copy, Default, Debug)]
struct Point { position: V3, normal: V3, color: [f32; 4], width: f32, uv: f32 }

#[derive(Clone, Copy, Default, Debug, PartialEq)]
pub struct Vertex { pub position: V3, pub normal: V3, pub tangent: V3, pub color: [f32; 4], pub uv: [f32; 2] }

impl Vertex {
    fn words(&self) -> [f32; VERTEX] {
        let p = self;
        [p.position[0], p.position[1], p.position[2], p.normal[0], p.normal[1], p.normal[2], p.tangent[0], p.tangent[1], p.tangent[2],
         p.color[0], p.color[1], p.color[2], p.color[3], p.uv[0], p.uv[1]]
    }
    pub fn from_words(w: &[f32]) -> Vertex {
        Vertex { position: [w[0], w[1], w[2]], normal: [w[3], w[4], w[5]], tangent: [w[6], w[7], w[8]], color: [w[9], w[10], w[11], w[12]], uv: [w[13], w[14]] }
    }
}

/// SceneLineObject.FaceMode, CapStyle
const CAMERA: i32 = 0;
const CYLINDER: i32 = 2;
const TRIANGLE: i32 = 1;
const ARROW: i32 = 2;
const ROUNDED: i32 = 3;

struct Shape { face: i32, start_cap: i32, end_cap: i32, rounded: u32, tessellation: u32, camera: V3 }

/// Which way the line faces at a point, and across it: its normal, the
/// tangent its width is laid along, and the binormal (CalculateLineOrientation).
fn orientation(s: &Shape, count: usize, prev: &Point, cur: &Point, next: &Point) -> (V3, V3, V3) {
    let normal = if s.face == CAMERA {
        normalize(sub(cur.position, s.camera))
    } else if count == 2 {
        let dir = normalize(sub(next.position, prev.position));
        let mut n = sub(cur.normal, mul(dir, dot(cur.normal, dir)));
        if length(n) < 1e-4 {
            let up = if dot(dir, [0.0, 1.0, 0.0]).abs() > 0.9 { [1.0, 0.0, 0.0] } else { [0.0, 1.0, 0.0] };
            n = sub(up, mul(dir, dot(up, dir)));
        }
        normalize(n)
    } else {
        normalize(cur.normal)
    };
    let (mut coming, mut going) = (sub(prev.position, cur.position), sub(cur.position, next.position));
    if s.face == CAMERA {
        coming = sub(coming, mul(normal, dot(coming, normal)));
        going = sub(going, mul(normal, dot(going, normal)));
    }
    let average = normalize(add(normalize(coming), normalize(going)));
    let tangent = normalize(cross(average, normal));
    (normal, tangent, normalize(cross(tangent, normal)))
}

/// A point round a cylinder of `width` at `angle` (CalculateCylindricalVertex).
fn cylindrical(position: V3, tangent: V3, normal: V3, width: f32, angle: f32) -> (V3, V3) {
    let n = normalize(add(mul(tangent, angle.sin()), mul(normal, angle.cos())));
    (add(position, add(mul(tangent, angle.sin() * width), mul(normal, angle.cos() * width))), n)
}

/// A point's left and right edges (CalculateLineVertices).
fn edges(s: &Shape, count: usize, width: f32, prev: &Point, cur: &Point, next: &Point) -> (Vertex, Vertex) {
    let (normal, tangent, binormal) = orientation(s, count, prev, cur, next);
    if s.face == CYLINDER {
        let (lp, ln) = cylindrical(cur.position, tangent, normal, width, 0.0);
        let (rp, rn) = cylindrical(cur.position, tangent, normal, width, std::f32::consts::PI);
        (Vertex { position: lp, normal: ln, tangent: binormal, color: cur.color, uv: [0.0, cur.uv] },
         Vertex { position: rp, normal: rn, tangent: binormal, color: cur.color, uv: [1.0, cur.uv] })
    } else {
        let left = Vertex { position: add(cur.position, mul(tangent, width)), normal: mul(normal, -1.0), tangent, color: cur.color, uv: [0.0, cur.uv] };
        (left, Vertex { position: sub(cur.position, mul(tangent, width)), uv: [1.0, cur.uv], ..left })
    }
}

fn cap_vertices(s: &Shape, cap: i32) -> u32 { match cap { TRIANGLE => 1, ARROW => 3, ROUNDED => s.rounded + 2, _ => 0 } }
fn cap_indices(s: &Shape, cap: i32) -> u32 { match cap { TRIANGLE | ARROW => 3, ROUNDED => s.rounded * 3, _ => 0 } }

/// `line_cs`: each point's row of vertices across the line (TessellationLevel
/// + 1 of them), the quads joining each row to the next, and the caps
/// after them all. Returns the vertices and indices as written.
fn line_cs(points: &[Point], s: &Shape, vertices: &mut [Vertex], indices: &mut [u32]) {
    let n = points.len();
    if n < 2 { return; }
    let t = s.tessellation.max(1);
    let mut set_v = |vertices: &mut [Vertex], i: usize, v: Vertex| if let Some(x) = vertices.get_mut(i) { *x = v };
    let set_i = |indices: &mut [u32], i: usize, v: u32| if let Some(x) = indices.get_mut(i) { *x = v };
    for i in 0..n {
        let cur = points[i];
        let mut prev = points[i.saturating_sub(1)];
        let mut next = points[(i + 1).min(n - 1)];
        if i == 0 { prev.position = add(cur.position, sub(cur.position, points[1].position)); }
        else if i == n - 1 { next.position = sub(next.position, sub(prev.position, cur.position)); }
        let (normal, tangent, binormal) = orientation(s, n, &prev, &cur, &next);
        let base = i * (t as usize + 1);
        for k in 0..=t {
            let u = k as f32 / t as f32;
            let v = if s.face == CYLINDER {
                let (p, vn) = cylindrical(cur.position, tangent, normal, -cur.width, u * std::f32::consts::PI * 2.0);
                Vertex { position: p, normal: mul(vn, -1.0), tangent: normalize(cross(binormal, vn)), color: cur.color, uv: [u, cur.uv] }
            } else {
                Vertex { position: add(cur.position, mul(tangent, cur.width * (1.0 - 2.0 * u))), normal: mul(normal, -1.0), tangent, color: cur.color, uv: [u, cur.uv] }
            };
            set_v(vertices, base + k as usize, v);
        }
        if i < n - 1 {
            for k in 0..t {
                let row = t + 1;
                let (v0, v1) = (i as u32 * row + k, i as u32 * row + k + 1);
                let (v2, v3) = ((i as u32 + 1) * row + k, (i as u32 + 1) * row + k + 1);
                let at = (i as u32 * t * 6 + k * 6) as usize;
                for (j, x) in [v0, v2, v1, v1, v2, v3].into_iter().enumerate() { set_i(indices, at + j, x); }
            }
        }
    }
    // the caps, after every row and quad
    let (vo, io) = (n as u32 * (t + 1), (n as u32 - 1) * t * 6);
    cap(points, s, vertices, indices, vo, io, s.start_cap, true);
    cap(points, s, vertices, indices, vo + cap_vertices(s, s.start_cap), io + cap_indices(s, s.start_cap), s.end_cap, false);
}

/// A cap at the start or end of the line (AddCap): a triangle, an arrow's
/// head, or a fan of `RoundedCapSegments`.
#[allow(clippy::too_many_arguments)]
fn cap(points: &[Point], s: &Shape, vertices: &mut [Vertex], indices: &mut [u32], vo: u32, io: u32, kind: i32, start: bool) {
    let n = points.len();
    if !matches!(kind, TRIANGLE | ARROW | ROUNDED) || n < 2 { return; }
    let t = s.tessellation.max(1);
    let (at, adjacent) = if start { (points[0], points[1]) } else { (points[n - 1], points[n - 2]) };
    let mut beyond = at;
    beyond.position = add(at.position, sub(at.position, adjacent.position));
    let base = if start { 0 } else { (n as u32 - 1) * (t + 1) };
    let (left, right) = if start { edges(s, n, at.width, &beyond, &at, &adjacent) } else { edges(s, n, at.width, &adjacent, &at, &beyond) };
    let direction = normalize(sub(at.position, adjacent.position));
    let tangent = normalize(sub(right.position, left.position));
    let half = length(sub(right.position, left.position)) * 0.5;
    let (center, normal, y) = (at.position, left.normal, at.uv);
    let dy = if start { -1.0 } else { 1.0 };
    let mut v = |i: u32, x: Vertex| if let Some(slot) = vertices.get_mut(i as usize) { *slot = x };
    let mut tri = |k: u32, a: u32, b: u32, c: u32| for (j, x) in [a, b, c].into_iter().enumerate() { if let Some(slot) = indices.get_mut((io + k * 3) as usize + j) { *slot = x } };
    match kind {
        TRIANGLE => {
            v(vo, Vertex { position: add(center, mul(direction, half)), tangent, normal, uv: [0.5, y + dy * 0.1], ..left });
            if start { tri(0, base, base + t, vo) } else { tri(0, base, vo, base + t) }
        }
        ARROW => {
            let across = mul(tangent, half * 2.0);
            v(vo, Vertex { position: add(center, across), normal, uv: [0.0, y + dy * 0.1], ..left });
            v(vo + 1, Vertex { position: sub(center, across), normal, uv: [1.0, y + dy * 0.1], ..right });
            v(vo + 2, Vertex { position: add(center, mul(direction, half * 2.0)), tangent, normal, uv: [0.5, y + dy * 0.1], ..left });
            if start { tri(0, vo, vo + 1, vo + 2) } else { tri(0, vo, vo + 2, vo + 1) }
        }
        _ => {
            let middle = Vertex { position: center, tangent, normal, uv: [0.5, y], ..left };
            v(vo, middle);
            let turn = if start { -1.0f32 } else { 1.0 };
            for k in 0..=s.rounded {
                let f = k as f32 / s.rounded.max(1) as f32;
                let angle = -std::f32::consts::FRAC_PI_2 + std::f32::consts::PI * f;
                let p = add(center, add(mul(tangent, half * (angle * turn).sin()), mul(direction, half * (angle * turn).cos())));
                v(vo + 1 + k, Vertex { position: p, uv: [if start { f } else { 1.0 - f }, y - dy * 0.1 * angle.cos()], ..middle });
            }
            for k in 0..s.rounded { tri(k, vo, vo + 1 + k, vo + 2 + k) }
        }
    }
}

/// line_cs dispatched: its attributes name the buffers and the line's shape.
pub fn dispatch(attrs: usize) {
    let values = { let d = gpu::device(); gpu::flatten(&d, attrs) };
    let get = |n: &str| values.get(&crate::string_token(n));
    let int = |n: &str, dflt: i32| match get(n) { Some(Val::Int(v)) => *v, Some(Val::Combo(v)) => *v as i32, Some(Val::Float(f)) => *f as i32, _ => dflt };
    let buffer = |n: &str| match get(n) { Some(Val::Buffer(b)) => *b, _ => 0 };
    let (pb, vb, ib) = (buffer("PointBuffer"), buffer("VertexBuffer"), buffer("IndexBuffer"));
    let count = int("PointCount", 0).max(0) as usize;
    let shape = Shape {
        face: int("FaceMode", 0), start_cap: int("StartCap", 0), end_cap: int("EndCap", 0),
        rounded: int("RoundedCapSegments", 6).max(1) as u32, tessellation: int("TessellationLevel", 1).max(1) as u32,
        camera: crate::sprites::eye().map(|e| e.position).unwrap_or_default(),
    };
    let mut d = gpu::device();
    let Some(src) = d.buffers.get(&pb).map(|b| b.data.clone()) else { return };
    let f = |i: usize| f32::from_le_bytes(src[i * 4..i * 4 + 4].try_into().unwrap());
    let points: Vec<Point> = (0..count.min(src.len() / (POINT * 4))).map(|i| {
        let w = i * POINT;
        Point { position: [f(w + 1), f(w + 2), f(w + 3)], normal: [f(w + 4), f(w + 5), f(w + 6)], color: [f(w + 7), f(w + 8), f(w + 9), f(w + 10)], width: f(w + 11), uv: f(w + 12) }
    }).collect();
    let (Some(vbuf), Some(ibuf)) = (d.buffers.get(&vb), d.buffers.get(&ib)) else { return };
    let mut vertices: Vec<Vertex> = vbuf.data.chunks_exact(VERTEX_BYTES).map(|c| Vertex::from_words(bytemuck_f32(c).as_slice())).collect();
    let mut indices: Vec<u32> = ibuf.data.chunks_exact(4).map(|c| u32::from_le_bytes(c.try_into().unwrap())).collect();
    line_cs(&points, &shape, &mut vertices, &mut indices);
    if let Some(b) = d.buffers.get_mut(&vb) { b.data = vertices.iter().flat_map(|v| v.words()).flat_map(|x| x.to_le_bytes()).collect(); }
    if let Some(b) = d.buffers.get_mut(&ib) { b.data = indices.iter().flat_map(|x| x.to_le_bytes()).collect(); }
    LINE_VERTICES.lock().unwrap().get_or_insert_with(Default::default).insert(vb);
}

/// A compiled material's colour texture (`g_tColor`), as a texture object.
fn file_texture(material: &str) -> Option<usize> {
    let bytes = crate::fs::read_compiled(material)?;
    let m = crate::assets::material::read(&crate::assets::resource::Resource::parse(bytes).ok()?).ok()?;
    m.textures.get("g_tColor").map(|t| crate::camera::texture(t))
}

fn bytemuck_f32(c: &[u8]) -> Vec<f32> { c.chunks_exact(4).map(|w| f32::from_le_bytes(w.try_into().unwrap())).collect() }

/// A line's GPU buffers are freed: forget it was one.
pub fn forget(b: usize) {
    if let Some(s) = LINE_VERTICES.lock().unwrap().as_mut() { s.remove(&b); }
}

/// A line as its draw drew it: the triangles it indexed, and how.
#[derive(Clone, Debug, Default)]
pub struct Line {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
    /// D_BLEND: added onto what's behind
    pub additive: bool,
    /// D_OPAQUE: not blended, writes depth
    pub opaque: bool,
    /// D_ENABLE_LIGHTING
    pub lit: bool,
    /// its material's colour texture (`g_tColor`), if it set one
    pub texture: Option<usize>,
    /// the material it was drawn with
    pub material: Option<usize>,
    /// its material names a colour texture that can't be read here (a
    /// material only as its source, a texture only as its settings)
    pub missing_texture: bool,
    /// its sampler wraps (else clamps), and filters (else is point-sampled)
    pub wrap: bool,
    pub filtered: bool,
}

/// A recorded draw read back as a line, if line_cs made its vertices.
pub fn line(draw: &gpu::Draw) -> Option<Line> {
    let (vb, offset) = draw.vertex_buffer?;
    if !LINE_VERTICES.lock().unwrap().as_ref().is_some_and(|s| s.contains(&vb)) { return None; }
    let bytes = draw.buffers.get(&vb)?;
    let all: Vec<Vertex> = bytes.get(offset.max(0) as usize..)?.chunks_exact(VERTEX_BYTES).map(|c| Vertex::from_words(&bytemuck_f32(c))).collect();
    let (ib, size, ioff) = draw.index_buffer?;
    let ibytes = draw.buffers.get(&ib)?;
    let size = if size == 2 { 2 } else { 4 };
    let read = |i: usize| -> Option<u32> {
        let at = ioff.max(0) as usize + i * size;
        let b = ibytes.get(at..at + size)?;
        Some(if size == 2 { u16::from_le_bytes([b[0], b[1]]) as u32 } else { u32::from_le_bytes(b.try_into().unwrap()) })
    };
    let first = draw.first.max(0) as usize;
    let indices: Vec<u32> = (first..first + draw.count.max(0) as usize).filter_map(read)
        .map(|i| (i as i64 + draw.base_vertex as i64).max(0) as u32).filter(|&i| (i as usize) < all.len()).collect();
    let get = |n: &str| draw.attrs.get(&crate::string_token(n));
    let combo = |n: &str| match get(n) { Some(Val::Combo(v)) => *v as i32, Some(Val::Int(v)) => *v, Some(Val::Bool(b)) => *b as i32, _ => 0 };
    let set = crate::camera::material_texture(draw.material, "g_tColor")
        .or_else(|| match get("g_tColor") { Some(Val::Texture(t)) => Some(*t), _ => None });
    // else its file's, if it can be read: a material that isn't the default line's names one
    let (texture, missing_texture) = match set {
        Some(t) => (Some(t), false),
        None => match crate::camera::material_name(draw.material) {
            Some(name) if !name.contains("default_line") => match file_texture(&name) {
                Some(t) => (Some(t), false),
                None => (None, true),
            },
            _ => (None, false),
        },
    };
    let sampler = match get("SamplerIndex") { Some(Val::Int(i)) => gpu::samplers_by_index().get(*i as usize).copied(), _ => None };
    Some(Line {
        vertices: all, indices, additive: combo("D_BLEND") == 1, opaque: combo("D_OPAQUE") == 1, lit: combo("D_ENABLE_LIGHTING") == 1, texture, missing_texture, material: Some(draw.material),
        wrap: sampler.is_none_or(|s| s.address[0] == 0), filtered: sampler.is_none_or(|s| s.filter != 0),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn points(ps: &[V3], width: f32) -> Vec<u8> {
        ps.iter().enumerate().flat_map(|(i, p)| {
            let w: [f32; POINT] = [0.0, p[0], p[1], p[2], 0.0, 0.0, 1.0, 1.0, 0.5, 0.25, 1.0, width, i as f32];
            w.into_iter().flat_map(|x| x.to_le_bytes()).collect::<Vec<u8>>()
        }).collect()
    }

    fn run(ps: &[V3], face: i32, start: i32, end: i32, camera: V3) -> (Vec<Vertex>, Vec<u32>) {
        let (pb, vb, ib) = (gpu::new_buffer(0, 8, (POINT * 4) as i32, "points"), gpu::new_buffer(0, 32, VERTEX_BYTES as i32, "vertices"), gpu::new_buffer(0, 64, 4, "indices"));
        gpu::write_buffer(pb, &points(ps, 2.0), 0);
        let a = gpu::new_attrs();
        let set = |n: &str, v: Val| gpu::set(a, crate::string_token(n), v);
        set("PointBuffer", Val::Buffer(pb));
        set("VertexBuffer", Val::Buffer(vb));
        set("IndexBuffer", Val::Buffer(ib));
        set("PointCount", Val::Int(ps.len() as i32));
        set("FaceMode", Val::Int(face));
        set("StartCap", Val::Int(start));
        set("EndCap", Val::Int(end));
        set("TessellationLevel", Val::Int(1));
        set("RoundedCapSegments", Val::Int(8));
        crate::sprites::set_eye(Some(crate::sprites::Eye { position: camera, forward: [1.0, 0.0, 0.0], right: [0.0, -1.0, 0.0], up: [0.0, 0.0, 1.0], angles: [0.0; 3] }));
        dispatch(a);
        crate::sprites::set_eye(None);
        let d = gpu::device();
        (d.buffers[&vb].data.chunks_exact(VERTEX_BYTES).map(|c| Vertex::from_words(&bytemuck_f32(c))).collect(),
         d.buffers[&ib].data.chunks_exact(4).map(|c| u32::from_le_bytes(c.try_into().unwrap())).collect())
    }

    #[test]
    fn a_line_faces_the_camera_with_its_width_across_it() {
        // along x, seen from above: its edges 2 either side of it in y
        let (v, i) = run(&[[0.0, 0.0, 0.0], [50.0, 0.0, 0.0], [100.0, 0.0, 0.0]], CAMERA, 0, 0, [50.0, 0.0, 500.0]);
        for k in 0..6 { assert!((v[k].position[1].abs() - 2.0).abs() < 1e-4 && v[k].position[2].abs() < 1e-4, "{:?}", v[k]); }
        assert_eq!(v[0].position[1], -v[1].position[1], "left and right");
        assert_eq!((v[2].uv, v[3].uv), ([0.0, 1.0], [1.0, 1.0]), "u across, v along from the point's coordinate");
        assert_eq!(v[4].color, [1.0, 0.5, 0.25, 1.0]);
        assert_eq!(&i[0..12], &[0, 2, 1, 1, 2, 3, 2, 4, 3, 3, 4, 5], "a quad between each row and the next");
    }

    #[test]
    fn caps_close_the_ends() {
        let (v, i) = run(&[[0.0, 0.0, 0.0], [100.0, 0.0, 0.0]], CAMERA, TRIANGLE, ARROW, [50.0, 0.0, 500.0]);
        // after the 4 strip vertices: the start's point, 2 out beyond it; the arrow's three
        assert!((v[4].position[0] + 2.0).abs() < 1e-4, "{:?}", v[4].position);
        assert!((v[7].position[0] - 104.0).abs() < 1e-4, "the arrow's tip, two widths on: {:?}", v[7].position);
        assert!((v[5].position[1].abs() - 4.0).abs() < 1e-4, "its barbs twice as wide");
        assert_eq!(&i[6..12], &[0, 1, 4, 5, 7, 6]);
        let (v, i) = run(&[[0.0, 0.0, 0.0], [100.0, 0.0, 0.0]], CAMERA, 0, ROUNDED, [50.0, 0.0, 500.0]);
        assert_eq!(v[4].position, [100.0, 0.0, 0.0], "the fan's middle");
        for k in 5..14 { assert!((length(sub(v[k].position, [100.0, 0.0, 0.0])) - 2.0).abs() < 1e-4, "round the end, 2 out"); }
        assert!(v[9].position[0] > 101.9, "its middle reaches furthest");
        assert_eq!(i.iter().skip(6).take(24).filter(|&&x| x == 4).count(), 8, "eight triangles round it");
    }

    #[test]
    fn a_cylinder_goes_round_its_line() {
        let (v, _) = run(&[[0.0, 0.0, 0.0], [100.0, 0.0, 0.0]], CYLINDER, 0, 0, [0.0; 3]);
        // its points face up (0, 0, 1): a ring 2 round the line, in y and z
        for k in 0..2 { assert!((length(sub(v[k].position, [0.0, 0.0, 0.0])) - 2.0).abs() < 1e-4 && v[k].position[0].abs() < 1e-4, "{:?}", v[k].position); }
        assert!((v[0].position[2].abs() - 2.0).abs() < 1e-4, "its first round the top or bottom");
    }
}
