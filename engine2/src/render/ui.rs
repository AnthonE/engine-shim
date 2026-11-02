//! The UI over a frame: s&box's batched UI box (ui.wgsl, ported from
//! `ui_cssbox_batched.shader`) drawn from the tables managed filled, over
//! what's been drawn, in gamma space.
//!
//! The shader is bindless in s&box: a box names its background image, its
//! border image and its text mask by their bindless index. Here a draw
//! binds eight textures, found by those indices; a draw whose instances name
//! more is split where they outgrow them, in order, so what's drawn over
//! what is unchanged.
use std::collections::HashMap;
use std::sync::Mutex;

use wgpu::util::DeviceExt;

/// One batched UI draw, as managed recorded it: its tables' bytes and where
/// its instances start.
pub struct UiDraw {
    pub boxes: Vec<u8>,
    pub texts: Vec<u8>,
    pub glyph_table: Vec<u8>,
    pub glyph_curves: Vec<u8>,
    pub glyph_bands: Vec<u8>,
    pub transforms: Vec<u8>,
    pub scissors: Vec<u8>,
    pub gradients: Vec<u8>,
    /// LayerMat's sixteen floats, as managed set them
    pub layer: [f32; 16],
    pub instance_offset: i32,
    pub instances: u32,
    /// D_BLENDMODE: 0 alpha, 1 multiply, 2 lighten, 3 premultiplied
    pub blend: u8,
    /// UIPanelOpacity, where D_PANEL_OPACITY is on
    pub opacity: Option<f32>,
}

/// What a UI layer drew, in order: batches of boxes and glyphs; grabs of
/// what's been drawn so far (into a texture, by managed's handle); and the
/// backdrops that read a grab.
pub enum UiItem {
    Batch(UiDraw),
    Grab(usize),
    Backdrop(Backdrop),
    /// what's drawn from here goes into a render target managed bound, or
    /// (None) the frame again
    Target(Option<Target>),
    /// what's drawn into, cleared to a colour (as authored)
    Clear([f32; 4]),
    /// a game's own shader over a quad (hud.wgsl)
    Quad(Quad),
}

/// A render target: a texture managed made, drawn into on the GPU and read
/// back by its bindless index or by the attributes that name it.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Target {
    pub texture: usize,
    pub width: u32,
    pub height: u32,
    /// managed's ImageFormat: RGBA16161616F (24) is a float target, the rest 8 bits a channel
    pub format: i32,
}

/// Which of hud.wgsl's shaders a quad draws with.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum QuadShader { Glow, Composite }

/// A quad drawn with a game's own shader (hud.wgsl), and what it reads.
pub struct Quad {
    pub shader: QuadShader,
    /// a triangle list: x, y, u, v, then r, g, b, a a vertex
    pub vertices: Vec<[f32; 8]>,
    pub transform: [f32; 16],
    pub layer: [f32; 16],
    /// glow, hud, scan: hud.wgsl's Params, as the attributes give them
    pub glow: [f32; 4],
    pub hud: [f32; 4],
    pub scan: [f32; 4],
    /// UIGammaOutput
    pub gamma_output: bool,
    /// the textures it reads, by managed's object: GlowSource or HudTexture,
    /// GlowTexture, FrameBufferCopyTexture (a grab's)
    pub textures: [usize; 3],
}

/// A backdrop-filter's quad (ui_backdropfilter.shader), as managed drew it.
pub struct Backdrop {
    /// a triangle list: x, y, u, v, then r, g, b, a (as authored, gamma) a vertex
    pub vertices: Vec<[f32; 8]>,
    /// TransformMat and LayerMat, as WGSL's columns
    pub transform: [f32; 16],
    pub layer: [f32; 16],
    /// BoxPosition and BoxSize, and how far the quad reaches past it (BoxBloat)
    pub rect: [f32; 4],
    pub bloat: f32,
    pub radii: [f32; 4],
    pub radii_v: [f32; 4],
    /// brightness, contrast, saturate, sepia, invert, hue-rotate, blur
    pub filter: [f32; 7],
    /// border-shape: kind (1 polygon, 2 circle), point count, eight points, the circle
    pub shape: (i32, i32, [f32; 16], [f32; 4]),
    /// FrameBufferCopyTexture: the grab it reads (else the last one)
    pub texture: usize,
    pub opacity: f32,
}

const BOX: usize = 272;
const SCISSOR: usize = 464;
const GRADIENT: usize = 200;
/// textures a draw binds (WebGPU allows sixteen a stage)
const SLOTS: usize = 8;

pub struct Ui {
    shader: wgpu::ShaderModule,
    layout: wgpu::BindGroupLayout,
    pipelines: Mutex<HashMap<(wgpu::TextureFormat, u8), wgpu::RenderPipeline>>,
    /// textures by object: their version (managed's writes; a file's is
    /// u64::MAX), their view and size
    textures: Mutex<HashMap<usize, (u64, wgpu::TextureView, [u32; 2])>>,
    blank: wgpu::TextureView,
    sampler: wgpu::Sampler,
    /// backdrops, and the grabs' mip chains they read (backdrop.wgsl)
    backdrop_shader: wgpu::ShaderModule,
    backdrop_layout: wgpu::BindGroupLayout,
    down_layout: wgpu::BindGroupLayout,
    backdrop_pipelines: Mutex<HashMap<wgpu::TextureFormat, (wgpu::RenderPipeline, wgpu::RenderPipeline)>>,
    /// render targets on the GPU, by managed's texture, and the draw that last used each
    targets: Mutex<HashMap<usize, (Target, wgpu::Texture, u64)>>,
    draws: std::sync::atomic::AtomicU64,
    /// a game's own shaders (hud.wgsl): their bindings, and pipelines by shader, format and blend
    quad_shader: wgpu::ShaderModule,
    quad_layout: wgpu::BindGroupLayout,
    quad_pipelines: Mutex<HashMap<(QuadShader, wgpu::TextureFormat), wgpu::RenderPipeline>>,
}

/// One thing the UI's encoder does, in order, and into what (None: the frame;
/// else a render target, by managed's texture).
enum Step {
    Draw { into: Option<usize>, pipeline: wgpu::RenderPipeline, group: wgpu::BindGroup, vertices: u32, instances: u32 },
    Clear { into: Option<usize>, color: wgpu::Color },
    /// what's been drawn into `from` copied into `texture`'s top level, then each level below drawn from the one above
    Grab { from: Option<usize>, texture: wgpu::Texture, levels: Vec<(wgpu::TextureView, wgpu::BindGroup)> },
}

