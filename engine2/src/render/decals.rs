//! Decals, as s&box projects them (`CDecalSceneObject`; its forward
//! shaders' `Decals::Resolve`, public in sbox-public's
//! common/classes/Decals.hlsl): a box in the world, and every opaque surface
//! inside it takes the decal's colour into its albedo before it's lit: mixed
//! over it, multiplied into it, or added as glow, fading as the surface turns
//! away from the decal's axis. Each decal's colour texture is copied into a
//! layer of one texture array (as native keeps its decals in an atlas), with
//! its mips; the shader (shader.wgsl) reads the decals and the array.
use std::collections::HashMap;
use std::sync::Mutex;

use glam::{Mat4, Vec3};
use wgpu::util::DeviceExt;

/// A decal as a frame draws it.
#[derive(Clone, Debug)]
pub struct Decal {
    /// world to the decal's box, a unit cube about the origin: x its depth
    /// (the axis it projects along), y and z across it
    pub to_decal: Mat4,
    /// the axis it projects along, in the world
    pub axis: Vec3,
    /// how much a surface turned away from it fades it (0 to 1)
    pub attenuation: f32,
    /// its colour times its texture's, alpha how much of it
    pub tint: [f32; 4],
    /// how much of its colour it lays down (DecalDefinition.ColorMix)
    pub color_mix: f32,
    /// DecalBlendMode: 0 over the surface, 1 multiplied into it, 2 added as glow
    pub blend_mode: u32,
    /// its colour texture (an object of camera.rs's or managed's)
    pub texture: Option<usize>,
    /// drawn in this order, later over earlier
    pub sort: u32,
}

impl Decal {
    /// A decal from where managed put its box: centred at `position`,
    /// turned by `rotation`, `size` across (depth, width, height).
    pub fn placed(position: Vec3, rotation: glam::Quat, size: Vec3) -> Decal {
        let to_decal = Mat4::from_scale(Vec3::ONE / size.max(Vec3::splat(1e-4))) * Mat4::from_quat(rotation.inverse()) * Mat4::from_translation(-position);
        Decal { to_decal, axis: rotation * Vec3::X, attenuation: 0.0, tint: [1.0; 4], color_mix: 1.0, blend_mode: 0, texture: None, sort: 0 }
    }
}

/// One decal as the shader reads it (shader.wgsl's DecalGpu).
#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
struct DecalGpu {
    to_decal: [[f32; 4]; 4],
    /// xyz: its axis; w: its attenuation
    axis: [f32; 4],
    tint: [f32; 4],
    /// x: its layer, y: its colour mix, z: its blend mode, w: 1 (0: none)
    params: [f32; 4],
}

const SIZE: u32 = 256;
const MIPS: u32 = 9;
const LAYERS: u32 = 32;

pub struct Decals {
    pub array: wgpu::Texture,
    pub array_view: wgpu::TextureView,
    pub sampler: wgpu::Sampler,
    blit: wgpu::RenderPipeline,
    blit_layout: wgpu::BindGroupLayout,
    blit_sampler: wgpu::Sampler,
    /// each texture's layer; layer 0 is what stands in for one that can't be read
    layers: Mutex<HashMap<usize, u32>>,
}

