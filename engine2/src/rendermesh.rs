//! Meshes made at runtime: what managed's `Mesh` builds (an editor mesh's
//! faces, a terrain's clipmap, a procedural prop) and `ModelBuilder` makes
//! a model of. MeshGlue's render meshes, their vertex and index buffers,
//! and the vertex layouts that say what a vertex holds.
//!
//! A vertex buffer is kept as managed wrote it, its fields described as a
//! compiled mesh's are (assets/mesh.rs: a semantic, an index, a format in
//! DXGI's numbering, which s&box's ColorFormat is, and an offset), so the
//! renderer reads one as it reads the other. A lock hands managed the
//! buffer's own bytes to write into; each write counts, so what was drawn
//! from a buffer is drawn again once it changes (an editor mesh painted).
use std::collections::HashMap;
use std::sync::{Mutex, MutexGuard, OnceLock};

use crate::assets::mesh::{Attribute, Buffers, Draw, IndexBuffer, Mesh as DrawMesh, VertexBuffer};

/// RenderPrimitiveType, native's numbering (managed's MeshPrimitiveType is
/// turned into it before it's handed over): what's drawn is triangles.
pub const TRIANGLES: i32 = 5;
pub const TRIANGLE_STRIP: i32 = 7;

/// A vertex struct's fields, as managed's VertexLayout describes it.
#[derive(Clone, Default)]
pub struct Layout {
    pub size: usize,
    pub fields: Vec<Attribute>,
}

pub struct Vertices {
    pub stride: usize,
    pub count: usize,
    pub fields: Vec<Attribute>,
    pub data: Vec<u8>,
    /// writes so far
    pub version: u64,
}

pub struct Indices {
    pub count: usize,
    /// 2 or 4
    pub size: usize,
    pub data: Vec<u8>,
    pub version: u64,
}

/// A draw call added to a mesh (AddMeshDrawCall).
#[derive(Clone, Debug, PartialEq)]
pub struct DrawCall {
    pub material: usize,
    pub primitive: i32,
    pub start_index: i32,
    pub index_count: i32,
    pub start_vertex: i32,
    pub vertex_count: i32,
}

#[derive(Default)]
pub struct RenderMesh {
    pub name: String,
    pub material: usize,
    pub primitive: i32,
    pub vertices: usize,
    pub indices: usize,
    /// first and how many (SetMeshVertexRange, SetMeshIndexRange)
    pub vertex_range: (i32, i32),
    pub index_range: (i32, i32),
    pub bounds: Option<([f32; 3], [f32; 3])>,
    pub uv_density: f32,
    pub draws: Vec<DrawCall>,
}

enum Thing {
    Layout(Layout),
    Vertices(Vertices),
    Indices(Indices),
    Mesh(RenderMesh),
}

#[derive(Default)]
pub struct Things {
    all: HashMap<usize, Thing>,
}

fn things() -> MutexGuard<'static, Things> {
    static T: OnceLock<Mutex<Things>> = OnceLock::new();
    T.get_or_init(Default::default).lock().unwrap_or_else(|e| e.into_inner())
}

fn make(t: Thing) -> usize {
    let p = crate::handles::object();
    things().all.insert(p, t);
    p
}

impl Things {
    fn layout(&mut self, p: usize) -> Option<&mut Layout> {
        match self.all.get_mut(&p) { Some(Thing::Layout(x)) => Some(x), _ => None }
    }
    fn vertices(&mut self, p: usize) -> Option<&mut Vertices> {
        match self.all.get_mut(&p) { Some(Thing::Vertices(x)) => Some(x), _ => None }
    }
    fn indices(&mut self, p: usize) -> Option<&mut Indices> {
        match self.all.get_mut(&p) { Some(Thing::Indices(x)) => Some(x), _ => None }
    }
    fn mesh(&mut self, p: usize) -> Option<&mut RenderMesh> {
        match self.all.get_mut(&p) { Some(Thing::Mesh(x)) => Some(x), _ => None }
    }
}

