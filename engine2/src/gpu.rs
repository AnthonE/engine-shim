//! The render device as a client's managed code sees it, kept as data:
//! render attributes (`CRenderAttributes`), GPU buffers and their bytes,
//! textures and samplers by their bindless indices, scene views and the
//! managed layers added to them, and a render context that records what
//! managed draws through it. The renderer (render/) reads what's recorded:
//! the UI, first.
use std::collections::HashMap;
use std::sync::{Mutex, MutexGuard, OnceLock};

/// One attribute's value, as managed sets it.
#[derive(Clone, Debug, PartialEq)]
pub enum Val {
    Float(f32),
    Vec2([f32; 2]),
    Vec3([f32; 3]),
    Vec4([f32; 4]),
    /// row by row, as VMatrix keeps it
    Matrix([f32; 16]),
    Int(i32),
    IntVec4([i32; 4]),
    Combo(u8),
    Bool(bool),
    String(String),
    Texture(usize),
    Sampler(usize),
    Buffer(usize),
    Ptr(usize),
}

#[derive(Default, Clone)]
pub struct Attrs {
    pub values: HashMap<u32, Val>,
    pub parent: usize,
}

#[derive(Default)]
pub struct Buffer {
    pub kind: i64,
    pub element_size: usize,
    pub elements: usize,
    pub data: Vec<u8>,
    pub name: String,
}

pub struct Layer {
    pub name: String,
    /// managed's `SboxProceduralLayerCallback`: (ManagedRenderSetup_t) by value
    pub callback: usize,
    pub attrs: usize,
}

#[derive(Default)]
pub struct View {
    pub camera_id: i32,
    pub attrs: usize,
    pub layers: Vec<usize>,
    pub width: i32,
    pub height: i32,
}

/// What managed drew through a render context.
#[derive(Clone, Debug)]
pub struct Draw {
    pub material: usize,
    /// the attributes in force, parents' under their children's
    pub attrs: HashMap<u32, Val>,
    pub primitive: i64,
    pub index_buffer: Option<(usize, i32, i32)>,
    pub first: i32,
    pub count: i32,
    pub instances: i32,
    pub base_vertex: i32,
    pub scissor: Option<[i32; 4]>,
}

#[derive(Default)]
pub struct Context {
    pub material: usize,
    pub attrs: HashMap<u32, Val>,
    pub index_buffer: Option<(usize, i32, i32)>,
    pub scissor: Option<[i32; 4]>,
    pub draws: Vec<Draw>,
}

#[derive(Default)]
pub struct Device {
    pub attrs: HashMap<usize, Attrs>,
    pub buffers: HashMap<usize, Buffer>,
    pub views: HashMap<usize, View>,
    pub layers: HashMap<usize, Layer>,
    pub contexts: HashMap<usize, Context>,
    /// bindless indices: a texture's (by texture), a sampler's
    pub texture_index: HashMap<usize, i32>,
    pub textures_by_index: Vec<usize>,
    pub sampler_index: HashMap<usize, i32>,
}

pub fn device() -> MutexGuard<'static, Device> {
    static D: OnceLock<Mutex<Device>> = OnceLock::new();
    D.get_or_init(Default::default).lock().unwrap_or_else(|e| e.into_inner())
}

fn object() -> usize { crate::handles::object() }

// ---- attributes

pub fn new_attrs() -> usize {
    let p = object();
    device().attrs.insert(p, Attrs::default());
    p
}

pub fn set(attrs: usize, key: u32, v: Val) {
    if attrs == 0 { return; }
    device().attrs.entry(attrs).or_default().values.insert(key, v);
}

pub fn get(attrs: usize, key: u32) -> Option<Val> {
    let d = device();
    let mut at = attrs;
    for _ in 0..16 {
        let a = d.attrs.get(&at)?;
        if let Some(v) = a.values.get(&key) { return Some(v.clone()); }
        at = a.parent;
    }
    None
}

pub fn delete(attrs: usize, key: u32) {
    if let Some(a) = device().attrs.get_mut(&attrs) { a.values.remove(&key); }
}

pub fn clear(attrs: usize) {
    if let Some(a) = device().attrs.get_mut(&attrs) { a.values.clear(); }
}

pub fn set_parent(attrs: usize, parent: usize) {
    if attrs != 0 && attrs != parent { device().attrs.entry(attrs).or_default().parent = parent; }
}

/// `from`'s values into `into`, over what's there.
pub fn merge(from: usize, into: usize) {
    let mut d = device();
    let Some(src) = d.attrs.get(&from).map(|a| a.values.clone()) else { return };
    d.attrs.entry(into).or_default().values.extend(src);
}

/// Every value in force for `attrs`: its parents' first, its own over them.
pub fn flatten(d: &Device, attrs: usize) -> HashMap<u32, Val> {
    let mut chain = Vec::new();
    let mut at = attrs;
    while at != 0 && chain.len() < 16 {
        let Some(a) = d.attrs.get(&at) else { break };
        chain.push(at);
        at = a.parent;
    }
    let mut out = HashMap::new();
    for a in chain.iter().rev() { out.extend(d.attrs[a].values.clone()); }
    out
}

// ---- GPU buffers

