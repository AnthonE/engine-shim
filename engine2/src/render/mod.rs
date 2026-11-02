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
    /// world to the sun's shadow map (x, y -1..1, depth 0..1)
    shadow: [[f32; 4]; 4],
    /// x: shadows (1), y: a texel of the map in uv, z: in the world
    shadow_params: [f32; 4],
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
    extra: [f32; 4],
}

/// A run of triangles to draw: its buffers (by index), its range, its material.
struct GpuDraw {
    vertex_buffer: usize,
    index_buffer: usize,
    range: std::ops::Range<u32>,
    base_vertex: i32,
    material: wgpu::BindGroup,
    kind: Kind,
    /// the material in its place in each of the model's material groups
    /// (skins), where it differs
    skins: Vec<Option<(wgpu::BindGroup, Kind)>>,
}

impl GpuDraw {
    /// Its material and how it's drawn, in skin `skin`.
    fn look(&self, skin: usize) -> (&wgpu::BindGroup, Kind) {
        match self.skins.get(skin) {
            Some(Some((g, k))) => (g, *k),
            _ => (&self.material, self.kind),
        }
    }
}

/// How a material is drawn: with what's opaque; over what's behind it
/// (translucent, cheap glass); or seeing what's behind it (glass).
#[derive(Clone, Copy, PartialEq, Debug)]
enum Kind {
    Opaque,
    Blend,
    Glass,
    CheapGlass,
}

/// A mesh's draws on the GPU, and the buffers they read.
struct GpuMesh {
    vertices: Vec<wgpu::Buffer>,
    indices: Vec<wgpu::Buffer>,
    draws: Vec<GpuDraw>,
    /// the mesh groups it's in
    group_mask: u64,
}

/// A model read and on the GPU, with what it covers.
pub struct Model {
    meshes: Vec<GpuMesh>,
    /// the mesh groups shown unless an object chooses others
    pub default_mask: u64,
    pub min: Vec3,
    pub max: Vec3,
    /// whether every vertex has a place in a map's lightmaps (a map's own
    /// geometry, compiled with baked lighting)
    pub lightmap_uv: bool,
    /// what was missing or couldn't be read, for whoever's looking
    pub notes: Vec<String>,
}

pub struct Renderer {
    /// what made the device: a window's surface is made with them (window.rs)
    pub instance: wgpu::Instance,
    pub gpu: wgpu::Adapter,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pipeline: wgpu::RenderPipeline,
    /// translucent, glass and cheap glass, and what glass's view and
    /// material groups hold (what's been drawn; its three textures)
    blend_pipeline: wgpu::RenderPipeline,
    /// the sun's shadow map: what's drawn into it, from what, read how
    shadow_pipeline: wgpu::RenderPipeline,
    uniform_layout: wgpu::BindGroupLayout,
    compare_sampler: wgpu::Sampler,
    empty: wgpu::BindGroup,
    glass_pipeline: wgpu::RenderPipeline,
    cheap_glass_pipeline: wgpu::RenderPipeline,
    /// the frame exposed and tonemapped (tonemap.wgsl)
    /// the frame exposed and tonemapped, into an output of each format it's asked for
    tonemap_shader: wgpu::ShaderModule,
    tonemap_pipelines: Mutex<HashMap<wgpu::TextureFormat, wgpu::RenderPipeline>>,
    tonemap_layout: wgpu::BindGroupLayout,
    glass_view_layout: wgpu::BindGroupLayout,
    glass_layout: wgpu::BindGroupLayout,
    view_layout: wgpu::BindGroupLayout,
    material_layout: wgpu::BindGroupLayout,
    object_layout: wgpu::BindGroupLayout,
    sky_pipeline: wgpu::RenderPipeline,
    sky_layout: wgpu::BindGroupLayout,
    skies: Mutex<HashMap<String, Arc<Sky>>>,
    lighting_layout: wgpu::BindGroupLayout,
    /// the lighting group of a view with no lightmaps, and a light probe
    /// volume's textures where there's none
    unlit: Lightmaps,
    none: wgpu::TextureView,
    clamp: wgpu::Sampler,
    sampler: wgpu::Sampler,
    /// whether the GPU takes BC1 to BC7 as they are (most desktop GPUs,
    /// lavapipe, WebGPU on desktops), and what's been read already
    bc: bool,
    textures: Mutex<HashMap<(String, bool), wgpu::TextureView>>,
    materials: Mutex<HashMap<String, (wgpu::BindGroup, Option<Kind>, Vec<String>)>>,
    /// for a material without a texture: white (a colour, occlusion, a
    /// tint mask), a flat normal (roughness 0.6), and a blendable's g_tRma
    /// of either version (not metal, not occluded, all revealed: version 1
    /// reads metalness, occlusion, mask; version 0 roughness 0.6,
    /// metalness, occlusion, mask)
    white: wgpu::TextureView,
    flat: wgpu::TextureView,
    rma: [wgpu::TextureView; 2],
    /// black: no glow where a material has no self-illumination mask
    none2d: wgpu::TextureView,
    pub adapter: String,
}

