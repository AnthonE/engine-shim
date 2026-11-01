//! What a client draws, begun: models as s&box compiles them (their
//! meshes, the materials they name, those materials' textures), read by
//! assets/ and drawn with wgpu (Vulkan here, WebGPU in a browser one day).
//! Each material is drawn as s&box's own shader for it would draw it, as
//! near as we get yet (shader.wgsl: complex and blendable, which nearly
//! every material s&box's content has is one of). The engine draws its
//! scene with it when managed asks a camera for pixels (impls.rs,
//! CCameraRenderer_RenderToBitmap); `render` draws one model.
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use crate::assets::{material, mesh, resource::Resource, texture};
use glam::{Mat4, Vec3};
use wgpu::util::DeviceExt;

/// Where a renderer reads content from, by game path.
pub trait Files {
    fn read(&self, path: &str) -> Option<Vec<u8>>;

    /// A compiled resource by its name (`x.vmat` is `x.vmat_c`).
    fn compiled(&self, name: &str) -> Option<Resource> {
        let name = if name.ends_with("_c") { name.to_string() } else { format!("{name}_c") };
        Resource::parse(self.read(&name)?).ok()
    }
}

/// Folders that hold files at their game paths (a package's, as
/// tools/package.sh fetches them), and paks (a map's).
pub struct Content(pub Vec<PathBuf>, pub Vec<crate::assets::vpk::Vpk>);

impl Content {
    /// From a list of places: a `.vpk` is a pak, anything else a folder.
    pub fn of(places: impl IntoIterator<Item = PathBuf>) -> Content {
        let (paks, dirs): (Vec<PathBuf>, Vec<PathBuf>) = places.into_iter().partition(|p| p.extension().is_some_and(|e| e == "vpk"));
        Content(dirs, paks.iter().filter_map(|p| crate::assets::vpk::Vpk::open(p).ok()).collect())
    }
}

impl Files for Content {
    fn read(&self, path: &str) -> Option<Vec<u8>> {
        let path = path.replace('\\', "/");
        let path = path.trim_start_matches('/');
        self.0.iter().find_map(|d| std::fs::read(d.join(path)).ok()).or_else(|| self.1.iter().find_map(|p| p.read(path)))
    }
}

/// The engine's own file system (fs.rs): what managed linked and mounted.
pub struct EngineFiles;

impl Files for EngineFiles {
    fn read(&self, path: &str) -> Option<Vec<u8>> { crate::fs::read(path) }
}

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
struct Vertex {
    position: [f32; 3],
    normal: [f32; 3],
    uv: [f32; 2],
    /// vertex paint: how much of a blendable's layer B (red), and a tint
    blend: [f32; 4],
    paint: [f32; 4],
    /// skinned: its bones in the model's skeleton, and how much each moves it
    bones: [u32; 4],
    weights: [f32; 4],
    /// where it is in the map's lightmaps (TEXCOORD3, before the world's scale)
    lightmap: [f32; 2],
}

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
struct ViewUniform {
    view_proj: [[f32; 4]; 4],
    inv_view_proj: [[f32; 4]; 4],
    eye: [f32; 4],
    sun: [f32; 4],
    sun_color: [f32; 4],
    sky: [f32; 4],
    ground: [f32; 4],
    sky_rotation: [[f32; 4]; 4],
}

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
struct ObjectUniform {
    model: [[f32; 4]; 4],
    tint: [f32; 4],
    /// x: skinned by the bone palette (1), y: lit from the view's lightmaps (1)
    flags: [f32; 4],
}

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
struct MaterialUniform {
    tint: [f32; 4],
    params: [f32; 4],
    mode: [f32; 4],
    tint_b: [f32; 4],
    tint_c: [f32; 4],
    blend: [f32; 4],
    scale: [f32; 4],
}

/// A run of triangles to draw: its buffers (by index), its range, its material.
struct GpuDraw {
    vertex_buffer: usize,
    index_buffer: usize,
    range: std::ops::Range<u32>,
    base_vertex: i32,
    material: wgpu::BindGroup,
}

/// A mesh's draws on the GPU, and the buffers they read.
struct GpuMesh {
    vertices: Vec<wgpu::Buffer>,
    indices: Vec<wgpu::Buffer>,
    draws: Vec<GpuDraw>,
}

/// A model read and on the GPU, with what it covers.
pub struct Model {
    meshes: Vec<GpuMesh>,
    pub min: Vec3,
    pub max: Vec3,
    /// whether every vertex has a place in a map's lightmaps (a map's own
    /// geometry, compiled with baked lighting)
    pub lightmap_uv: bool,
    /// what was missing or couldn't be read, for whoever's looking
    pub notes: Vec<String>,
}

pub struct Renderer {
    device: wgpu::Device,
    queue: wgpu::Queue,
    pipeline: wgpu::RenderPipeline,
    view_layout: wgpu::BindGroupLayout,
    material_layout: wgpu::BindGroupLayout,
    object_layout: wgpu::BindGroupLayout,
    sky_pipeline: wgpu::RenderPipeline,
    sky_layout: wgpu::BindGroupLayout,
    skies: Mutex<HashMap<String, Arc<Sky>>>,
    lighting_layout: wgpu::BindGroupLayout,
    /// the lighting group of a view with no lightmaps
    unlit: Lightmaps,
    clamp: wgpu::Sampler,
    sampler: wgpu::Sampler,
    /// whether the GPU takes BC1 to BC7 as they are (most desktop GPUs,
    /// lavapipe, WebGPU on desktops), and what's been read already
    bc: bool,
    textures: Mutex<HashMap<(String, bool), wgpu::TextureView>>,
    materials: Mutex<HashMap<String, (wgpu::BindGroup, bool, Vec<String>)>>,
    /// for a material without a texture: white (a colour, occlusion, a
    /// tint mask), a flat normal (roughness 0.6), and a blendable's g_tRma
    /// of either version (not metal, not occluded, all revealed: version 1
    /// reads metalness, occlusion, mask; version 0 roughness 0.6,
    /// metalness, occlusion, mask)
    white: wgpu::TextureView,
    flat: wgpu::TextureView,
    rma: [wgpu::TextureView; 2],
    pub adapter: String,
}