/// `n` bytes managed handed over at `p` (none if `p` is null).
unsafe fn bytes<'a>(p: usize, n: usize) -> &'a [u8] {
    if p == 0 || n == 0 { &[] } else { unsafe { core::slice::from_raw_parts(p as *const u8, n) } }
}

/// Bytes written into `into` at `at`, as far as it reaches.
fn write(into: &mut [u8], at: usize, from: &[u8]) {
    if at >= into.len() { return; }
    let n = from.len().min(into.len() - at);
    into[at..at + n].copy_from_slice(&from[..n]);
}

// ---- vertex layouts (VertexLayout: Create, Add, Build)

pub fn new_layout(size: i32) -> usize {
    let p = make(Thing::Layout(Layout { size: size.max(0) as usize, fields: Vec::new() }));
    crate::state::set::<i32>(p, crate::slots::PROP__VertexLayout_m_Size, size);
    p
}

pub fn add_field(layout: usize, semantic: &str, index: i32, format: u32, offset: i32) {
    if let Some(l) = things().layout(layout) {
        l.fields.push(Attribute { name: semantic.to_string(), index: index.max(0) as u32, format, offset: offset.max(0) as usize });
    }
}

pub fn layout_of(p: usize) -> Option<Layout> {
    things().layout(p).cloned()
}

pub fn free(p: usize) {
    things().all.remove(&p);
}

// ---- buffers

/// A vertex buffer of `count` vertices of `layout`, from `data` if given
/// (`count` of them).
pub unsafe fn new_vertices(count: i32, layout: &Layout, data: usize) -> usize {
    let count = count.max(0) as usize;
    let mut v = Vertices { stride: layout.size, count, fields: layout.fields.clone(), data: vec![0; count * layout.size], version: 0 };
    let all = v.data.len();
    write(&mut v.data, 0, unsafe { bytes(data, all) });
    make(Thing::Vertices(v))
}

/// A vertex buffer of fields managed lists itself (MeshGlue's other
/// CreateVertexBuffer: each field's format, offset, semantic index, and its
/// name's place in `names`).
pub unsafe fn new_vertices_of(stride: i32, count: i32, names: &str, fields: usize, n: i32, data: usize, given: i32) -> usize {
    #[repr(C)]
    struct Field { format: u32, offset: i32, name_size: i32, name_offset: i32, semantic_index: i32 }
    let fields: &[Field] = if fields == 0 || n <= 0 { &[] } else { unsafe { core::slice::from_raw_parts(fields as *const Field, n as usize) } };
    let layout = Layout {
        size: stride.max(0) as usize,
        fields: fields.iter().map(|f| {
            let (at, len) = (f.name_offset.max(0) as usize, f.name_size.max(0) as usize);
            Attribute { name: names.get(at..at + len).unwrap_or("").to_string(), index: f.semantic_index.max(0) as u32, format: f.format, offset: f.offset.max(0) as usize }
        }).collect(),
    };
    let p = unsafe { new_vertices(count, &layout, 0) };
    if let Some(v) = things().vertices(p) {
        let n = given.clamp(0, count.max(0)) as usize * v.stride;
        write(&mut v.data, 0, unsafe { bytes(data, n) });
    }
    p
}

pub unsafe fn new_indices(count: i32, wide: bool, data: usize, given: i32) -> usize {
    let size = if wide { 4 } else { 2 };
    let count = count.max(0) as usize;
    let mut i = Indices { count, size, data: vec![0; count * size], version: 0 };
    let n = given.clamp(0, count as i32) as usize * size;
    write(&mut i.data, 0, unsafe { bytes(data, n) });
    make(Thing::Indices(i))
}

/// Bytes into a buffer at a byte offset (SetVertexBufferData, SetIndexBufferData).
pub unsafe fn set_data(buffer: usize, data: usize, size: i32, offset: i32) {
    let from = unsafe { bytes(data, size.max(0) as usize) };
    let mut t = things();
    let at = offset.max(0) as usize;
    if let Some(v) = t.vertices(buffer) { write(&mut v.data, at, from); v.version += 1; return; }
    if let Some(i) = t.indices(buffer) { write(&mut i.data, at, from); i.version += 1; }
}

