//! Sprite batches drawn (sprites.wgsl, after s&box's sprite_ps): each
//! batch's sprites as crate::sprites read them back, in their order, one
//! instanced quad each, into the scene's colour (linear HDR) over what's
//! been drawn, tested against its depth. A sprite names its texture by its
//! bindless index; a draw binds eight, so a batch that names more is drawn
//! in runs, in order.
use std::collections::HashMap;
use std::sync::Mutex;

use wgpu::util::DeviceExt;

use crate::sprites::{at, Batch, STRIDE, TEXT};

/// What a frame gives its sprites: the camera, and the light a lit sprite takes.
#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct SpriteView {
    pub view_proj: [[f32; 4]; 4],
    pub eye: [f32; 4],
    pub right: [f32; 4],
    pub up: [f32; 4],
    pub forward: [f32; 4],
    pub sun_color: [f32; 4],
    pub ambient: [f32; 4],
}

const SLOTS: usize = 8;

pub struct Sprites {
    shader: wgpu::ShaderModule,
    view_layout: wgpu::BindGroupLayout,
    texture_layout: wgpu::BindGroupLayout,
    linear: wgpu::Sampler,
    point: wgpu::Sampler,
    /// by (added rather than blended, writes depth, samples a pixel)
    pipelines: Mutex<HashMap<(bool, bool, u32), wgpu::RenderPipeline>>,
    /// each texture on the GPU as sRGB, by its object, and the version of it
    /// (none: it couldn't be read, and isn't looked for again)
    textures: Mutex<HashMap<usize, (Option<u64>, Option<wgpu::TextureView>)>>,
    white: wgpu::TextureView,
    /// what stands in for a texture that isn't found: a soft round glow
    glow: wgpu::TextureView,
}