const COLOR: wgpu::TextureFormat = wgpu::TextureFormat::Rgba8UnormSrgb;
const DEPTH: wgpu::TextureFormat = wgpu::TextureFormat::Depth32Float;
const SAMPLES: u32 = 4;

impl Renderer {
    pub fn new() -> Result<Renderer, String> {
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor { backends: wgpu::Backends::VULKAN | wgpu::Backends::GL | wgpu::Backends::METAL | wgpu::Backends::DX12, ..Default::default() });
        let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions::default())).map_err(|e| format!("no adapter: {e}"))?;
        let info = adapter.get_info();
        let bc = adapter.features().contains(wgpu::Features::TEXTURE_COMPRESSION_BC);
        let (device, queue) = pollster::block_on(adapter.request_device(&wgpu::DeviceDescriptor {
            required_features: if bc { wgpu::Features::TEXTURE_COMPRESSION_BC } else { wgpu::Features::empty() },
            required_limits: adapter.limits(),
            ..Default::default()
        })).map_err(|e| e.to_string())?;
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor { label: Some("complex"), source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()) });
        let uniform = |binding, visibility| wgpu::BindGroupLayoutEntry { binding, visibility, ty: wgpu::BindingType::Buffer { ty: wgpu::BufferBindingType::Uniform, has_dynamic_offset: false, min_binding_size: None }, count: None };
        let tex = |binding| wgpu::BindGroupLayoutEntry { binding, visibility: wgpu::ShaderStages::FRAGMENT, ty: wgpu::BindingType::Texture { sample_type: wgpu::TextureSampleType::Float { filterable: true }, view_dimension: wgpu::TextureViewDimension::D2, multisampled: false }, count: None };
        let view_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor { label: Some("view"), entries: &[uniform(0, wgpu::ShaderStages::VERTEX_FRAGMENT)] });
        let material_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor { label: Some("material"), entries: &[
            uniform(0, wgpu::ShaderStages::FRAGMENT), tex(1), tex(2),
            wgpu::BindGroupLayoutEntry { binding: 3, visibility: wgpu::ShaderStages::FRAGMENT, ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering), count: None },
            tex(4), tex(5), tex(6), tex(7), tex(8), tex(9), tex(10), tex(11),
        ] });
        let object_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor { label: Some("object"), entries: &[
            uniform(0, wgpu::ShaderStages::VERTEX_FRAGMENT),
            wgpu::BindGroupLayoutEntry { binding: 1, visibility: wgpu::ShaderStages::VERTEX, ty: wgpu::BindingType::Buffer { ty: wgpu::BufferBindingType::Storage { read_only: true }, has_dynamic_offset: false, min_binding_size: None }, count: None },
        ] });
        // a map's baked lighting (as s&box's vr_lighting.fxc reads it): its
        // lightmaps (direct lights' indices and strengths, irradiance, and
        // the directional part of it) and the lights the indices name
        let lighting_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor { label: Some("lighting"), entries: &[
            uniform(0, wgpu::ShaderStages::VERTEX_FRAGMENT), tex(1), tex(2),
            wgpu::BindGroupLayoutEntry { binding: 3, visibility: wgpu::ShaderStages::FRAGMENT, ty: wgpu::BindingType::Texture { sample_type: wgpu::TextureSampleType::Float { filterable: true }, view_dimension: wgpu::TextureViewDimension::D2, multisampled: false }, count: None },
            tex(4),
            wgpu::BindGroupLayoutEntry { binding: 5, visibility: wgpu::ShaderStages::FRAGMENT, ty: wgpu::BindingType::Buffer { ty: wgpu::BufferBindingType::Storage { read_only: true }, has_dynamic_offset: false, min_binding_size: None }, count: None },
            wgpu::BindGroupLayoutEntry { binding: 6, visibility: wgpu::ShaderStages::FRAGMENT, ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering), count: None },
        ] });
        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor { label: None, bind_group_layouts: &[&view_layout, &material_layout, &object_layout, &lighting_layout], push_constant_ranges: &[] });
        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("complex"),
            layout: Some(&layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                compilation_options: Default::default(),
                buffers: &[wgpu::VertexBufferLayout {
                    array_stride: std::mem::size_of::<Vertex>() as u64,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3, 2 => Float32x2, 3 => Float32x4, 4 => Float32x4, 5 => Uint32x4, 6 => Float32x4, 7 => Float32x2],
                }],
            },
            fragment: Some(wgpu::FragmentState { module: &shader, entry_point: Some("fs_main"), compilation_options: Default::default(), targets: &[Some(COLOR.into())] }),
            primitive: wgpu::PrimitiveState { cull_mode: None, ..Default::default() },
            depth_stencil: Some(wgpu::DepthStencilState { format: DEPTH, depth_write_enabled: true, depth_compare: wgpu::CompareFunction::Less, stencil: Default::default(), bias: Default::default() }),
            multisample: wgpu::MultisampleState { count: SAMPLES, ..Default::default() },
            multiview: None,
            cache: None,
        });
        // the sky: a triangle over the screen, its cube looked up per pixel, behind everything
        let sky_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor { label: Some("sky"), source: wgpu::ShaderSource::Wgsl(include_str!("sky.wgsl").into()) });
        let sky_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor { label: Some("sky"), entries: &[
            uniform(0, wgpu::ShaderStages::FRAGMENT),
            wgpu::BindGroupLayoutEntry { binding: 1, visibility: wgpu::ShaderStages::FRAGMENT, ty: wgpu::BindingType::Texture { sample_type: wgpu::TextureSampleType::Float { filterable: true }, view_dimension: wgpu::TextureViewDimension::Cube, multisampled: false }, count: None },
            wgpu::BindGroupLayoutEntry { binding: 2, visibility: wgpu::ShaderStages::FRAGMENT, ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering), count: None },
        ] });
        let sky_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("sky"),
            layout: Some(&device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor { label: None, bind_group_layouts: &[&view_layout, &sky_layout], push_constant_ranges: &[] })),
            vertex: wgpu::VertexState { module: &sky_shader, entry_point: Some("vs_main"), compilation_options: Default::default(), buffers: &[] },
            fragment: Some(wgpu::FragmentState { module: &sky_shader, entry_point: Some("fs_main"), compilation_options: Default::default(), targets: &[Some(COLOR.into())] }),
            primitive: Default::default(),
            depth_stencil: Some(wgpu::DepthStencilState { format: DEPTH, depth_write_enabled: false, depth_compare: wgpu::CompareFunction::Always, stencil: Default::default(), bias: Default::default() }),
            multisample: wgpu::MultisampleState { count: SAMPLES, ..Default::default() },
            multiview: None,
            cache: None,
        });
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::Repeat, address_mode_v: wgpu::AddressMode::Repeat,
            mag_filter: wgpu::FilterMode::Linear, min_filter: wgpu::FilterMode::Linear, mipmap_filter: wgpu::FilterMode::Linear,
            anisotropy_clamp: 8, ..Default::default()
        });
        let pixel = |rgba: [u8; 4]| device.create_texture_with_data(&queue, &wgpu::TextureDescriptor {
            label: None, size: wgpu::Extent3d { width: 1, height: 1, depth_or_array_layers: 1 }, mip_level_count: 1, sample_count: 1,
            dimension: wgpu::TextureDimension::D2, format: wgpu::TextureFormat::Rgba8Unorm, usage: wgpu::TextureUsages::TEXTURE_BINDING, view_formats: &[],
        }, Default::default(), &rgba).create_view(&Default::default());
        let (white, flat, rma) = (pixel([255; 4]), pixel([128, 128, 153, 255]), [pixel([153, 0, 255, 0]), pixel([0, 255, 0, 255])]);
        let clamp = device.create_sampler(&wgpu::SamplerDescriptor {
            mag_filter: wgpu::FilterMode::Linear, min_filter: wgpu::FilterMode::Linear, mipmap_filter: wgpu::FilterMode::Linear, ..Default::default()
        });
        let unlit = Lightmaps::group(&device, &lighting_layout, &clamp, [0.0; 4], [&white, &white, &white, &white], &[BakedLight::default()]);
        Ok(Renderer { bc, textures: Default::default(), materials: Default::default(), skies: Default::default(), sky_pipeline, sky_layout, lighting_layout, unlit, clamp, device, queue, pipeline, view_layout, material_layout, object_layout, sampler, white, flat, rma, adapter: format!("{} ({:?})", info.name, info.backend) })
    }

    /// A texture by name, every mip the file holds, on the GPU.
    fn texture(&self, content: &dyn Files, name: &str, srgb: bool) -> Result<wgpu::TextureView, String> {
        let key = (crate::fs::normal(name), srgb);
        if let Some(v) = self.textures.lock().unwrap().get(&key) { return Ok(v.clone()); }
        let t = texture::read(content.compiled(name).ok_or_else(|| format!("{name}: not found"))?)?;
        let top = t.available(0).ok_or_else(|| format!("{name}: no mips"))?;
        let (w, h) = t.size(top);
        use texture::format as f;
        // its blocks as they are, where the GPU takes them (and its size is whole blocks)
        let blocks = match (t.format, srgb) {
            _ if !self.bc || w % 4 != 0 || h % 4 != 0 => None,
            (f::DXT1, true) => Some((wgpu::TextureFormat::Bc1RgbaUnormSrgb, 8)),
            (f::DXT1, false) => Some((wgpu::TextureFormat::Bc1RgbaUnorm, 8)),
            (f::DXT5, true) => Some((wgpu::TextureFormat::Bc3RgbaUnormSrgb, 16)),
            (f::DXT5, false) => Some((wgpu::TextureFormat::Bc3RgbaUnorm, 16)),
            (f::BC7, true) => Some((wgpu::TextureFormat::Bc7RgbaUnormSrgb, 16)),
            (f::BC7, false) => Some((wgpu::TextureFormat::Bc7RgbaUnorm, 16)),
            (f::ATI1N, _) => Some((wgpu::TextureFormat::Bc4RUnorm, 8)),
            (f::ATI2N, _) => Some((wgpu::TextureFormat::Bc5RgUnorm, 16)),
            (f::BC6H, _) => Some((wgpu::TextureFormat::Bc6hRgbUfloat, 16)),
            _ => None,
        };
        let (format, levels): (wgpu::TextureFormat, Vec<(usize, usize, Vec<u8>, u32, u32)>) = match blocks {
            Some((format, bytes)) => (format, (top..t.mips).map(|m| {
                let (lw, lh) = t.size(m);
                let (bx, by) = (lw.div_ceil(4), lh.div_ceil(4));
                t.raw(m).map(|d| (bx * 4, by * 4, d[..(bx * by * bytes).min(d.len())].to_vec(), (bx * bytes) as u32, by as u32))
            }).collect::<Result<_, _>>()?),
            None => (if srgb { wgpu::TextureFormat::Rgba8UnormSrgb } else { wgpu::TextureFormat::Rgba8Unorm },
                (top..t.mips).map(|m| t.rgba(m).map(|(lw, lh, px)| (lw, lh, px, lw as u32 * 4, lh as u32))).collect::<Result<_, _>>()?),
        };
        let gpu = self.device.create_texture(&wgpu::TextureDescriptor {
            label: Some(name), size: wgpu::Extent3d { width: w as u32, height: h as u32, depth_or_array_layers: 1 }, mip_level_count: levels.len() as u32, sample_count: 1,
            dimension: wgpu::TextureDimension::D2, format,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST, view_formats: &[],
        });
        for (i, (lw, lh, data, row, rows)) in levels.iter().enumerate() {
            self.queue.write_texture(
                wgpu::TexelCopyTextureInfo { texture: &gpu, mip_level: i as u32, origin: wgpu::Origin3d::ZERO, aspect: wgpu::TextureAspect::All },
                data,
                wgpu::TexelCopyBufferLayout { offset: 0, bytes_per_row: Some(*row), rows_per_image: Some(*rows) },
                wgpu::Extent3d { width: *lw as u32, height: *lh as u32, depth_or_array_layers: 1 },
            );
        }
        let view = gpu.create_view(&Default::default());
        self.textures.lock().unwrap().insert(key, view.clone());
        Ok(view)
    }

    /// A sky material (`sky.shader`): its cube on the GPU, read once.
    pub fn sky(&self, content: &dyn Files, name: &str) -> Result<Arc<Sky>, String> {
        let key = crate::fs::normal(name);
        if let Some(s) = self.skies.lock().unwrap().get(&key) { return Ok(s.clone()); }
        let m = material::read(&content.compiled(name).ok_or_else(|| format!("{name}: not found"))?)?;
        let tex = m.textures.get("g_tSkyTexture").ok_or_else(|| format!("{name}: no g_tSkyTexture"))?;
        let t = texture::read(content.compiled(tex).ok_or_else(|| format!("{tex}: not found"))?)?;
        let (format, bytes) = match t.format {
            texture::format::BC6H => (wgpu::TextureFormat::Bc6hRgbUfloat, 16),
            texture::format::BC7 => (wgpu::TextureFormat::Bc7RgbaUnormSrgb, 16),
            texture::format::DXT1 => (wgpu::TextureFormat::Bc1RgbaUnormSrgb, 8),
            f => return Err(format!("{tex}: a sky of format {f} not drawn yet")),
        };
        if !self.bc || t.faces() != 6 { return Err(format!("{tex}: not a cube this GPU takes")); }
        let top = t.available(0).ok_or_else(|| format!("{tex}: no mips"))?;
        let (w, h) = t.size(top);
        let gpu = self.device.create_texture(&wgpu::TextureDescriptor {
            label: Some(tex), size: wgpu::Extent3d { width: w as u32, height: h as u32, depth_or_array_layers: 6 }, mip_level_count: (t.mips - top) as u32, sample_count: 1,
            dimension: wgpu::TextureDimension::D2, format, usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST, view_formats: &[],
        });
        for (i, m) in (top..t.mips).enumerate() {
            let (lw, lh) = t.size(m);
            let (bx, by) = (lw.div_ceil(4), lh.div_ceil(4));
            let face = bx * by * bytes;
            let data = t.raw(m)?;
            if data.len() < face * 6 { return Err(format!("{tex}: mip {m} short")); }
            self.queue.write_texture(
                wgpu::TexelCopyTextureInfo { texture: &gpu, mip_level: i as u32, origin: wgpu::Origin3d::ZERO, aspect: wgpu::TextureAspect::All },
                &data[..face * 6],
                wgpu::TexelCopyBufferLayout { offset: 0, bytes_per_row: Some((bx * bytes) as u32), rows_per_image: Some(by as u32) },
                wgpu::Extent3d { width: (bx * 4) as u32, height: (by * 4) as u32, depth_or_array_layers: 6 },
            );
        }
        let view = gpu.create_view(&wgpu::TextureViewDescriptor { dimension: Some(wgpu::TextureViewDimension::Cube), ..Default::default() });
        let brightness = 2f32.powf(m.float("g_flBrightnessExposureBias", 0.0));
        let buffer = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor { label: Some("sky"), contents: bytemuck::cast_slice(&[brightness, 0.0, 0.0, 0.0]), usage: wgpu::BufferUsages::UNIFORM });
        let group = self.device.create_bind_group(&wgpu::BindGroupDescriptor { label: Some(name), layout: &self.sky_layout, entries: &[
            wgpu::BindGroupEntry { binding: 0, resource: buffer.as_entire_binding() },
            at(1, &view),
            wgpu::BindGroupEntry { binding: 2, resource: wgpu::BindingResource::Sampler(&self.sampler) },
        ] });
        let sky = Arc::new(Sky { group });
        self.skies.lock().unwrap().insert(key, sky.clone());
        Ok(sky)
    }

    /// A map's baked lighting (`maps/<name>`: its world's lighting info
    /// and the lightmaps it names), lit by `lights`, by their bake indices.
    /// None if the map was compiled without it.
    pub fn lightmaps(&self, content: &dyn Files, folder: &str, lights: &[BakedLight]) -> Result<Option<Lightmaps>, String> {
        let world = content.compiled(&format!("{folder}/world.vwrld")).ok_or_else(|| format!("{folder}: no world"))?;
        let d = world.kv3("DATA").ok_or("vwrld: no DATA")?.map_err(|e| e.to_string())?;
        let Some(info) = d.get("m_worldLightingInfo") else { return Ok(None) };
        let maps: Vec<&str> = info.get("m_lightMaps").map(crate::assets::kv3::Value::as_array).unwrap_or(&[]).iter().filter_map(|v| v.as_str()).collect();
        if info.get("m_bHasLightmaps").and_then(|v| v.as_i64()) == Some(0) || maps.len() < 4 { return Ok(None); }
        let version = info.i64("m_nLightmapGameVersionNumber");
        if version != 1 { return Err(format!("{folder}: lightmaps of game version {version} not read yet")); }
        let scale = info.get("m_vLightmapUvScale").map(|v| v.floats()).unwrap_or_default();
        // in the order the world lists them: indices, strengths, irradiance, directional
        let views = [0, 1, 2, 3].map(|i| self.texture(content, maps[i], false));
        let [Ok(a), Ok(b), Ok(c), Ok(e)] = views else {
            return Err(views.into_iter().filter_map(Result::err).collect::<Vec<_>>().join("; "));
        };
        let size = content.compiled(maps[0]).and_then(|r| texture::read(r).ok()).map(|t| (t.width as f32, t.height as f32)).unwrap_or((1.0, 1.0));
        let uniform = [scale.first().copied().unwrap_or(1.0), scale.get(1).copied().unwrap_or(1.0), size.0, size.1];
        Ok(Some(Lightmaps::group(&self.device, &self.lighting_layout, &self.clamp, uniform, [&a, &b, &c, &e], lights)))
    }

    /// A material's bind group, and whether it's drawn at all.
    fn material(&self, content: &dyn Files, name: &str, notes: &mut Vec<String>) -> (wgpu::BindGroup, bool) {
        let key = crate::fs::normal(name);
        if let Some((g, drawn, n)) = self.materials.lock().unwrap().get(&key) { notes.extend(n.iter().cloned()); return (g.clone(), *drawn); }
        let mut own = Vec::new();
        let (g, drawn) = self.read_material(content, name, &mut own);
        notes.extend(own.iter().cloned());
        self.materials.lock().unwrap().insert(key, (g.clone(), drawn, own));
        (g, drawn)
    }

    fn read_material(&self, content: &dyn Files, name: &str, notes: &mut Vec<String>) -> (wgpu::BindGroup, bool) {
        let m = content.compiled(name).ok_or_else(|| format!("{name}: not found")).and_then(|r| material::read(&r));
        let m = m.unwrap_or_else(|e| { notes.push(e); Default::default() });
        let mut tex = |param: &str, srgb: bool, default: &wgpu::TextureView| match m.textures.get(param) {
            Some(t) => self.texture(content, t, srgb).unwrap_or_else(|e| { notes.push(e); default.clone() }),
            None => default.clone(),
        };
        let blendable = m.shader.contains("blendable");
        // blendable's layers (F_MULTIBLEND: 0 one, 1 two, 2 three), and how its textures are packed
        let version = m.shader_version.clamp(0, 1) as usize;
        let layers = if blendable { (m.int("F_MULTIBLEND") + 1).clamp(1, 3) } else { 0 };
        let views = if blendable {
            let rma = &self.rma[version];
            [tex("g_tColorA", true, &self.white), tex("g_tRmaA", false, rma), tex("g_tNormalA", false, &self.flat),
             tex("g_tColorB", true, &self.white), tex("g_tNormalB", false, &self.flat), tex("g_tRmaB", false, rma), self.white.clone(),
             tex("g_tColorC", true, &self.white), tex("g_tNormalC", false, &self.flat), tex("g_tRmaC", false, rma)]
        } else {
            [tex("g_tColor", true, &self.white), tex("g_tAmbientOcclusion", false, &self.white), tex("g_tNormal", false, &self.flat),
             self.white.clone(), self.flat.clone(), self.white.clone(), tex("g_tTintMask", false, &self.white),
             self.white.clone(), self.flat.clone(), self.white.clone()]
        };
        let has_normal = m.textures.keys().any(|t| t.starts_with("g_tNormal"));
        let (tint, tint_b, tint_c) = if blendable {
            (m.vector("g_flTintColorA", [1.0; 4]), m.vector("g_flTintColorB", [1.0; 4]), m.vector("g_flTintColorC", [1.0; 4]))
        } else {
            (m.vector("g_vColorTint", [1.0; 4]), [1.0; 4], [1.0; 4])
        };
        let (scale_b, scale_c) = (m.vector("g_vTexCoordScale2", [1.0; 4]), m.vector("g_vTexCoordScale3", [1.0; 4]));
        let alpha_test = if m.int("F_ALPHA_TEST") != 0 { m.float("g_flAlphaTestReference", 0.5) } else { 0.0 };
        let flag = |b: bool| b as i32 as f32;
        let uniform = MaterialUniform {
            tint: [tint[0], tint[1], tint[2], 1.0],
            params: [alpha_test, m.float("g_flMetalness", 0.0), flag(m.int("F_METALNESS_TEXTURE") != 0), flag(has_normal)],
            mode: [layers as f32, flag(m.int("F_TINT_MASK") != 0), flag(if blendable { m.int("F_USE_TINT_MASKS_IN_VERTEX_PAINT") != 0 } else { m.int("F_PAINT_VERTEX_COLORS") != 0 }), version as f32],
            tint_b: [tint_b[0], tint_b[1], tint_b[2], 1.0],
            tint_c: [tint_c[0], tint_c[1], tint_c[2], 1.0],
            blend: [m.float("g_flBlendSoftnessB", 0.5), m.float("g_flBlendSoftnessC", 0.5), m.float("g_flRoughnessScaleFactor", 1.0), 0.0],
            scale: [scale_b[0], scale_b[1], scale_c[0], scale_c[1]],
        };
        let buffer = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor { label: Some(name), contents: bytemuck::bytes_of(&uniform), usage: wgpu::BufferUsages::UNIFORM });
        let group = self.device.create_bind_group(&wgpu::BindGroupDescriptor { label: Some(name), layout: &self.material_layout, entries: &[
            wgpu::BindGroupEntry { binding: 0, resource: buffer.as_entire_binding() },
            wgpu::BindGroupEntry { binding: 3, resource: wgpu::BindingResource::Sampler(&self.sampler) },
            at(1, &views[0]), at(2, &views[1]), at(4, &views[2]), at(5, &views[3]), at(6, &views[4]), at(7, &views[5]), at(8, &views[6]),
            at(9, &views[7]), at(10, &views[8]), at(11, &views[9]),
        ] });
        // translucent and decal materials wait for their own pass
        let drawn = !(m.int("F_TRANSLUCENT") != 0 || m.int("F_DECAL") != 0 || m.shader.contains("glass"));
        (group, drawn)
    }

    /// A model (`models/x.vmdl`) read from `content` onto the GPU.
    pub fn model(&self, content: &dyn Files, name: &str) -> Result<Model, String> {
        let r = content.compiled(name).ok_or_else(|| format!("{name}: not found"))?;
        let meshes = mesh::model(&r, |n| content.read(&if n.ends_with("_c") { n.to_string() } else { format!("{n}_c") }))?;
        let mut notes = Vec::new();
        let (mut min, mut max) = (Vec3::splat(f32::MAX), Vec3::splat(f32::MIN));
        let mut out = Vec::new();
        let mut lightmap_uv = true;
        for m in &meshes {
            let mut vertices = Vec::new();
            for vb in &m.buffers.vertices {
                let positions = vb.floats("POSITION", 0).ok_or("no POSITION")?;
                let normals = vb.normals().unwrap_or_else(|| vec![[0.0, 0.0, 1.0]; vb.count]);
                let uvs = vb.floats("TEXCOORD", 0).unwrap_or_else(|| vec![[0.0; 4]; vb.count]);
                let blend = vb.floats("TEXCOORD", 4).unwrap_or_else(|| vec![[0.0; 4]; vb.count]);
                // a skinned vertex's bones, in the skeleton's numbering
                let skin = vb.skin().unwrap_or_else(|| vec![([0; 4], [0.0; 4]); vb.count]);
                let remap = |b: u32| m.bones.get(b as usize).copied().unwrap_or(b);
                let paint = vb.floats("TEXCOORD", 5).unwrap_or_else(|| vec![[1.0; 4]; vb.count]);
                // a map's geometry has its place in the lightmaps as its
                // TEXCOORD3, named in lower case (s&box's LightmapUV semantic)
                let lightmap = vb.floats("TEXCOORD", 3).filter(|_| vb.attribute("TEXCOORD", 3).is_some_and(|a| a.name == "texcoord"));
                lightmap_uv &= lightmap.is_some();
                let lightmap = lightmap.unwrap_or_else(|| vec![[0.0; 4]; vb.count]);
                let v: Vec<Vertex> = (0..vb.count).map(|i| {
                    let p = positions[i];
                    min = min.min(Vec3::new(p[0], p[1], p[2]));
                    max = max.max(Vec3::new(p[0], p[1], p[2]));
                    Vertex { position: [p[0], p[1], p[2]], normal: normals[i], uv: [uvs[i][0], uvs[i][1]], blend: blend[i], paint: paint[i], bones: skin[i].0.map(remap), weights: skin[i].1, lightmap: [lightmap[i][0], lightmap[i][1]] }
                }).collect();
                vertices.push(self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor { label: None, contents: bytemuck::cast_slice(&v), usage: wgpu::BufferUsages::VERTEX }));
            }
            let index_lists: Vec<Vec<u32>> = m.buffers.indices.iter().map(|ib| ib.indices()).collect();
            let draws = m.draws.iter().filter_map(|d| {
                let len = index_lists.get(d.index_buffer)?.len();
                if d.vertex_buffer >= vertices.len() { return None; }
                let (material, drawn) = self.material(content, &d.material, &mut notes);
                let end = (d.start_index + d.index_count).min(len);
                drawn.then(|| GpuDraw { vertex_buffer: d.vertex_buffer, index_buffer: d.index_buffer, range: d.start_index as u32..end as u32, base_vertex: d.base_vertex as i32, material })
            }).collect();
            let indices = index_lists.iter().map(|i| self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor { label: None, contents: bytemuck::cast_slice(i), usage: wgpu::BufferUsages::INDEX })).collect();
            out.push(GpuMesh { vertices, indices, draws });
        }
        if out.is_empty() { return Err(format!("{name}: no meshes")); }
        Ok(Model { meshes: out, min, max, notes, lightmap_uv })
    }

    /// Objects drawn from a view: an RGBA image, `width` by `height`, its
    /// top row first.
    pub fn draw(&self, objects: &[Instance], view: &View, width: u32, height: u32) -> Vec<u8> {
        let (sun, sun_color) = view.sun.unwrap_or((Vec3::new(0.6, 0.35, 0.75), Vec3::new(1.6, 1.55, 1.45)));
        // no auto exposure yet: s&box's default never goes below 1 (its
        // range is 1 to 3), and a map's sun is lit for that
        let exposure = 1.0;
        let ambient = 1.0;
        let view_proj = view.proj * view.view;
        let uniform = ViewUniform {
            view_proj: view_proj.to_cols_array_2d(),
            inv_view_proj: view_proj.inverse().to_cols_array_2d(),
            eye: view.eye.extend(exposure).into(),
            sun: sun.normalize().extend(0.0).into(),
            sun_color: sun_color.extend(0.0).into(),
            sky: [0.55 * ambient, 0.6 * ambient, 0.7 * ambient, 0.0],
            // RENDER_DEBUG: 1 base colour, 2 normal, 3 occlusion, 4 roughness
            ground: [0.25 * ambient, 0.23 * ambient, 0.2 * ambient, std::env::var("RENDER_DEBUG").ok().and_then(|v| v.parse().ok()).unwrap_or(0.0)],
            sky_rotation: Mat4::from_quat(view.sky_rotation.inverse()).to_cols_array_2d(),
        };
        let d = &self.device;
        let view_buffer = d.create_buffer_init(&wgpu::util::BufferInitDescriptor { label: Some("view"), contents: bytemuck::bytes_of(&uniform), usage: wgpu::BufferUsages::UNIFORM });
        let view_group = d.create_bind_group(&wgpu::BindGroupDescriptor { label: Some("view"), layout: &self.view_layout, entries: &[wgpu::BindGroupEntry { binding: 0, resource: view_buffer.as_entire_binding() }] });
        let object_groups: Vec<wgpu::BindGroup> = objects.iter().map(|o| {
            let lit = o.lightmapped && o.model.lightmap_uv && view.lightmaps.is_some();
            let u = ObjectUniform { model: o.transform.to_cols_array_2d(), tint: o.tint, flags: [(!o.bones.is_empty()) as i32 as f32, lit as i32 as f32, 0.0, 0.0] };
            let b = d.create_buffer_init(&wgpu::util::BufferInitDescriptor { label: Some("object"), contents: bytemuck::bytes_of(&u), usage: wgpu::BufferUsages::UNIFORM });
            let palette: Vec<[[f32; 4]; 4]> = if o.bones.is_empty() { vec![Mat4::IDENTITY.to_cols_array_2d()] } else { o.bones.iter().map(|m| m.to_cols_array_2d()).collect() };
            let p = d.create_buffer_init(&wgpu::util::BufferInitDescriptor { label: Some("bones"), contents: bytemuck::cast_slice(&palette), usage: wgpu::BufferUsages::STORAGE });
            d.create_bind_group(&wgpu::BindGroupDescriptor { label: Some("object"), layout: &self.object_layout, entries: &[
                wgpu::BindGroupEntry { binding: 0, resource: b.as_entire_binding() },
                wgpu::BindGroupEntry { binding: 1, resource: p.as_entire_binding() },
            ] })
        }).collect();
        let target = |format, samples, usage| d.create_texture(&wgpu::TextureDescriptor {
            label: None, size: wgpu::Extent3d { width, height, depth_or_array_layers: 1 }, mip_level_count: 1, sample_count: samples,
            dimension: wgpu::TextureDimension::D2, format, usage, view_formats: &[],
        });
        let msaa = target(COLOR, SAMPLES, wgpu::TextureUsages::RENDER_ATTACHMENT).create_view(&Default::default());
        let resolved = target(COLOR, 1, wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC);
        let depth = target(DEPTH, SAMPLES, wgpu::TextureUsages::RENDER_ATTACHMENT).create_view(&Default::default());
        let mut encoder = d.create_command_encoder(&Default::default());
        {
            let [r, g, b, a] = view.clear;
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &msaa,
                    resolve_target: Some(&resolved.create_view(&Default::default())),
                    ops: wgpu::Operations { load: wgpu::LoadOp::Clear(wgpu::Color { r, g, b, a }), store: wgpu::StoreOp::Discard },
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment { view: &depth, depth_ops: Some(wgpu::Operations { load: wgpu::LoadOp::Clear(1.0), store: wgpu::StoreOp::Discard }), stencil_ops: None }),
                timestamp_writes: None,
                occlusion_query_set: None,
            });
            if let Some(sky) = &view.sky {
                pass.set_pipeline(&self.sky_pipeline);
                pass.set_bind_group(0, &view_group, &[]);
                pass.set_bind_group(1, &sky.group, &[]);
                pass.draw(0..3, 0..1);
            }
            pass.set_pipeline(&self.pipeline);
            pass.set_bind_group(0, &view_group, &[]);
            pass.set_bind_group(3, &view.lightmaps.as_deref().unwrap_or(&self.unlit).group, &[]);
            for (o, group) in objects.iter().zip(&object_groups) {
                pass.set_bind_group(2, group, &[]);
                for m in &o.model.meshes {
                    for d in &m.draws {
                        pass.set_vertex_buffer(0, m.vertices[d.vertex_buffer].slice(..));
                        pass.set_index_buffer(m.indices[d.index_buffer].slice(..), wgpu::IndexFormat::Uint32);
                        pass.set_bind_group(1, &d.material, &[]);
                        pass.draw_indexed(d.range.clone(), d.base_vertex, 0..1);
                    }
                }
            }
        }
        // read it back, a row at a time as copies align them
        let row = (width * 4).div_ceil(256) * 256;
        let readback = d.create_buffer(&wgpu::BufferDescriptor { label: None, size: (row * height) as u64, usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ, mapped_at_creation: false });
        encoder.copy_texture_to_buffer(
            wgpu::TexelCopyTextureInfo { texture: &resolved, mip_level: 0, origin: wgpu::Origin3d::ZERO, aspect: wgpu::TextureAspect::All },
            wgpu::TexelCopyBufferInfo { buffer: &readback, layout: wgpu::TexelCopyBufferLayout { offset: 0, bytes_per_row: Some(row), rows_per_image: Some(height) } },
            wgpu::Extent3d { width, height, depth_or_array_layers: 1 },
        );
        self.queue.submit([encoder.finish()]);
        let slice = readback.slice(..);
        slice.map_async(wgpu::MapMode::Read, |_| {});
        let _ = d.poll(wgpu::PollType::Wait);
        let data = slice.get_mapped_range();
        (0..height as usize).flat_map(|y| data[y * row as usize..y * row as usize + width as usize * 4].to_vec()).collect()
    }
}

