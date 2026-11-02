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
    /// the bytes of each buffer its attributes name, as they were when it was
    /// drawn (managed refills its tables between draws, as a GPU would see)
    pub buffers: HashMap<usize, std::sync::Arc<Vec<u8>>>,
    /// the vertices it carried itself (RenderTools.Draw), if it did
    pub inline: Option<Inline>,
    /// not a draw: what's been drawn so far copied into this texture
    /// (RenderTools.ResolveFrameBuffer: a grab for a backdrop to read)
    pub grab: Option<usize>,
}

/// Vertices a draw carried (managed's Graphics.Draw, DrawQuad): in a vertex
/// layout's fields, as managed wrote them, and the indices if any.
#[derive(Clone, Debug)]
pub struct Inline {
    pub layout: usize,
    pub stride: usize,
    pub count: usize,
    pub bytes: Vec<u8>,
    pub indices: Vec<u16>,
}

#[derive(Default)]
pub struct Context {
    /// what it draws into: x, y, width, height (IRenderContext.GetViewport)
    pub viewport: [i32; 4],
    pub material: usize,
    pub attrs: HashMap<u32, Val>,
    pub index_buffer: Option<(usize, i32, i32)>,
    pub scissor: Option<[i32; 4]>,
    pub draws: Vec<Draw>,
}

/// CTextureDesc, as managed reads it back through the pointer
/// GetTextureDesc gives it (28 bytes).
#[repr(C)]
#[derive(Clone, Copy, Default, Debug, PartialEq)]
pub struct TextureDesc {
    pub width: i16,
    pub height: i16,
    pub depth: i16,
    pub mips: i16,
    pub decode: i32,
    /// managed's ImageFormat
    pub format: i32,
    pub flags: u32,
    pub display_width: i16,
    pub display_height: i16,
    pub motion: i16,
    pub pad: i16,
}

/// A texture managed made (FindOrCreateTexture2): an image it decoded, an
/// SVG it rasterised, a thumbnail it downloaded, a render target. Its top
/// mip, in its own format, as managed wrote it.
pub struct Texture {
    pub name: String,
    /// boxed, so the pointer managed holds stays put
    pub desc: Box<TextureDesc>,
    pub data: Vec<u8>,
    /// bumped by each write, so a copy on the GPU knows it's stale
    pub version: u64,
}

/// A sampler state, as managed described it (CSamplerStateDesc).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub struct Sampler {
    /// FilterMode: 0 point, 0x14 bilinear, 0x15 trilinear, 0x55 anisotropic
    pub filter: u8,
    /// TextureAddressMode for u, v, w: 0 wrap, 1 mirror, 2 clamp, 3 border, 4 mirror once
    pub address: [u8; 3],
    /// RGBA, a byte each
    pub border: u32,
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
    /// textures managed made, by object; the named ones by name
    pub textures: HashMap<usize, Texture>,
    pub texture_names: HashMap<String, usize>,
    /// a file's texture's description, read from its header when asked
    pub file_descs: HashMap<usize, Box<TextureDesc>>,
    /// samplers, one object per description
    pub samplers: HashMap<usize, Sampler>,
    pub sampler_objects: HashMap<Sampler, usize>,
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

/// A texture's bindless index. Zero is no texture (the UI box shader reads
/// a TextureIndex of 0 as none), so the first is 1.
pub fn texture_index(texture: usize) -> i32 {
    let mut d = device();
    if let Some(&i) = d.texture_index.get(&texture) { return i; }
    if d.textures_by_index.is_empty() { d.textures_by_index.push(0); }
    let i = d.textures_by_index.len() as i32;
    d.textures_by_index.push(texture);
    d.texture_index.insert(texture, i);
    i
}

/// The texture at a bindless index.
pub fn texture_at(index: i32) -> Option<usize> {
    if index <= 0 { return None; }
    device().textures_by_index.get(index as usize).copied().filter(|&t| t != 0)
}

pub fn sampler_index(sampler: usize) -> i32 {
    let mut d = device();
    let n = d.sampler_index.len() as i32;
    *d.sampler_index.entry(sampler).or_insert(n)
}

/// The samplers by bindless index, as far as the highest given out.
pub fn samplers_by_index() -> Vec<Sampler> {
    let d = device();
    let mut out = vec![Sampler::default(); d.sampler_index.len()];
    for (s, &i) in &d.sampler_index {
        if let (Some(slot), Some(desc)) = (out.get_mut(i as usize), d.samplers.get(s)) { *slot = *desc; }
    }
    out
}