impl Ui {
    pub fn new(device: &wgpu::Device, queue: &wgpu::Queue) -> Ui {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor { label: Some("ui"), source: wgpu::ShaderSource::Wgsl(include_str!("ui.wgsl").into()) });
        let storage = |binding| wgpu::BindGroupLayoutEntry {
            binding,
            visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
            ty: wgpu::BindingType::Buffer { ty: wgpu::BufferBindingType::Storage { read_only: true }, has_dynamic_offset: false, min_binding_size: None },
            count: None,
        };
        let texture = |binding| wgpu::BindGroupLayoutEntry {
            binding,
            visibility: wgpu::ShaderStages::FRAGMENT,
            ty: wgpu::BindingType::Texture { sample_type: wgpu::TextureSampleType::Float { filterable: true }, view_dimension: wgpu::TextureViewDimension::D2, multisampled: false },
            count: None,
        };
        let mut entries = vec![wgpu::BindGroupLayoutEntry { binding: 0, visibility: wgpu::ShaderStages::VERTEX_FRAGMENT, ty: wgpu::BindingType::Buffer { ty: wgpu::BufferBindingType::Uniform, has_dynamic_offset: false, min_binding_size: None }, count: None }];
        entries.extend((1..=7).map(storage));
        entries.extend((8..8 + SLOTS as u32).map(texture));
        entries.push(wgpu::BindGroupLayoutEntry { binding: 16, visibility: wgpu::ShaderStages::FRAGMENT, ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering), count: None });
        let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor { label: Some("ui"), entries: &entries });
        let blank = device.create_texture_with_data(queue, &wgpu::TextureDescriptor {
            label: Some("ui blank"), size: wgpu::Extent3d { width: 1, height: 1, depth_or_array_layers: 1 }, mip_level_count: 1, sample_count: 1,
            dimension: wgpu::TextureDimension::D2, format: wgpu::TextureFormat::Rgba8Unorm, usage: wgpu::TextureUsages::TEXTURE_BINDING, view_formats: &[],
        }, wgpu::util::TextureDataOrder::LayerMajor, &[255, 255, 255, 255]).create_view(&Default::default());
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("ui"), mag_filter: wgpu::FilterMode::Linear, min_filter: wgpu::FilterMode::Linear, mipmap_filter: wgpu::FilterMode::Linear, ..Default::default()
        });
        let backdrop_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor { label: Some("ui backdrop"), source: wgpu::ShaderSource::Wgsl(include_str!("backdrop.wgsl").into()) });
        let tex = |binding| wgpu::BindGroupLayoutEntry {
            binding, visibility: wgpu::ShaderStages::FRAGMENT,
            ty: wgpu::BindingType::Texture { sample_type: wgpu::TextureSampleType::Float { filterable: true }, view_dimension: wgpu::TextureViewDimension::D2, multisampled: false }, count: None,
        };
        let samp = |binding| wgpu::BindGroupLayoutEntry { binding, visibility: wgpu::ShaderStages::FRAGMENT, ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering), count: None };
        let backdrop_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor { label: Some("ui backdrop"), entries: &[
            wgpu::BindGroupLayoutEntry { binding: 0, visibility: wgpu::ShaderStages::VERTEX_FRAGMENT, ty: wgpu::BindingType::Buffer { ty: wgpu::BufferBindingType::Uniform, has_dynamic_offset: false, min_binding_size: None }, count: None },
            storage(1), tex(2), samp(3),
        ] });
        let down_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor { label: Some("ui grab mips"), entries: &[tex(0), samp(1)] });
        let quad_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor { label: Some("ui quad"), source: wgpu::ShaderSource::Wgsl(include_str!("hud.wgsl").into()) });
        let quad_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor { label: Some("ui quad"), entries: &[
            wgpu::BindGroupLayoutEntry { binding: 0, visibility: wgpu::ShaderStages::VERTEX_FRAGMENT, ty: wgpu::BindingType::Buffer { ty: wgpu::BufferBindingType::Uniform, has_dynamic_offset: false, min_binding_size: None }, count: None },
            storage(1), tex(2), tex(3), tex(4), samp(5),
        ] });
        Ui { shader, layout, pipelines: Default::default(), textures: Default::default(), blank, sampler, backdrop_shader, backdrop_layout, down_layout, backdrop_pipelines: Default::default(),
            targets: Default::default(), draws: Default::default(), quad_shader, quad_layout, quad_pipelines: Default::default() }
    }

    fn pipeline(&self, device: &wgpu::Device, format: wgpu::TextureFormat, blend: u8) -> wgpu::RenderPipeline {
        use wgpu::{BlendComponent as C, BlendFactor as F, BlendOperation::Add};
        // as s&box's common/blendmode.hlsl sets them
        let state = match blend {
            1 => wgpu::BlendState { color: C { src_factor: F::Dst, dst_factor: F::Zero, operation: Add }, alpha: C { src_factor: F::One, dst_factor: F::Zero, operation: Add } },
            2 => wgpu::BlendState { color: C { src_factor: F::SrcAlpha, dst_factor: F::One, operation: Add }, alpha: C { src_factor: F::One, dst_factor: F::One, operation: Add } },
            3 => wgpu::BlendState::PREMULTIPLIED_ALPHA_BLENDING,
            _ => wgpu::BlendState::ALPHA_BLENDING,
        };
        let mut all = self.pipelines.lock().unwrap();
        all.entry((format, blend)).or_insert_with(|| device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("ui"),
            layout: Some(&device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor { label: None, bind_group_layouts: &[&self.layout], push_constant_ranges: &[] })),
            vertex: wgpu::VertexState { module: &self.shader, entry_point: Some("vs_main"), compilation_options: Default::default(), buffers: &[] },
            fragment: Some(wgpu::FragmentState {
                module: &self.shader,
                entry_point: Some("fs_main"),
                compilation_options: Default::default(),
                targets: &[Some(wgpu::ColorTargetState { format, blend: Some(state), write_mask: wgpu::ColorWrites::ALL })],
            }),
            primitive: wgpu::PrimitiveState { cull_mode: None, ..Default::default() },
            depth_stencil: None,
            multisample: Default::default(),
            multiview: None,
            cache: None,
        })).clone()
    }

    /// The backdrop's pipeline and the grab's downsampling one, for a target's format.
    fn backdrop_pipelines(&self, device: &wgpu::Device, format: wgpu::TextureFormat) -> (wgpu::RenderPipeline, wgpu::RenderPipeline) {
        let mut all = self.backdrop_pipelines.lock().unwrap();
        all.entry(format).or_insert_with(|| {
            let make = |label, layout: &wgpu::BindGroupLayout, vs, fs, blend| device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some(label),
                layout: Some(&device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor { label: None, bind_group_layouts: &[layout], push_constant_ranges: &[] })),
                vertex: wgpu::VertexState { module: &self.backdrop_shader, entry_point: Some(vs), compilation_options: Default::default(), buffers: &[] },
                fragment: Some(wgpu::FragmentState { module: &self.backdrop_shader, entry_point: Some(fs), compilation_options: Default::default(), targets: &[Some(wgpu::ColorTargetState { format, blend, write_mask: wgpu::ColorWrites::ALL })] }),
                primitive: wgpu::PrimitiveState { cull_mode: None, ..Default::default() },
                depth_stencil: None, multisample: Default::default(), multiview: None, cache: None,
            });
            (make("ui backdrop", &self.backdrop_layout, "vs_backdrop", "fs_backdrop", Some(wgpu::BlendState::ALPHA_BLENDING)), make("ui grab mips", &self.down_layout, "vs_down", "fs_down", None))
        }).clone()
    }

    /// The texture at a bindless index on the GPU, and its size: one managed
    /// made (put there again once managed writes to it), or a file's.
    fn texture(&self, r: &super::Renderer, index: i32) -> Option<(wgpu::TextureView, [u32; 2])> {
        self.texture_of(r, crate::gpu::texture_at(index)?)
    }

    /// A texture managed made, on the GPU, by its object: a render target as
    /// it was last drawn, or one of the others (`texture`).
    fn texture_of(&self, r: &super::Renderer, t: usize) -> Option<(wgpu::TextureView, [u32; 2])> {
        if let Some((target, texture, _)) = self.targets.lock().unwrap().get(&t) { return Some((texture.create_view(&Default::default()), [target.width, target.height])); }
        let index = crate::gpu::texture_index(t);
        let version = crate::gpu::texture_version(t);
        if let Some((v, view, size)) = self.textures.lock().unwrap().get(&t) {
            if Some(*v) == version || (version.is_none() && *v == u64::MAX) { return Some((view.clone(), *size)); }
        }
        let made = if version.is_some() {
            let (w, h, v, px) = crate::gpu::texture_rgba(t)?;
            static SAID: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
            if SAID.fetch_add(1, std::sync::atomic::Ordering::Relaxed) < 64 {
                let seen = px.chunks_exact(4).filter(|p| p[3] > 0).count();
                let name = crate::gpu::device().textures.get(&t).map(|x| x.name.clone()).unwrap_or_default();
                crate::trace::note(&format!("ui: image {index} '{name}' {w}x{h} (version {v}), {}% of it showing", seen * 100 / (px.len() / 4).max(1)));
            }
            let view = r.device.create_texture_with_data(&r.queue, &wgpu::TextureDescriptor {
                label: Some("ui image"), size: wgpu::Extent3d { width: w, height: h, depth_or_array_layers: 1 }, mip_level_count: 1, sample_count: 1,
                dimension: wgpu::TextureDimension::D2, format: wgpu::TextureFormat::Rgba8Unorm, usage: wgpu::TextureUsages::TEXTURE_BINDING, view_formats: &[],
            }, wgpu::util::TextureDataOrder::LayerMajor, &px).create_view(&Default::default());
            (v, view, [w, h])
        } else {
            // a file's: its texels as they're stored, as gamma-space UI reads them
            let name = crate::camera::texture_name(t)?;
            let view = r.texture(&super::EngineFiles, &name, false).map_err(|e| crate::trace::note(&format!("ui: {e}"))).ok()?;
            let desc = unsafe { *crate::gpu::texture_desc(t) };
            (u64::MAX, view, [desc.width.max(1) as u32, desc.height.max(1) as u32])
        };
        let out = (made.1.clone(), made.2);
        self.textures.lock().unwrap().insert(t, made);
        Some(out)
    }

    /// The GPU's texture for a render target, made (again, if its size or
    /// format changed) as it's bound.
    fn target(&self, device: &wgpu::Device, t: Target, draw: u64) -> wgpu::Texture {
        let mut all = self.targets.lock().unwrap();
        if let Some((was, texture, used)) = all.get_mut(&t.texture) {
            if *was == t { *used = draw; return texture.clone(); }
        }
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("ui render target"), size: wgpu::Extent3d { width: t.width.max(1), height: t.height.max(1), depth_or_array_layers: 1 }, mip_level_count: 1, sample_count: 1,
            dimension: wgpu::TextureDimension::D2, format: target_format(t.format),
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC | wgpu::TextureUsages::COPY_DST, view_formats: &[],
        });
        all.insert(t.texture, (t, texture.clone(), draw));
        // a target unbound for a while is one managed let go of
        all.retain(|_, (_, _, used)| draw.saturating_sub(*used) < 600);
        texture
    }

    /// A game's own shader's pipeline (hud.wgsl), for a target's format.
    fn quad_pipeline(&self, device: &wgpu::Device, shader: QuadShader, format: wgpu::TextureFormat) -> wgpu::RenderPipeline {
        let mut all = self.quad_pipelines.lock().unwrap();
        all.entry((shader, format)).or_insert_with(|| {
            // hud_glow writes without blending; hud_composite as D_BLENDMODE Normal
            let (vs, fs, blend) = match shader { QuadShader::Glow => ("vs_blit", "fs_glow", None), QuadShader::Composite => ("vs_ui", "fs_composite", Some(wgpu::BlendState::ALPHA_BLENDING)) };
            device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some(fs),
                layout: Some(&device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor { label: None, bind_group_layouts: &[&self.quad_layout], push_constant_ranges: &[] })),
                vertex: wgpu::VertexState { module: &self.quad_shader, entry_point: Some(vs), compilation_options: Default::default(), buffers: &[] },
                fragment: Some(wgpu::FragmentState { module: &self.quad_shader, entry_point: Some(fs), compilation_options: Default::default(), targets: &[Some(wgpu::ColorTargetState { format, blend, write_mask: wgpu::ColorWrites::ALL })] }),
                primitive: wgpu::PrimitiveState { cull_mode: None, ..Default::default() },
                depth_stencil: None, multisample: Default::default(), multiview: None, cache: None,
            })
        }).clone()
    }

    /// Draws over `target` (a gamma-space view of the frame), `width` by
    /// `height`, in order; `texture` is the target's, for grabs to copy from
    /// (without it a backdrop reads nothing, and isn't drawn). What managed
    /// drew into a render target of its own goes into one on the GPU, kept
    /// for what reads it, this frame and after.
    pub fn draw(&self, r: &super::Renderer, target: &wgpu::TextureView, texture: Option<&wgpu::Texture>, format: wgpu::TextureFormat, width: u32, height: u32, items: &[UiItem]) {
        if items.is_empty() { return; }
        let device = &r.device;
        let draw_no = self.draws.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        static SAID: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
        if !SAID.swap(true, std::sync::atomic::Ordering::Relaxed) {
            crate::trace::note(&format!("ui: drawing {} items over {width}x{height} ({format:?}): {:?}", items.len(), items.iter().map(|i| match i {
                UiItem::Batch(d) => format!("batch {} from {} ({} scissors, {} gradients, blend {})", d.instances, d.instance_offset, d.scissors.len() / SCISSOR, d.gradients.len() / GRADIENT, d.blend),
                UiItem::Grab(t) => format!("grab {t:#x}"),
                UiItem::Backdrop(b) => format!("backdrop {:?} blur {}", b.rect, b.filter[6]),
                UiItem::Target(t) => format!("into {t:?}"),
                UiItem::Clear(c) => format!("clear {c:?}"),
                UiItem::Quad(q) => format!("{:?} quad, {} vertices", q.shader, q.vertices.len()),
            }).collect::<Vec<_>>()));
        }
        // the samplers by bindless index: filter and address modes a word, the border colour another
        let samplers: Vec<u8> = crate::gpu::samplers_by_index().iter().flat_map(|s| {
            let w = s.filter as u32 | (s.address[0] as u32) << 8 | (s.address[1] as u32) << 16;
            [w.to_le_bytes(), s.border.to_le_bytes()].concat()
        }).collect();
        // a table can't be empty; a word of nothing stands in
        let buffer = |bytes: &[u8]| {
            let contents: Vec<u8> = if bytes.len() < 16 { let mut v = bytes.to_vec(); v.resize(16, 0); v } else { bytes[..bytes.len() / 4 * 4].to_vec() };
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor { label: Some("ui table"), contents: &contents, usage: wgpu::BufferUsages::STORAGE })
        };
        let solid = std::env::var("ENGINE2_UI_SOLID").is_ok();
        let mut steps: Vec<Step> = Vec::new();
        // the grabs made so far, by managed's handle, and the last
        let mut grabs: HashMap<usize, wgpu::TextureView> = HashMap::new();
        let mut last_grab: Option<wgpu::TextureView> = None;
        // each grab's format, by handle; the last's
        let mut grab_formats: HashMap<usize, wgpu::TextureFormat> = HashMap::new();
        let mut last_format: Option<wgpu::TextureFormat> = None;
        // what's drawn into now: the frame, or a render target (its GPU texture)
        let mut into: Option<(Target, wgpu::Texture)> = None;
        for item in items {
            let (w, h, fmt) = match &into { Some((t, x)) => (t.width, t.height, x.format()), None => (width, height, format) };
            let at = into.as_ref().map(|(t, _)| t.texture);
            match item {
                UiItem::Target(t) => {
                    into = t.map(|t| (t, self.target(device, t, draw_no)));
                }
                UiItem::Clear(c) => {
                    let [r_, g, b, a] = c.map(|x| x as f64);
                    steps.push(Step::Clear { into: at, color: wgpu::Color { r: r_, g, b, a } });
                }
                UiItem::Batch(d) => {
                    if d.instances == 0 { continue; }
                    let (scissors, gradients) = (&d.scissors[..d.scissors.len() / SCISSOR * SCISSOR], &d.gradients[..d.gradients.len() / GRADIENT * GRADIENT]);
                    let mut extra = Vec::with_capacity(scissors.len() + gradients.len() + samplers.len());
                    extra.extend_from_slice(scissors);
                    extra.extend_from_slice(gradients);
                    extra.extend_from_slice(&samplers);
                    let (scissor_at, gradient_at, sampler_at) = (0u32, (scissors.len() / 4) as u32, ((scissors.len() + gradients.len()) / 4) as u32);
                    let tables = [buffer(&d.boxes), buffer(&d.texts), buffer(&d.glyph_table), buffer(&d.glyph_curves), buffer(&d.glyph_bands), buffer(&d.transforms), buffer(&extra)];
                    let pipeline = self.pipeline(device, fmt, d.blend.min(3));
                    for (start, count, slots) in runs(d) {
                        // a target isn't read while it's drawn into
                        let bound: Vec<Option<(wgpu::TextureView, [u32; 2])>> = slots.iter().map(|&i| if at.is_some() && crate::gpu::texture_at(i) == at { None } else { self.texture(r, i) }).collect();
                        let mut params = vec![0.0f32, 0.0, w as f32, h as f32];
                        params.extend_from_slice(&d.layer);
                        let mut bytes: Vec<u8> = bytemuck::cast_slice(&params).to_vec();
                        let flags = solid as u32 | ((d.blend == 3) as u32) << 1 | (d.opacity.is_some() as u32) << 2;
                        for w in [start as u32, flags, d.opacity.unwrap_or(1.0).to_bits(), 0,
                            scissor_at, (scissors.len() / SCISSOR) as u32, gradient_at, (gradients.len() / GRADIENT) as u32,
                            sampler_at, (samplers.len() / 8) as u32, 0, 0] {
                            bytes.extend_from_slice(&w.to_le_bytes());
                        }
                        for k in 0..SLOTS {
                            let (index, size) = match (slots.get(k), bound.get(k)) { (Some(&i), Some(Some((_, s)))) => (i, *s), _ => (0, [1, 1]) };
                            for w in [index, size[0] as i32, size[1] as i32, 0] { bytes.extend_from_slice(&w.to_le_bytes()); }
                        }
                        let uniform = device.create_buffer_init(&wgpu::util::BufferInitDescriptor { label: Some("ui params"), contents: &bytes, usage: wgpu::BufferUsages::UNIFORM });
                        let mut entries = vec![wgpu::BindGroupEntry { binding: 0, resource: uniform.as_entire_binding() }];
                        for (k, t) in tables.iter().enumerate() { entries.push(wgpu::BindGroupEntry { binding: k as u32 + 1, resource: t.as_entire_binding() }); }
                        let views: Vec<&wgpu::TextureView> = (0..SLOTS).map(|k| bound.get(k).and_then(|b| b.as_ref()).map(|b| &b.0).unwrap_or(&self.blank)).collect();
                        for (k, v) in views.iter().enumerate() { entries.push(wgpu::BindGroupEntry { binding: 8 + k as u32, resource: wgpu::BindingResource::TextureView(v) }); }
                        entries.push(wgpu::BindGroupEntry { binding: 16, resource: wgpu::BindingResource::Sampler(&self.sampler) });
                        let group = device.create_bind_group(&wgpu::BindGroupDescriptor { label: Some("ui"), layout: &self.layout, entries: &entries });
                        steps.push(Step::Draw { into: at, pipeline: pipeline.clone(), group, vertices: 6, instances: count });
                    }
                }
                UiItem::Grab(handle) => {
                    let Some(source_format) = (match &into { Some((_, x)) => Some(x.format()), None => texture.map(|t| t.format()) }) else { continue };
                    // what's been drawn so far, and a mip chain below it, each level blurred from the one above
                    let levels = 32 - w.max(h).max(1).leading_zeros();
                    let grab = device.create_texture(&wgpu::TextureDescriptor {
                        label: Some("ui grab"), size: wgpu::Extent3d { width: w, height: h, depth_or_array_layers: 1 }, mip_level_count: levels, sample_count: 1,
                        dimension: wgpu::TextureDimension::D2, format: source_format,
                        usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_DST, view_formats: &[],
                    });
                    let (_, down) = self.backdrop_pipelines(device, source_format);
                    let level = |l: u32| grab.create_view(&wgpu::TextureViewDescriptor { base_mip_level: l, mip_level_count: Some(1), ..Default::default() });
                    let chain = (1..levels).map(|l| {
                        let group = device.create_bind_group(&wgpu::BindGroupDescriptor { label: Some("ui grab mip"), layout: &down.get_bind_group_layout(0), entries: &[
                            wgpu::BindGroupEntry { binding: 0, resource: wgpu::BindingResource::TextureView(&level(l - 1)) },
                            wgpu::BindGroupEntry { binding: 1, resource: wgpu::BindingResource::Sampler(&self.sampler) },
                        ] });
                        (level(l), group)
                    }).collect();
                    let view = grab.create_view(&Default::default());
                    grabs.insert(*handle, view.clone());
                    last_grab = Some(view);
                    grab_formats.insert(*handle, source_format);
                    last_format = Some(source_format);
                    steps.push(Step::Grab { from: at, texture: grab, levels: chain });
                }
                UiItem::Backdrop(b) => {
                    let Some(view) = grabs.get(&b.texture).or(last_grab.as_ref()) else { continue };
                    if b.vertices.is_empty() { continue; }
                    let (pipeline, _) = self.backdrop_pipelines(device, fmt);
                    let mut u: Vec<f32> = vec![0.0, 0.0, w as f32, h as f32];
                    u.extend_from_slice(&b.layer);
                    u.extend_from_slice(&b.transform);
                    u.extend_from_slice(&b.rect);
                    u.extend_from_slice(&b.radii);
                    u.extend_from_slice(&b.radii_v);
                    u.extend_from_slice(&b.filter[..4]);
                    let levels = (32 - w.max(h).max(1).leading_zeros()) as f32;
                    u.extend_from_slice(&[b.filter[4], b.filter[5], b.filter[6], levels]);
                    u.extend_from_slice(&[b.shape.0 as f32, b.shape.1 as f32, b.bloat, b.opacity]);
                    u.extend_from_slice(&b.shape.2);
                    u.extend_from_slice(&b.shape.3);
                    let uniform = device.create_buffer_init(&wgpu::util::BufferInitDescriptor { label: Some("ui backdrop"), contents: bytemuck::cast_slice(&u), usage: wgpu::BufferUsages::UNIFORM });
                    let vertices = device.create_buffer_init(&wgpu::util::BufferInitDescriptor { label: Some("ui backdrop vertices"), contents: bytemuck::cast_slice(&b.vertices), usage: wgpu::BufferUsages::STORAGE });
                    let group = device.create_bind_group(&wgpu::BindGroupDescriptor { label: Some("ui backdrop"), layout: &self.backdrop_layout, entries: &[
                        wgpu::BindGroupEntry { binding: 0, resource: uniform.as_entire_binding() },
                        wgpu::BindGroupEntry { binding: 1, resource: vertices.as_entire_binding() },
                        wgpu::BindGroupEntry { binding: 2, resource: wgpu::BindingResource::TextureView(view) },
                        wgpu::BindGroupEntry { binding: 3, resource: wgpu::BindingResource::Sampler(&self.sampler) },
                    ] });
                    steps.push(Step::Draw { into: at, pipeline, group, vertices: b.vertices.len() as u32, instances: 1 });
                }
                UiItem::Quad(q) => {
                    if q.vertices.is_empty() { continue; }
                    // what it reads: render targets as drawn, a grab by its handle, else a texture managed made
                    let read = |t: usize, grab: bool| -> Option<wgpu::TextureView> {
                        if t == 0 || Some(t) == at { return None; }
                        if grab { if let Some(v) = grabs.get(&t).or(last_grab.as_ref()) { return Some(v.clone()); } }
                        self.texture_of(r, t).map(|x| x.0)
                    };
                    let views = [read(q.textures[0], false), read(q.textures[1], false), read(q.textures[2], true)];
                    if q.shader == QuadShader::Composite && views[0].is_none() { continue; }
                    let grab_srgb = grab_formats.get(&q.textures[2]).copied().or(last_format).is_some_and(|f| f.is_srgb());
                    let mut u: Vec<f32> = vec![0.0, 0.0, w as f32, h as f32];
                    u.extend_from_slice(&q.layer);
                    u.extend_from_slice(&q.transform);
                    u.extend_from_slice(&q.glow);
                    u.extend_from_slice(&q.hud);
                    u.extend_from_slice(&q.scan);
                    // the grab's bytes are sRGB where its format doesn't decode them
                    u.extend_from_slice(&[(!grab_srgb) as u32 as f32, q.gamma_output as u32 as f32, 1.0, 1.0]);
                    let uniform = device.create_buffer_init(&wgpu::util::BufferInitDescriptor { label: Some("ui quad"), contents: bytemuck::cast_slice(&u), usage: wgpu::BufferUsages::UNIFORM });
                    let vertices = device.create_buffer_init(&wgpu::util::BufferInitDescriptor { label: Some("ui quad vertices"), contents: bytemuck::cast_slice(&q.vertices), usage: wgpu::BufferUsages::STORAGE });
                    let view = |k: usize| views[k].as_ref().unwrap_or(&self.blank);
                    let group = device.create_bind_group(&wgpu::BindGroupDescriptor { label: Some("ui quad"), layout: &self.quad_layout, entries: &[
                        wgpu::BindGroupEntry { binding: 0, resource: uniform.as_entire_binding() },
                        wgpu::BindGroupEntry { binding: 1, resource: vertices.as_entire_binding() },
                        wgpu::BindGroupEntry { binding: 2, resource: wgpu::BindingResource::TextureView(view(0)) },
                        wgpu::BindGroupEntry { binding: 3, resource: wgpu::BindingResource::TextureView(view(1)) },
                        wgpu::BindGroupEntry { binding: 4, resource: wgpu::BindingResource::TextureView(view(2)) },
                        wgpu::BindGroupEntry { binding: 5, resource: wgpu::BindingResource::Sampler(&self.sampler) },
                    ] });
                    steps.push(Step::Draw { into: at, pipeline: self.quad_pipeline(device, q.shader, fmt), group, vertices: q.vertices.len() as u32, instances: 1 });
                }
            }
        }
        // the GPU's textures of the targets drawn into, by managed's
        let targets: HashMap<usize, wgpu::Texture> = self.targets.lock().unwrap().iter().map(|(k, (_, t, _))| (*k, t.clone())).collect();
        let view_of = |into: Option<usize>| -> Option<wgpu::TextureView> {
            match into { None => Some(target.clone()), Some(t) => targets.get(&t).map(|x| x.create_view(&Default::default())) }
        };
        let mut encoder = device.create_command_encoder(&Default::default());
        let mut i = 0;
        while i < steps.len() {
            match &steps[i] {
                // the draws into one target, in one pass over it: cleared first, if that's where it starts
                Step::Draw { into, .. } | Step::Clear { into, .. } => {
                    let into = *into;
                    let load = match &steps[i] { Step::Clear { color, .. } => { i += 1; wgpu::LoadOp::Clear(*color) } _ => wgpu::LoadOp::Load };
                    let Some(view) = view_of(into) else { i += 1; continue };
                    let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                        label: Some("ui"),
                        color_attachments: &[Some(wgpu::RenderPassColorAttachment { view: &view, resolve_target: None, ops: wgpu::Operations { load, store: wgpu::StoreOp::Store } })],
                        depth_stencil_attachment: None,
                        timestamp_writes: None,
                        occlusion_query_set: None,
                    });
                    while let Some(Step::Draw { into: x, pipeline, group, vertices, instances }) = steps.get(i) {
                        if *x != into { break; }
                        pass.set_pipeline(pipeline);
                        pass.set_bind_group(0, group, &[]);
                        pass.draw(0..*vertices, 0..*instances);
                        i += 1;
                    }
                }
                // a grab: what's been drawn copied, and its levels drawn down
                Step::Grab { from, texture: grab, levels } => {
                    i += 1;
                    let source = match from { None => texture.cloned(), Some(t) => targets.get(t).cloned() };
                    let Some(source) = source else { continue };
                    let size = grab.size();
                    encoder.copy_texture_to_texture(source.as_image_copy(), grab.as_image_copy(), wgpu::Extent3d { width: size.width.min(source.width()), height: size.height.min(source.height()), depth_or_array_layers: 1 });
                    let (_, down) = self.backdrop_pipelines(device, source.format());
                    for (view, group) in levels {
                        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                            label: Some("ui grab mip"),
                            color_attachments: &[Some(wgpu::RenderPassColorAttachment { view, resolve_target: None, ops: wgpu::Operations { load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT), store: wgpu::StoreOp::Store } })],
                            depth_stencil_attachment: None,
                            timestamp_writes: None,
                            occlusion_query_set: None,
                        });
                        pass.set_pipeline(&down);
                        pass.set_bind_group(0, group, &[]);
                        pass.draw(0..3, 0..1);
                    }
                }
            }
        }
        r.queue.submit([encoder.finish()]);
    }
}

