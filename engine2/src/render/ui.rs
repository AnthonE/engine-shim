//! The UI over a frame: s&box's batched UI box (ui.wgsl, ported from
//! `ui_cssbox_batched.shader`) drawn from the tables managed filled, over
//! what's been drawn, in gamma space.
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
    /// LayerMat's sixteen floats, as managed set them
    pub layer: [f32; 16],
    pub instance_offset: i32,
    pub instances: u32,
}

pub struct Ui {
    shader: wgpu::ShaderModule,
    layout: wgpu::BindGroupLayout,
    pipelines: Mutex<HashMap<wgpu::TextureFormat, wgpu::RenderPipeline>>,
}

impl Ui {
    pub fn new(device: &wgpu::Device) -> Ui {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor { label: Some("ui"), source: wgpu::ShaderSource::Wgsl(include_str!("ui.wgsl").into()) });
        let storage = |binding| wgpu::BindGroupLayoutEntry {
            binding,
            visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
            ty: wgpu::BindingType::Buffer { ty: wgpu::BufferBindingType::Storage { read_only: true }, has_dynamic_offset: false, min_binding_size: None },
            count: None,
        };
        let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor { label: Some("ui"), entries: &[
            wgpu::BindGroupLayoutEntry { binding: 0, visibility: wgpu::ShaderStages::VERTEX_FRAGMENT, ty: wgpu::BindingType::Buffer { ty: wgpu::BufferBindingType::Uniform, has_dynamic_offset: false, min_binding_size: None }, count: None },
            storage(1), storage(2), storage(3), storage(4), storage(5), storage(6),
        ] });
        Ui { shader, layout, pipelines: Default::default() }
    }

    fn pipeline(&self, device: &wgpu::Device, format: wgpu::TextureFormat) -> wgpu::RenderPipeline {
        let mut all = self.pipelines.lock().unwrap();
        all.entry(format).or_insert_with(|| device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("ui"),
            layout: Some(&device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor { label: None, bind_group_layouts: &[&self.layout], push_constant_ranges: &[] })),
            vertex: wgpu::VertexState { module: &self.shader, entry_point: Some("vs_main"), compilation_options: Default::default(), buffers: &[] },
            fragment: Some(wgpu::FragmentState {
                module: &self.shader,
                entry_point: Some("fs_main"),
                compilation_options: Default::default(),
                targets: &[Some(wgpu::ColorTargetState { format, blend: Some(wgpu::BlendState::ALPHA_BLENDING), write_mask: wgpu::ColorWrites::ALL })],
            }),
            primitive: wgpu::PrimitiveState { cull_mode: None, ..Default::default() },
            depth_stencil: None,
            multisample: Default::default(),
            multiview: None,
            cache: None,
        })).clone()
    }

    /// Draws over `target` (a gamma-space view of the frame), `width` by `height`.
    pub fn draw(&self, device: &wgpu::Device, queue: &wgpu::Queue, target: &wgpu::TextureView, format: wgpu::TextureFormat, width: u32, height: u32, draws: &[UiDraw]) {
        if draws.is_empty() { return; }
        static SAID: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
        if !SAID.swap(true, std::sync::atomic::Ordering::Relaxed) {
            crate::trace::note(&format!("ui: drawing {} batches over {width}x{height} ({format:?}): {:?}", draws.len(),
                draws.iter().map(|d| (d.instances, d.instance_offset, d.boxes.len(), d.texts.len(), d.glyph_table.len(), d.glyph_curves.len())).collect::<Vec<_>>()));
            let t: &[f32] = bytemuck::cast_slice(&draws[0].transforms[..64.min(draws[0].transforms.len()) / 4 * 4]);
            crate::trace::note(&format!("ui: transform 0 {t:?}; layer {:?}", draws[0].layer));
        }
        let pipeline = self.pipeline(device, format);
        // a table can't be empty; a word of nothing stands in
        let buffer = |bytes: &[u8]| {
            let contents: Vec<u8> = if bytes.len() < 16 { let mut v = bytes.to_vec(); v.resize(16, 0); v } else { bytes[..bytes.len() / 4 * 4].to_vec() };
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor { label: Some("ui table"), contents: &contents, usage: wgpu::BufferUsages::STORAGE })
        };
        let groups: Vec<wgpu::BindGroup> = draws.iter().map(|d| {
            let mut params = vec![0.0f32, 0.0, width as f32, height as f32];
            params.extend_from_slice(&d.layer);
            let mut bytes: Vec<u8> = bytemuck::cast_slice(&params).to_vec();
            bytes.extend_from_slice(&d.instance_offset.to_le_bytes());
            let solid = std::env::var("ENGINE2_UI_SOLID").is_ok() as i32;
            bytes.extend_from_slice(&solid.to_le_bytes());
            bytes.resize(bytes.len() + 8, 0);
            let uniform = device.create_buffer_init(&wgpu::util::BufferInitDescriptor { label: Some("ui params"), contents: &bytes, usage: wgpu::BufferUsages::UNIFORM });
            let tables = [buffer(&d.boxes), buffer(&d.texts), buffer(&d.glyph_table), buffer(&d.glyph_curves), buffer(&d.glyph_bands), buffer(&d.transforms)];
            let mut entries = vec![wgpu::BindGroupEntry { binding: 0, resource: uniform.as_entire_binding() }];
            for (k, t) in tables.iter().enumerate() { entries.push(wgpu::BindGroupEntry { binding: k as u32 + 1, resource: t.as_entire_binding() }); }
            device.create_bind_group(&wgpu::BindGroupDescriptor { label: Some("ui"), layout: &self.layout, entries: &entries })
        }).collect();
        let mut encoder = device.create_command_encoder(&Default::default());
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("ui"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment { view: target, resolve_target: None, ops: wgpu::Operations { load: wgpu::LoadOp::Load, store: wgpu::StoreOp::Store } })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });
            pass.set_pipeline(&pipeline);
            for (d, g) in draws.iter().zip(&groups) {
                if d.instances == 0 { continue; }
                pass.set_bind_group(0, g, &[]);
                pass.draw(0..6, 0..d.instances);
            }
        }
        queue.submit([encoder.finish()]);
    }
}