fn at(binding: u32, view: &wgpu::TextureView) -> wgpu::BindGroupEntry<'_> {
    wgpu::BindGroupEntry { binding, resource: wgpu::BindingResource::TextureView(view) }
}

/// A model placed in the world: where, tinted how, and posed how (each
/// bone's skinning matrix, in the model's space: its pose over its bind
/// pose's inverse; none, as it was modelled).
pub struct Instance<'a> {
    pub model: &'a Model,
    pub transform: Mat4,
    pub tint: [f32; 4],
    pub bones: Vec<Mat4>,
    /// lit by the view's lightmaps, if its model has a place in them (a
    /// map's own geometry)
    pub lightmapped: bool,
}

/// Where a view is from: the eye, its view and projection, what's behind
/// everything.
pub struct View {
    pub eye: Vec3,
    pub view: Mat4,
    pub proj: Mat4,
    pub clear: [f64; 4],
    /// the sun: towards it, and its colour (none: one of ours)
    pub sun: Option<(Vec3, Vec3)>,
    /// the sky behind everything (none: the clear colour), and how it's turned
    pub sky: Option<Arc<Sky>>,
    pub sky_rotation: glam::Quat,
    /// the map's baked lighting, for the objects it lights
    pub lightmaps: Option<Arc<Lightmaps>>,
}