/// A render target's format on the GPU, for managed's ImageFormat: a float
/// one (RGBA16161616F, 24), else eight bits a channel, as the UI writes them
/// (gamma-encoded, so not sRGB).
pub fn target_format(format: i32) -> wgpu::TextureFormat {
    match format { 24 | 25 => wgpu::TextureFormat::Rgba16Float, _ => wgpu::TextureFormat::Rgba8Unorm }
}

/// The bindless textures a box instance samples: its background image, its
/// border image, its text mask. A glyph samples none, nor a shadow or outline.
pub fn instance_textures(boxes: &[u8], i: usize) -> Vec<i32> {
    let Some(b) = boxes.get(i * BOX..(i + 1) * BOX) else { return Vec::new() };
    let int = |at: usize| i32::from_le_bytes(b[at..at + 4].try_into().unwrap());
    let flags = int(228) as u32;
    if flags & (1 << 14) != 0 || flags & 3 != 0 { return Vec::new(); }
    let mut out = Vec::new();
    if int(144) > 0 { out.push(int(144)); }
    if (flags >> 2) & 3 != 0 && int(188) > 0 { out.push(int(188)); }
    if (flags >> 12) & 3 == 3 && int(244) > 0 { out.push(int(244)); }
    out
}

/// A draw's instances in runs that each need no more textures than a draw
/// binds: (first instance, how many, the textures' bindless indices).
pub fn runs(d: &UiDraw) -> Vec<(i32, u32, Vec<i32>)> {
    let mut out = Vec::new();
    let (first, end) = (d.instance_offset.max(0), d.instance_offset.max(0) + d.instances as i32);
    let (mut start, mut slots) = (first, Vec::<i32>::new());
    for i in first..end {
        let mut need = instance_textures(&d.boxes, i as usize);
        need.retain(|t| !slots.contains(t));
        need.dedup();
        if slots.len() + need.len() > SLOTS {
            out.push((start, (i - start) as u32, std::mem::take(&mut slots)));
            start = i;
        }
        slots.extend(need);
    }
    if end > start { out.push((start, (end - start) as u32, slots)); }
    out
}

