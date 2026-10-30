//! What a client draws, begun: a model as s&box compiles it (its meshes,
//! the materials they name, those materials' textures), read by engine2's
//! readers and drawn with wgpu (Vulkan here, WebGPU in a browser), into
//! an image. Each material is drawn as `complex.shader` would draw it,
//! as near as we get yet (shader.wgsl): nearly every material s&box's
//! content has is one of its.
use std::path::PathBuf;

use engine2::assets::{material, mesh, resource::Resource, texture};
use glam::{Mat4, Vec3};
use wgpu::util::DeviceExt;

/// Where content is: folders that hold files at their game paths (a
/// package's, as tools/package.sh fetches them).
pub struct Content(pub Vec<PathBuf>);

impl Content {
    pub fn read(&self, path: &str) -> Option<Vec<u8>> {
        let path = path.replace('\\', "/");
        let path = path.trim_start_matches('/');
        self.0.iter().find_map(|d| std::fs::read(d.join(path)).ok())
    }

    /// A compiled resource by its name (`x.vmat` is `x.vmat_c`).
    pub fn compiled(&self, name: &str) -> Option<Resource> {
        let name = if name.ends_with("_c") { name.to_string() } else { format!("{name}_c") };
        Resource::parse(self.read(&name)?).ok()
    }
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
}

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
struct ViewUniform {
    view_proj: [[f32; 4]; 4],
    eye: [f32; 4],
    sun: [f32; 4],
    sun_color: [f32; 4],
    sky: [f32; 4],
    ground: [f32; 4],
}

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
struct MaterialUniform {
    tint: [f32; 4],
    params: [f32; 4],
    mode: [f32; 4],
    tint_b: [f32; 4],
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
    /// what was missing or couldn't be read, for whoever's looking
    pub notes: Vec<String>,
}