/// A buffer's own bytes for managed to write into (null if it doesn't
/// reach); the write counts when it's unlocked.
pub fn lock(buffer: usize, size: i32, offset: i32) -> usize {
    let (at, n) = (offset.max(0) as usize, size.max(0) as usize);
    let mut t = things();
    let data = match t.all.get_mut(&buffer) {
        Some(Thing::Vertices(v)) => &mut v.data,
        Some(Thing::Indices(i)) => &mut i.data,
        _ => return 0,
    };
    if n == 0 || at + n > data.len() { return 0; }
    unsafe { data.as_mut_ptr().add(at) as usize }
}

pub fn unlock(buffer: usize) {
    let mut t = things();
    if let Some(v) = t.vertices(buffer) { v.version += 1; return; }
    if let Some(i) = t.indices(buffer) { i.version += 1; }
}

/// A buffer resized to `count` elements, what it held kept: the same buffer.
pub fn resize(buffer: usize, count: i32) -> usize {
    let count = count.max(0) as usize;
    let mut t = things();
    if let Some(v) = t.vertices(buffer) { v.count = count; v.data.resize(count * v.stride, 0); v.version += 1; }
    else if let Some(i) = t.indices(buffer) { i.count = count; i.data.resize(count * i.size, 0); i.version += 1; }
    buffer
}

// ---- render meshes

pub fn new_mesh(material: usize, primitive: i32, name: &str) -> usize {
    make(Thing::Mesh(RenderMesh { name: name.to_string(), material, primitive, uv_density: 0.0, ..Default::default() }))
}

/// A change to a mesh.
pub fn with_mesh(mesh: usize, f: impl FnOnce(&mut RenderMesh)) {
    if let Some(m) = things().mesh(mesh) { f(m) }
}

/// A buffer given to a mesh, with its data again if managed passes it
/// (`given` elements from the first).
pub unsafe fn give(mesh: usize, buffer: usize, data: usize, given: i32) {
    let mut t = things();
    let from_vertices = t.vertices(buffer).map(|v| v.stride);
    if let Some(stride) = from_vertices {
        if let Some(m) = t.mesh(mesh) { m.vertices = buffer; }
        if let Some(v) = t.vertices(buffer) {
            let n = given.max(0) as usize * stride;
            if n > 0 { write(&mut v.data, 0, unsafe { bytes(data, n) }); v.version += 1; }
        }
        return;
    }
    let from_indices = t.indices(buffer).map(|i| i.size);
    if let Some(size) = from_indices {
        if let Some(m) = t.mesh(mesh) { m.indices = buffer; }
        if let Some(i) = t.indices(buffer) {
            let n = given.max(0) as usize * size;
            if n > 0 { write(&mut i.data, 0, unsafe { bytes(data, n) }); i.version += 1; }
        }
    }
}

/// What a mesh is, for a model's description: its bounds (as set, else its
/// vertices'), its vertex and index counts, and its draw calls.
pub struct Summary {
    pub min: [f32; 3],
    pub max: [f32; 3],
    pub vertices: i32,
    pub draws: Vec<DrawCall>,
    pub uv_density: f32,
}

/// The draw calls a mesh makes: those added, else one over its ranges with
/// its own material.
fn draws_of(m: &RenderMesh, vertices: usize, indices: usize) -> Vec<DrawCall> {
    if !m.draws.is_empty() { return m.draws.clone(); }
    let (start_vertex, vertex_count) = if m.vertex_range.1 > 0 { m.vertex_range } else { (0, vertices as i32) };
    let (start_index, index_count) = if m.index_range.1 > 0 { m.index_range } else if m.indices != 0 { (0, indices as i32) } else { (0, vertex_count) };
    vec![DrawCall { material: m.material, primitive: m.primitive, start_index, index_count, start_vertex, vertex_count }]
}