/// A light a map's lightmaps name by its bake index (managed's
/// `SetBakeLightIndex`, a light entity's `bakelightindex`): their texels
/// hold how much of it reaches them, its shadows baked in, and it's shaded
/// as it's seen from there.
#[derive(Clone, Copy, Debug, Default)]
pub struct BakedLight {
    pub index: u8,
    /// a sun: shining from `towards` (else a light at `position`)
    pub sun: bool,
    pub position: Vec3,
    pub towards: Vec3,
    /// its colour times its brightness, times its bake scale
    pub color: Vec3,
}

impl BakedLight {
    /// The baked lights among a map's entities, as managed's map loader
    /// (SceneMapLoader) makes them: a light's colour times its brightness
    /// (a local light's times its attenuation's scale too), times the bake
    /// scale the compiler gave it.
    pub fn from_entities(entities: &[crate::assets::entities::Entity]) -> Vec<BakedLight> {
        let mut out = Vec::new();
        for e in entities {
            let get = |k: &str| e.get(crate::string_token(k));
            let floats = |k: &str| get(k).map(|v| v.split_whitespace().filter_map(|x| x.parse::<f32>().ok()).collect::<Vec<_>>()).unwrap_or_default();
            let num = |k: &str, d: f32| floats(k).first().copied().unwrap_or(d);
            let sun = match get("classname") {
                Some("light_environment" | "light_directional") => true,
                Some("light_spot" | "light_omni" | "light_rect" | "light_capsule") => false,
                _ => continue,
            };
            let index = num("bakelightindex", -1.0);
            if !(0.0..255.0).contains(&index) || num("enabled", 1.0) == 0.0 { continue; }
            let a = floats("angles");
            let angle = |i: usize| a.get(i).copied().unwrap_or(0.0).to_radians();
            let rot = glam::Quat::from_rotation_z(angle(1)) * glam::Quat::from_rotation_y(angle(0)) * glam::Quat::from_rotation_x(angle(2));
            let c = floats("color");
            let mut color = Vec3::new(c.first().copied().unwrap_or(255.0), c.get(1).copied().unwrap_or(255.0), c.get(2).copied().unwrap_or(255.0)) / 255.0 * num("brightness", 1.0);
            if !sun {
                let scale = num("attenuation2", 1.0) * 10000.0 + num("attenuation1", 0.0) * 100.0 + num("attenuation0", 0.0);
                if scale > 0.0 { color *= scale; }
            }
            let o = floats("origin");
            out.push(BakedLight {
                index: index as u8,
                sun,
                position: Vec3::new(o.first().copied().unwrap_or(0.0), o.get(1).copied().unwrap_or(0.0), o.get(2).copied().unwrap_or(0.0)),
                towards: rot * -Vec3::X,
                color: color * num("bakelightindexscale", 1.0),
            });
        }
        out
    }
}