pub struct Renderer {
    device: wgpu::Device,
    queue: wgpu::Queue,
    pipeline: wgpu::RenderPipeline,
    view_layout: wgpu::BindGroupLayout,
    material_layout: wgpu::BindGroupLayout,
    sampler: wgpu::Sampler,
    /// for a material without a texture: white (a colour, occlusion, a
    /// tint mask), a flat normal (roughness 0.6), and a blendable's
    /// metalness, occlusion and roughness (not metal, not occluded, 0.6)
    white: wgpu::TextureView,
    flat: wgpu::TextureView,
    unoccluded: wgpu::TextureView,
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
        let (device, queue) = pollster::block_on(adapter.request_device(&wgpu::DeviceDescriptor::default())).map_err(|e| e.to_string())?;
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor { label: Some("complex"), source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()) });
        let uniform = |binding, visibility| wgpu::BindGroupLayoutEntry { binding, visibility, ty: wgpu::BindingType::Buffer { ty: wgpu::BufferBindingType::Uniform, has_dynamic_offset: false, min_binding_size: None }, count: None };
        let tex = |binding| wgpu::BindGroupLayoutEntry { binding, visibility: wgpu::ShaderStages::FRAGMENT, ty: wgpu::BindingType::Texture { sample_type: wgpu::TextureSampleType::Float { filterable: true }, view_dimension: wgpu::TextureViewDimension::D2, multisampled: false }, count: None };
        let view_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor { label: Some("view"), entries: &[uniform(0, wgpu::ShaderStages::VERTEX_FRAGMENT)] });
        let material_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor { label: Some("material"), entries: &[
            uniform(0, wgpu::ShaderStages::FRAGMENT), tex(1), tex(2),
            wgpu::BindGroupLayoutEntry { binding: 3, visibility: wgpu::ShaderStages::FRAGMENT, ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering), count: None },
            tex(4), tex(5), tex(6), tex(7), tex(8),
        ] });
        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor { label: None, bind_group_layouts: &[&view_layout, &material_layout], push_constant_ranges: &[] });
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
                    attributes: &wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3, 2 => Float32x2, 3 => Float32x4, 4 => Float32x4],
                }],
            },
            fragment: Some(wgpu::FragmentState { module: &shader, entry_point: Some("fs_main"), compilation_options: Default::default(), targets: &[Some(COLOR.into())] }),
            primitive: wgpu::PrimitiveState { cull_mode: None, ..Default::default() },
            depth_stencil: Some(wgpu::DepthStencilState { format: DEPTH, depth_write_enabled: true, depth_compare: wgpu::CompareFunction::Less, stencil: Default::default(), bias: Default::default() }),
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
        let (white, flat, unoccluded) = (pixel([255; 4]), pixel([128, 128, 153, 255]), pixel([0, 255, 153, 255]));
        Ok(Renderer { device, queue, pipeline, view_layout, material_layout, sampler, white, flat, unoccluded, adapter: format!("{} ({:?})", info.name, info.backend) })
    }

    /// A texture by name, every mip the file holds, on the GPU.
    fn texture(&self, content: &Content, name: &str, srgb: bool) -> Result<wgpu::TextureView, String> {
        let t = texture::read(content.compiled(name).ok_or_else(|| format!("{name}: not found"))?)?;
        let top = t.available(0).ok_or_else(|| format!("{name}: no mips"))?;
        let (w, h) = t.size(top);
        let levels = (top..t.mips).map(|m| t.rgba(m)).collect::<Result<Vec<_>, _>>()?;
        let gpu = self.device.create_texture(&wgpu::TextureDescriptor {
            label: Some(name), size: wgpu::Extent3d { width: w as u32, height: h as u32, depth_or_array_layers: 1 }, mip_level_count: levels.len() as u32, sample_count: 1,
            dimension: wgpu::TextureDimension::D2, format: if srgb { wgpu::TextureFormat::Rgba8UnormSrgb } else { wgpu::TextureFormat::Rgba8Unorm },
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST, view_formats: &[],
        });
        for (i, (lw, lh, px)) in levels.iter().enumerate() {
            self.queue.write_texture(
                wgpu::TexelCopyTextureInfo { texture: &gpu, mip_level: i as u32, origin: wgpu::Origin3d::ZERO, aspect: wgpu::TextureAspect::All },
                px,
                wgpu::TexelCopyBufferLayout { offset: 0, bytes_per_row: Some(*lw as u32 * 4), rows_per_image: Some(*lh as u32) },
                wgpu::Extent3d { width: *lw as u32, height: *lh as u32, depth_or_array_layers: 1 },
            );
        }
        Ok(gpu.create_view(&Default::default()))
    }

    /// A material's bind group, and whether it's drawn at all.
    fn material(&self, content: &Content, name: &str, notes: &mut Vec<String>) -> (wgpu::BindGroup, bool) {
        let m = content.compiled(name).ok_or_else(|| format!("{name}: not found")).and_then(|r| material::read(&r));
        let m = m.unwrap_or_else(|e| { notes.push(e); Default::default() });
        let mut tex = |param: &str, srgb: bool, default: &wgpu::TextureView| match m.textures.get(param) {
            Some(t) => self.texture(content, t, srgb).unwrap_or_else(|e| { notes.push(e); default.clone() }),
            None => default.clone(),
        };
        let blendable = m.shader.contains("blendable");
        let views = if blendable {
            [tex("g_tColorA", true, &self.white), tex("g_tRmaA", false, &self.unoccluded), tex("g_tNormalA", false, &self.flat),
             tex("g_tColorB", true, &self.white), tex("g_tNormalB", false, &self.flat), tex("g_tRmaB", false, &self.unoccluded), self.white.clone()]
        } else {
            [tex("g_tColor", true, &self.white), tex("g_tAmbientOcclusion", false, &self.white), tex("g_tNormal", false, &self.flat),
             self.white.clone(), self.flat.clone(), self.white.clone(), tex("g_tTintMask", false, &self.white)]
        };
        let has_normal = m.textures.keys().any(|t| t.starts_with("g_tNormal"));
        let (tint, tint_b) = if blendable {
            (m.vector("g_flTintColorA", [1.0; 4]), m.vector("g_flTintColorB", [1.0; 4]))
        } else {
            (m.vector("g_vColorTint", [1.0; 4]), [1.0; 4])
        };
        let alpha_test = if m.int("F_ALPHA_TEST") != 0 { m.float("g_flAlphaTestReference", 0.5) } else { 0.0 };
        let flag = |b: bool| b as i32 as f32;
        let uniform = MaterialUniform {
            tint: [tint[0], tint[1], tint[2], 1.0],
            params: [alpha_test, m.float("g_flMetalness", 0.0), flag(m.int("F_METALNESS_TEXTURE") != 0), flag(has_normal)],
            mode: [flag(blendable), flag(m.int("F_TINT_MASK") != 0), flag(blendable || m.int("F_PAINT_VERTEX_COLORS") != 0), 0.0],
            tint_b: [tint_b[0], tint_b[1], tint_b[2], 1.0],
        };
        let buffer = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor { label: Some(name), contents: bytemuck::bytes_of(&uniform), usage: wgpu::BufferUsages::UNIFORM });
        let group = self.device.create_bind_group(&wgpu::BindGroupDescriptor { label: Some(name), layout: &self.material_layout, entries: &[
            wgpu::BindGroupEntry { binding: 0, resource: buffer.as_entire_binding() },
            wgpu::BindGroupEntry { binding: 3, resource: wgpu::BindingResource::Sampler(&self.sampler) },
            at(1, &views[0]), at(2, &views[1]), at(4, &views[2]), at(5, &views[3]), at(6, &views[4]), at(7, &views[5]), at(8, &views[6]),
        ] });
        // translucent and decal materials wait for their own pass
        let drawn = !(m.int("F_TRANSLUCENT") != 0 || m.int("F_DECAL") != 0 || m.shader.contains("glass"));
        (group, drawn)
    }

    /// A model (`models/x.vmdl`) read from `content` onto the GPU.
    pub fn model(&self, content: &Content, name: &str) -> Result<Model, String> {
        let r = content.compiled(name).ok_or_else(|| format!("{name}: not found"))?;
        let meshes = mesh::model(&r, |n| content.read(&if n.ends_with("_c") { n.to_string() } else { format!("{n}_c") }))?;
        let mut notes = Vec::new();
        let (mut min, mut max) = (Vec3::splat(f32::MAX), Vec3::splat(f32::MIN));
        let mut out = Vec::new();
        for m in &meshes {
            let mut vertices = Vec::new();
            for vb in &m.buffers.vertices {
                let positions = vb.floats("POSITION", 0).ok_or("no POSITION")?;
                let normals = vb.normals().unwrap_or_else(|| vec![[0.0, 0.0, 1.0]; vb.count]);
                let uvs = vb.floats("TEXCOORD", 0).unwrap_or_else(|| vec![[0.0; 4]; vb.count]);
                let blend = vb.floats("TEXCOORD", 4).unwrap_or_else(|| vec![[0.0; 4]; vb.count]);
                let paint = vb.floats("TEXCOORD", 5).unwrap_or_else(|| vec![[1.0; 4]; vb.count]);
                let v: Vec<Vertex> = (0..vb.count).map(|i| {
                    let p = positions[i];
                    min = min.min(Vec3::new(p[0], p[1], p[2]));
                    max = max.max(Vec3::new(p[0], p[1], p[2]));
                    Vertex { position: [p[0], p[1], p[2]], normal: normals[i], uv: [uvs[i][0], uvs[i][1]], blend: blend[i], paint: paint[i] }
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
        Ok(Model { meshes: out, min, max, notes })
    }

    /// The model drawn from `yaw` and `pitch` degrees round it (0, 0: from
    /// its front, +x), framed: an RGBA image, `size` square.
    pub fn draw(&self, model: &Model, size: u32, yaw: f32, pitch: f32) -> Vec<u8> {
        let center = (model.min + model.max) * 0.5;
        let radius = (model.max - model.min).length() * 0.5;
        let fov = 30f32.to_radians();
        let dir = Vec3::new(yaw.to_radians().cos() * pitch.to_radians().cos(), yaw.to_radians().sin() * pitch.to_radians().cos(), pitch.to_radians().sin());
        let eye = center + dir * (radius / (fov * 0.5).sin()) * 1.05;
        let proj = Mat4::perspective_rh(fov, 1.0, radius * 0.05, radius * 10.0) * Mat4::look_at_rh(eye, center, Vec3::Z);
        let sun = Vec3::new(0.6, 0.35, 0.75).normalize();
        let view = ViewUniform {
            view_proj: proj.to_cols_array_2d(),
            eye: eye.extend(1.0).into(),
            sun: sun.extend(0.0).into(),
            sun_color: [1.6, 1.55, 1.45, 0.0],
            sky: [0.55, 0.6, 0.7, 0.0],
            // RENDER_DEBUG: 1 base colour, 2 normal, 3 occlusion, 4 roughness
            ground: [0.25, 0.23, 0.2, std::env::var("RENDER_DEBUG").ok().and_then(|v| v.parse().ok()).unwrap_or(0.0)],
        };
        let d = &self.device;
        let view_buffer = d.create_buffer_init(&wgpu::util::BufferInitDescriptor { label: Some("view"), contents: bytemuck::bytes_of(&view), usage: wgpu::BufferUsages::UNIFORM });
        let view_group = d.create_bind_group(&wgpu::BindGroupDescriptor { label: Some("view"), layout: &self.view_layout, entries: &[wgpu::BindGroupEntry { binding: 0, resource: view_buffer.as_entire_binding() }] });
        let target = |format, samples, usage| d.create_texture(&wgpu::TextureDescriptor {
            label: None, size: wgpu::Extent3d { width: size, height: size, depth_or_array_layers: 1 }, mip_level_count: 1, sample_count: samples,
            dimension: wgpu::TextureDimension::D2, format, usage, view_formats: &[],
        });
        let msaa = target(COLOR, SAMPLES, wgpu::TextureUsages::RENDER_ATTACHMENT).create_view(&Default::default());
        let resolved = target(COLOR, 1, wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC);
        let depth = target(DEPTH, SAMPLES, wgpu::TextureUsages::RENDER_ATTACHMENT).create_view(&Default::default());
        let mut encoder = d.create_command_encoder(&Default::default());
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &msaa,
                    resolve_target: Some(&resolved.create_view(&Default::default())),
                    ops: wgpu::Operations { load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT), store: wgpu::StoreOp::Discard },
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment { view: &depth, depth_ops: Some(wgpu::Operations { load: wgpu::LoadOp::Clear(1.0), store: wgpu::StoreOp::Discard }), stencil_ops: None }),
                timestamp_writes: None,
                occlusion_query_set: None,
            });
            pass.set_pipeline(&self.pipeline);
            pass.set_bind_group(0, &view_group, &[]);
            for m in &model.meshes {
                for d in &m.draws {
                    pass.set_vertex_buffer(0, m.vertices[d.vertex_buffer].slice(..));
                    pass.set_index_buffer(m.indices[d.index_buffer].slice(..), wgpu::IndexFormat::Uint32);
                    pass.set_bind_group(1, &d.material, &[]);
                    pass.draw_indexed(d.range.clone(), d.base_vertex, 0..1);
                }
            }
        }
        // read it back, a row at a time as copies align them
        let row = (size * 4).div_ceil(256) * 256;
        let readback = d.create_buffer(&wgpu::BufferDescriptor { label: None, size: (row * size) as u64, usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ, mapped_at_creation: false });
        encoder.copy_texture_to_buffer(
            wgpu::TexelCopyTextureInfo { texture: &resolved, mip_level: 0, origin: wgpu::Origin3d::ZERO, aspect: wgpu::TextureAspect::All },
            wgpu::TexelCopyBufferInfo { buffer: &readback, layout: wgpu::TexelCopyBufferLayout { offset: 0, bytes_per_row: Some(row), rows_per_image: Some(size) } },
            wgpu::Extent3d { width: size, height: size, depth_or_array_layers: 1 },
        );
        self.queue.submit([encoder.finish()]);
        let slice = readback.slice(..);
        slice.map_async(wgpu::MapMode::Read, |_| {});
        let _ = d.poll(wgpu::PollType::Wait);
        let data = slice.get_mapped_range();
        (0..size as usize).flat_map(|y| data[y * row as usize..y * row as usize + size as usize * 4].to_vec()).collect()
    }
}