pub fn summary(mesh: usize) -> Option<Summary> {
    let mut t = things();
    let m = match t.all.get(&mesh) { Some(Thing::Mesh(m)) => m, _ => return None };
    let (vb, ib, set_bounds, uv) = (m.vertices, m.indices, m.bounds, m.uv_density);
    let (count, positions) = match t.all.get(&vb) {
        Some(Thing::Vertices(v)) => (v.count, VertexBuffer { count: v.count, stride: v.stride, attributes: v.fields.clone(), data: v.data.clone() }.floats("POSITION", 0)),
        _ => (0, None),
    };
    let icount = match t.all.get(&ib) { Some(Thing::Indices(i)) => i.count, _ => 0 };
    let m = t.mesh(mesh)?;
    let draws = draws_of(m, count, icount);
    let (min, max) = set_bounds.unwrap_or_else(|| {
        let ps = positions.unwrap_or_default();
        if ps.is_empty() { return ([0.0; 3], [0.0; 3]); }
        ps.iter().fold(([f32::MAX; 3], [f32::MIN; 3]), |(lo, hi), p| ([lo[0].min(p[0]), lo[1].min(p[1]), lo[2].min(p[2])], [hi[0].max(p[0]), hi[1].max(p[1]), hi[2].max(p[2])]))
    });
    Some(Summary { min, max, vertices: count as i32, draws, uv_density: uv })
}

/// How many writes the meshes' buffers have had: what was drawn from them
/// is stale once this changes.
pub fn version(meshes: &[usize]) -> u64 {
    let t = things();
    let of = |b: usize| match t.all.get(&b) {
        Some(Thing::Vertices(v)) => v.version + v.count as u64,
        Some(Thing::Indices(i)) => i.version + i.count as u64,
        _ => 0,
    };
    meshes.iter().map(|m| match t.all.get(m) {
        Some(Thing::Mesh(x)) => of(x.vertices).wrapping_mul(31).wrapping_add(of(x.indices)).wrapping_add(x.draws.len() as u64),
        _ => 0,
    }).fold(0u64, |a, b| a.wrapping_mul(1_000_003).wrapping_add(b))
}