/// The frame as it's drawn (linear, HDR), and as it's shown (tonemapped)
const COLOR: wgpu::TextureFormat = wgpu::TextureFormat::Rgba16Float;
const SHOWN: wgpu::TextureFormat = wgpu::TextureFormat::Rgba8UnormSrgb;
const DEPTH: wgpu::TextureFormat = wgpu::TextureFormat::Depth32Float;
const SAMPLES: u32 = 4;
/// the sun's shadow map's size, and how far from the eye what casts into it may be
const SHADOW_SIZE: u32 = 2048;
const SHADOW_REACH: f32 = 4000.0;

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
        let array = |binding| wgpu::BindGroupLayoutEntry { binding, visibility: wgpu::ShaderStages::FRAGMENT, ty: wgpu::BindingType::Texture { sample_type: wgpu::TextureSampleType::Float { filterable: true }, view_dimension: wgpu::TextureViewDimension::D2Array, multisampled: false }, count: None };
        // the view, and the sun's shadow map, compared with
        let shadow_map = wgpu::BindGroupLayoutEntry { binding: 2, visibility: wgpu::ShaderStages::FRAGMENT, ty: wgpu::BindingType::Texture { sample_type: wgpu::TextureSampleType::Depth, view_dimension: wgpu::TextureViewDimension::D2, multisampled: false }, count: None };
        let compare = wgpu::BindGroupLayoutEntry { binding: 3, visibility: wgpu::ShaderStages::FRAGMENT, ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Comparison), count: None };
        let view_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor { label: Some("view"), entries: &[uniform(0, wgpu::ShaderStages::VERTEX_FRAGMENT), shadow_map, compare] });
        // the shadow map's own pass: the view alone
        let uniform_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor { label: Some("shadow view"), entries: &[uniform(0, wgpu::ShaderStages::VERTEX_FRAGMENT)] });
        let empty_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor { label: None, entries: &[] });
        let material_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor { label: Some("material"), entries: &[
            uniform(0, wgpu::ShaderStages::FRAGMENT), tex(1), tex(2),
            wgpu::BindGroupLayoutEntry { binding: 3, visibility: wgpu::ShaderStages::FRAGMENT, ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering), count: None },
            tex(4), tex(5), tex(6), tex(7), tex(9), tex(10), tex(11),
        ] });
        let object_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor { label: Some("object"), entries: &[
            uniform(0, wgpu::ShaderStages::VERTEX_FRAGMENT),
            wgpu::BindGroupLayoutEntry { binding: 1, visibility: wgpu::ShaderStages::VERTEX, ty: wgpu::BindingType::Buffer { ty: wgpu::BufferBindingType::Storage { read_only: true }, has_dynamic_offset: false, min_binding_size: None }, count: None },
        ] });
        // a map's baked lighting (as s&box's vr_lighting.fxc reads it): its
        // lightmaps (direct lights' indices and strengths, irradiance, and
        // the directional part of it) or a probe volume's (indices,
        // strengths, ambient cubes: an object reads one or the other, so
        // they share their places), and the lights the indices name
        let lighting_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor { label: Some("lighting"), entries: &[
            uniform(0, wgpu::ShaderStages::VERTEX_FRAGMENT), array(1), array(2), array(3), array(4),
            wgpu::BindGroupLayoutEntry { binding: 5, visibility: wgpu::ShaderStages::FRAGMENT, ty: wgpu::BindingType::Buffer { ty: wgpu::BufferBindingType::Storage { read_only: true }, has_dynamic_offset: false, min_binding_size: None }, count: None },
            wgpu::BindGroupLayoutEntry { binding: 6, visibility: wgpu::ShaderStages::FRAGMENT, ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering), count: None },
        ] });
        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor { label: None, bind_group_layouts: &[&view_layout, &material_layout, &object_layout, &lighting_layout], push_constant_ranges: &[] });
        let glass_view_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor { label: Some("glass view"), entries: &[uniform(0, wgpu::ShaderStages::VERTEX_FRAGMENT), tex(1), shadow_map, compare] });
        let glass_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor { label: Some("glass"), entries: &[
            uniform(0, wgpu::ShaderStages::FRAGMENT), tex(1), tex(2),
            wgpu::BindGroupLayoutEntry { binding: 3, visibility: wgpu::ShaderStages::FRAGMENT, ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering), count: None },
            tex(4),
        ] });
        let glass_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor { label: None, bind_group_layouts: &[&glass_view_layout, &glass_layout, &object_layout, &lighting_layout], push_constant_ranges: &[] });
        let vertex_layout = [wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as u64,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3, 2 => Float32x2, 3 => Float32x4, 4 => Float32x4, 5 => Uint32x4, 6 => Float32x4, 7 => Float32x2],
        }];
        // what's opaque writes depth; what's drawn over what's behind doesn't
        let make = |label, layout: &wgpu::PipelineLayout, entry, blend: Option<wgpu::BlendState>, depth_write| device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some(label),
            layout: Some(layout),
            vertex: wgpu::VertexState { module: &shader, entry_point: Some("vs_main"), compilation_options: Default::default(), buffers: &vertex_layout },
            fragment: Some(wgpu::FragmentState { module: &shader, entry_point: Some(entry), compilation_options: Default::default(), targets: &[Some(wgpu::ColorTargetState { format: COLOR, blend, write_mask: wgpu::ColorWrites::ALL })] }),
            primitive: wgpu::PrimitiveState { cull_mode: None, ..Default::default() },
            depth_stencil: Some(wgpu::DepthStencilState { format: DEPTH, depth_write_enabled: depth_write, depth_compare: wgpu::CompareFunction::Less, stencil: Default::default(), bias: Default::default() }),
            multisample: wgpu::MultisampleState { count: SAMPLES, ..Default::default() },
            multiview: None,
            cache: None,
        });
        let pipeline = make("complex", &layout, "fs_main", None, true);
        let shadow_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("shadow"),
            layout: Some(&device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor { label: None, bind_group_layouts: &[&uniform_layout, &empty_layout, &object_layout], push_constant_ranges: &[] })),
            vertex: wgpu::VertexState { module: &shader, entry_point: Some("vs_shadow"), compilation_options: Default::default(), buffers: &vertex_layout },
            fragment: None,
            primitive: wgpu::PrimitiveState { cull_mode: None, ..Default::default() },
            depth_stencil: Some(wgpu::DepthStencilState { format: DEPTH, depth_write_enabled: true, depth_compare: wgpu::CompareFunction::Less, stencil: Default::default(), bias: wgpu::DepthBiasState { constant: 2, slope_scale: 2.0, clamp: 0.0 } }),
            multisample: Default::default(),
            multiview: None,
            cache: None,
        });
        let empty = device.create_bind_group(&wgpu::BindGroupDescriptor { label: None, layout: &empty_layout, entries: &[] });
        let compare_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            mag_filter: wgpu::FilterMode::Linear, min_filter: wgpu::FilterMode::Linear, compare: Some(wgpu::CompareFunction::LessEqual), ..Default::default()
        });
        let blend_pipeline = make("translucent", &layout, "fs_main", Some(wgpu::BlendState::ALPHA_BLENDING), false);
        let glass_pipeline = make("glass", &glass_pipeline_layout, "fs_glass", None, false);
        let cheap_glass_pipeline = make("cheap glass", &glass_pipeline_layout, "fs_glass", Some(wgpu::BlendState::ALPHA_BLENDING), false);
        let tonemap_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor { label: Some("tonemap"), source: wgpu::ShaderSource::Wgsl(include_str!("tonemap.wgsl").into()) });
        let tonemap_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor { label: Some("tonemap"), entries: &[uniform(0, wgpu::ShaderStages::FRAGMENT), tex(1)] });

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
        let (white, flat, rma, none2d) = (pixel([255; 4]), pixel([128, 128, 153, 255]), [pixel([153, 0, 255, 0]), pixel([0, 255, 0, 255])], pixel([0, 0, 0, 255]));
        let clamp = device.create_sampler(&wgpu::SamplerDescriptor {
            mag_filter: wgpu::FilterMode::Linear, min_filter: wgpu::FilterMode::Linear, mipmap_filter: wgpu::FilterMode::Linear, ..Default::default()
        });
        let none = device.create_texture_with_data(&queue, &wgpu::TextureDescriptor {
            label: None, size: wgpu::Extent3d { width: 1, height: 1, depth_or_array_layers: 1 }, mip_level_count: 1, sample_count: 1,
            dimension: wgpu::TextureDimension::D2, format: wgpu::TextureFormat::Rgba8Unorm, usage: wgpu::TextureUsages::TEXTURE_BINDING, view_formats: &[],
        }, Default::default(), &[0; 4]).create_view(&wgpu::TextureViewDescriptor { dimension: Some(wgpu::TextureViewDimension::D2Array), ..Default::default() });
        let unlit = Lightmaps::group(&device, &lighting_layout, &clamp, LightingUniform::default(), [&none, &none, &none, &none], &[BakedLight::default()]);
        Ok(Renderer { instance, gpu: adapter, tonemap_shader, tonemap_pipelines: Default::default(), tonemap_layout, empty, shadow_pipeline, uniform_layout, compare_sampler, blend_pipeline, glass_pipeline, cheap_glass_pipeline, glass_view_layout, glass_layout, none, bc, textures: Default::default(), materials: Default::default(), skies: Default::default(), sky_pipeline, sky_layout, lighting_layout, unlit, clamp, device, queue, pipeline, view_layout, material_layout, object_layout, sampler, white, flat, rma, none2d, adapter: format!("{} ({:?})", info.name, info.backend) })
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
        let views = [0, 1, 2, 3].map(|i| self.slices(content, maps[i]));
        let [Ok((a, size)), Ok((b, _)), Ok((c, _)), Ok((e, _))] = views else {
            return Err(views.into_iter().filter_map(Result::err).collect::<Vec<_>>().join("; "));
        };
        let size = (size[0] as f32, size[1] as f32);
        let uniform = LightingUniform { lightmap: [scale.first().copied().unwrap_or(1.0), scale.get(1).copied().unwrap_or(1.0), size.0, size.1], ..Default::default() };
        Ok(Some(Lightmaps::group(&self.device, &self.lighting_layout, &self.clamp, uniform, [&a, &b, &c, &e], lights)))
    }

    /// A texture of slices (a light probe volume's), each slice a layer,
    /// with its size: its blocks as they are where the GPU takes them.
    fn slices(&self, content: &dyn Files, name: &str) -> Result<(wgpu::TextureView, [u32; 3]), String> {
        let t = texture::read(content.compiled(name).ok_or_else(|| format!("{name}: not found"))?)?;
        use texture::format as f;
        let (w, h, d) = (t.width as u32, t.height as u32, t.depth as u32);
        let (format, row, rows) = match t.format {
            f::BC6H if self.bc && w % 4 == 0 && h % 4 == 0 => (wgpu::TextureFormat::Bc6hRgbUfloat, w / 4 * 16, h / 4),
            f::BC7 if self.bc && w % 4 == 0 && h % 4 == 0 => (wgpu::TextureFormat::Bc7RgbaUnorm, w / 4 * 16, h / 4),
            f::RGBA8888 => (wgpu::TextureFormat::Rgba8Unorm, w * 4, h),
            f => return Err(format!("{name}: a volume of format {f} not drawn yet")),
        };
        let data = t.raw(0)?;
        if data.len() < (row * rows * d) as usize { return Err(format!("{name}: short")); }
        let gpu = self.device.create_texture(&wgpu::TextureDescriptor {
            label: Some(name), size: wgpu::Extent3d { width: w, height: h, depth_or_array_layers: d }, mip_level_count: 1, sample_count: 1,
            dimension: wgpu::TextureDimension::D2, format, usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST, view_formats: &[],
        });
        self.queue.write_texture(
            wgpu::TexelCopyTextureInfo { texture: &gpu, mip_level: 0, origin: wgpu::Origin3d::ZERO, aspect: wgpu::TextureAspect::All },
            &data[..(row * rows * d) as usize],
            wgpu::TexelCopyBufferLayout { offset: 0, bytes_per_row: Some(row), rows_per_image: Some(rows) },
            wgpu::Extent3d { width: w, height: h, depth_or_array_layers: d },
        );
        Ok((gpu.create_view(&wgpu::TextureViewDescriptor { dimension: Some(wgpu::TextureViewDimension::D2Array), ..Default::default() }), [w, h, d]))
    }

    /// A light probe volume (a map's `env_light_probe_volume`, or managed's
    /// SceneLightProbe): a box of probes, `transform` placing it, each
    /// probe the light from each of six ways (its `texture`, the six
    /// stacked in depth) and the lights it names and how much of each
    /// reaches it (`indices`, `strengths`: as a lightmap's), lit by `lights`.
    pub fn probe_volume(&self, content: &dyn Files, texture: &str, indices: &str, strengths: &str, place: ProbePlace, lights: &[BakedLight]) -> Result<ProbeVolume, String> {
        let (cube, [w, h, d]) = self.slices(content, texture)?;
        let (dli, _) = self.slices(content, indices)?;
        let (dls, _) = self.slices(content, strengths)?;
        // world to the box's own space, then to 0..1 across it
        let size = (place.maxs - place.mins).max(Vec3::splat(1e-3));
        let to_local = Mat4::from_scale(1.0 / size) * Mat4::from_translation(-place.mins) * place.transform.inverse();
        let uniform = LightingUniform { to_local: to_local.to_cols_array_2d(), probes: [w as f32, h as f32, (d / 6).max(1) as f32, 0.0], ..Default::default() };
        let lightmaps = Lightmaps::group(&self.device, &self.lighting_layout, &self.clamp, uniform, [&dli, &dls, &cube, &self.none], lights);
        Ok(ProbeVolume { place, lighting: lightmaps })
    }

    /// A material's bind group, and how it's drawn (none: not yet).
    fn material(&self, content: &dyn Files, name: &str, notes: &mut Vec<String>) -> (wgpu::BindGroup, Option<Kind>) {
        let key = crate::fs::normal(name);
        if let Some((g, drawn, n)) = self.materials.lock().unwrap().get(&key) { notes.extend(n.iter().cloned()); return (g.clone(), *drawn); }
        let mut own = Vec::new();
        let (g, drawn) = self.read_material(content, name, &mut own);
        notes.extend(own.iter().cloned());
        self.materials.lock().unwrap().insert(key, (g.clone(), drawn, own));
        (g, drawn)
    }

    fn read_material(&self, content: &dyn Files, name: &str, notes: &mut Vec<String>) -> (wgpu::BindGroup, Option<Kind>) {
        let m = content.compiled(name).ok_or_else(|| format!("{name}: not found")).and_then(|r| material::read(&r));
        let m = m.unwrap_or_else(|e| { notes.push(e); Default::default() });
        let mut tex = |param: &str, srgb: bool, default: &wgpu::TextureView| match m.textures.get(param) {
            Some(t) => self.texture(content, t, srgb).unwrap_or_else(|e| { notes.push(e); default.clone() }),
            None => default.clone(),
        };
        let flag = |b: bool| b as i32 as f32;
        if m.shader.contains("glass") {
            // glass.shader: its colour (alpha how opaque, for cheap glass), a
            // plain normal (alpha a tint mask), roughness, metalness, occlusion
            let tint = m.vector("g_flTintColor", [1.0; 4]);
            let cheap = m.int("F_CHEAP_GLASS") != 0;
            let uniform = MaterialUniform {
                tint: [tint[0], tint[1], tint[2], 1.0],
                blend: [0.5, 0.5, m.float("g_flRoughnessScaleFactor", 1.0), 0.0],
                extra: [m.float("g_flBlurAmount", 0.0), m.float("AlbedoAbsorption", 0.0), flag(cheap), 0.0],
                ..bytemuck::Zeroable::zeroed()
            };
            let views = [tex("g_tColor", true, &self.white), tex("g_tNormal", false, &self.flat), tex("g_tRma", false, &self.rma[0])];
            let buffer = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor { label: Some(name), contents: bytemuck::bytes_of(&uniform), usage: wgpu::BufferUsages::UNIFORM });
            let group = self.device.create_bind_group(&wgpu::BindGroupDescriptor { label: Some(name), layout: &self.glass_layout, entries: &[
                wgpu::BindGroupEntry { binding: 0, resource: buffer.as_entire_binding() },
                wgpu::BindGroupEntry { binding: 3, resource: wgpu::BindingResource::Sampler(&self.sampler) },
                at(1, &views[0]), at(4, &views[1]), at(2, &views[2]),
            ] });
            return (group, Some(if cheap { Kind::CheapGlass } else { Kind::Glass }));
        }
        let blendable = m.shader.contains("blendable");
        // blendable's layers (F_MULTIBLEND: 0 one, 1 two, 2 three), and how its textures are packed
        let version = m.shader_version.clamp(0, 1) as usize;
        let layers = if blendable { (m.int("F_MULTIBLEND") + 1).clamp(1, 3) } else { 0 };
        let views = if blendable {
            let rma = &self.rma[version];
            [tex("g_tColorA", true, &self.white), tex("g_tRmaA", false, rma), tex("g_tNormalA", false, &self.flat),
             tex("g_tColorB", true, &self.white), tex("g_tNormalB", false, &self.flat), tex("g_tRmaB", false, rma),
             tex("g_tColorC", true, &self.white), tex("g_tNormalC", false, &self.flat), tex("g_tRmaC", false, rma)]
        } else {
            // its tint mask, self-illumination mask and metalness where
            // blendable has layer B's colour, normal and g_tRma
            [tex("g_tColor", true, &self.white), tex("g_tAmbientOcclusion", false, &self.white), tex("g_tNormal", false, &self.flat),
             tex("g_tTintMask", false, &self.white), tex("g_tSelfIllumMask", true, &self.none2d), tex("g_tMetalness", false, &self.white),
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
        let translucent = !blendable && m.int("F_TRANSLUCENT") != 0;
        // metalness: from g_tMetalness where it has one, else the colour's alpha
        let metal_from = if m.int("F_METALNESS_TEXTURE") == 0 { 0.0 } else if m.textures.contains_key("g_tMetalness") { 2.0 } else { 1.0 };
        // its glow (vr_shared_standard_ps_code.fxc): 2^brightness, scale, tint (sRGB)
        let illum = !blendable && m.int("F_SELF_ILLUM") != 0;
        let glow = {
            let t = m.vector("g_vSelfIllumTint", [1.0; 4]);
            let k = 2f32.powf(m.float("g_flSelfIllumBrightness", 0.0)) * m.float("g_flSelfIllumScale", 1.0).clamp(0.0, 1.0);
            let lin = |c: f32| if c <= 0.04045 { c / 12.92 } else { ((c + 0.055) / 1.055).powf(2.4) };
            [lin(t[0]) * k, lin(t[1]) * k, lin(t[2]) * k, m.float("g_flSelfIllumAlbedoFactor", 1.0)]
        };
        let uniform = MaterialUniform {
            tint: [tint[0], tint[1], tint[2], 1.0],
            params: [alpha_test, m.float("g_flMetalness", 0.0), metal_from, flag(has_normal)],
            mode: [layers as f32, flag(m.int("F_TINT_MASK") != 0), flag(if blendable { m.int("F_USE_TINT_MASKS_IN_VERTEX_PAINT") != 0 } else { m.int("F_PAINT_VERTEX_COLORS") != 0 }), version as f32],
            tint_b: [tint_b[0], tint_b[1], tint_b[2], 1.0],
            // complex's glow where blendable has layer C's tint
            tint_c: if illum { glow } else { [tint_c[0], tint_c[1], tint_c[2], 1.0] },
            blend: [m.float("g_flBlendSoftnessB", 0.5), m.float("g_flBlendSoftnessC", 0.5), m.float("g_flRoughnessScaleFactor", 1.0), 0.0],
            scale: [scale_b[0], scale_b[1], scale_c[0], scale_c[1]],
            extra: [if translucent { m.float("g_flOpacityScale", 1.0).max(1e-3) } else { 0.0 }, 0.0, 0.0, flag(illum)],
        };
        let buffer = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor { label: Some(name), contents: bytemuck::bytes_of(&uniform), usage: wgpu::BufferUsages::UNIFORM });
        let group = self.device.create_bind_group(&wgpu::BindGroupDescriptor { label: Some(name), layout: &self.material_layout, entries: &[
            wgpu::BindGroupEntry { binding: 0, resource: buffer.as_entire_binding() },
            wgpu::BindGroupEntry { binding: 3, resource: wgpu::BindingResource::Sampler(&self.sampler) },
            at(1, &views[0]), at(2, &views[1]), at(4, &views[2]), at(5, &views[3]), at(6, &views[4]), at(7, &views[5]),
            at(9, &views[6]), at(10, &views[7]), at(11, &views[8]),
        ] });
        // decals wait for their own pass
        let kind = if m.int("F_DECAL") != 0 || m.shader.contains("overlay") { None } else if translucent { Some(Kind::Blend) } else { Some(Kind::Opaque) };
        (group, kind)
    }

    /// A model (`models/x.vmdl`) read from `content` onto the GPU.
    pub fn model(&self, content: &dyn Files, name: &str) -> Result<Model, String> {
        let r = content.compiled(name).ok_or_else(|| format!("{name}: not found"))?;
        let meshes = mesh::model(&r, |n| content.read(&if n.ends_with("_c") { n.to_string() } else { format!("{n}_c") }))?;
        let (default_mask, skins) = mesh::skins(&r);
        self.model_of(content, name, &meshes, default_mask, &skins)
    }

    /// A model of these meshes (a file's, or ones built at runtime:
    /// rendermesh.rs), its materials found in `content`; `name` says whose.
    /// `default_mask`: the mesh groups shown by default; `skins`: its
    /// material groups, the first the materials its meshes name.
    pub fn model_of(&self, content: &dyn Files, name: &str, meshes: &[mesh::Mesh], default_mask: u64, skins: &[Vec<String>]) -> Result<Model, String> {
        let mut notes = Vec::new();
        let (mut min, mut max) = (Vec3::splat(f32::MAX), Vec3::splat(f32::MIN));
        let mut out = Vec::new();
        let mut lightmap_uv = true;
        for m in meshes {
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
                let (material, kind) = self.material(content, &d.material, &mut notes);
                let end = (d.start_index + d.index_count).min(len);
                // its place in the first material group, and what's there in each other
                let place = skins.first().and_then(|g| g.iter().position(|m| crate::fs::normal(m) == crate::fs::normal(&d.material)));
                let skins = skins.iter().map(|g| {
                    let other = g.get(place?)?;
                    if crate::fs::normal(other) == crate::fs::normal(&d.material) { return None; }
                    let (m, k) = self.material(content, other, &mut notes);
                    Some((m, k?))
                }).collect();
                kind.map(|kind| GpuDraw { vertex_buffer: d.vertex_buffer, index_buffer: d.index_buffer, range: d.start_index as u32..end as u32, base_vertex: d.base_vertex as i32, material, kind, skins })
            }).collect();
            let indices = index_lists.iter().map(|i| self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor { label: None, contents: bytemuck::cast_slice(i), usage: wgpu::BufferUsages::INDEX })).collect();
            out.push(GpuMesh { vertices, indices, draws, group_mask: m.group_mask });
        }
        if out.is_empty() { return Err(format!("{name}: no meshes")); }
        Ok(Model { meshes: out, default_mask, min, max, notes, lightmap_uv })
    }

    /// Objects drawn from a view: an RGBA image, `width` by `height`, its
    /// top row first.
    pub fn draw(&self, objects: &[Instance], view: &View, width: u32, height: u32) -> Vec<u8> {
        self.draw_into(objects, view, width, height, None)
    }

    /// The tonemap pass for an output of `format`, made the first time it's asked for.
    fn tonemap_pipeline(&self, format: wgpu::TextureFormat) -> wgpu::RenderPipeline {
        let mut all = self.tonemap_pipelines.lock().unwrap();
        all.entry(format).or_insert_with(|| self.device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("tonemap"),
            layout: Some(&self.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor { label: None, bind_group_layouts: &[&self.tonemap_layout], push_constant_ranges: &[] })),
            vertex: wgpu::VertexState { module: &self.tonemap_shader, entry_point: Some("vs_main"), compilation_options: Default::default(), buffers: &[] },
            fragment: Some(wgpu::FragmentState { module: &self.tonemap_shader, entry_point: Some("fs_main"), compilation_options: Default::default(), targets: &[Some(format.into())] }),
            primitive: Default::default(),
            depth_stencil: None,
            multisample: Default::default(),
            multiview: None,
            cache: None,
        })).clone()
    }

    /// Objects drawn from a view, into `target` (a window's frame: drawn
    /// there, nothing read back, an empty result) or, without one, as an
    /// RGBA image read back.
    pub fn draw_into(&self, objects: &[Instance], view: &View, width: u32, height: u32, target_view: Option<(&wgpu::TextureView, wgpu::TextureFormat)>) -> Vec<u8> {
        let (sun, sun_color) = view.sun.unwrap_or((Vec3::new(0.6, 0.35, 0.75), Vec3::new(1.6, 1.55, 1.45)));
        // no auto exposure yet: s&box's default never goes below 1 (its
        // range is 1 to 3), and a map's sun is lit for that
        let exposure = 1.0;
        let ambient = 1.0;
        let view_proj = view.proj * view.view;
        // what casts the sun's shadow: what isn't in the lightmaps (their
        // shadows are baked in, as s&box's cascades leave a baked sun's
        // static casters out), near enough to be seen
        // (a map's own objects, lightmapped or lit by its probes, were all
        // there when it was baked)
        let casts = |o: &Instance| !(o.lightmapped && view.lightmaps.is_some());
        let towards = sun.normalize();
        let light_view = Mat4::look_at_rh(Vec3::ZERO, -towards, if towards.z.abs() > 0.99 { Vec3::X } else { Vec3::Z });
        let (mut lo, mut hi) = (Vec3::splat(f32::MAX), Vec3::splat(f32::MIN));
        let mut casters = 0;
        for o in objects.iter().filter(|o| casts(o)) {
            let (a, b) = (o.model.min, o.model.max);
            if o.transform.transform_point3((a + b) * 0.5).distance(view.eye) > SHADOW_REACH { continue; }
            casters += 1;
            for k in 0..8 {
                let c = Vec3::new(if k & 1 == 0 { a.x } else { b.x }, if k & 2 == 0 { a.y } else { b.y }, if k & 4 == 0 { a.z } else { b.z });
                let p = light_view.transform_point3(o.transform.transform_point3(c));
                lo = lo.min(p);
                hi = hi.max(p);
            }
        }
        // the casters' box as the sun sees it (looking down -z), and far
        // enough beyond for what they fall on
        let shadow = if casters > 0 {
            let pad = 4.0;
            Some(Mat4::orthographic_rh(lo.x - pad, hi.x + pad, lo.y - pad, hi.y + pad, -hi.z - 100.0, -lo.z + SHADOW_REACH) * light_view)
        } else { None };
        let texel = if casters > 0 { ((hi.x - lo.x).max(hi.y - lo.y) + 8.0) / SHADOW_SIZE as f32 } else { 0.0 };
        let uniform = ViewUniform {
            shadow: shadow.unwrap_or(Mat4::IDENTITY).to_cols_array_2d(),
            shadow_params: [shadow.is_some() as i32 as f32, 1.0 / SHADOW_SIZE as f32, texel, 0.0],
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
        let shadow_size = if shadow.is_some() { SHADOW_SIZE } else { 1 };
        let shadow_map = d.create_texture(&wgpu::TextureDescriptor {
            label: Some("shadow"), size: wgpu::Extent3d { width: shadow_size, height: shadow_size, depth_or_array_layers: 1 }, mip_level_count: 1, sample_count: 1,
            dimension: wgpu::TextureDimension::D2, format: DEPTH, usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING, view_formats: &[],
        }).create_view(&Default::default());
        let view_group = d.create_bind_group(&wgpu::BindGroupDescriptor { label: Some("view"), layout: &self.view_layout, entries: &[
            wgpu::BindGroupEntry { binding: 0, resource: view_buffer.as_entire_binding() },
            at(2, &shadow_map),
            wgpu::BindGroupEntry { binding: 3, resource: wgpu::BindingResource::Sampler(&self.compare_sampler) },
        ] });
        // how each object is lit: by the view's lightmaps (1), the probe
        // volume that's chosen for it (2), or neither
        let places: Vec<ProbePlace> = view.probes.iter().map(|v| v.place).collect();
        let lit = |o: &Instance| -> (f32, &Lightmaps) {
            if o.lightmapped && o.model.lightmap_uv { if let Some(l) = &view.lightmaps { return (1.0, l); } }
            let center = o.transform.transform_point3((o.model.min + o.model.max) * 0.5);
            let extents = o.transform.transform_vector3((o.model.max - o.model.min) * 0.5).abs();
            match ProbePlace::choose(&places, o.origin.unwrap_or(center), center, extents) {
                Some(i) => (2.0, &view.probes[i].lighting),
                None => (0.0, &self.unlit),
            }
        };
        let object_groups: Vec<wgpu::BindGroup> = objects.iter().map(|o| {
            let u = ObjectUniform { model: o.transform.to_cols_array_2d(), tint: o.tint, flags: [(!o.bones.is_empty()) as i32 as f32, lit(o).0, 0.0, 0.0] };
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
        let resolved = target(COLOR, 1, wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC | wgpu::TextureUsages::TEXTURE_BINDING);
        let depth = target(DEPTH, SAMPLES, wgpu::TextureUsages::RENDER_ATTACHMENT).create_view(&Default::default());
        // what's drawn over what's behind it, farthest first
        let mut later: Vec<(f32, usize, usize, usize)> = Vec::new();
        for (k, o) in objects.iter().enumerate() {
            let at = o.transform.transform_point3((o.model.min + o.model.max) * 0.5).distance_squared(view.eye);
            for (mi, m) in o.model.meshes.iter().enumerate().filter(|(_, m)| o.shows(m)) {
                for (di, dr) in m.draws.iter().enumerate() {
                    if dr.look(o.skin).1 != Kind::Opaque { later.push((at, k, mi, di)); }
                }
            }
        }
        later.sort_by(|a, b| b.0.total_cmp(&a.0));
        let store = if later.is_empty() { wgpu::StoreOp::Discard } else { wgpu::StoreOp::Store };
        let resolved_view = resolved.create_view(&Default::default());
        let mut encoder = d.create_command_encoder(&Default::default());
        {
            // the sun's shadow map (cleared to nothing in the way when there's nothing to cast)
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("shadow"),
                color_attachments: &[],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment { view: &shadow_map, depth_ops: Some(wgpu::Operations { load: wgpu::LoadOp::Clear(1.0), store: wgpu::StoreOp::Store }), stencil_ops: None }),
                timestamp_writes: None,
                occlusion_query_set: None,
            });
            if shadow.is_some() {
                let shadow_view = d.create_bind_group(&wgpu::BindGroupDescriptor { label: Some("shadow view"), layout: &self.uniform_layout, entries: &[wgpu::BindGroupEntry { binding: 0, resource: view_buffer.as_entire_binding() }] });
                pass.set_pipeline(&self.shadow_pipeline);
                pass.set_bind_group(0, &shadow_view, &[]);
                pass.set_bind_group(1, &self.empty, &[]);
                for (o, group) in objects.iter().zip(&object_groups).filter(|(o, _)| casts(o)) {
                    pass.set_bind_group(2, group, &[]);
                    for m in o.model.meshes.iter().filter(|m| o.shows(m)) {
                        for dr in m.draws.iter().filter(|dr| dr.look(o.skin).1 == Kind::Opaque) {
                            pass.set_vertex_buffer(0, m.vertices[dr.vertex_buffer].slice(..));
                            pass.set_index_buffer(m.indices[dr.index_buffer].slice(..), wgpu::IndexFormat::Uint32);
                            pass.draw_indexed(dr.range.clone(), dr.base_vertex, 0..1);
                        }
                    }
                }
            }
        }
        {
            let [r, g, b, a] = view.clear;
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &msaa,
                    resolve_target: Some(&resolved_view),
                    ops: wgpu::Operations { load: wgpu::LoadOp::Clear(wgpu::Color { r, g, b, a }), store },
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment { view: &depth, depth_ops: Some(wgpu::Operations { load: wgpu::LoadOp::Clear(1.0), store }), stencil_ops: None }),
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
            for (o, group) in objects.iter().zip(&object_groups) {
                pass.set_bind_group(2, group, &[]);
                pass.set_bind_group(3, &lit(o).1.group, &[]);
                for m in o.model.meshes.iter().filter(|m| o.shows(m)) {
                    for d in m.draws.iter().filter(|d| d.look(o.skin).1 == Kind::Opaque) {
                        pass.set_vertex_buffer(0, m.vertices[d.vertex_buffer].slice(..));
                        pass.set_index_buffer(m.indices[d.index_buffer].slice(..), wgpu::IndexFormat::Uint32);
                        pass.set_bind_group(1, d.look(o.skin).0, &[]);
                        pass.draw_indexed(d.range.clone(), d.base_vertex, 0..1);
                    }
                }
            }
        }
        if !later.is_empty() {
            // what's been drawn, for glass to see through, then what's left over it
            let behind = target(COLOR, 1, wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST);
            encoder.copy_texture_to_texture(resolved.as_image_copy(), behind.as_image_copy(), wgpu::Extent3d { width, height, depth_or_array_layers: 1 });
            let glass_view = d.create_bind_group(&wgpu::BindGroupDescriptor { label: Some("glass view"), layout: &self.glass_view_layout, entries: &[
                wgpu::BindGroupEntry { binding: 0, resource: view_buffer.as_entire_binding() },
                at(1, &behind.create_view(&Default::default())),
                at(2, &shadow_map),
                wgpu::BindGroupEntry { binding: 3, resource: wgpu::BindingResource::Sampler(&self.compare_sampler) },
            ] });
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &msaa,
                    resolve_target: Some(&resolved_view),
                    ops: wgpu::Operations { load: wgpu::LoadOp::Load, store: wgpu::StoreOp::Discard },
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment { view: &depth, depth_ops: Some(wgpu::Operations { load: wgpu::LoadOp::Load, store: wgpu::StoreOp::Discard }), stencil_ops: None }),
                timestamp_writes: None,
                occlusion_query_set: None,
            });
            for &(_, k, mi, di) in &later {
                let (o, m) = (&objects[k], &objects[k].model.meshes[mi]);
                let dr = &m.draws[di];
                let (material, kind) = dr.look(o.skin);
                let (pipeline, view_group) = match kind {
                    Kind::Blend => (&self.blend_pipeline, &view_group),
                    Kind::Glass => (&self.glass_pipeline, &glass_view),
                    _ => (&self.cheap_glass_pipeline, &glass_view),
                };
                pass.set_pipeline(pipeline);
                pass.set_bind_group(0, view_group, &[]);
                pass.set_bind_group(1, material, &[]);
                pass.set_bind_group(2, &object_groups[k], &[]);
                pass.set_bind_group(3, &lit(o).1.group, &[]);
                pass.set_vertex_buffer(0, m.vertices[dr.vertex_buffer].slice(..));
                pass.set_index_buffer(m.indices[dr.index_buffer].slice(..), wgpu::IndexFormat::Uint32);
                pass.draw_indexed(dr.range.clone(), dr.base_vertex, 0..1);
            }
        }
        // its exposure: as asked, or from what's been drawn
        let debug = std::env::var("RENDER_DEBUG").ok().and_then(|v| v.parse::<f32>().ok()).unwrap_or(0.0) != 0.0;
        let exposure = match view.exposure {
            Exposure::Fixed(e) => e,
            Exposure::Auto { min, max, compensation } => {
                let row = (width * 8).div_ceil(256) * 256;
                let readback = d.create_buffer(&wgpu::BufferDescriptor { label: None, size: (row * height) as u64, usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ, mapped_at_creation: false });
                encoder.copy_texture_to_buffer(resolved.as_image_copy(), wgpu::TexelCopyBufferInfo { buffer: &readback, layout: wgpu::TexelCopyBufferLayout { offset: 0, bytes_per_row: Some(row), rows_per_image: Some(height) } }, wgpu::Extent3d { width, height, depth_or_array_layers: 1 });
                self.queue.submit([std::mem::replace(&mut encoder, d.create_command_encoder(&Default::default())).finish()]);
                let slice = readback.slice(..);
                slice.map_async(wgpu::MapMode::Read, |_| {});
                let _ = d.poll(wgpu::PollType::Wait);
                let data = slice.get_mapped_range();
                let h = |at: usize| crate::assets::mesh::half(u16::from_le_bytes([data[at], data[at + 1]]));
                // every other pixel that's drawn on (not the clear of a model's frame)
                let mut lum: Vec<f32> = (0..height as usize).step_by(2).flat_map(|y| (0..width as usize).step_by(2).map(move |x| y * row as usize + x * 8))
                    .filter(|&at| h(at + 6) > 0.0)
                    .map(|at| 0.2126 * h(at) + 0.7152 * h(at + 2) + 0.0722 * h(at + 4))
                    .filter(|l| l.is_finite())
                    .collect();
                auto_exposure(&mut lum, min, max, compensation)
            }
        };
        let shown = target(SHOWN, 1, wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC);
        let shown_view = shown.create_view(&Default::default());
        let (out_view, out_format) = target_view.unwrap_or((&shown_view, SHOWN));
        {
            let curve = match view.tonemap { Tonemap::Linear => 0.0, Tonemap::Hable => 1.0, Tonemap::Aces => 2.0, Tonemap::ReinhardJodie => 3.0, Tonemap::Agx => 4.0 };
            let u = d.create_buffer_init(&wgpu::util::BufferInitDescriptor { label: Some("tonemap"), contents: bytemuck::cast_slice(&[exposure, curve, debug as i32 as f32, 0.0]), usage: wgpu::BufferUsages::UNIFORM });
            let group = d.create_bind_group(&wgpu::BindGroupDescriptor { label: Some("tonemap"), layout: &self.tonemap_layout, entries: &[
                wgpu::BindGroupEntry { binding: 0, resource: u.as_entire_binding() },
                at(1, &resolved_view),
            ] });
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("tonemap"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment { view: out_view, resolve_target: None, ops: wgpu::Operations { load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT), store: wgpu::StoreOp::Store } })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });
            pass.set_pipeline(&self.tonemap_pipeline(out_format));
            pass.set_bind_group(0, &group, &[]);
            pass.draw(0..3, 0..1);
        }
        if target_view.is_some() {
            self.queue.submit([encoder.finish()]);
            return Vec::new();
        }
        let resolved = shown;
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
    /// map's own geometry); else by the probe volume chosen for it
    pub lightmapped: bool,
    /// where it's lit from, choosing a probe volume (none: its centre)
    pub origin: Option<Vec3>,
    /// the mesh groups it shows (its body groups' choices; none: its
    /// model's default) and its material group
    pub mask: Option<u64>,
    pub skin: usize,
}