impl Decals {
    pub fn new(device: &wgpu::Device, queue: &wgpu::Queue) -> Decals {
        let array = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("decals"), size: wgpu::Extent3d { width: SIZE, height: SIZE, depth_or_array_layers: LAYERS }, mip_level_count: MIPS, sample_count: 1,
            dimension: wgpu::TextureDimension::D2, format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::RENDER_ATTACHMENT, view_formats: &[],
        });
        let array_view = array.create_view(&wgpu::TextureViewDescriptor { dimension: Some(wgpu::TextureViewDimension::D2Array), ..Default::default() });
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge, address_mode_v: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear, min_filter: wgpu::FilterMode::Linear, mipmap_filter: wgpu::FilterMode::Linear, ..Default::default()
        });
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor { label: Some("decal blit"), source: wgpu::ShaderSource::Wgsl(BLIT.into()) });
        let blit_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor { label: Some("decal blit"), entries: &[
            wgpu::BindGroupLayoutEntry { binding: 0, visibility: wgpu::ShaderStages::FRAGMENT, ty: wgpu::BindingType::Texture { sample_type: wgpu::TextureSampleType::Float { filterable: true }, view_dimension: wgpu::TextureViewDimension::D2, multisampled: false }, count: None },
            wgpu::BindGroupLayoutEntry { binding: 1, visibility: wgpu::ShaderStages::FRAGMENT, ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering), count: None },
        ] });
        let blit = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("decal blit"),
            layout: Some(&device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor { label: None, bind_group_layouts: &[&blit_layout], push_constant_ranges: &[] })),
            vertex: wgpu::VertexState { module: &shader, entry_point: Some("vs"), buffers: &[], compilation_options: Default::default() },
            fragment: Some(wgpu::FragmentState { module: &shader, entry_point: Some("fs"), targets: &[Some(wgpu::TextureFormat::Rgba8UnormSrgb.into())], compilation_options: Default::default() }),
            primitive: Default::default(), depth_stencil: None, multisample: Default::default(), multiview: None, cache: None,
        });
        let blit_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge, address_mode_v: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear, min_filter: wgpu::FilterMode::Linear, mipmap_filter: wgpu::FilterMode::Linear, ..Default::default()
        });
        let d = Decals { array, array_view, sampler, blit, blit_layout, blit_sampler, layers: Mutex::new(HashMap::new()) };
        // layer 0: a dark mark, round and soft-edged, for a texture that isn't found
        let px: Vec<u8> = (0..64 * 64).flat_map(|i| {
            let (x, y) = ((i % 64) as f32 + 0.5 - 32.0, (i / 64) as f32 + 0.5 - 32.0);
            let r = ((x * x + y * y).sqrt() / 32.0).min(1.0);
            [40, 36, 32, (((1.0 - r) * 3.0).min(1.0) * 255.0) as u8]
        }).collect();
        let mark = device.create_texture_with_data(queue, &wgpu::TextureDescriptor {
            label: Some("decal stand-in"), size: wgpu::Extent3d { width: 64, height: 64, depth_or_array_layers: 1 }, mip_level_count: 1, sample_count: 1,
            dimension: wgpu::TextureDimension::D2, format: wgpu::TextureFormat::Rgba8UnormSrgb, usage: wgpu::TextureUsages::TEXTURE_BINDING, view_formats: &[],
        }, wgpu::util::TextureDataOrder::LayerMajor, &px).create_view(&Default::default());
        d.fill(device, queue, &mark, 0);
        d
    }

    /// `source` drawn into every mip of a layer, each from the source's mip
    /// nearest its size (the sampler picks it, as the pixels step across it).
    fn fill(&self, device: &wgpu::Device, queue: &wgpu::Queue, source: &wgpu::TextureView, layer: u32) {
        let group = device.create_bind_group(&wgpu::BindGroupDescriptor { label: None, layout: &self.blit_layout, entries: &[
            wgpu::BindGroupEntry { binding: 0, resource: wgpu::BindingResource::TextureView(source) },
            wgpu::BindGroupEntry { binding: 1, resource: wgpu::BindingResource::Sampler(&self.blit_sampler) },
        ] });
        let mut encoder = device.create_command_encoder(&Default::default());
        for mip in 0..MIPS {
            let view = self.array.create_view(&wgpu::TextureViewDescriptor {
                dimension: Some(wgpu::TextureViewDimension::D2), base_mip_level: mip, mip_level_count: Some(1), base_array_layer: layer, array_layer_count: Some(1), ..Default::default()
            });
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("decal blit"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment { view: &view, resolve_target: None, ops: wgpu::Operations { load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT), store: wgpu::StoreOp::Store } })],
                depth_stencil_attachment: None, timestamp_writes: None, occlusion_query_set: None,
            });
            pass.set_pipeline(&self.blit);
            pass.set_bind_group(0, &group, &[]);
            pass.draw(0..3, 0..1);
        }
        queue.submit([encoder.finish()]);
    }

    /// The layer a decal's texture is in, put there the first time: 0 (the
    /// stand-in) for one that can't be read.
    fn layer(&self, r: &super::Renderer, texture: Option<usize>) -> u32 {
        let Some(t) = texture else { return 0 };
        if let Some(&l) = self.layers.lock().unwrap().get(&t) { return l; }
        let Some(view) = r.sprites.try_texture_of(r, t) else {
            self.layers.lock().unwrap().insert(t, 0);
            return 0;
        };
        let mut layers = self.layers.lock().unwrap();
        // full: start again (what's in use is put back as it's asked for)
        if layers.values().filter(|&&l| l > 0).count() as u32 >= LAYERS - 1 { layers.retain(|_, l| *l == 0); }
        let l = (1..LAYERS).find(|l| !layers.values().any(|x| x == l)).unwrap_or(1);
        layers.insert(t, l);
        drop(layers);
        self.fill(&r.device, &r.queue, &view, l);
        l
    }

    /// The decals as the shader's storage buffer reads them, in their order
    /// (one that does nothing if there are none: a buffer can't be empty).
    pub fn buffer(&self, r: &super::Renderer, decals: &[Decal]) -> wgpu::Buffer {
        let mut sorted: Vec<&Decal> = decals.iter().collect();
        sorted.sort_by_key(|d| d.sort);
        let mut gpu: Vec<DecalGpu> = sorted.iter().map(|d| DecalGpu {
            to_decal: d.to_decal.to_cols_array_2d(),
            axis: d.axis.extend(d.attenuation).into(),
            tint: d.tint,
            params: [self.layer(r, d.texture) as f32, d.color_mix, d.blend_mode as f32, 1.0],
        }).collect();
        if gpu.is_empty() { gpu.push(DecalGpu::zeroed()); }
        r.device.create_buffer_init(&wgpu::util::BufferInitDescriptor { label: Some("decals"), contents: bytemuck::cast_slice(&gpu), usage: wgpu::BufferUsages::STORAGE })
    }
}