/// What a draw's instances are, counted: glyphs, plain boxes, images,
/// gradients, borders, shadows, outlines, shapes, and how many are clipped.
pub fn census(d: &UiDraw) -> String {
    let (mut glyphs, mut boxes, mut images, mut gradients, mut borders, mut shadows, mut outlines, mut shapes, mut clipped) = (0, 0, 0, 0, 0, 0, 0, 0, 0);
    let first = d.instance_offset.max(0) as usize;
    for i in first..first + d.instances as usize {
        let Some(b) = d.boxes.get(i * BOX..(i + 1) * BOX) else { break };
        let int = |at: usize| i32::from_le_bytes(b[at..at + 4].try_into().unwrap());
        let float = |at: usize| f32::from_le_bytes(b[at..at + 4].try_into().unwrap());
        let flags = int(228) as u32;
        if int(232) >= 0 { clipped += 1; }
        if flags & (1 << 14) != 0 { glyphs += 1; continue; }
        match flags & 3 { 1 | 2 => { shadows += 1; continue; } 3 => { outlines += 1; continue; } _ => {} }
        if int(268) >= 0 { shapes += 1; }
        if int(144) > 0 { images += 1; } else if int(144) < 0 { gradients += 1; } else { boxes += 1; }
        if (64..80).step_by(4).any(|at| float(at) != 0.0) || (flags >> 2) & 3 != 0 { borders += 1; }
    }
    format!("{glyphs} glyphs, {boxes} boxes, {images} images, {gradients} gradients, {borders} borders, {shadows} shadows, {outlines} outlines, {shapes} shapes; {clipped} clipped")
}