impl Instance<'_> {
    /// Whether it shows this mesh of its model.
    fn shows(&self, m: &GpuMesh) -> bool {
        m.group_mask & self.mask.unwrap_or(self.model.default_mask) != 0
    }
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
    /// its light probe volumes, for everything else
    pub probes: Vec<Arc<ProbeVolume>>,
    /// how it's brought into what a screen shows
    pub tonemap: Tonemap,
    pub exposure: Exposure,
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
    /// `maps`: the light indices, their strengths, the irradiance (a
    /// volume's ambient cubes) and its direction (none for a volume).
    fn group(device: &wgpu::Device, layout: &wgpu::BindGroupLayout, sampler: &wgpu::Sampler, uniform: LightingUniform, maps: [&wgpu::TextureView; 4], lights: &[BakedLight]) -> Lightmaps {
        // every index has a light: black, if none was baked with it
        let mut table = vec![[[0f32; 4]; 3]; 256];
        for l in lights {
            table[l.index as usize] = [
                [l.position.x, l.position.y, l.position.z, if l.sun { 2.0 } else { 1.0 }],
                l.towards.normalize_or_zero().extend(0.0).into(),
                l.color.extend(0.0).into(),
            ];
        }
        let u = device.create_buffer_init(&wgpu::util::BufferInitDescriptor { label: Some("lightmaps"), contents: bytemuck::bytes_of(&uniform), usage: wgpu::BufferUsages::UNIFORM });
        let l = device.create_buffer_init(&wgpu::util::BufferInitDescriptor { label: Some("baked lights"), contents: bytemuck::cast_slice(&table), usage: wgpu::BufferUsages::STORAGE });
        Lightmaps { group: device.create_bind_group(&wgpu::BindGroupDescriptor { label: Some("lightmaps"), layout, entries: &[
            wgpu::BindGroupEntry { binding: 0, resource: u.as_entire_binding() },
            at(1, maps[0]), at(2, maps[1]), at(3, maps[2]), at(4, maps[3]),
            wgpu::BindGroupEntry { binding: 5, resource: l.as_entire_binding() },
            wgpu::BindGroupEntry { binding: 6, resource: wgpu::BindingResource::Sampler(sampler) },
        ] }) }
    }
}