use bytemuck::Zeroable;

const BLIT: &str = r#"
@group(0) @binding(0) var source: texture_2d<f32>;
@group(0) @binding(1) var samp: sampler;
struct Out { @builtin(position) p: vec4<f32>, @location(0) uv: vec2<f32> };
@vertex fn vs(@builtin(vertex_index) i: u32) -> Out {
    let uv = vec2<f32>(f32((i << 1u) & 2u), f32(i & 2u));
    var o: Out;
    o.p = vec4<f32>(uv * vec2<f32>(2.0, -2.0) + vec2<f32>(-1.0, 1.0), 0.0, 1.0);
    o.uv = uv;
    return o;
}
@fragment fn fs(i: Out) -> @location(0) vec4<f32> { return textureSample(source, samp, i.uv); }
"#;

#[cfg(test)]
mod tests {
    use glam::{Quat, Vec3};

    #[test]
    fn a_decal_box_is_a_unit_cube_about_its_middle() {
        // 8 deep, 32 wide, 16 high, at (100, 0, 50), facing down (its x along -z)
        let r = Quat::from_rotation_y(std::f32::consts::FRAC_PI_2);
        let d = super::Decal::placed(Vec3::new(100.0, 0.0, 50.0), r, Vec3::new(8.0, 32.0, 16.0));
        assert!((d.axis - Vec3::new(0.0, 0.0, -1.0)).length() < 1e-5, "{:?}", d.axis);
        let at = |p: Vec3| d.to_decal.transform_point3(p);
        assert!(at(Vec3::new(100.0, 0.0, 50.0)).length() < 1e-5, "its middle");
        assert!((at(Vec3::new(100.0, 0.0, 46.0)).x - 0.5).abs() < 1e-5, "4 along its axis: its far face");
        assert!((at(Vec3::new(100.0, 16.0, 50.0)).y - 0.5).abs() < 1e-5, "16 across: its edge");
    }