/// A draw's first image instances as managed filled them (ENGINE2_UI_DEBUG):
/// where, in what colour, the image's tile and tint, its sampler and texture.
pub fn images(d: &UiDraw, most: usize) -> Vec<String> {
    let samplers = crate::gpu::samplers_by_index();
    let first = d.instance_offset.max(0) as usize;
    (first..first + d.instances as usize).filter_map(|i| {
        let b = d.boxes.get(i * BOX..(i + 1) * BOX)?;
        let int = |at: usize| i32::from_le_bytes(b[at..at + 4].try_into().unwrap());
        let v4 = |at: usize| -> [f32; 4] { bytemuck::cast_slice::<u8, f32>(&b[at..at + 16]).try_into().unwrap() };
        let flags = int(228) as u32;
        if flags & (1 << 14) != 0 || flags & 3 != 0 || int(144) <= 0 { return None; }
        let t = crate::gpu::texture_at(int(144));
        let what = t.map(|t| {
            let desc = unsafe { *crate::gpu::texture_desc(t) };
            let name = crate::gpu::device().textures.get(&t).map(|x| x.name.clone()).or_else(|| crate::camera::texture_name(t)).unwrap_or_default();
            format!("'{name}' {}x{} format {}", desc.width, desc.height, desc.format)
        }).unwrap_or_else(|| "none".into());
        Some(format!("box {i}: rect {:?} color {:?} image {} {what}, tile {:?} tint {:?} sampler {} {:?} flags {flags:#x} clip {}",
            v4(0), v4(16), int(144), v4(156), v4(172), int(148), samplers.get(int(148).max(0) as usize), int(232)))
    }).take(most).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A box instance: its rect and colour, the rest as the batcher leaves a plain box.
    fn instance(rect: [f32; 4], color: [f32; 4]) -> Vec<u8> {
        let mut b = vec![0u8; BOX];
        let mut put = |at: usize, v: &[u8]| b[at..at + v.len()].copy_from_slice(v);
        put(0, bytemuck::cast_slice(&rect));
        put(16, bytemuck::cast_slice(&color));
        for at in [232usize, 240, 268] { put(at, &(-1i32).to_le_bytes()); } // no clip, no inverse clip, no shape
        b
    }
    fn set_i(b: &mut [u8], at: usize, v: i32) { b[at..at + 4].copy_from_slice(&v.to_le_bytes()); }
    fn set_f(b: &mut [u8], at: usize, v: &[f32]) { b[at..at + v.len() * 4].copy_from_slice(bytemuck::cast_slice(v)); }

    /// One draw of these box instances over nothing, `w` by `h`, read back as RGBA.
    fn drawn(r: &crate::render::Renderer, w: u32, h: u32, boxes: Vec<u8>) -> Vec<u8> {
        let identity = [1.0f32, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0];
        let instances = (boxes.len() / BOX) as u32;
        let draw = UiDraw { boxes, texts: vec![], glyph_table: vec![], glyph_curves: vec![], glyph_bands: vec![], transforms: bytemuck::cast_slice(&identity).to_vec(),
            scissors: vec![], gradients: vec![], layer: identity, instance_offset: 0, instances, blend: 0, opacity: None };
        let target = r.device.create_texture(&wgpu::TextureDescriptor {
            label: None, size: wgpu::Extent3d { width: w, height: h, depth_or_array_layers: 1 }, mip_level_count: 1, sample_count: 1, dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8Unorm, usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC, view_formats: &[],
        });
        r.ui.draw(r, &target.create_view(&Default::default()), Some(&target), wgpu::TextureFormat::Rgba8Unorm, w, h, &[UiItem::Batch(draw)]);
        let row = (w * 4).div_ceil(256) * 256;
        let readback = r.device.create_buffer(&wgpu::BufferDescriptor { label: None, size: (row * h) as u64, usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ, mapped_at_creation: false });
        let mut encoder = r.device.create_command_encoder(&Default::default());
        encoder.copy_texture_to_buffer(
            wgpu::TexelCopyTextureInfo { texture: &target, mip_level: 0, origin: wgpu::Origin3d::ZERO, aspect: wgpu::TextureAspect::All },
            wgpu::TexelCopyBufferInfo { buffer: &readback, layout: wgpu::TexelCopyBufferLayout { offset: 0, bytes_per_row: Some(row), rows_per_image: Some(h) } },
            wgpu::Extent3d { width: w, height: h, depth_or_array_layers: 1 },
        );
        r.queue.submit([encoder.finish()]);
        readback.slice(..).map_async(wgpu::MapMode::Read, |_| {});
        let _ = r.device.poll(wgpu::PollType::Wait);
        let data = readback.slice(..).get_mapped_range();
        (0..h as usize).flat_map(|y| data[y * row as usize..y * row as usize + w as usize * 4].to_vec()).collect()
    }

    /// A texture's top level read back, `w` by `h`, four bytes a texel.
    fn read_back(r: &crate::render::Renderer, texture: &wgpu::Texture, w: u32, h: u32) -> Vec<u8> {
        let row = (w * 4).div_ceil(256) * 256;
        let readback = r.device.create_buffer(&wgpu::BufferDescriptor { label: None, size: (row * h) as u64, usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ, mapped_at_creation: false });
        let mut encoder = r.device.create_command_encoder(&Default::default());
        encoder.copy_texture_to_buffer(texture.as_image_copy(), wgpu::TexelCopyBufferInfo { buffer: &readback, layout: wgpu::TexelCopyBufferLayout { offset: 0, bytes_per_row: Some(row), rows_per_image: Some(h) } }, wgpu::Extent3d { width: w, height: h, depth_or_array_layers: 1 });
        r.queue.submit([encoder.finish()]);
        readback.slice(..).map_async(wgpu::MapMode::Read, |_| {});
        let _ = r.device.poll(wgpu::PollType::Wait);
        let data = readback.slice(..).get_mapped_range();
        (0..h as usize).flat_map(|y| data[y * row as usize..y * row as usize + w as usize * 4].to_vec()).collect()
    }

    #[test]
    fn what_is_painted_into_a_render_target_is_composited_with_its_glow() {
        let Ok(r) = crate::render::Renderer::new() else { eprintln!("no adapter: skipped"); return };
        let (w, h) = (64u32, 32u32);
        // two float render targets managed made (RGBA16161616F, TSPEC_RENDER_TARGET): the HUD and its glow
        let mut config = [0u8; 44];
        config[0..2].copy_from_slice(&(w as i16).to_le_bytes()); config[2..4].copy_from_slice(&(h as i16).to_le_bytes());
        config[4] = 1; config[6] = 1; config[12] = 24; config[16] = crate::gpu::RENDER_TARGET as u8;
        let (hud, glow) = (crate::gpu::create_texture("hud", true, &config, &[]), crate::gpu::create_texture("glow", true, &config, &[]));
        assert!(crate::gpu::is_render_target(hud));
        let (hud_t, glow_t) = (Target { texture: hud, width: w, height: h, format: 24 }, Target { texture: glow, width: w, height: h, format: 24 });
        // the frame: mid grey
        let frame = r.device.create_texture(&wgpu::TextureDescriptor {
            label: None, size: wgpu::Extent3d { width: w, height: h, depth_or_array_layers: 1 }, mip_level_count: 1, sample_count: 1, dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8Unorm, usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC | wgpu::TextureUsages::COPY_DST, view_formats: &[],
        });
        r.queue.write_texture(frame.as_image_copy(), &vec![128u8; (w * h * 4) as usize], wgpu::TexelCopyBufferLayout { offset: 0, bytes_per_row: Some(w * 4), rows_per_image: Some(h) }, wgpu::Extent3d { width: w, height: h, depth_or_array_layers: 1 });
        let identity = [1.0f32, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0];
        // painted: a box four times brighter than white (Sandbox's digits glow so), at 8..24
        let paint = UiDraw { boxes: instance([8.0, 8.0, 16.0, 16.0], [4.0, 4.0, 4.0, 1.0]), texts: vec![], glyph_table: vec![], glyph_curves: vec![], glyph_bands: vec![],
            transforms: bytemuck::cast_slice(&identity).to_vec(), scissors: vec![], gradients: vec![], layer: identity, instance_offset: 0, instances: 1, blend: 0, opacity: None };
        let v = |x: f32, y: f32, u: f32, vv: f32| [x, y, u, vv, 1.0, 1.0, 1.0, 1.0];
        // a blit's (positions the target's) and a UI quad's over the frame (its pixels)
        let blit = vec![v(-1.0, -1.0, 0.0, 1.0), v(1.0, -1.0, 1.0, 1.0), v(-1.0, 1.0, 0.0, 0.0), v(1.0, -1.0, 1.0, 1.0), v(-1.0, 1.0, 0.0, 0.0), v(1.0, 1.0, 1.0, 0.0)];
        let (fw, fh) = (w as f32, h as f32);
        let region = vec![v(0.0, 0.0, 0.0, 0.0), v(fw, 0.0, 1.0, 0.0), v(0.0, fh, 0.0, 1.0), v(fw, 0.0, 1.0, 0.0), v(0.0, fh, 0.0, 1.0), v(fw, fh, 1.0, 1.0)];
        let items = [
            UiItem::Target(Some(hud_t)),
            UiItem::Clear([0.0; 4]),
            UiItem::Batch(paint),
            // what's past white, blurred across, into the glow
            UiItem::Target(Some(glow_t)),
            UiItem::Quad(Quad { shader: QuadShader::Glow, vertices: blit, transform: identity, layer: identity, glow: [1.0 / fw, 0.0, 6.0, 1.0], hud: [0.0; 4], scan: [0.0; 4], gamma_output: false, textures: [hud, 0, 0] }),
            // then over the frame: grabbed, and both read
            UiItem::Target(None),
            UiItem::Grab(7),
            UiItem::Quad(Quad { shader: QuadShader::Composite, vertices: region, transform: identity, layer: identity, glow: [0.0; 4], hud: [1.0 / fw, 1.0 / fh, 0.22, 1.0],
                scan: [0.0, 2.0, 0.55, 0.04], gamma_output: true, textures: [hud, glow, 7] }),
        ];
        r.ui.draw(&r, &frame.create_view(&Default::default()), Some(&frame), wgpu::TextureFormat::Rgba8Unorm, w, h, &items);
        let px = read_back(&r, &frame, w, h);
        let at = |x: u32, y: u32| px[((y * w + x) * 4) as usize];
        eprintln!("render target: inside {}, beside it {} then {}, far off {}", at(16, 16), at(25, 16), at(28, 16), at(56, 16));
        assert_eq!(at(16, 16), 255, "the box, painted into the target, over the frame");
        assert!((at(56, 16) as i32 - 128).abs() <= 2, "far from it, the frame as it was: {}", at(56, 16));
        assert!(at(25, 16) > at(28, 16) && at(28, 16) > 140, "beside it, its glow, fading: {} {}", at(25, 16), at(28, 16));
        assert!((at(16, 4) as i32 - 128).abs() <= 2, "above it the glow, blurred across only, adds nothing: {}", at(16, 4));
    }

    #[test]
    fn a_backdrop_blurs_and_filters_what_was_drawn_behind_it() {
        let Ok(r) = crate::render::Renderer::new() else { eprintln!("no adapter: skipped"); return };
        let (w, h) = (128u32, 64u32);
        let target = r.device.create_texture(&wgpu::TextureDescriptor {
            label: None, size: wgpu::Extent3d { width: w, height: h, depth_or_array_layers: 1 }, mip_level_count: 1, sample_count: 1, dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8Unorm, usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC | wgpu::TextureUsages::COPY_DST, view_formats: &[],
        });
        // what's behind: a checkerboard of 4-pixel black and white squares
        let checker: Vec<u8> = (0..w * h).flat_map(|i| { let (x, y) = (i % w, i / w); let c = if (x / 4 + y / 4) % 2 == 0 { 255 } else { 0 }; [c, c, c, 255] }).collect();
        r.queue.write_texture(target.as_image_copy(), &checker, wgpu::TexelCopyBufferLayout { offset: 0, bytes_per_row: Some(w * 4), rows_per_image: Some(h) }, wgpu::Extent3d { width: w, height: h, depth_or_array_layers: 1 });
        let identity = [1.0f32, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0];
        // a quad over a box, grown a pixel, as DrawQuad strips it
        let quad = |x: f32, y: f32, s: f32| -> Vec<[f32; 8]> {
            let v = |px: f32, py: f32, u: f32, vv: f32| [px, py, u, vv, 1.0, 1.0, 1.0, 1.0];
            let (a, b, c, d) = (v(x - 1.0, y - 1.0, 0.0, 0.0), v(x + s + 1.0, y - 1.0, 1.0, 0.0), v(x - 1.0, y + s + 1.0, 0.0, 1.0), v(x + s + 1.0, y + s + 1.0, 1.0, 1.0));
            vec![a, b, c, b, c, d]
        };
        let backdrop = |x: f32, filter: [f32; 7]| Backdrop {
            vertices: quad(x, 8.0, 48.0), transform: identity, layer: identity, rect: [x, 8.0, 48.0, 48.0], bloat: 1.0,
            radii: [12.0; 4], radii_v: [12.0; 4], filter, shape: (0, 0, [0.0; 16], [0.0; 4]), texture: 1, opacity: 1.0,
        };
        let items = [
            UiItem::Grab(1),
            // blurred 10, at 8..56
            UiItem::Backdrop(backdrop(8.0, [1.0, 1.0, 1.0, 0.0, 0.0, 0.0, 10.0])),
            // inverted, not blurred, at 72..120
            UiItem::Backdrop(backdrop(72.0, [1.0, 1.0, 1.0, 0.0, 1.0, 0.0, 0.0])),
        ];
        r.ui.draw(&r, &target.create_view(&Default::default()), Some(&target), wgpu::TextureFormat::Rgba8Unorm, w, h, &items);
        let row = (w * 4).div_ceil(256) * 256;
        let readback = r.device.create_buffer(&wgpu::BufferDescriptor { label: None, size: (row * h) as u64, usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ, mapped_at_creation: false });
        let mut encoder = r.device.create_command_encoder(&Default::default());
        encoder.copy_texture_to_buffer(target.as_image_copy(), wgpu::TexelCopyBufferInfo { buffer: &readback, layout: wgpu::TexelCopyBufferLayout { offset: 0, bytes_per_row: Some(row), rows_per_image: Some(h) } }, wgpu::Extent3d { width: w, height: h, depth_or_array_layers: 1 });
        r.queue.submit([encoder.finish()]);
        readback.slice(..).map_async(wgpu::MapMode::Read, |_| {});
        let _ = r.device.poll(wgpu::PollType::Wait);
        let data = readback.slice(..).get_mapped_range();
        let px = |x: u32, y: u32| data[(y * row + x * 4) as usize];
        // inside the blurred one the squares run together, near their mean
        let inside: Vec<u8> = (20..44).flat_map(|y| (20..44).map(move |x| (x, y))).map(|(x, y)| px(x, y)).collect();
        let (lo, hi) = (*inside.iter().min().unwrap(), *inside.iter().max().unwrap());
        eprintln!("backdrop: blurred, the middle's squares between {lo} and {hi}");
        assert!(lo > 60 && hi < 195, "blurred: {lo}..{hi}");
        // outside it, and in its rounded corner, the squares as they were
        assert_eq!((px(2, 2), px(6, 2)), (255, 0), "left of it");
        // (pixel centres 16 and 14 away from the corner's centre at 20,20: past its radius of 12 and the pixel of its edge)
        assert_eq!((px(8, 8), px(8, 12)), (255, 0), "its corner, rounded away");
        // the inverted one: black where white was, and the other way
        for (x, y) in [(90, 30), (94, 30), (90, 34), (100, 41)] {
            let was = checker[((y * w + x) * 4) as usize];
            assert_eq!(px(x, y), 255 - was, "inverted at {x},{y}");
        }
    }

    #[test]
    fn an_image_is_read_bilinear_as_the_ui_reads_its_images() {
        let Ok(r) = crate::render::Renderer::new() else { eprintln!("no adapter: skipped"); return };
        // 8x8: transparent but for an opaque white 4x4 square in its middle
        let mut px = vec![0u8; 8 * 8 * 4];
        for y in 2..6 { for x in 2..6 { px[(y * 8 + x) * 4..(y * 8 + x) * 4 + 4].copy_from_slice(&[255, 255, 255, 255]); } }
        let image = crate::gpu::create_texture("ui bilinear image", true, &{ let mut c = [0u8; 44]; c[0] = 8; c[2] = 8; c }, &px);
        let index = crate::gpu::texture_index(image);
        // as UICssBoxBatched asks for a no-repeat background: bilinear, border on both axes
        let sampler = crate::gpu::sampler_index(crate::gpu::sampler(&[0x14, 0, 0, 3, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]));
        // a 64 box, its image contained in it (64 square), no-repeat, centred
        let mut b = instance([0.0, 0.0, 64.0, 64.0], [0.0, 0.0, 0.0, 0.0]);
        set_i(&mut b, 144, index);
        set_i(&mut b, 148, sampler);
        set_f(&mut b, 156, &[0.0, 0.0, 64.0, 64.0, 1.0, 1.0, 1.0, 1.0]);
        set_i(&mut b, 228, 3 << 9);
        let px = drawn(&r, 64, 64, b);
        let a = |x: usize, y: usize| px[(y * 64 + x) * 4 + 3];
        assert_eq!(a(32, 32), 255, "the square's middle");
        assert_eq!(a(4, 4), 0, "the transparent corner");
        assert_eq!(a(60, 32), 0, "the transparent right");
        let showing = (0..64 * 64).filter(|i| px[i * 4 + 3] > 127).count();
        assert!((900..1200).contains(&showing), "about a quarter of the box shows: {showing}");
    }

    #[test]
    fn boxes_are_clipped_shadowed_outlined_and_filled_with_gradients_and_images() {
        let Ok(r) = crate::render::Renderer::new() else { eprintln!("no adapter: skipped"); return };
        const W: u32 = 96;
        // a 2x2 image managed made, its texels red, green, blue, white; read point-sampled
        let image = crate::gpu::create_texture("ui test image", true, &{ let mut c = [0u8; 44]; c[0] = 2; c[2] = 2; c }, &[255, 0, 0, 255, 0, 255, 0, 255, 0, 0, 255, 255, 255, 255, 255, 255]);
        let index = crate::gpu::texture_index(image);
        let point = crate::gpu::sampler(&[0, 0, 0, 2, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
        let point_index = crate::gpu::sampler_index(point);

        let mut boxes = Vec::new();
        // 0: a gradient down the box, red to blue (GradientBuffer 0)
        let mut b = instance([0.0, 0.0, 32.0, 32.0], [0.0, 0.0, 0.0, 0.0]);
        set_i(&mut b, 144, -1);
        set_f(&mut b, 156, &[0.0, 0.0, 32.0, 32.0, 1.0, 1.0, 1.0, 1.0]); // background rect, tint
        boxes.extend(b);
        // 1: white, clipped to its top-left quarter (ScissorBuffer 0)
        let mut b = instance([32.0, 0.0, 32.0, 32.0], [1.0, 1.0, 1.0, 1.0]);
        set_i(&mut b, 232, 0);
        boxes.extend(b);
        // 2: an outset shadow, white, of the 16 square in the middle of its quad, blurred 8
        let mut b = instance([0.0, 32.0, 32.0, 32.0], [1.0, 1.0, 1.0, 1.0]);
        set_f(&mut b, 152, &[8.0, 8.0, 8.0, 16.0, 16.0]);
        set_i(&mut b, 228, 1);
        boxes.extend(b);
        // 3: the image over the whole box
        let mut b = instance([32.0, 32.0, 32.0, 32.0], [0.0, 0.0, 0.0, 0.0]);
        set_i(&mut b, 144, index);
        set_i(&mut b, 148, point_index);
        set_f(&mut b, 156, &[0.0, 0.0, 32.0, 32.0, 1.0, 1.0, 1.0, 1.0]);
        set_i(&mut b, 228, 3 << 9); // no-repeat
        boxes.extend(b);
        // 4: an outline 2 wide, 2 out from a 16 square at (72, 8): its quad grown by 5
        let mut b = instance([67.0, 3.0, 26.0, 26.0], [0.0, 1.0, 0.0, 1.0]);
        set_f(&mut b, 152, &[5.0, 16.0, 16.0, 2.0, 2.0]);
        set_i(&mut b, 228, 3);
        boxes.extend(b);

        let mut gradient = vec![0u8; GRADIENT];
        set_f(&mut gradient, 0, &[1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 1.0]); // stop colours
        set_f(&mut gradient, 128, &[0.0, 1.0]); // their offsets
        set_i(&mut gradient, 160, 2); // two stops, angle 0 (down the box), linear
        let mut scissor = vec![0u8; SCISSOR];
        set_i(&mut scissor, 0, 1); // one clip
        set_i(&mut scissor, 8, -1); // nothing after it
        set_f(&mut scissor, 16, &[32.0, 0.0, 48.0, 16.0]); // left, top, right, bottom
        set_f(&mut scissor, 64, &[1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0]);
        let identity = [1.0f32, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0];
        let draw = UiDraw { boxes, texts: vec![], glyph_table: vec![], glyph_curves: vec![], glyph_bands: vec![], transforms: bytemuck::cast_slice(&identity).to_vec(),
            scissors: scissor, gradients: gradient, layer: identity, instance_offset: 0, instances: 5, blend: 0, opacity: None };

        let target = r.device.create_texture(&wgpu::TextureDescriptor {
            label: None, size: wgpu::Extent3d { width: W, height: 64, depth_or_array_layers: 1 }, mip_level_count: 1, sample_count: 1, dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8Unorm, usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC, view_formats: &[],
        });
        let view = target.create_view(&Default::default());
        r.ui.draw(&r, &view, Some(&target), wgpu::TextureFormat::Rgba8Unorm, W, 64, &[UiItem::Batch(draw)]);
        let row = (W * 4).div_ceil(256) * 256;
        let readback = r.device.create_buffer(&wgpu::BufferDescriptor { label: None, size: (row * 64) as u64, usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ, mapped_at_creation: false });
        let mut encoder = r.device.create_command_encoder(&Default::default());
        encoder.copy_texture_to_buffer(
            wgpu::TexelCopyTextureInfo { texture: &target, mip_level: 0, origin: wgpu::Origin3d::ZERO, aspect: wgpu::TextureAspect::All },
            wgpu::TexelCopyBufferInfo { buffer: &readback, layout: wgpu::TexelCopyBufferLayout { offset: 0, bytes_per_row: Some(row), rows_per_image: Some(64) } },
            wgpu::Extent3d { width: W, height: 64, depth_or_array_layers: 1 },
        );
        r.queue.submit([encoder.finish()]);
        readback.slice(..).map_async(wgpu::MapMode::Read, |_| {});
        let _ = r.device.poll(wgpu::PollType::Wait);
        let data = readback.slice(..).get_mapped_range();
        let px = |x: u32, y: u32| { let at = (y * row + x * 4) as usize; [data[at], data[at + 1], data[at + 2], data[at + 3]] };

        // the gradient: red at its top, blue at its bottom, half and half between
        let (top, mid, bottom) = (px(16, 1), px(16, 16), px(16, 30));
        assert!(top[0] > 230 && top[2] < 25, "top {top:?}");
        assert!(bottom[2] > 230 && bottom[0] < 25, "bottom {bottom:?}");
        assert!((110..146).contains(&mid[0]) && (110..146).contains(&mid[2]), "middle {mid:?}");
        // the clip: white inside its quarter, nothing past it
        assert_eq!(px(40, 8), [255, 255, 255, 255]);
        assert_eq!(px(56, 8)[3], 0, "right of the clip");
        assert_eq!(px(40, 24)[3], 0, "below the clip");
        // the shadow: brightest in the middle, fading past its shape's edge, nothing far out
        let (centre, edge, out) = (px(16, 48)[3], px(8, 48)[3], px(1, 33)[3]);
        assert!(centre > 150 && centre < 255, "centre {centre}");
        assert!(edge > 60 && edge < centre, "edge {edge}");
        assert!(out < edge / 4, "far out {out}");
        // the image, point-sampled: red, green over blue, white
        assert_eq!(px(40, 40), [255, 0, 0, 255]);
        assert_eq!(px(56, 40), [0, 255, 0, 255]);
        assert_eq!(px(40, 56), [0, 0, 255, 255]);
        assert_eq!(px(56, 56), [255, 255, 255, 255]);
        // the outline: green in its band (2 to 4 out from the square), nothing inside or past it
        assert_eq!(px(80, 5), [0, 255, 0, 255], "in the band, above");
        assert_eq!(px(80, 16)[3], 0, "inside the square");
        assert_eq!(px(80, 7)[3], 0, "between the square and the band");
        assert_eq!(px(80, 1)[3], 0, "past the band");
    }

    #[test]
    fn borders_take_their_sides_colours_and_styles() {
        let Ok(r) = crate::render::Renderer::new() else { eprintln!("no adapter: skipped"); return };
        let identity = [1.0f32, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0];
        let mut boxes = Vec::new();
        // 0: solid, 4 wide: red left, green top, blue right, white bottom, over a black box
        let mut b = instance([0.0, 0.0, 40.0, 40.0], [0.0, 0.0, 0.0, 1.0]);
        set_f(&mut b, 64, &[4.0, 4.0, 4.0, 4.0, 1.0, 0.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0]);
        boxes.extend(b);
        // 1: dashed, 2 wide, yellow, nothing inside
        let mut b = instance([48.0, 0.0, 96.0, 40.0], [0.0, 0.0, 0.0, 0.0]);
        set_f(&mut b, 64, &[2.0, 2.0, 2.0, 2.0]);
        for side in 0..4 { set_f(&mut b, 80 + side * 16, &[1.0, 1.0, 0.0, 1.0]); }
        set_i(&mut b, 228, 4 << 4);
        boxes.extend(b);
        // 2: double, 6 wide, white: two lines of 2 with 2 between
        let mut b = instance([0.0, 48.0, 40.0, 40.0], [0.0, 0.0, 0.0, 0.0]);
        set_f(&mut b, 64, &[6.0, 6.0, 6.0, 6.0]);
        for side in 0..4 { set_f(&mut b, 80 + side * 16, &[1.0, 1.0, 1.0, 1.0]); }
        set_i(&mut b, 228, 5 << 4);
        boxes.extend(b);
        let draw = UiDraw { boxes, texts: vec![], glyph_table: vec![], glyph_curves: vec![], glyph_bands: vec![], transforms: bytemuck::cast_slice(&identity).to_vec(),
            scissors: vec![], gradients: vec![], layer: identity, instance_offset: 0, instances: 3, blend: 0, opacity: None };
        let (w, h) = (160u32, 96u32);
        let target = r.device.create_texture(&wgpu::TextureDescriptor {
            label: None, size: wgpu::Extent3d { width: w, height: h, depth_or_array_layers: 1 }, mip_level_count: 1, sample_count: 1, dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8Unorm, usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC, view_formats: &[],
        });
        r.ui.draw(&r, &target.create_view(&Default::default()), Some(&target), wgpu::TextureFormat::Rgba8Unorm, w, h, &[UiItem::Batch(draw)]);
        let row = (w * 4).div_ceil(256) * 256;
        let readback = r.device.create_buffer(&wgpu::BufferDescriptor { label: None, size: (row * h) as u64, usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ, mapped_at_creation: false });
        let mut encoder = r.device.create_command_encoder(&Default::default());
        encoder.copy_texture_to_buffer(
            wgpu::TexelCopyTextureInfo { texture: &target, mip_level: 0, origin: wgpu::Origin3d::ZERO, aspect: wgpu::TextureAspect::All },
            wgpu::TexelCopyBufferInfo { buffer: &readback, layout: wgpu::TexelCopyBufferLayout { offset: 0, bytes_per_row: Some(row), rows_per_image: Some(h) } },
            wgpu::Extent3d { width: w, height: h, depth_or_array_layers: 1 },
        );
        r.queue.submit([encoder.finish()]);
        readback.slice(..).map_async(wgpu::MapMode::Read, |_| {});
        let _ = r.device.poll(wgpu::PollType::Wait);
        let data = readback.slice(..).get_mapped_range();
        let px = |x: u32, y: u32| { let at = (y * row + x * 4) as usize; [data[at], data[at + 1], data[at + 2], data[at + 3]] };

        // each side its own colour, the box's own inside
        assert_eq!(px(1, 20), [255, 0, 0, 255], "left");
        assert_eq!(px(20, 1), [0, 255, 0, 255], "top");
        assert_eq!(px(38, 20), [0, 0, 255, 255], "right");
        assert_eq!(px(20, 38), [255, 255, 255, 255], "bottom");
        assert_eq!(px(20, 20), [0, 0, 0, 255], "inside");
        // the corner split along its diagonal, antialiased on the join
        assert_eq!(px(1, 3)[0], 255, "left of the top-left join");
        assert_eq!(px(3, 1)[1], 255, "above the top-left join");
        // dashed: along the top edge, runs of yellow and runs of nothing, about half each
        let along: Vec<u8> = (52..140).map(|x| px(x, 1)[3]).collect();
        let on = along.iter().filter(|&&a| a > 200).count();
        let off = along.iter().filter(|&&a| a < 50).count();
        let runs = along.windows(2).filter(|p| (p[0] > 127) != (p[1] > 127)).count();
        assert!(on > 25 && off > 25, "dashes {on} on, {off} off: {along:?}");
        assert!(runs >= 10, "{runs} edges between dashes");
        assert_eq!(px(96, 20)[3], 0, "nothing inside");
        // double: line, gap, line across its six pixels
        let across: Vec<u8> = (0..6).map(|x| px(x, 68)[3]).collect();
        assert!(across[0] > 200 && across[1] > 200, "outer line {across:?}");
        assert!(across[2] < 60 && across[3] < 60, "gap {across:?}");
        assert!(across[4] > 200 && across[5] > 200, "inner line {across:?}");
    }

    fn boxed(textures: &[i32]) -> Vec<u8> {
        let mut b = vec![0u8; textures.len() * BOX];
        for (i, &t) in textures.iter().enumerate() { b[i * BOX + 144..i * BOX + 148].copy_from_slice(&t.to_le_bytes()); }
        b
    }

    #[test]
    fn a_draw_splits_where_its_images_outgrow_the_bindings() {
        // ten distinct images, then one already bound: two runs, in order
        let textures = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 9, -1, 0];
        let d = UiDraw { boxes: boxed(&textures), texts: vec![], glyph_table: vec![], glyph_curves: vec![], glyph_bands: vec![], transforms: vec![], scissors: vec![], gradients: vec![],
            layer: [0.0; 16], instance_offset: 0, instances: textures.len() as u32, blend: 0, opacity: None };
        let r = runs(&d);
        assert_eq!(r.len(), 2);
        assert_eq!((r[0].0, r[0].1, r[0].2.clone()), (0, 8, vec![1, 2, 3, 4, 5, 6, 7, 8]));
        assert_eq!((r[1].0, r[1].1, r[1].2.clone()), (8, 5, vec![9, 10]), "a gradient (-1) and no image (0) bind nothing");
        let total: u32 = r.iter().map(|x| x.1).sum();
        assert_eq!(total, textures.len() as u32);
    }
}