/// What a lighting group's shader is told: the lightmaps' UV scale and
/// size; a probe volume's world to its box (0..1), and its probes' count
/// each way.
#[repr(C)]
#[derive(Clone, Copy, Default, bytemuck::Pod, bytemuck::Zeroable)]
struct LightingUniform {
    lightmap: [f32; 4],
    to_local: [[f32; 4]; 4],
    probes: [f32; 4],
}

/// Where a light probe volume is: its box (in its own space), what places
/// it in the world (rotation and translation), and its priority among the
/// volumes that hold an object.
#[derive(Clone, Copy, Debug)]
pub struct ProbePlace {
    pub transform: Mat4,
    pub mins: Vec3,
    pub maxs: Vec3,
    pub priority: i32,
}

/// A light probe volume on the GPU (Renderer::probe_volume).
pub struct ProbeVolume {
    pub place: ProbePlace,
    lighting: Lightmaps,
}

impl ProbePlace {
    /// The volume that lights an object, as s&box chooses it
    /// (CLightBinner2::ChooseLightProbeVolume, as Sandbox.SceneRenderer's
    /// LightProbeVolume.Choose has it): one holding its origin over one
    /// that doesn't, then the higher priority, then the nearest; one that
    /// doesn't hold it must touch its bounds.
    pub fn choose(volumes: &[ProbePlace], origin: Vec3, center: Vec3, extents: Vec3) -> Option<usize> {
        let to_box = |mins: Vec3, maxs: Vec3, p: Vec3| ((mins - p).max(Vec3::ZERO) + (p - maxs).max(Vec3::ZERO)).length_squared();
        let local = |v: &ProbePlace, p: Vec3| v.transform.inverse().transform_point3(p);
        let (mut best, mut best_priority, mut best_dist, mut best_contains) = (None, i32::MIN, f32::MAX, false);
        for (i, v) in volumes.iter().enumerate() {
            let at = local(v, origin);
            let outside = to_box(v.mins, v.maxs, at);
            let contains = outside == 0.0;
            let dist = if contains { at.length_squared() } else { outside };
            if contains == best_contains {
                if contains && v.priority != best_priority {
                    if v.priority < best_priority { continue; }
                } else if dist > best_dist {
                    continue;
                }
                if !contains && extents.length_squared() < to_box(v.mins, v.maxs, local(v, center)) { continue; }
            } else if !contains {
                continue;
            }
            (best, best_priority, best_dist, best_contains) = (Some(i), v.priority, dist, contains);
        }
        best
    }
}