/// Meshes as the renderer draws a compiled model's (assets/mesh.rs): their
/// buffers copied, their draws of triangles (a strip unrolled into them),
/// each naming its material.
pub fn for_drawing(meshes: &[usize], material_name: impl Fn(usize) -> String) -> Vec<DrawMesh> {
    let t = things();
    let mut out = Vec::new();
    for p in meshes {
        let Some(Thing::Mesh(m)) = t.all.get(p) else { continue };
        let Some(Thing::Vertices(v)) = t.all.get(&m.vertices) else { continue };
        let vb = VertexBuffer { count: v.count, stride: v.stride, attributes: v.fields.clone(), data: v.data.clone() };
        let given = match t.all.get(&m.indices) { Some(Thing::Indices(i)) => Some(IndexBuffer { count: i.count, size: i.size, data: i.data.clone() }.indices()), _ => None };
        let icount = given.as_ref().map_or(0, |i| i.len());
        // every draw's own triangles, in one list
        let mut list: Vec<u32> = Vec::new();
        let mut draws = Vec::new();
        for d in draws_of(m, v.count, icount) {
            let run: Vec<u32> = match &given {
                Some(ix) => {
                    let (a, n) = (d.start_index.max(0) as usize, d.index_count.max(0) as usize);
                    ix.get(a..(a + n).min(ix.len())).unwrap_or(&[]).iter().map(|&i| i + d.start_vertex.max(0) as u32).collect()
                }
                None => (d.start_vertex.max(0) as u32..(d.start_vertex.max(0) + d.vertex_count.max(0)) as u32).collect(),
            };
            let run = match d.primitive {
                TRIANGLES => run,
                TRIANGLE_STRIP => run.windows(3).enumerate().flat_map(|(k, w)| if k % 2 == 0 { [w[0], w[1], w[2]] } else { [w[1], w[0], w[2]] }).collect(),
                _ => continue,
            };
            let run: Vec<u32> = run.into_iter().filter(|&i| (i as usize) < v.count).collect();
            let run = &run[..run.len() / 3 * 3];
            if run.is_empty() { continue; }
            draws.push(Draw { material: material_name(d.material), vertex_buffer: 0, index_buffer: 0, base_vertex: 0, start_index: list.len(), index_count: run.len(), tint: [1.0; 3] });
            list.extend_from_slice(run);
        }
        if draws.is_empty() { continue; }
        let ib = IndexBuffer { count: list.len(), size: 4, data: list.iter().flat_map(|i| i.to_le_bytes()).collect() };
        out.push(DrawMesh { draws, buffers: Buffers { vertices: vec![vb], indices: vec![ib] }, bones: Vec::new() });
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    /// PolygonMesh's MeshVertex, as managed's VertexLayout describes it.
    fn mesh_vertex() -> usize {
        let l = new_layout(64);
        for (s, i, f, o) in [("position", 0, 6, 0), ("normal", 0, 6, 12), ("tangent", 0, 2, 24), ("texcoord", 0, 16, 40), ("texcoord", 4, 28, 48), ("texcoord", 5, 28, 52)] {
            add_field(l, s, i, f, o);
        }
        l
    }

    fn vertex(p: [f32; 3], uv: [f32; 2]) -> [u8; 64] {
        let mut b = [0u8; 64];
        for (k, x) in p.iter().chain(&[0.0, 0.0, 1.0]).chain(&[1.0, 0.0, 0.0, 1.0]).chain(&uv).enumerate() { b[k * 4..k * 4 + 4].copy_from_slice(&x.to_le_bytes()); }
        b[48..52].copy_from_slice(&[255, 0, 0, 0]);
        b[52..56].copy_from_slice(&[255, 128, 0, 255]);
        b
    }

    #[test]
    fn a_mesh_reads_as_a_compiled_ones() {
        let l = mesh_vertex();
        assert_eq!(crate::state::get::<i32>(l, crate::slots::PROP__VertexLayout_m_Size), 64);
        let layout = things().layout(l).unwrap().clone();
        let quad: Vec<u8> = [[0.0, 0.0, 0.0], [10.0, 0.0, 0.0], [10.0, 10.0, 0.0], [0.0, 10.0, 0.0]].iter().enumerate().flat_map(|(i, p)| vertex(*p, [i as f32, 0.5])).collect();
        let vb = unsafe { new_vertices(4, &layout, quad.as_ptr() as usize) };
        let idx: [i32; 6] = [0, 1, 2, 0, 2, 3];
        let ib = unsafe { new_indices(6, true, idx.as_ptr() as usize, 6) };
        let mesh = new_mesh(42, TRIANGLES, "quad");
        unsafe { give(mesh, vb, 0, 0); give(mesh, ib, 0, 0); }
        with_mesh(mesh, |m| m.vertex_range = (0, 4));

        let s = summary(mesh).unwrap();
        assert_eq!((s.min, s.max, s.vertices), ([0.0; 3], [10.0, 10.0, 0.0], 4), "bounds from its vertices when none are set");
        assert_eq!(s.draws, [DrawCall { material: 42, primitive: TRIANGLES, start_index: 0, index_count: 6, start_vertex: 0, vertex_count: 4 }]);

        let d = for_drawing(&[mesh], |m| format!("materials/{m}.vmat"));
        assert_eq!(d.len(), 1);
        let v = &d[0].buffers.vertices[0];
        assert_eq!(v.floats("POSITION", 0).unwrap()[2], [10.0, 10.0, 0.0, 0.0]);
        assert_eq!(v.normals().unwrap()[0], [0.0, 0.0, 1.0]);
        assert_eq!(v.floats("TEXCOORD", 0).unwrap()[3], [3.0, 0.5, 0.0, 0.0]);
        assert_eq!(v.floats("TEXCOORD", 5).unwrap()[0][1], 128.0 / 255.0, "its paint, as an editor mesh's colour");
        assert_eq!(d[0].buffers.indices[0].indices(), [0, 1, 2, 0, 2, 3]);
        assert_eq!((d[0].draws[0].material.as_str(), d[0].draws[0].index_count), ("materials/42.vmat", 6));

        // managed writes into a lock: the buffer holds it, and it's newer
        let before = version(&[mesh]);
        let p = lock(vb, 64, 64);
        assert_ne!(p, 0);
        unsafe { core::ptr::copy_nonoverlapping(vertex([20.0, 0.0, 0.0], [0.0; 2]).as_ptr(), p as *mut u8, 64) };
        unlock(vb);
        assert_ne!(version(&[mesh]), before);
        assert_eq!(for_drawing(&[mesh], |_| String::new())[0].buffers.vertices[0].floats("POSITION", 0).unwrap()[1][0], 20.0);
        assert_eq!(lock(vb, 64, 4 * 64), 0, "past its end");
    }

    #[test]
    fn draw_calls_strips_and_no_indices() {
        let l = mesh_vertex();
        let layout = things().layout(l).unwrap().clone();
        let six: Vec<u8> = (0..6).flat_map(|i| vertex([i as f32, (i % 2) as f32, 0.0], [0.0; 2])).collect();
        let vb = unsafe { new_vertices(6, &layout, six.as_ptr() as usize) };
        // no index buffer: the vertices in order
        let list = new_mesh(1, TRIANGLES, "list");
        unsafe { give(list, vb, 0, 0) };
        assert_eq!(for_drawing(&[list], |_| String::new())[0].buffers.indices[0].indices(), [0, 1, 2, 3, 4, 5]);
        // a strip, its odd triangles turned back round
        let strip = new_mesh(1, TRIANGLE_STRIP, "strip");
        unsafe { give(strip, vb, 0, 0) };
        with_mesh(strip, |m| m.vertex_range = (0, 5));
        assert_eq!(for_drawing(&[strip], |_| String::new())[0].buffers.indices[0].indices(), [0, 1, 2, 2, 1, 3, 2, 3, 4]);
        // two draw calls of two materials over one buffer
        let two = new_mesh(1, TRIANGLES, "two");
        unsafe { give(two, vb, 0, 0) };
        with_mesh(two, |m| m.draws = vec![
            DrawCall { material: 7, primitive: TRIANGLES, start_index: 0, index_count: 0, start_vertex: 0, vertex_count: 3 },
            DrawCall { material: 8, primitive: TRIANGLES, start_index: 0, index_count: 0, start_vertex: 3, vertex_count: 3 },
            DrawCall { material: 9, primitive: 1, start_index: 0, index_count: 0, start_vertex: 0, vertex_count: 2 },
        ]);
        let d = for_drawing(&[two], |m| m.to_string());
        let names: Vec<_> = d[0].draws.iter().map(|x| (x.material.clone(), x.start_index, x.index_count)).collect();
        assert_eq!(names, [("7".to_string(), 0, 3), ("8".to_string(), 3, 3)], "lines aren't drawn");
        assert_eq!(d[0].buffers.indices[0].indices(), [0, 1, 2, 3, 4, 5]);
    }

    #[test]
    fn fields_listed_by_name() {
        // MeshGlue's CreateVertexBuffer of VertexAttributes: names in one string
        #[repr(C)]
        struct Field { format: u32, offset: i32, name_size: i32, name_offset: i32, semantic_index: i32 }
        let names = "positiontexcoord";
        let fields = [Field { format: 6, offset: 0, name_size: 8, name_offset: 0, semantic_index: 0 }, Field { format: 16, offset: 12, name_size: 8, name_offset: 8, semantic_index: 0 }];
        let data: Vec<f32> = vec![1.0, 2.0, 3.0, 0.25, 0.75];
        let vb = unsafe { new_vertices_of(20, 1, names, fields.as_ptr() as usize, 2, data.as_ptr() as usize, 1) };
        let v = match things().all.get(&vb) { Some(Thing::Vertices(v)) => VertexBuffer { count: v.count, stride: v.stride, attributes: v.fields.clone(), data: v.data.clone() }, _ => panic!() };
        assert_eq!(v.floats("POSITION", 0).unwrap(), [[1.0, 2.0, 3.0, 0.0]]);
        assert_eq!(v.floats("TEXCOORD", 0).unwrap(), [[0.25, 0.75, 0.0, 0.0]]);
        assert_eq!(resize(vb, 3), vb);
        assert_eq!(match things().all.get(&vb) { Some(Thing::Vertices(v)) => v.data.len(), _ => 0 }, 60);
    }
}