fn at(binding: u32, view: &wgpu::TextureView) -> wgpu::BindGroupEntry<'_> {
    wgpu::BindGroupEntry { binding, resource: wgpu::BindingResource::TextureView(view) }
}

/// An RGBA image as a PNG.
pub fn png(path: &std::path::Path, size: u32, rgba: &[u8]) -> Result<(), String> {
    let file = std::fs::File::create(path).map_err(|e| e.to_string())?;
    let mut e = png::Encoder::new(std::io::BufWriter::new(file), size, size);
    e.set_color(png::ColorType::Rgba);
    e.set_depth(png::BitDepth::Eight);
    e.write_header().and_then(|mut w| w.write_image_data(rgba)).map_err(|e| e.to_string())
}

#[cfg(test)]
mod tests {
    /// The TV from sbox.game (tools/package.sh facepunch.tv), if this
    /// machine has it and a GPU (or lavapipe): drawn, it's there, and dark.
    #[test]
    fn a_downloaded_model_is_drawn() {
        let Ok(dir) = std::env::var("ENGINE2_CONTENT") else { return };
        let content = super::Content(vec![dir.into()]);
        if content.read("models/sbox_props/tv/tv.vmdl_c").is_none() { return; }
        let Ok(r) = super::Renderer::new() else { eprintln!("no adapter: skipped"); return };
        let m = r.model(&content, "models/sbox_props/tv/tv.vmdl").unwrap();
        assert!(m.notes.is_empty(), "{:?}", m.notes);
        let image = r.draw(&m, 128, 35.0, 15.0);
        let covered: Vec<&[u8]> = image.chunks_exact(4).filter(|p| p[3] > 0).collect();
        let mean = covered.iter().map(|p| p[0] as f32 + p[1] as f32 + p[2] as f32).sum::<f32>() / (3 * covered.len()) as f32;
        eprintln!("{} of {} pixels covered, mean {mean:.0}", covered.len(), 128 * 128);
        assert!(covered.len() > 128 * 128 / 4, "it fills the frame");
        assert!(mean > 15.0 && mean < 110.0, "a black television, lit: {mean}");
    }
}