impl ProbeVolume {
    /// A map's probe volumes, from its entities (`env_light_probe_volume`,
    /// `env_combined_light_probe_volume`, as managed's map loader reads
    /// them): each one's place and textures.
    pub fn from_entities(entities: &[crate::assets::entities::Entity]) -> Vec<(ProbePlace, [String; 3])> {
        let mut out = Vec::new();
        for e in entities {
            let get = |k: &str| e.get(crate::string_token(k));
            if !matches!(get("classname"), Some("env_light_probe_volume" | "env_combined_light_probe_volume")) { continue; }
            let floats = |k: &str, d: [f32; 3]| { let f: Vec<f32> = get(k).map(|v| v.split_whitespace().filter_map(|x| x.parse().ok()).collect()).unwrap_or_default(); Vec3::from(core::array::from_fn::<f32, 3, _>(|i| f.get(i).copied().unwrap_or(d[i]))) };
            let (Some(t), Some(i), Some(s)) = (get("lightprobetexture"), get("lightprobetexture_dli"), get("lightprobetexture_dls")) else { continue };
            let a = floats("angles", [0.0; 3]);
            let rot = glam::Quat::from_rotation_z(a.y.to_radians()) * glam::Quat::from_rotation_y(a.x.to_radians()) * glam::Quat::from_rotation_x(a.z.to_radians());
            out.push((ProbePlace {
                transform: Mat4::from_rotation_translation(rot, floats("origin", [0.0; 3])),
                mins: floats("box_mins", [-72.0; 3]),
                maxs: floats("box_maxs", [72.0; 3]),
                priority: get("indoor_outdoor_level").and_then(|v| v.parse::<f32>().ok()).unwrap_or(0.0) as i32,
            }, [t.to_string(), i.to_string(), s.to_string()]));
        }
        out
    }
}