/// A map's baked lighting on the GPU (Renderer::lightmaps).
pub struct Lightmaps {
    group: wgpu::BindGroup,
}

impl Lightmaps {
    fn group(device: &wgpu::Device, layout: &wgpu::BindGroupLayout, sampler: &wgpu::Sampler, uniform: [f32; 4], maps: [&wgpu::TextureView; 4], lights: &[BakedLight]) -> Lightmaps {
        // every index has a light: black, if none was baked with it
        let mut table = vec![[[0f32; 4]; 3]; 256];
        for l in lights {
            table[l.index as usize] = [
                [l.position.x, l.position.y, l.position.z, if l.sun { 2.0 } else { 1.0 }],
                l.towards.normalize_or_zero().extend(0.0).into(),
                l.color.extend(0.0).into(),
            ];
        }
        let u = device.create_buffer_init(&wgpu::util::BufferInitDescriptor { label: Some("lightmaps"), contents: bytemuck::cast_slice(&uniform), usage: wgpu::BufferUsages::UNIFORM });
        let l = device.create_buffer_init(&wgpu::util::BufferInitDescriptor { label: Some("baked lights"), contents: bytemuck::cast_slice(&table), usage: wgpu::BufferUsages::STORAGE });
        Lightmaps { group: device.create_bind_group(&wgpu::BindGroupDescriptor { label: Some("lightmaps"), layout, entries: &[
            wgpu::BindGroupEntry { binding: 0, resource: u.as_entire_binding() },
            at(1, maps[0]), at(2, maps[1]), at(3, maps[2]), at(4, maps[3]),
            wgpu::BindGroupEntry { binding: 5, resource: l.as_entire_binding() },
            wgpu::BindGroupEntry { binding: 6, resource: wgpu::BindingResource::Sampler(sampler) },
        ] }) }
    }
}