impl Sprites {
    pub fn new(device: &wgpu::Device, queue: &wgpu::Queue) -> Sprites {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor { label: Some("sprites"), source: wgpu::ShaderSource::Wgsl(include_str!("sprites.wgsl").into()) });
        let stage = wgpu::ShaderStages::VERTEX_FRAGMENT;
        let view_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor { label: Some("sprites"), entries: &[
            wgpu::BindGroupLayoutEntry { binding: 0, visibility: stage, ty: wgpu::BindingType::Buffer { ty: wgpu::BufferBindingType::Uniform, has_dynamic_offset: false, min_binding_size: None }, count: None },
            wgpu::BindGroupLayoutEntry { binding: 1, visibility: stage, ty: wgpu::BindingType::Buffer { ty: wgpu::BufferBindingType::Storage { read_only: true }, has_dynamic_offset: false, min_binding_size: None }, count: None },
            wgpu::BindGroupLayoutEntry { binding: 2, visibility: wgpu::ShaderStages::FRAGMENT, ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering), count: None },
            wgpu::BindGroupLayoutEntry { binding: 3, visibility: wgpu::ShaderStages::FRAGMENT, ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering), count: None },
        ] });
        let texture_entries: Vec<wgpu::BindGroupLayoutEntry> = (0..SLOTS as u32).map(|b| wgpu::BindGroupLayoutEntry {
            binding: b, visibility: wgpu::ShaderStages::FRAGMENT,
            ty: wgpu::BindingType::Texture { sample_type: wgpu::TextureSampleType::Float { filterable: true }, view_dimension: wgpu::TextureViewDimension::D2, multisampled: false }, count: None,
        }).collect();
        let texture_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor { label: Some("sprite textures"), entries: &texture_entries });
        let sampler = |filter| device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge, address_mode_v: wgpu::AddressMode::ClampToEdge,
            mag_filter: filter, min_filter: filter, mipmap_filter: filter, ..Default::default()
        });
        let white = device.create_texture_with_data(queue, &wgpu::TextureDescriptor {
            label: Some("sprite white"), size: wgpu::Extent3d { width: 1, height: 1, depth_or_array_layers: 1 }, mip_level_count: 1, sample_count: 1,
            dimension: wgpu::TextureDimension::D2, format: wgpu::TextureFormat::Rgba8UnormSrgb, usage: wgpu::TextureUsages::TEXTURE_BINDING, view_formats: &[],
        }, wgpu::util::TextureDataOrder::LayerMajor, &[255; 4]).create_view(&Default::default());
        // white, its alpha falling from 1 in the middle to 0 at the edge as (1 - r)^2
        let glow_px: Vec<u8> = (0..64 * 64).flat_map(|i| {
            let (x, y) = ((i % 64) as f32 + 0.5 - 32.0, (i / 64) as f32 + 0.5 - 32.0);
            let r = ((x * x + y * y).sqrt() / 32.0).min(1.0);
            [255, 255, 255, ((1.0 - r) * (1.0 - r) * 255.0) as u8]
        }).collect();
        let glow = device.create_texture_with_data(queue, &wgpu::TextureDescriptor {
            label: Some("sprite glow"), size: wgpu::Extent3d { width: 64, height: 64, depth_or_array_layers: 1 }, mip_level_count: 1, sample_count: 1,
            dimension: wgpu::TextureDimension::D2, format: wgpu::TextureFormat::Rgba8UnormSrgb, usage: wgpu::TextureUsages::TEXTURE_BINDING, view_formats: &[],
        }, wgpu::util::TextureDataOrder::LayerMajor, &glow_px).create_view(&Default::default());
        Sprites {
            shader, view_layout, texture_layout,
            linear: sampler(wgpu::FilterMode::Linear), point: sampler(wgpu::FilterMode::Nearest),
            pipelines: Mutex::new(HashMap::new()), textures: Mutex::new(HashMap::new()), white, glow,
        }
    }

    fn pipeline(&self, device: &wgpu::Device, additive: bool, opaque: bool, samples: u32) -> wgpu::RenderPipeline {
        self.pipelines.lock().unwrap().entry((additive, opaque, samples)).or_insert_with(|| {
            let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor { label: None, bind_group_layouts: &[&self.view_layout, &self.texture_layout], push_constant_ranges: &[] });
            // D_BLEND 1: src alpha, one; else src alpha, inverse src alpha (alpha: one, inverse src alpha)
            let blend = if additive {
                wgpu::BlendState { color: wgpu::BlendComponent { src_factor: wgpu::BlendFactor::SrcAlpha, dst_factor: wgpu::BlendFactor::One, operation: wgpu::BlendOperation::Add }, alpha: wgpu::BlendComponent { src_factor: wgpu::BlendFactor::Zero, dst_factor: wgpu::BlendFactor::One, operation: wgpu::BlendOperation::Add } }
            } else {
                wgpu::BlendState { color: wgpu::BlendComponent { src_factor: wgpu::BlendFactor::SrcAlpha, dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha, operation: wgpu::BlendOperation::Add }, alpha: wgpu::BlendComponent { src_factor: wgpu::BlendFactor::One, dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha, operation: wgpu::BlendOperation::Add } }
            };
            device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("sprites"),
                layout: Some(&layout),
                vertex: wgpu::VertexState { module: &self.shader, entry_point: Some("vs_main"), buffers: &[], compilation_options: Default::default() },
                fragment: Some(wgpu::FragmentState { module: &self.shader, entry_point: Some("fs_main"), targets: &[Some(wgpu::ColorTargetState { format: super::COLOR, blend: Some(blend), write_mask: wgpu::ColorWrites::ALL })], compilation_options: Default::default() }),
                primitive: wgpu::PrimitiveState { cull_mode: None, ..Default::default() },
                depth_stencil: Some(wgpu::DepthStencilState { format: super::DEPTH, depth_write_enabled: opaque && !additive, depth_compare: wgpu::CompareFunction::LessEqual, stencil: Default::default(), bias: Default::default() }),
                multisample: wgpu::MultisampleState { count: samples, ..Default::default() },
                multiview: None,
                cache: None,
            })
        }).clone()
    }

    /// The texture at a bindless index as sRGB on the GPU: one managed made
    /// (put there again once managed writes to it), or a file's. None names
    /// none: white. One that can't be read (core's particle textures come
    /// with Facepunch's download, not sbox-public) is a soft glow in its
    /// place, so what uses it shows where and how large it is.
    fn texture(&self, r: &super::Renderer, index: i32) -> wgpu::TextureView {
        self.texture_of(r, crate::gpu::texture_at(index))
    }

    /// A texture by its object, as `texture` puts it on the GPU (none: white).
    pub(super) fn texture_of(&self, r: &super::Renderer, t: Option<usize>) -> wgpu::TextureView {
        let Some(t) = t else { return self.white.clone() };
        self.try_texture_of(r, t).unwrap_or_else(|| self.glow.clone())
    }

    /// A texture by its object as sRGB on the GPU, or none where it can't be
    /// read (and isn't looked for again until managed writes to it).
    pub fn try_texture_of(&self, r: &super::Renderer, t: usize) -> Option<wgpu::TextureView> {
        let version = crate::gpu::texture_version(t);
        if let Some((v, view)) = self.textures.lock().unwrap().get(&t) {
            if *v == version { return view.clone(); }
        }
        let view = if version.is_some() {
            crate::gpu::texture_rgba(t).map(|(w, h, _, px)| r.device.create_texture_with_data(&r.queue, &wgpu::TextureDescriptor {
                label: Some("sprite image"), size: wgpu::Extent3d { width: w, height: h, depth_or_array_layers: 1 }, mip_level_count: 1, sample_count: 1,
                dimension: wgpu::TextureDimension::D2, format: wgpu::TextureFormat::Rgba8UnormSrgb, usage: wgpu::TextureUsages::TEXTURE_BINDING, view_formats: &[],
            }, wgpu::util::TextureDataOrder::LayerMajor, &px).create_view(&Default::default()))
        } else {
            crate::camera::texture_name(t).and_then(|name| r.texture(&super::EngineFiles, &name, true)
                .map_err(|e| crate::trace::note(&format!("textures: {e}; a stand-in in its place"))).ok())
        };
        self.textures.lock().unwrap().insert(t, (version, view.clone()));
        view
    }

    /// Draws `batches` in `pass`, which draws into the scene's colour and depth.
    pub fn draw<'a>(&self, r: &super::Renderer, pass: &mut wgpu::RenderPass<'a>, view: &SpriteView, batches: &[Batch], samples: u32) {
        let d = &r.device;
        let uniform = d.create_buffer_init(&wgpu::util::BufferInitDescriptor { label: Some("sprite view"), contents: bytemuck::bytes_of(view), usage: wgpu::BufferUsages::UNIFORM });
        for b in batches.iter().filter(|b| !b.is_empty()) {
            let (records, runs) = runs(b);
            if records.is_empty() { continue; }
            let buffer = d.create_buffer_init(&wgpu::util::BufferInitDescriptor { label: Some("sprites"), contents: &records, usage: wgpu::BufferUsages::STORAGE });
            let group = d.create_bind_group(&wgpu::BindGroupDescriptor { label: Some("sprites"), layout: &self.view_layout, entries: &[
                wgpu::BindGroupEntry { binding: 0, resource: uniform.as_entire_binding() },
                wgpu::BindGroupEntry { binding: 1, resource: buffer.as_entire_binding() },
                wgpu::BindGroupEntry { binding: 2, resource: wgpu::BindingResource::Sampler(&self.linear) },
                wgpu::BindGroupEntry { binding: 3, resource: wgpu::BindingResource::Sampler(&self.point) },
            ] });
            pass.set_pipeline(&self.pipeline(d, b.additive, b.opaque, samples));
            pass.set_bind_group(0, &group, &[]);
            for (first, count, indices) in runs {
                let views: Vec<wgpu::TextureView> = (0..SLOTS).map(|k| indices.get(k).map(|&i| self.texture(r, i)).unwrap_or_else(|| self.white.clone())).collect();
                let entries: Vec<wgpu::BindGroupEntry> = views.iter().enumerate().map(|(k, v)| wgpu::BindGroupEntry { binding: k as u32, resource: wgpu::BindingResource::TextureView(v) }).collect();
                let textures = d.create_bind_group(&wgpu::BindGroupDescriptor { label: Some("sprite textures"), layout: &self.texture_layout, entries: &entries });
                pass.set_bind_group(1, &textures, &[]);
                pass.draw(0..6, first..first + count);
            }
        }
    }
}