/// How a frame's light is brought into what a screen shows: s&box's
/// Tonemapping post-process's curves (Hable's filmic its default).
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Tonemap {
    Linear,
    Hable,
    Aces,
    ReinhardJodie,
    Agx,
}

impl Tonemap {
    pub fn named(name: &str) -> Option<Tonemap> {
        Some(match name.to_ascii_lowercase().as_str() {
            "linear" => Tonemap::Linear,
            "hable" | "hablefilmic" | "filmic" => Tonemap::Hable,
            "aces" => Tonemap::Aces,
            "reinhard" | "reinhardjodie" => Tonemap::ReinhardJodie,
            "agx" => Tonemap::Agx,
            _ => return None,
        })
    }
}

/// How bright a frame is shown: a scale, or chosen from the frame as
/// s&box's auto exposure chooses it, within its range (1 to 3 by default),
/// `compensation` stops more or less.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Exposure {
    Fixed(f32),
    Auto { min: f32, max: f32, compensation: f32 },
}

impl Default for Exposure {
    fn default() -> Exposure { Exposure::Auto { min: 1.0, max: 3.0, compensation: 0.0 } }
}

/// The exposure Source's tonemapper settles on for a frame of these
/// luminances (its defaults, mat_tonemap_percent_bright_pixels 2 and
/// mat_tonemap_percent_target 60): what puts the pixel 2% of them are
/// brighter than at 60% of white, within the range. None drawn: 1.
fn auto_exposure(lum: &mut [f32], min: f32, max: f32, compensation: f32) -> f32 {
    if lum.is_empty() { return 1.0; }
    let k = ((lum.len() as f32) * 0.98) as usize;
    let k = k.min(lum.len() - 1);
    let (_, bright, _) = lum.select_nth_unstable_by(k, |a, b| a.total_cmp(b));
    (0.6 / bright.max(1e-4)).clamp(min, max) * 2f32.powf(compensation)
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
            probes: Vec::new(),
            tonemap: Tonemap::Hable,
            exposure: Exposure::default(),
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
        View { eye, view: Mat4::look_at_rh(eye, center, Vec3::Z), proj: Mat4::perspective_rh(fov, 1.0, radius * 0.05, radius * 10.0), clear: [0.0; 4], sun: None, sky: None, sky_rotation: glam::Quat::IDENTITY, lightmaps: None, probes: Vec::new(), tonemap: Tonemap::Hable, exposure: Exposure::default() }
    }
}