pub fn new_buffer(kind: i64, elements: i32, element_size: i32, name: &str) -> usize {
    let p = object();
    let (elements, element_size) = (elements.max(0) as usize, element_size.max(0) as usize);
    device().buffers.insert(p, Buffer { kind, element_size, elements, data: vec![0; elements * element_size], name: name.to_string() });
    p
}

pub fn free_buffer(p: usize) {
    device().buffers.remove(&p);
}

/// Bytes into a buffer at a byte offset, as far as it reaches.
pub fn write_buffer(p: usize, data: &[u8], offset: usize) {
    let mut d = device();
    let Some(b) = d.buffers.get_mut(&p) else { return };
    if offset >= b.data.len() { return; }
    let n = data.len().min(b.data.len() - offset);
    b.data[offset..offset + n].copy_from_slice(&data[..n]);
}

// ---- bindless indices

pub fn texture_index(texture: usize) -> i32 {
    let mut d = device();
    if let Some(&i) = d.texture_index.get(&texture) { return i; }
    let i = d.textures_by_index.len() as i32;
    d.textures_by_index.push(texture);
    d.texture_index.insert(texture, i);
    i
}

pub fn sampler_index(sampler: usize) -> i32 {
    let mut d = device();
    let n = d.sampler_index.len() as i32;
    *d.sampler_index.entry(sampler).or_insert(n)
}

// ---- views and their managed layers

pub fn new_view(camera_id: i32, width: i32, height: i32) -> usize {
    let p = object();
    let attrs = new_attrs();
    device().views.insert(p, View { camera_id, attrs, layers: Vec::new(), width, height });
    crate::state::set::<i32>(p, crate::slots::PROP__ISceneView_m_ManagedCameraId, camera_id);
    p
}

pub fn add_layer(view: usize, name: &str, callback: usize) -> usize {
    let p = object();
    let attrs = new_attrs();
    let mut d = device();
    if let Some(v) = d.views.get(&view) { let parent = v.attrs; d.attrs.entry(attrs).or_default().parent = parent; }
    d.layers.insert(p, Layer { name: name.to_string(), callback, attrs });
    if let Some(v) = d.views.get_mut(&view) { v.layers.push(p); }
    p
}

pub fn free_view(view: usize) {
    let mut d = device();
    if let Some(v) = d.views.remove(&view) {
        d.attrs.remove(&v.attrs);
        for l in v.layers { if let Some(l) = d.layers.remove(&l) { d.attrs.remove(&l.attrs); } }
    }
}

// ---- a render context: what managed draws, recorded

pub fn new_context() -> usize {
    let p = object();
    device().contexts.insert(p, Context::default());
    p
}

pub fn take_context(p: usize) -> Option<Context> {
    device().contexts.remove(&p)
}

/// The material and attributes the next draws are made with (SetRenderState).
pub fn set_state(ctx: usize, attrs: usize, material: usize) -> bool {
    let mut d = device();
    let values = flatten(&d, attrs);
    let Some(c) = d.contexts.get_mut(&ctx) else { return false };
    c.material = material;
    c.attrs = values;
    true
}

pub fn with_context(ctx: usize, f: impl FnOnce(&mut Context)) {
    if let Some(c) = device().contexts.get_mut(&ctx) { f(c) }
}

pub fn record(ctx: usize, primitive: i64, first: i32, count: i32, instances: i32, base_vertex: i32, indexed: bool) {
    with_context(ctx, |c| {
        let index_buffer = if indexed { c.index_buffer } else { None };
        c.draws.push(Draw { material: c.material, attrs: c.attrs.clone(), primitive, index_buffer, first, count, instances, base_vertex, scissor: c.scissor });
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn attributes_inherit_and_draws_record_them() {
        let (parent, child) = (new_attrs(), new_attrs());
        set_parent(child, parent);
        set(parent, 1, Val::Float(1.0));
        set(parent, 2, Val::Int(7));
        set(child, 1, Val::Float(2.0));
        assert_eq!(get(child, 1), Some(Val::Float(2.0)), "its own over its parent's");
        assert_eq!(get(child, 2), Some(Val::Int(7)), "its parent's where it has none");
        let b = new_buffer(0, 4, 8, "boxes");
        write_buffer(b, &[1, 2, 3], 30);
        assert_eq!(&device().buffers[&b].data[30..32], &[1, 2], "as far as it reaches");
        set(child, 3, Val::Buffer(b));

        let ctx = new_context();
        assert!(set_state(ctx, child, 42));
        with_context(ctx, |c| c.index_buffer = Some((b, 4, 0)));
        record(ctx, 5, 0, 6, 10, 0, true);
        let c = take_context(ctx).unwrap();
        assert_eq!(c.draws.len(), 1);
        let d = &c.draws[0];
        assert_eq!((d.material, d.count, d.instances), (42, 6, 10));
        assert_eq!(d.attrs.get(&1), Some(&Val::Float(2.0)));
        assert_eq!(d.attrs.get(&2), Some(&Val::Int(7)));
        assert_eq!(d.attrs.get(&3), Some(&Val::Buffer(b)));
        assert_eq!(texture_index(77), texture_index(77));
    }
}
