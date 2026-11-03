//! Lines drawn (lines.wgsl, after s&box's line.shader): each line's
//! triangles as crate::lines read them back, into the scene's colour (linear
//! HDR) over what's been drawn, tested against its depth.
use std::collections::HashMap;
use std::sync::Mutex;

use wgpu::util::DeviceExt;

use crate::lines::{Line, VERTEX_BYTES};

/// What a frame gives its lines: the camera, and the light a lit line takes.
#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct LineView {
    pub view_proj: [[f32; 4]; 4],
    pub eye: [f32; 4],
    pub sun: [f32; 4],
    pub sun_color: [f32; 4],
    pub ambient: [f32; 4],
}

pub struct Lines {
    shader: wgpu::ShaderModule,
    view_layout: wgpu::BindGroupLayout,
    line_layout: wgpu::BindGroupLayout,
    /// by (wraps, filters)
    samplers: HashMap<(bool, bool), wgpu::Sampler>,
    /// by (added, opaque, samples a pixel)
    pipelines: Mutex<HashMap<(bool, bool, u32), wgpu::RenderPipeline>>,
    /// what stands in for a colour texture that can't be read: a band across
    /// the line, opaque in its middle and fading to nothing at its edges
    band: wgpu::TextureView,
}

impl Lines {
    pub fn new(device: &wgpu::Device, queue: &wgpu::Queue) -> Lines {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor { label: Some("lines"), source: wgpu::ShaderSource::Wgsl(include_str!("lines.wgsl").into()) });
        let uniform = |binding, visibility| wgpu::BindGroupLayoutEntry { binding, visibility, ty: wgpu::BindingType::Buffer { ty: wgpu::BufferBindingType::Uniform, has_dynamic_offset: false, min_binding_size: None }, count: None };
        let view_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor { label: Some("lines"), entries: &[uniform(0, wgpu::ShaderStages::VERTEX_FRAGMENT)] });
        let line_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor { label: Some("line"), entries: &[
            wgpu::BindGroupLayoutEntry { binding: 0, visibility: wgpu::ShaderStages::FRAGMENT, ty: wgpu::BindingType::Texture { sample_type: wgpu::TextureSampleType::Float { filterable: true }, view_dimension: wgpu::TextureViewDimension::D2, multisampled: false }, count: None },
            wgpu::BindGroupLayoutEntry { binding: 1, visibility: wgpu::ShaderStages::FRAGMENT, ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering), count: None },
            uniform(2, wgpu::ShaderStages::FRAGMENT),
        ] });
        let mut samplers = HashMap::new();
        for wrap in [false, true] {
            for filtered in [false, true] {
                let address = if wrap { wgpu::AddressMode::Repeat } else { wgpu::AddressMode::ClampToEdge };
                let filter = if filtered { wgpu::FilterMode::Linear } else { wgpu::FilterMode::Nearest };
                samplers.insert((wrap, filtered), device.create_sampler(&wgpu::SamplerDescriptor {
                    address_mode_u: address, address_mode_v: address, mag_filter: filter, min_filter: filter, mipmap_filter: filter, ..Default::default()
                }));
            }
        }
        // u across the line: alpha (1 - |2u - 1|)^2
        let px: Vec<u8> = (0..64).flat_map(|i| {
            let a = 1.0 - ((i as f32 + 0.5) / 32.0 - 1.0).abs();
            [255, 255, 255, (a * a * 255.0) as u8]
        }).collect();
        let band = device.create_texture_with_data(queue, &wgpu::TextureDescriptor {
            label: Some("line band"), size: wgpu::Extent3d { width: 64, height: 1, depth_or_array_layers: 1 }, mip_level_count: 1, sample_count: 1,
            dimension: wgpu::TextureDimension::D2, format: wgpu::TextureFormat::Rgba8UnormSrgb, usage: wgpu::TextureUsages::TEXTURE_BINDING, view_formats: &[],
        }, wgpu::util::TextureDataOrder::LayerMajor, &px).create_view(&Default::default());
        Lines { shader, view_layout, line_layout, samplers, pipelines: Mutex::new(HashMap::new()), band }
    }

    fn pipeline(&self, device: &wgpu::Device, additive: bool, opaque: bool, samples: u32) -> wgpu::RenderPipeline {
        self.pipelines.lock().unwrap().entry((additive, opaque, samples)).or_insert_with(|| {
            let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor { label: None, bind_group_layouts: &[&self.view_layout, &self.line_layout], push_constant_ranges: &[] });
            // D_BLEND: src alpha, one; D_OPAQUE: none; else alpha blending (D_BLENDMODE 0)
            let blend = if additive {
                Some(wgpu::BlendState { color: wgpu::BlendComponent { src_factor: wgpu::BlendFactor::SrcAlpha, dst_factor: wgpu::BlendFactor::One, operation: wgpu::BlendOperation::Add }, alpha: wgpu::BlendComponent { src_factor: wgpu::BlendFactor::Zero, dst_factor: wgpu::BlendFactor::One, operation: wgpu::BlendOperation::Add } })
            } else if opaque {
                None
            } else {
                Some(wgpu::BlendState { color: wgpu::BlendComponent { src_factor: wgpu::BlendFactor::SrcAlpha, dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha, operation: wgpu::BlendOperation::Add }, alpha: wgpu::BlendComponent { src_factor: wgpu::BlendFactor::One, dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha, operation: wgpu::BlendOperation::Add } })
            };
            let attributes = wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3, 2 => Float32x3, 3 => Float32x4, 4 => Float32x2];
            device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("lines"),
                layout: Some(&layout),
                vertex: wgpu::VertexState { module: &self.shader, entry_point: Some("vs_main"), buffers: &[wgpu::VertexBufferLayout { array_stride: VERTEX_BYTES as u64, step_mode: wgpu::VertexStepMode::Vertex, attributes: &attributes }], compilation_options: Default::default() },
                fragment: Some(wgpu::FragmentState { module: &self.shader, entry_point: Some("fs_main"), targets: &[Some(wgpu::ColorTargetState { format: super::COLOR, blend, write_mask: wgpu::ColorWrites::ALL })], compilation_options: Default::default() }),
                primitive: wgpu::PrimitiveState { cull_mode: None, ..Default::default() },
                depth_stencil: Some(wgpu::DepthStencilState { format: super::DEPTH, depth_write_enabled: opaque && !additive, depth_compare: wgpu::CompareFunction::LessEqual, stencil: Default::default(), bias: Default::default() }),
                multisample: wgpu::MultisampleState { count: samples, ..Default::default() },
                multiview: None,
                cache: None,
            })
        }).clone()
    }

    /// Draws `lines` in `pass`, which draws into the scene's colour and depth:
    /// the opaque ones first, then the rest in their order.
    pub fn draw<'a>(&self, r: &super::Renderer, pass: &mut wgpu::RenderPass<'a>, view: &LineView, lines: &[Line], samples: u32) {
        let d = &r.device;
        let uniform = d.create_buffer_init(&wgpu::util::BufferInitDescriptor { label: Some("line view"), contents: bytemuck::bytes_of(view), usage: wgpu::BufferUsages::UNIFORM });
        let view_group = d.create_bind_group(&wgpu::BindGroupDescriptor { label: Some("lines"), layout: &self.view_layout, entries: &[wgpu::BindGroupEntry { binding: 0, resource: uniform.as_entire_binding() }] });
        let order = lines.iter().filter(|l| l.opaque && !l.additive).chain(lines.iter().filter(|l| !(l.opaque && !l.additive)));
        for l in order.filter(|l| !l.indices.is_empty()) {
            let vertices: Vec<u8> = l.vertices.iter().flat_map(|v| [v.position.as_slice(), &v.normal, &v.tangent, &v.color, &v.uv].concat()).flat_map(|x| x.to_le_bytes()).collect();
            let vb = d.create_buffer_init(&wgpu::util::BufferInitDescriptor { label: Some("line vertices"), contents: &vertices, usage: wgpu::BufferUsages::VERTEX });
            let ib = d.create_buffer_init(&wgpu::util::BufferInitDescriptor { label: Some("line indices"), contents: bytemuck::cast_slice(&l.indices), usage: wgpu::BufferUsages::INDEX });
            let params = d.create_buffer_init(&wgpu::util::BufferInitDescriptor { label: Some("line"), contents: bytemuck::cast_slice(&[l.lit as i32 as f32, 0.0, 0.0, 0.0]), usage: wgpu::BufferUsages::UNIFORM });
            let texture = if l.missing_texture { self.band.clone() } else { r.sprites.texture_of(r, l.texture) };
            let group = d.create_bind_group(&wgpu::BindGroupDescriptor { label: Some("line"), layout: &self.line_layout, entries: &[
                wgpu::BindGroupEntry { binding: 0, resource: wgpu::BindingResource::TextureView(&texture) },
                wgpu::BindGroupEntry { binding: 1, resource: wgpu::BindingResource::Sampler(&self.samplers[&(l.wrap, l.filtered)]) },
                wgpu::BindGroupEntry { binding: 2, resource: params.as_entire_binding() },
            ] });
            pass.set_pipeline(&self.pipeline(d, l.additive, l.opaque, samples));
            pass.set_bind_group(0, &view_group, &[]);
            pass.set_bind_group(1, &group, &[]);
            pass.set_vertex_buffer(0, vb.slice(..));
            pass.set_index_buffer(ib.slice(..), wgpu::IndexFormat::Uint32);
            pass.draw_indexed(0..l.indices.len() as u32, 0, 0..1);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::lines::{Line, Vertex};

    /// A line 40 wide across the middle of the frame, 100 in front of a
    /// camera that sees 200 across (a fifth of its height), its colour
    /// orange as gamma (1, 0.5, 0): drawn opaque, it's that colour in
    /// linear light; added at half alpha, half of it; behind an opaque one,
    /// hidden.
    #[test]
    fn a_line_is_drawn_as_its_triangles() {
        let Ok(r) = super::super::Renderer::new() else { eprintln!("no adapter: skipped"); return };
        // from y 100 to -100, 20 either side of z 0, at x 100: facing the camera
        let strip = |x: f32, color: [f32; 4]| {
            let v = |y: f32, z: f32, u: f32| Vertex { position: [x, y, z], normal: [-1.0, 0.0, 0.0], tangent: [0.0, 0.0, 1.0], color, uv: [u, 0.0] };
            vec![v(100.0, 20.0, 0.0), v(100.0, -20.0, 1.0), v(-100.0, 20.0, 0.0), v(-100.0, -20.0, 1.0)]
        };
        let line = |x, color, additive, opaque| Line { vertices: strip(x, color), indices: vec![0, 2, 1, 1, 2, 3], additive, opaque, lit: false, texture: None, missing_texture: false, material: None, wrap: true, filtered: true };
        let draw = |lines: Vec<Line>| {
            let mut view = super::super::View::camera(glam::Vec3::ZERO, [0.0, 0.0, 0.0], 90.0, 1.0, 1000.0, 1.0);
            view.clear = [0.0, 0.0, 0.0, 1.0];
            view.exposure = super::super::Exposure::Fixed(1.0);
            view.tonemap = super::super::Tonemap::Linear;
            view.lines = lines;
            r.draw(&[], &view, 64, 64)
        };
        let orange = [1.0, 0.5, 0.0, 1.0];
        let image = draw(vec![line(100.0, orange, false, true)]);
        let px = |img: &[u8], x: usize, y: usize| [img[(y * 64 + x) * 4], img[(y * 64 + x) * 4 + 1], img[(y * 64 + x) * 4 + 2]];
        let rows: Vec<usize> = (0..64).filter(|&y| px(&image, 32, y)[0] > 128).collect();
        eprintln!("rows {rows:?}; middle {:?}", px(&image, 32, 32));
        assert!((rows.len() as i32 - 13).abs() <= 1, "a fifth of the height: {}", rows.len());
        // shown as sRGB: gamma 0.5 is linear 0.21, and back to 0.5 (128) on screen
        let m = px(&image, 32, 32);
        assert!(m[0] > 250 && (m[1] as i32 - 128).abs() <= 3 && m[2] < 3, "{m:?}");
        assert_eq!(px(&image, 32, 5), [0, 0, 0], "nothing above it");
        let half = draw(vec![line(100.0, [1.0, 1.0, 1.0, 0.5], true, false)]);
        let h = px(&half, 32, 32)[0];
        assert!((h as i32 - 188).abs() <= 3, "added at half: linear 0.5, 188 as sRGB: {h}");
        let hidden = draw(vec![line(200.0, [1.0, 1.0, 1.0, 1.0], true, false), line(100.0, [0.0, 0.0, 1.0, 1.0], false, true)]);
        assert_eq!(px(&hidden, 32, 32), [0, 0, 255], "the opaque one in front hides it");
    }
}