/// A sky on the GPU (Renderer::sky).
pub struct Sky {
    group: wgpu::BindGroup,
}

impl View {
    /// A camera as Source 2 has one: at `position`, turned by `angles`
    /// (pitch, yaw, roll in degrees; x forward, z up), with a horizontal
    /// field of view in degrees.
    pub fn camera(position: Vec3, angles: [f32; 3], fov: f32, near: f32, far: f32, aspect: f32) -> View {
        let rot = glam::Quat::from_rotation_z(angles[1].to_radians()) * glam::Quat::from_rotation_y(angles[0].to_radians()) * glam::Quat::from_rotation_x(angles[2].to_radians());
        let vertical = 2.0 * ((fov.to_radians() * 0.5).tan() / aspect.max(1e-3)).atan();
        View {
            eye: position,
            view: Mat4::look_at_rh(position, position + rot * Vec3::X, rot * Vec3::Z),
            proj: Mat4::perspective_rh(vertical, aspect, near.max(0.1), far.max(near + 1.0)),
            clear: [0.55, 0.6, 0.7, 1.0],
            sun: None,
            sky: None,
            sky_rotation: glam::Quat::IDENTITY,
            lightmaps: None,
        }
    }

    /// A model framed from `yaw` and `pitch` degrees round it (0, 0: from
    /// its front, +x), on nothing.
    pub fn framing(model: &Model, yaw: f32, pitch: f32) -> View {
        let center = (model.min + model.max) * 0.5;
        let radius = (model.max - model.min).length() * 0.5;
        let fov = 30f32.to_radians();
        let dir = Vec3::new(yaw.to_radians().cos() * pitch.to_radians().cos(), yaw.to_radians().sin() * pitch.to_radians().cos(), pitch.to_radians().sin());
        let eye = center + dir * (radius / (fov * 0.5).sin()) * 1.05;
        View { eye, view: Mat4::look_at_rh(eye, center, Vec3::Z), proj: Mat4::perspective_rh(fov, 1.0, radius * 0.05, radius * 10.0), clear: [0.0; 4], sun: None, sky: None, sky_rotation: glam::Quat::IDENTITY, lightmaps: None }
    }
}