// ---- samplers and textures managed makes

/// A sampler for a CSamplerStateDesc (filter, lod bias and its sign, the
/// three address modes, anisotropy, comparison, a global bias flag, min
/// and max lod, then the border colour at 12): one object per description.
pub fn sampler(desc: &[u8; 16]) -> usize {
    let s = Sampler { filter: desc[0], address: [desc[3], desc[4], desc[5]], border: u32::from_le_bytes(desc[12..16].try_into().unwrap()) };
    let mut d = device();
    if let Some(&p) = d.sampler_objects.get(&s) { return p; }
    let p = object();
    d.samplers.insert(p, s);
    d.sampler_objects.insert(s, p);
    p
}

/// Bytes a pixel takes in an ImageFormat, for the uncompressed ones.
pub fn pixel_bytes(format: i32) -> Option<usize> {
    Some(match format {
        // RGBA8888 ABGR8888 ARGB8888 BGRA8888 BGRX8888, their LINEAR_, LE_ and _LINEAR kin, RGBX8888, RGBA1010102 BGRA1010102, RG1616F, R32F, RG1616
        0 | 1 | 11 | 12 | 16 | 41..=45 | 51 | 52 | 55 | 57 | 58 | 53 | 27 | 79 | 81..=83 => 4,
        2 | 3 | 46 | 47 => 3,           // RGB888 BGR888
        4 | 17 | 18 | 19 | 21 | 6 | 59 | 80 => 2, // 565s, 5551s, 4444, IA88, R16F, R16
        5 | 8 | 49 => 1,                 // I8 A8
        24 | 25 | 50 | 54 => 8,          // RGBA16161616(F), RG3232F
        28 => 12,
        29 => 16,
        _ => return None,
    })
}

/// A texture as managed asks for one (FindOrCreateTexture2): its config
/// (TextureCreationConfig_t: width, height, depth and mips as shorts, the
/// decode flags, the ImageFormat, the flags, then the display rect's size)
/// and its first pixels if it gives any. A named one that isn't anonymous
/// is found again by its name.
pub fn create_texture(name: &str, anonymous: bool, config: &[u8], data: &[u8]) -> usize {
    let i16_at = |at: usize| config.get(at..at + 2).map(|b| i16::from_le_bytes(b.try_into().unwrap())).unwrap_or(0);
    let i32_at = |at: usize| config.get(at..at + 4).map(|b| i32::from_le_bytes(b.try_into().unwrap())).unwrap_or(0);
    let desc = TextureDesc {
        width: i16_at(0).max(1), height: i16_at(2).max(1), depth: i16_at(4).max(1), mips: i16_at(6).max(1),
        decode: i32_at(8), format: i32_at(12), flags: i32_at(16) as u32,
        display_width: i16_at(20), display_height: i16_at(22), motion: i16_at(24), pad: 0,
    };
    let mut d = device();
    if !anonymous {
        if let Some(&p) = d.texture_names.get(name) {
            if let Some(t) = d.textures.get_mut(&p) {
                if *t.desc == desc {
                    if !data.is_empty() { fill_top(t, data); }
                    return p;
                }
            }
        }
    }
    let size = desc.width as usize * desc.height as usize * pixel_bytes(desc.format).unwrap_or(0);
    let mut t = Texture { name: name.to_string(), desc: Box::new(desc), data: vec![0; size], version: 0 };
    if !data.is_empty() { fill_top(&mut t, data); }
    let p = object();
    d.textures.insert(p, t);
    if !anonymous { d.texture_names.insert(name.to_string(), p); }
    p
}

/// The first mip's bytes from data that starts with it (managed may give every mip).
fn fill_top(t: &mut Texture, data: &[u8]) {
    let n = t.data.len().min(data.len());
    t.data[..n].copy_from_slice(&data[..n]);
    t.version += 1;
}

/// Pixels into a texture's top mip (AsyncSetTextureData2), rows of the
/// rect's width from `data`, at the rect's place (x, y, z, w, h, d).
pub fn write_texture(t: usize, data: &[u8], rect: [i32; 6]) {
    let mut d = device();
    let Some(t) = d.textures.get_mut(&t) else { return };
    let Some(bpp) = pixel_bytes(t.desc.format) else { return };
    let (tw, th) = (t.desc.width as usize, t.desc.height as usize);
    let [x, y, _, w, h, _] = rect.map(|v| v.max(0) as usize);
    let (w, h) = (if w == 0 { tw } else { w }, if h == 0 { th } else { h });
    let row = w * bpp;
    for r in 0..h {
        let (ty, from) = (y + r, r * row);
        if ty >= th || from >= data.len() || x >= tw { break; }
        let n = (w.min(tw - x) * bpp).min(data.len() - from);
        let at = (ty * tw + x) * bpp;
        t.data[at..at + n].copy_from_slice(&data[from..from + n]);
    }
    t.version += 1;
}

