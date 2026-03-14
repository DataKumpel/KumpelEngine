//===== IMPORTS =====//
use std::sync::Arc;

use wgpu::util::DeviceExt;
use winit::window::Window;

use crate::input::InputState;
use crate::vertex::Vertex;
use crate::camera::{Camera, CameraController, CameraUniform};
use crate::texture;
//===== IMPORTS =====//


//===== GFX STATE STRUCTURE =====//
pub struct GfxState {
    // Gfx base state:
    pub surface: wgpu::Surface<'static>,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,
    pub size: winit::dpi::PhysicalSize<u32>,
    pub window: Arc<Window>,

    // Render pipeline state:
    pub render_pipeline: wgpu::RenderPipeline,
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub num_indices: u32,

    // Camera state:
    pub camera: Camera,
    pub camera_uniform: CameraUniform,
    pub camera_buffer: wgpu::Buffer,
    pub camera_bind_group: wgpu::BindGroup,
    pub camera_controller: CameraController,

    // Textures:
    pub depth_texture: wgpu::TextureView,
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

        // ---> Create depth texture:
        let depth_texture = texture::create_depth_texture(&device, &config, "Depth Texture");

        // ---> Initialise Camera:
        let camera = Camera {
            eye: glam::Vec3::new(0.0, 1.0, 2.0),
            target: glam::Vec3::new(0.0, 0.0, 0.0),
            up: glam::Vec3::Y,
            aspect: config.width as f32 / config.height as f32,
            fovy: 66.0f32.to_radians(),
            clip_near: 0.1,
            clip_far: 100.0,
        };

        let camera_controller = CameraController::new(0.001);

        let mut camera_uniform = CameraUniform::new();
        camera_uniform.update_view_proj(&camera);

        let camera_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Camera Buffer"),
            contents: bytemuck::cast_slice(&[camera_uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let camera_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor { 
            label: Some("Camera Bind Group Layout"), 
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer { 
                        ty: wgpu::BufferBindingType::Uniform, 
                        has_dynamic_offset: false, 
                        min_binding_size: None, 
                    },
                    count: None,
                },
            ], 
        });

        let camera_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor { 
            label: Some("Camera Bind Group"), 
            layout: &camera_bind_group_layout, 
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: camera_buffer.as_entire_binding(),
                },
            ], 
        });
        
        // ---> Define Hello World Triangle and indices:
        //const VERTICES: &[Vertex] = &[
        //    Vertex { pos: [ 0.0,  0.5, 0.0], color: [1.0, 0.0, 0.0] },
        //    Vertex { pos: [-0.5, -0.5, 0.0], color: [0.0, 1.0, 0.0] },
        //    Vertex { pos: [ 0.5, -0.5, 0.0], color: [0.0, 0.0, 1.0] },
        //];
        //
        //const INDICES: &[u16] = &[0, 1, 2];

        // ---> Define Hello World Cube vertices and indices:
        const VERTICES: &[Vertex] = &[
            Vertex { pos: [-0.5, -0.5,  0.5], color: [1.0, 0.0, 0.0] }, // 0
            Vertex { pos: [ 0.5, -0.5,  0.5], color: [0.0, 1.0, 0.0] }, // 1
            Vertex { pos: [ 0.5,  0.5,  0.5], color: [0.0, 0.0, 1.0] }, // 2
            Vertex { pos: [-0.5,  0.5,  0.5], color: [1.0, 1.0, 0.0] }, // 3
            Vertex { pos: [-0.5, -0.5, -0.5], color: [1.0, 0.0, 1.0] }, // 4
            Vertex { pos: [ 0.5, -0.5, -0.5], color: [0.0, 1.0, 1.0] }, // 5
            Vertex { pos: [ 0.5,  0.5, -0.5], color: [1.0, 1.0, 1.0] }, // 6
            Vertex { pos: [-0.5,  0.5, -0.5], color: [0.0, 0.0, 0.0] }, // 7
        ];

        const INDICES: &[u16] = &[
            0, 1, 2, 2, 3, 0, // front
            1, 5, 6, 6, 2, 1, // right
            7, 6, 5, 5, 4, 7, // back
            4, 0, 3, 3, 7, 4, // left
            4, 5, 1, 1, 0, 4, // bottom
            3, 2, 6, 6, 7, 3, // top
        ];
        
        // ---> Create Buffers:
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(VERTICES),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor{
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(INDICES),
            usage: wgpu::BufferUsages::INDEX,
        });

        let num_indices = INDICES.len() as u32;

        // ---> Configure render pipeline:
        let shader = device.create_shader_module(wgpu::include_wgsl!("triangle_shader.wgsl"));
        let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor { 
            label: Some("Render Pipeline Layout"), 
            bind_group_layouts: &[
                &camera_bind_group_layout
            ], 
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
                buffers: &[Vertex::desc()], 
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
            depth_stencil: Some(wgpu::DepthStencilState { 
                format: texture::DEPTH_FORMAT, 
                depth_write_enabled: true, 
                depth_compare: wgpu::CompareFunction::Less, 
                stencil: wgpu::StencilState::default(), 
                bias: wgpu::DepthBiasState::default(),
            }), 
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

        Self { 
            surface, 
            device, 
            queue, 
            config, 
            size, 
            window, 
            render_pipeline, 
            vertex_buffer, 
            index_buffer, 
            num_indices,
            camera,
            camera_bind_group,
            camera_buffer,
            camera_uniform,
            camera_controller,
            depth_texture,
        }
    }

    pub fn update(&mut self, input: &InputState) {
        self.camera_controller.update_camera(&mut self.camera, input);
        self.camera_uniform.update_view_proj(&self.camera);
        self.queue.write_buffer(&self.camera_buffer, 0, bytemuck::cast_slice(&[self.camera_uniform]));
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);

            self.depth_texture = texture::create_depth_texture(&self.device, &self.config, "Depth Texture");
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
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment { 
                    view: &self.depth_texture, 
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: wgpu::StoreOp::Store,
                    }), 
                    stencil_ops: None, 
                }),
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            render_pass.set_pipeline(&self.render_pipeline);

            // ---> Set bind groups:
            render_pass.set_bind_group(0, &self.camera_bind_group, &[]);

            // ---> Bind Buffers:
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);

            // Drawing with indices:
            render_pass.draw_indexed(0..self.num_indices, 0, 0..1);
        } // End of Render Pass

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();
        Ok(())
    }
}
//===== GFX STATE STRUCTURE =====//