#[cfg(test)]
mod tests {
    use glam::{Mat4, Vec3};

    /// s&box's own download cache (`ENGINE2_SAMPLE_ASSETS`): its files at
    /// their game paths, a hash in each name (`x.<hash>.vmdl_c`).
    struct Cache(std::path::PathBuf);

    impl super::Files for Cache {
        fn read(&self, path: &str) -> Option<Vec<u8>> {
            let path = path.replace('\\', "/").to_ascii_lowercase();
            let (dir, file) = path.rsplit_once('/').unwrap_or(("", &path));
            let (stem, ext) = file.split_once('.')?;
            let found = std::fs::read_dir(self.0.join(dir)).ok()?.filter_map(|e| e.ok()).map(|e| e.path()).find(|p| {
                let n = p.file_name().map(|n| n.to_string_lossy().to_ascii_lowercase()).unwrap_or_default();
                n == file || (n.starts_with(&format!("{stem}.")) && n.ends_with(&format!(".{ext}")) && n.matches('.').count() == 2)
            })?;
            std::fs::read(found).ok()
        }
    }

    /// Packages fetched (`ENGINE2_CONTENT`), then the download cache.
    struct Both(super::Content, Cache);

    impl super::Files for Both {
        fn read(&self, path: &str) -> Option<Vec<u8>> { self.0.read(path).or_else(|| self.1.read(path)) }
    }

    /// An object drawn alone, framed; and how many pixels differ from the
    /// same view with nothing in it, and the frame.
    fn alone(r: &super::Renderer, m: &super::Model, mask: Option<u64>, skin: usize) -> (usize, Vec<u8>) {
        let view = super::View::framing(m, 35.0, 15.0);
        let image = r.draw(&[super::Instance { model: m, transform: Mat4::IDENTITY, tint: [1.0; 4], bones: Vec::new(), lightmapped: false, origin: None, mask, skin }], &view, 96, 96);
        let empty = r.draw(&[], &view, 96, 96);
        let covered = image.chunks_exact(4).zip(empty.chunks_exact(4)).filter(|(a, b)| a[..3].iter().zip(&b[..3]).any(|(x, y)| x.abs_diff(*y) > 2)).count();
        (covered, image)
    }

    fn differ(a: &[u8], b: &[u8]) -> usize {
        a.chunks_exact(4).zip(b.chunks_exact(4)).filter(|(a, b)| a[..3].iter().zip(&b[..3]).any(|(x, y)| x.abs_diff(*y) > 8)).count()
    }

    /// A model's skins and body groups, as an object chooses them: the
    /// cola can (three material groups) drawn in each, and the MP5 as it
    /// comes, without its magazine, and with a rail mount.
    #[test]
    fn skins_and_body_groups_are_drawn_as_chosen() {
        let Ok(dir) = std::env::var("ENGINE2_SAMPLE_ASSETS") else { return };
        let Ok(r) = super::Renderer::new() else { eprintln!("no adapter: skipped"); return };
        let dirs: Vec<std::path::PathBuf> = std::env::var("ENGINE2_CONTENT").map(|d| std::env::split_paths(&d).collect()).unwrap_or_default();
        let cache = Both(super::Content(dirs, Vec::new()), Cache(dir.into()));
        use super::Files;
        if cache.read("models/sbox_props/cola_can/cola_can.vmdl_c").is_none() { eprintln!("no cola can: skipped"); return; }
        let can = r.model(&cache, "models/sbox_props/cola_can/cola_can.vmdl").unwrap();
        if !can.notes.is_empty() { eprintln!("the can's textures aren't here (tools/package.sh facepunch.soda_can): skipped"); return; }
        let skins: Vec<(usize, Vec<u8>)> = (0..3).map(|s| alone(&r, &can, None, s)).collect();
        for (i, (c, _)) in skins.iter().enumerate() { eprintln!("can skin {i}: {c} pixels"); }
        assert!(skins.iter().all(|(c, _)| *c == skins[0].0), "the same can, in each");
        assert!(differ(&skins[0].1, &skins[1].1) > skins[0].0 / 4 && differ(&skins[1].1, &skins[2].1) > skins[0].0 / 4, "in another livery");

        let gun = r.model(&cache, "models/weapons/sbox_smg_mp5/w_mp5.vmdl").unwrap();
        // MP5_@0, Magazine_@0, Magazine_@1, top_rail_mount_@0, top_rail_mount_@1:
        // its body, a magazine or none, no rail mount or one (its meshes are
        // in 1, 2 and 16; the other choices are empty)
        assert_eq!(gun.default_mask, 0b01011);
        assert_eq!(gun.meshes.iter().map(|m| m.group_mask).collect::<Vec<_>>(), [1, 2, 16]);
        let (default, d) = alone(&r, &gun, None, 0);
        let (empty, e) = alone(&r, &gun, Some(0b01101), 0);
        let (mounted, m) = alone(&r, &gun, Some(0b10011), 0);
        eprintln!("mp5: as it comes {default} pixels, without its magazine {empty}, with the rail mount {mounted}");
        assert!(empty < default && differ(&d, &e) > 20, "its magazine taken out");
        assert!(mounted > default && differ(&d, &m) > 20, "a rail mount put on");
    }

    /// A mesh made at runtime (rendermesh.rs, as managed's Mesh makes an
    /// editor mesh's faces) is drawn as a file's is: a floor 64 square,
    /// looked down on, fills the middle of the frame and not its corners.
    /// With `ENGINE2_CONTENT` naming flatgrass's materials it's drawn in
    /// one of them, found and read.
    #[test]
    fn a_mesh_built_at_runtime_is_drawn() {
        let Ok(r) = super::Renderer::new() else { eprintln!("no adapter: skipped"); return };
        use crate::rendermesh as rm;
        let l = rm::new_layout(32);
        for (s, f, o) in [("position", 6, 0), ("normal", 6, 12), ("texcoord", 16, 24)] { rm::add_field(l, s, 0, f, o); }
        let floor: Vec<f32> = [[-32.0, -32.0], [32.0, -32.0], [32.0, 32.0], [-32.0, 32.0]].iter().flat_map(|p| [p[0], p[1], 0.0, 0.0, 0.0, 1.0, p[0] / 64.0, p[1] / 64.0]).collect();
        let vb = unsafe { rm::new_vertices(4, &rm::layout_of(l).unwrap(), floor.as_ptr() as usize) };
        let idx: [i32; 6] = [0, 1, 2, 0, 2, 3];
        let ib = unsafe { rm::new_indices(6, true, idx.as_ptr() as usize, 6) };
        let mesh = rm::new_mesh(0, rm::TRIANGLES, "floor");
        unsafe { rm::give(mesh, vb, 0, 0); rm::give(mesh, ib, 0, 0); }

        let dirs: Vec<std::path::PathBuf> = std::env::var("ENGINE2_CONTENT").map(|d| std::env::split_paths(&d).collect()).unwrap_or_default();
        let content = super::Content(dirs, Vec::new());
        use super::Files;
        let material = if content.read("materials/dev/reflectivity_30.vmat_c").is_some() { "materials/dev/reflectivity_30.vmat" } else { "" };
        let meshes = rm::for_drawing(&[mesh], &[], |_| material.to_string());
        let m = r.model_of(&content, "a floor", &meshes, u64::MAX, &[]).unwrap();
        if !material.is_empty() { assert!(m.notes.is_empty(), "{:?}", m.notes); }
        assert_eq!((m.min, m.max), (Vec3::new(-32.0, -32.0, 0.0), Vec3::new(32.0, 32.0, 0.0)));
        let view = super::View::camera(Vec3::new(0.0, 0.0, 100.0), [90.0, 0.0, 0.0], 60.0, 1.0, 1000.0, 1.0);
        let image = r.draw(&[super::Instance { model: &m, transform: Mat4::IDENTITY, tint: [1.0; 4], bones: Vec::new(), lightmapped: false, origin: None, mask: None, skin: 0 }], &view, 64, 64);
        // against the same view with nothing in it (the background is opaque)
        let empty = r.draw(&[], &view, 64, 64);
        let differs = |i: usize| image[i * 4..i * 4 + 3].iter().zip(&empty[i * 4..i * 4 + 3]).any(|(a, b)| a.abs_diff(*b) > 2);
        let covered = (0..64 * 64).filter(|&i| differs(i)).count();
        eprintln!("{covered} of {} pixels covered; middle {:?}, corner {:?}", 64 * 64, &image[(32 * 64 + 32) * 4..][..4], &image[(64 + 1) * 4..][..4]);
        // the floor spans 64 of the 115 the view sees at 100 away: the middle third or so
        assert!(differs(32 * 64 + 32), "the middle is the floor");
        assert!(!differs(64 + 1), "a corner isn't");
        assert!(covered > 64 * 64 / 5 && covered < 64 * 64 / 2, "{covered}");
    }