/// A batch's sprites as this draw reads them (text sprites left out: their
/// glyphs aren't drawn here yet), each one's texture rewritten as its slot
/// among its run's and its sampler as whether it's point-filtered; and the
/// runs: (first sprite, how many, the bindless indices bound).
pub fn runs(b: &Batch) -> (Vec<u8>, Vec<(u32, u32, Vec<i32>)>) {
    let samplers = crate::gpu::samplers_by_index();
    let mut records = Vec::with_capacity(b.records.len());
    let mut runs: Vec<(u32, u32, Vec<i32>)> = Vec::new();
    for i in 0..b.len() {
        if b.word(i, at::FLAGS) & TEXT != 0 { continue; }
        let index = b.word(i, at::TEXTURE) as i32;
        let n = (records.len() / STRIDE) as u32;
        let need_new = match runs.last() { None => true, Some((_, _, ix)) => !ix.contains(&index) && ix.len() == SLOTS };
        if need_new { runs.push((n, 0, Vec::new())); }
        let run = runs.last_mut().unwrap();
        let slot = match run.2.iter().position(|&x| x == index) { Some(s) => s, None => { run.2.push(index); run.2.len() - 1 } };
        run.1 += 1;
        let mut r = b.record(i).to_vec();
        let sampler = b.word(i, at::SAMPLER) as usize;
        let point = samplers.get(sampler).is_some_and(|s| s.filter == 0) as u32;
        r[at::TEXTURE * 4..at::TEXTURE * 4 + 4].copy_from_slice(&(slot as u32).to_le_bytes());
        r[at::SAMPLER * 4..at::SAMPLER * 4 + 4].copy_from_slice(&point.to_le_bytes());
        records.extend(r);
    }
    (records, runs)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_batch_is_drawn_in_runs_of_eight_textures() {
        let mut b = Batch::default();
        for t in [5, 6, 5, 7, 8, 9, 10, 11, 12, 13, 5] {
            let mut r = vec![0u8; STRIDE];
            r[at::TEXTURE * 4..at::TEXTURE * 4 + 4].copy_from_slice(&(t as u32).to_le_bytes());
            b.records.extend(r);
        }
        let mut text = vec![0u8; STRIDE];
        text[at::FLAGS * 4..at::FLAGS * 4 + 4].copy_from_slice(&TEXT.to_le_bytes());
        b.records.extend(text);
        let (records, runs) = runs(&b);
        assert_eq!(records.len(), 11 * STRIDE, "the text sprite left out");
        assert_eq!(runs, vec![(0, 9, vec![5, 6, 7, 8, 9, 10, 11, 12]), (9, 2, vec![13, 5])], "the ninth texture starts a run, and what follows joins it");
        let slot = |i: usize| u32::from_le_bytes(records[i * STRIDE + at::TEXTURE * 4..][..4].try_into().unwrap());
        assert_eq!((slot(0), slot(2), slot(8), slot(9), slot(10)), (0, 0, 7, 0, 1));
    }

    /// A white sprite 50 across its middle, 100 in front of a camera that
    /// sees 200 across: a quarter of the frame, a square half its width, as
    /// bright as its tint (192 is 1.0 at exponent 129). Turned 45 degrees,
    /// it's a diamond of the same area.
    #[test]
    fn a_sprite_is_drawn_facing_the_camera() {
        let Ok(r) = super::super::Renderer::new() else { eprintln!("no adapter: skipped"); return };
        let sprite = |x: f32, roll: f32, cutoff: u32| {
            let mut s = vec![0u8; STRIDE];
            let mut put = |w: usize, v: u32| s[w * 4..w * 4 + 4].copy_from_slice(&v.to_le_bytes());
            put(at::POSITION, x.to_bits());
            put(at::ROTATION + 2, roll.to_bits());
            put(at::SCALE, 50f32.to_bits());
            put(at::SCALE + 1, 50f32.to_bits());
            put(at::TINT, 0xffc0_c0c0);
            put(at::LIGHTING, 129 << 16);
            put(at::FOG_CUTOUT, cutoff << 16);
            put(at::OFFSET, 0.5f32.to_bits());
            put(at::OFFSET + 1, 0.5f32.to_bits());
            put(at::ROTATION_OFFSET, (-1f32).to_bits());
            s
        };
        let draw = |batch: Vec<u8>| {
            let mut view = super::super::View::camera(glam::Vec3::ZERO, [0.0, 0.0, 0.0], 90.0, 1.0, 1000.0, 1.0);
            view.clear = [0.0, 0.0, 0.0, 1.0];
            view.exposure = super::super::Exposure::Fixed(1.0);
            view.tonemap = super::super::Tonemap::Linear;
            view.sprites = vec![Batch { records: batch, additive: true, opaque: false }];
            r.draw(&[], &view, 64, 64)
        };
        let lit = |image: &[u8]| (0..64 * 64).filter(|&i| image[i * 4] > 128).count();
        let image = draw(sprite(100.0, 0.0, 0));
        let covered = lit(&image);
        eprintln!("{covered} pixels lit; middle {:?}, corner {:?}", &image[(32 * 64 + 32) * 4..][..4], &image[..4]);
        assert!((covered as i32 - 32 * 32).abs() <= 64, "a square half the frame's width: {covered}");
        assert!(image[(32 * 64 + 32) * 4] > 250 && image[0] < 5, "white in its middle, black outside");
        assert!(image[(17 * 64 + 17) * 4] > 128 && image[(14 * 64 + 14) * 4] < 5, "its corners where a square's are");
        let two = sprite(100.0, 45.0, 0);
        let turned = draw(two);
        let covered = lit(&turned);
        assert!((covered as i32 - 32 * 32).abs() <= 64, "turned, the same area: {covered}");
        assert!(turned[(17 * 64 + 17) * 4] < 5 && turned[(32 * 64 + 10) * 4] > 128, "a diamond: no corner at 17,17; a point out to the left");
    }
}