/// A texture filled with one colour (ClearTexture), for the formats of a byte a channel.
pub fn clear_texture(t: usize, rgba: [u8; 4]) {
    let mut d = device();
    let Some(t) = d.textures.get_mut(&t) else { return };
    let px: Vec<u8> = match t.desc.format {
        0 | 42 | 55 | 81 => rgba.to_vec(),
        1 | 43 => vec![rgba[3], rgba[2], rgba[1], rgba[0]],
        11 | 44 => vec![rgba[3], rgba[0], rgba[1], rgba[2]],
        12 | 16 | 41 | 45 | 51 | 52 | 82 | 83 => vec![rgba[2], rgba[1], rgba[0], rgba[3]],
        5 | 49 => vec![rgba[0]],
        8 => vec![rgba[3]],
        _ => return,
    };
    for c in t.data.chunks_exact_mut(px.len()) { c.copy_from_slice(&px); }
    t.version += 1;
}

/// A texture's description for managed to read: one managed made, or a
/// file's (its header's size, format and mips), read once.
pub fn texture_desc(t: usize) -> *const TextureDesc {
    let mut d = device();
    if let Some(x) = d.textures.get(&t) { return &*x.desc; }
    if let Some(x) = d.file_descs.get(&t) { return &**x; }
    let desc = crate::camera::texture_name(t).and_then(|name| {
        let name = if name.ends_with("_c") { name } else { format!("{name}_c") };
        let r = crate::assets::resource::Resource::parse(crate::fs::read(&name)?).ok()?;
        let x = crate::assets::texture::read(r).ok()?;
        Some(TextureDesc { width: x.width as i16, height: x.height as i16, depth: x.depth as i16, mips: x.mips as i16, format: 0, ..Default::default() })
    }).unwrap_or(TextureDesc { width: 1, height: 1, depth: 1, mips: 1, ..Default::default() });
    let b = Box::new(desc);
    let p: *const TextureDesc = &*b;
    d.file_descs.insert(t, b);
    p
}

/// How many times a texture managed made has been written, or None for a
/// texture it didn't make (a file's).
pub fn texture_version(t: usize) -> Option<u64> {
    device().textures.get(&t).map(|x| x.version)
}

/// A texture managed made, as RGBA8 (straight alpha), its size and version:
/// what the renderer puts on the GPU.
pub fn texture_rgba(t: usize) -> Option<(u32, u32, u64, Vec<u8>)> {
    let d = device();
    let x = d.textures.get(&t)?;
    let (w, h) = (x.desc.width as u32, x.desc.height as u32);
    let n = (w * h) as usize;
    let s = &x.data;
    let f16 = |b: &[u8]| (half_to_f32(u16::from_le_bytes([b[0], b[1]])).clamp(0.0, 1.0) * 255.0 + 0.5) as u8;
    let px: Vec<u8> = match x.desc.format {
        0 | 42 | 55 | 81 => s.clone(),
        1 | 43 => s.chunks_exact(4).flat_map(|c| [c[3], c[2], c[1], c[0]]).collect(),
        11 | 44 => s.chunks_exact(4).flat_map(|c| [c[1], c[2], c[3], c[0]]).collect(),
        12 | 45 | 52 | 82 => s.chunks_exact(4).flat_map(|c| [c[2], c[1], c[0], c[3]]).collect(),
        16 | 41 | 51 | 83 => s.chunks_exact(4).flat_map(|c| [c[2], c[1], c[0], 255]).collect(),
        2 | 46 => s.chunks_exact(3).flat_map(|c| [c[0], c[1], c[2], 255]).collect(),
        3 | 47 => s.chunks_exact(3).flat_map(|c| [c[2], c[1], c[0], 255]).collect(),
        5 | 49 => s.iter().flat_map(|&c| [c, c, c, 255]).collect(),
        8 => s.iter().flat_map(|&c| [255, 255, 255, c]).collect(),
        6 => s.chunks_exact(2).flat_map(|c| [c[0], c[0], c[0], c[1]]).collect(),
        24 => s.chunks_exact(8).flat_map(|c| [f16(&c[0..2]), f16(&c[2..4]), f16(&c[4..6]), f16(&c[6..8])]).collect(),
        _ => return None,
    };
    if px.len() < n * 4 { return None; }
    Some((w, h, x.version, px))
}