    /// Source's auto exposure: the pixel 2% are brighter than goes to 60%
    /// of white, within s&box's range (1 to 3).
    #[test]
    fn a_frame_is_exposed_as_source_exposes_it() {
        // 98 of 100 pixels at 0.1, two at 1: the 98th is 0.4 (0.6 / 0.4)
        let mut lum: Vec<f32> = (0..100).map(|i| if i < 97 { 0.1 } else { 0.4 }).collect();
        assert!((super::auto_exposure(&mut lum, 1.0, 3.0, 0.0) - 1.5).abs() < 1e-5);
        assert_eq!(super::auto_exposure(&mut vec![0.05; 100], 1.0, 3.0, 0.0), 3.0, "a dark frame: as bright as it may be");
        assert_eq!(super::auto_exposure(&mut vec![5.0; 100], 1.0, 3.0, 0.0), 1.0, "a bright one: never below 1");
        assert_eq!(super::auto_exposure(&mut vec![5.0; 100], 1.0, 3.0, 1.0), 2.0, "a stop more");
        assert_eq!(super::auto_exposure(&mut [], 1.0, 3.0, 0.0), 1.0, "nothing drawn");
    }

    /// As s&box's own test of it (BakedLightTests.LightProbeVolumeIsChosenAsNativeChoosesIt).
    #[test]
    fn a_probe_volume_is_chosen_as_native_chooses_it() {
        let volume = |x: f32, size: f32, priority: i32| super::ProbePlace { transform: Mat4::from_translation(Vec3::new(x, 0.0, 0.0)), mins: Vec3::splat(-size), maxs: Vec3::splat(size), priority };
        let volumes = [volume(0.0, 100.0, 0), volume(50.0, 20.0, 1), volume(300.0, 50.0, 0)];
        let at = |x: f32| Vec3::new(x, 0.0, 0.0);
        let choose = |x: f32, extents: f32| super::ProbePlace::choose(&volumes, at(x), at(x), Vec3::splat(extents));
        assert_eq!(choose(0.0, 1.0), Some(0), "inside only the outdoor one");
        assert_eq!(choose(50.0, 1.0), Some(1), "inside both: the higher priority");
        assert_eq!(choose(245.0, 10.0), Some(2), "inside none: the nearest it touches");
        assert_eq!(choose(200.0, 1.0), None, "inside none, touching none");
    }

    /// Flatgrass (ENGINE2_SAMPLE_MAP its pak; ENGINE2_CONTENT its
    /// materials and the oil drum from sbox.game), if this machine has it:
    /// a drum stood on its sunlit pavement shadows the ground on the far
    /// side of it from the sun, and not the near side; the map's own
    /// objects cast nothing (their shadows are baked).
    #[test]
    fn what_moves_casts_the_suns_shadow() {
        use super::Files;
        let (Ok(map), Ok(dirs)) = (std::env::var("ENGINE2_SAMPLE_MAP"), std::env::var("ENGINE2_CONTENT")) else { return };
        let content = super::Content::of(std::iter::once(map.into()).chain(dirs.split(':').map(Into::into)));
        if content.read("models/sbox_props/oil_drum/oil_drum_a.vmdl_c").is_none() || content.read("maps/flatgrass/world.vwrld_c").is_none() { return; }
        let Ok(r) = super::Renderer::new() else { eprintln!("no adapter: skipped"); return };
        let lump = content.compiled("maps/flatgrass/entities/default_ents.vents").and_then(|r| r.kv3("DATA")?.ok()).map(|d| crate::assets::entities::read_lump(&d)).unwrap();
        let lights = super::BakedLight::from_entities(&lump);
        let sun = lights.iter().find(|l| l.sun).unwrap();
        let mut view = super::View::camera(Vec3::new(-560.0, 560.0, 400.0), [89.9, 0.0, 0.0], 60.0, 4.0, 40000.0, 1.0);
        view.sun = Some((sun.towards, sun.color));
        view.lightmaps = r.lightmaps(&content, "maps/flatgrass", &lights).unwrap().map(std::sync::Arc::new);
        // one exposure for both frames
        view.exposure = super::Exposure::Fixed(1.0);
        let world: Vec<(super::Model, Mat4)> = crate::assets::worldnode::scene_objects("maps/flatgrass", |p| content.read(p)).into_iter().filter_map(|(name, rows, _)| {
            let m = r.model(&content, &name).ok()?;
            Some((m, Mat4::from_cols_array_2d(&[[rows[0][0], rows[1][0], rows[2][0], 0.0], [rows[0][1], rows[1][1], rows[2][1], 0.0], [rows[0][2], rows[1][2], rows[2][2], 0.0], [rows[0][3], rows[1][3], rows[2][3], 1.0]])))
        }).collect();
        let drum = r.model(&content, "models/sbox_props/oil_drum/oil_drum_a.vmdl").unwrap();
        let mut objects: Vec<super::Instance> = world.iter().map(|(m, t)| super::Instance { model: m, transform: *t, tint: [1.0; 4], bones: Vec::new(), lightmapped: true, origin: None, mask: None, skin: 0 }).collect();
        let size = 256;
        let luma = |image: &[u8], p: Vec3| {
            let c = (view.proj * view.view).project_point3(p);
            let (x, y) = (((c.x * 0.5 + 0.5) * size as f32) as usize, ((0.5 - c.y * 0.5) * size as f32) as usize);
            image[(y * size + x) * 4..][..3].iter().map(|&v| v as f32).sum::<f32>() / 3.0
        };
        // the far side from the sun (its shadow falls along -towards) and the near side
        let away = Vec3::new(-sun.towards.x, -sun.towards.y, 0.0).normalize();
        let (behind, before) = (Vec3::new(-560.0, 560.0, 4.0) + away * 30.0, Vec3::new(-560.0, 560.0, 4.0) - away * 30.0);
        let bare = r.draw(&objects, &view, size as u32, size as u32);
        objects.push(super::Instance { model: &drum, transform: Mat4::from_translation(Vec3::new(-560.0, 560.0, 4.0)), tint: [1.0; 4], bones: Vec::new(), lightmapped: false, origin: None, mask: None, skin: 0 });
        let with = r.draw(&objects, &view, size as u32, size as u32);
        let (b0, f0, b1, f1) = (luma(&bare, behind), luma(&bare, before), luma(&with, behind), luma(&with, before));
        eprintln!("without the drum: far side {b0:.0}, near side {f0:.0}; with it: {b1:.0}, {f1:.0}");
        assert!(b0 > f0 * 0.6, "sunlit pavement either side");
        assert!(b1 < b0 * 0.6, "its shadow behind it");
        assert!((f1 - f0).abs() < 10.0, "none before it");
    }

    /// The microwave from sbox.game (tools/package.sh facepunch.microwave),
    /// if this machine has it: its window is glass, drawn over what's
    /// behind it (its inside), seen through, darker than the white body.
    #[test]
    fn a_window_is_glass() {
        let Ok(dir) = std::env::var("ENGINE2_CONTENT") else { return };
        let content = super::Content(vec![dir.into()], Vec::new());
        use super::Files;
        if content.read("models/props/microwave/microwave.vmdl_c").is_none() { return; }
        let Ok(r) = super::Renderer::new() else { eprintln!("no adapter: skipped"); return };
        let m = r.model(&content, "models/props/microwave/microwave.vmdl").unwrap();
        let kinds: Vec<super::Kind> = m.meshes.iter().flat_map(|x| x.draws.iter().map(|d| d.kind)).collect();
        eprintln!("{kinds:?}");
        assert!(kinds.contains(&super::Kind::Glass), "its window");
        let image = r.draw(&[super::Instance { model: &m, transform: glam::Mat4::IDENTITY, tint: [1.0; 4], bones: Vec::new(), lightmapped: false, origin: None, mask: None, skin: 0 }], &super::View::framing(&m, 0.0, 0.0), 128, 128);
        assert!(image.chunks_exact(4).filter(|p| p[3] > 0).count() > 128 * 128 / 4, "it fills the frame");
    }

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
        let image = r.draw(&[super::Instance { model: &m, transform: glam::Mat4::IDENTITY, tint: [1.0; 4], bones: Vec::new(), lightmapped: false, origin: None, mask: None, skin: 0 }], &super::View::framing(&m, 35.0, 15.0), 128, 128);
        let covered: Vec<&[u8]> = image.chunks_exact(4).filter(|p| p[3] > 0).collect();
        let mean = covered.iter().map(|p| p[0] as f32 + p[1] as f32 + p[2] as f32).sum::<f32>() / (3 * covered.len()) as f32;
        eprintln!("{} of {} pixels covered, mean {mean:.0}", covered.len(), 128 * 128);
        assert!(covered.len() > 128 * 128 / 4, "it fills the frame");
        assert!(mean > 15.0 && mean < 110.0, "a black television, lit: {mean}");
    }
}