    /// A floor 64 square, looked straight down on from 100 with a 60° view
    /// (115 across), and a red decal 32 square projected down onto its
    /// middle: a square of red about 18 pixels across where it lands, the
    /// rest of the floor as it was. Multiplied in instead, red over the white
    /// floor is red too, its green gone; off the floor, it touches nothing.
    #[test]
    fn a_decal_lays_its_colour_into_the_floor() {
        let Ok(r) = crate::render::Renderer::new() else { eprintln!("no adapter: skipped"); return };
        use crate::rendermesh as rm;
        let l = rm::new_layout(32);
        for (s, f, o) in [("position", 6, 0), ("normal", 6, 12), ("texcoord", 16, 24)] { rm::add_field(l, s, 0, f, o); }
        let floor: Vec<f32> = [[-32.0, -32.0], [32.0, -32.0], [32.0, 32.0], [-32.0, 32.0]].iter().flat_map(|p| [p[0], p[1], 0.0, 0.0, 0.0, 1.0, p[0] / 64.0, p[1] / 64.0]).collect();
        let vb = unsafe { rm::new_vertices(4, &rm::layout_of(l).unwrap(), floor.as_ptr() as usize) };
        let idx: [i32; 6] = [0, 1, 2, 0, 2, 3];
        let ib = unsafe { rm::new_indices(6, true, idx.as_ptr() as usize, 6) };
        let mesh = rm::new_mesh(0, rm::TRIANGLES, "decal floor");
        unsafe { rm::give(mesh, vb, 0, 0); rm::give(mesh, ib, 0, 0); }
        let meshes = rm::for_drawing(&[mesh], &[], |_| String::new());
        let m = r.model_of(&crate::render::Content(Vec::new(), Vec::new()), "decal floor", &meshes, u64::MAX, &[]).unwrap();
        // a 2x2 red texture, as managed makes one (RGBA8888)
        let mut config = [0u8; 44];
        config[0] = 2; config[2] = 2; config[4] = 1; config[6] = 1;
        let red = crate::gpu::create_texture("decal test red", true, &config, &[255, 0, 0, 255].repeat(4));
        let down = Quat::from_rotation_y(std::f32::consts::FRAC_PI_2);
        let draw = |decals: Vec<super::Decal>| {
            let mut view = crate::render::View::camera(Vec3::new(0.0, 0.0, 100.0), [90.0, 0.0, 0.0], 60.0, 1.0, 1000.0, 1.0);
            view.exposure = crate::render::Exposure::Fixed(1.0);
            view.decals = decals;
            r.draw(&[crate::render::Instance { model: &m, transform: glam::Mat4::IDENTITY, tint: [1.0; 4], bones: Vec::new(), lightmapped: false, origin: None, mask: None, skin: 0 }], &view, 64, 64)
        };
        let decal = |at: Vec3, mode: u32| super::Decal { texture: Some(red), blend_mode: mode, ..super::Decal::placed(at, down, Vec3::new(8.0, 32.0, 32.0)) };
        let plain = draw(Vec::new());
        let px = |img: &[u8], x: usize, y: usize| [img[(y * 64 + x) * 4] as i32, img[(y * 64 + x) * 4 + 1] as i32, img[(y * 64 + x) * 4 + 2] as i32];
        let reds = |img: &[u8]| (0..64 * 64).filter(|&i| { let (x, y) = (i % 64, i / 64); let p = px(img, x, y); p[0] > p[1] + 40 }).count();
        assert_eq!(reds(&plain), 0, "the floor isn't red");
        let over = draw(vec![decal(Vec3::ZERO, 0)]);
        let n = reds(&over);
        eprintln!("{n} red pixels; middle {:?} (was {:?}), corner {:?}", px(&over, 32, 32), px(&plain, 32, 32), px(&over, 24, 24));
        assert!((n as i32 - 18 * 18).abs() < 60, "a square about 18 across: {n}");
        let (a, b) = (px(&over, 32, 32), px(&plain, 32, 32));
        assert!(a[0] > 2 * a[1].max(1) && (px(&over, 12, 12)[0] - px(&plain, 12, 12)[0]).abs() <= 1, "red in its middle, the floor as it was outside it: {a:?} over {b:?}");
        let multiplied = draw(vec![decal(Vec3::ZERO, 1)]);
        assert!((reds(&multiplied) as i32 - n as i32).abs() < 30, "multiplied, the same square");
        assert_eq!(reds(&draw(vec![decal(Vec3::new(0.0, 0.0, 40.0), 0)])), 0, "40 above the floor, its box reaches nothing");
    }
}