#[cfg(test)]
mod tests {
    /// The TV from sbox.game (tools/package.sh facepunch.tv), if this
    /// machine has it and a GPU (or lavapipe): drawn, it's there, and dark.
    #[test]
    fn a_downloaded_model_is_drawn() {
        let Ok(dir) = std::env::var("ENGINE2_CONTENT") else { return };
        let content = super::Content(vec![dir.into()], Vec::new());
        use super::Files;
        if content.read("models/sbox_props/tv/tv.vmdl_c").is_none() { return; }
        let Ok(r) = super::Renderer::new() else { eprintln!("no adapter: skipped"); return };
        let m = r.model(&content, "models/sbox_props/tv/tv.vmdl").unwrap();
        assert!(m.notes.is_empty(), "{:?}", m.notes);
        let image = r.draw(&[super::Instance { model: &m, transform: glam::Mat4::IDENTITY, tint: [1.0; 4], bones: Vec::new(), lightmapped: false }], &super::View::framing(&m, 35.0, 15.0), 128, 128);
        let covered: Vec<&[u8]> = image.chunks_exact(4).filter(|p| p[3] > 0).collect();
        let mean = covered.iter().map(|p| p[0] as f32 + p[1] as f32 + p[2] as f32).sum::<f32>() / (3 * covered.len()) as f32;
        eprintln!("{} of {} pixels covered, mean {mean:.0}", covered.len(), 128 * 128);
        assert!(covered.len() > 128 * 128 / 4, "it fills the frame");
        assert!(mean > 15.0 && mean < 110.0, "a black television, lit: {mean}");
    }
}