fn half_to_f32(h: u16) -> f32 {
    let (sign, exp, frac) = ((h >> 15) as u32, ((h >> 10) & 0x1f) as u32, (h & 0x3ff) as u32);
    let s = if sign == 1 { -1.0 } else { 1.0 };
    match exp {
        0 => s * frac as f32 * (2.0f32).powi(-24),
        31 => f32::from_bits((sign << 31) | 0x7f80_0000 | (frac << 13)),
        _ => f32::from_bits((sign << 31) | ((exp + 112) << 23) | (frac << 13)),
    }
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
    let mut d = device();
    let Some(c) = d.contexts.get(&ctx) else { return };
    let attrs = c.attrs.clone();
    let index_buffer = if indexed { c.index_buffer } else { None };
    let (material, scissor) = (c.material, c.scissor);
    let buffers = attrs.values().filter_map(|v| match v { Val::Buffer(b) => d.buffers.get(b).map(|x| (*b, std::sync::Arc::new(x.data.clone()))), _ => None }).collect();
    if let Some(c) = d.contexts.get_mut(&ctx) {
        c.draws.push(Draw { material, attrs, primitive, index_buffer, first, count, instances, base_vertex, scissor, buffers, inline: None, grab: None });
    }
}

/// A draw with its own vertices (RenderTools.Draw): recorded as the others
/// are, the vertices and indices kept with it.
pub fn record_inline(ctx: usize, primitive: i64, inline: Inline) {
    let count = inline.indices.len().max(inline.count) as i32;
    record(ctx, primitive, 0, count, 1, 0, false);
    with_context(ctx, |c| if let Some(d) = c.draws.last_mut() { d.inline = Some(inline) });
}

/// What's been drawn so far, to be copied into `texture` here in the order of
/// things (RenderTools.ResolveFrameBuffer).
pub fn record_grab(ctx: usize, texture: usize) {
    with_context(ctx, |c| c.draws.push(Draw {
        material: 0, attrs: HashMap::new(), primitive: 0, index_buffer: None, first: 0, count: 0, instances: 0, base_vertex: 0,
        scissor: None, buffers: HashMap::new(), inline: None, grab: Some(texture),
    }));
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
        assert!(texture_index(78) > 0, "zero is no texture");
    }

    #[test]
    fn textures_keep_what_managed_writes() {
        // 4x2 BGRA8888 (ImageFormat 12), anonymous, no pixels yet
        let mut config = [0u8; 44];
        config[0] = 4; config[2] = 2; config[4] = 1; config[6] = 1; config[12] = 12;
        let t = create_texture("test bgra", true, &config, &[]);
        let desc = unsafe { *texture_desc(t) };
        assert_eq!((desc.width, desc.height, desc.format), (4, 2, 12));
        assert_eq!(std::mem::size_of::<TextureDesc>(), 28, "as managed's CTextureDesc");
        clear_texture(t, [10, 20, 30, 255]);
        // a 2x1 rect at (1, 1): blue, then green (as BGRA)
        write_texture(t, &[255, 0, 0, 255, 0, 255, 0, 255], [1, 1, 0, 2, 1, 1]);
        let (w, h, version, px) = texture_rgba(t).unwrap();
        assert_eq!((w, h, version), (4, 2, 2));
        assert_eq!(&px[0..4], &[10, 20, 30, 255], "cleared");
        assert_eq!(&px[20..24], &[0, 0, 255, 255], "(1, 1) blue");
        assert_eq!(&px[24..28], &[0, 255, 0, 255], "(2, 1) green");
        assert_eq!(&px[28..32], &[10, 20, 30, 255], "(3, 1) untouched");
        // named ones are found again by name; anonymous ones never
        let named = create_texture("test named", false, &config, &[]);
        assert_eq!(create_texture("test named", false, &config, &[]), named);
        assert_ne!(create_texture("test bgra", true, &config, &[]), t);
        // samplers: one object per description, its address modes kept
        let clamp = sampler(&[0x14, 0, 0, 2, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
        assert_eq!(sampler(&[0x14, 0, 0, 2, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]), clamp);
        let i = sampler_index(clamp);
        assert_eq!(samplers_by_index()[i as usize].address, [2, 2, 2]);
        assert!((half_to_f32(0x3c00) - 1.0).abs() < 1e-6 && (half_to_f32(0x3800) - 0.5).abs() < 1e-6 && half_to_f32(0x0001) > 0.0);
    }
}
