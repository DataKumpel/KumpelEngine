//===== IMPORTS =====//
use std::sync::Arc;

use winit::window::Window;
//===== IMPORTS =====//


//===== GFX STATE STRUCTURE =====//
pub struct GfxState {
    pub surface: wgpu::Surface<'static>,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,
    pub size: winit::dpi::PhysicalSize<u32>,
    pub window: Arc<Window>,
    pub render_pipeline: wgpu::RenderPipeline,
}

impl GfxState {
    pub async fn new(window: Arc<Window>) -> Self {
        let size = window.inner_size();
        let instance = wgpu::Instance::default();

        // ---> Surface is our painting canvas in the window:
        let surface = instance.create_surface(window.clone()).unwrap();

        // ---> Adapter is our physical graphics card:
        let adapter = instance.request_adapter(
            &wgpu::RequestAdapterOptionsBase { 
                power_preference: wgpu::PowerPreference::HighPerformance, 
                force_fallback_adapter: false,
                compatible_surface: Some(&surface),
            }).await.expect("No WGPU adapter was found...");

        // ---> Device (logical connection) and queue (for commands):
        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor::default(), None)
            .await.expect("WGPU device could not be created...");

        let surface_caps = surface.get_capabilities(&adapter);
        let surface_fmt = surface_caps.formats.iter().copied()
            .find(|fmt| fmt.is_srgb())
            .unwrap_or(surface_caps.formats[0]);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_fmt,
            width: size.width.max(1),
            height: size.height.max(1),
            present_mode: wgpu::PresentMode::AutoNoVsync,
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };
        surface.configure(&device, &config);

        // ---> Configure render pipeline:
        let shader = device.create_shader_module(wgpu::include_wgsl!("triangle_shader.wgsl"));
        let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor { 
            label: Some("Render Pipeline Layout"), 
            bind_group_layouts: &[], 
            push_constant_ranges: &[],
        });

        // ---> Create render pipeline:
        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor { 
            label: Some("Render Pipeline"), 
            layout: Some(&render_pipeline_layout), 
            vertex: wgpu::VertexState { 
                module: &shader, 
                entry_point: "vs_main", 
                compilation_options: wgpu::PipelineCompilationOptions::default(), 
                buffers: &[], 
            }, 
            primitive: wgpu::PrimitiveState { 
                topology: wgpu::PrimitiveTopology::TriangleList, 
                strip_index_format: None, 
                front_face: wgpu::FrontFace::Ccw, 
                cull_mode: Some(wgpu::Face::Back), 
                unclipped_depth: false, 
                polygon_mode: wgpu::PolygonMode::Fill, 
                conservative: false, 
            }, 
            depth_stencil: None, 
            multisample: wgpu::MultisampleState { 
                count: 1, 
                mask: !0, 
                alpha_to_coverage_enabled: false,
            }, 
            fragment: Some(wgpu::FragmentState { 
                module: &shader, 
                entry_point: "fs_main", 
                compilation_options: wgpu::PipelineCompilationOptions::default(), 
                targets: &[Some(wgpu::ColorTargetState { 
                    format: config.format, 
                    blend: Some(wgpu::BlendState::REPLACE), 
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }), 
            multiview: None,
        });

        Self { surface, device, queue, config, size, window, render_pipeline }
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Main Render Encoder"),
        });

        // ---> Start Render Pass:
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Main Render Pass"),
                color_attachments: &[
                    Some(wgpu::RenderPassColorAttachment {
                        view: &view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color { r: 0.1, g: 0.2, b: 0.8, a: 1.0 }), // Background color...
                            store: wgpu::StoreOp::Store,
                        },
                    }),
                ],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.draw(0..3, 0..1); // draw 3 vertices, 1 instance
        } // End of Render Pass

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();
        Ok(())
    }
}
//===== GFX STATE STRUCTURE =====//

