//***** IMPORTS ***********************************************************************************
use std::sync::Arc;
use std::collections::HashMap;

use wgpu::util::DeviceExt;
use winit::window::Window;

use crate::assets::{AssetManager, MeshHandle, TextureHandle};
use crate::components::InstanceRaw;
use crate::input::InputState;
use crate::mesh::{self, Mesh};
use crate::vertex::Vertex;
use crate::camera::{Camera, CameraController, CameraUniform};
use crate::texture;
use crate::light::LightUniform;
//***** IMPORTS ***********************************************************************************


//***** GFX STATE STRUCTURE ***********************************************************************
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
    pub cube_mesh: Mesh,
    pub instance_buffer: wgpu::Buffer,

    // Camera state:
    pub camera: Camera,
    pub camera_uniform: CameraUniform,
    pub camera_buffer: wgpu::Buffer,
    pub camera_bind_group: wgpu::BindGroup,
    pub camera_controller: CameraController,

    // Textures:
    pub depth_texture: wgpu::TextureView,
    pub texture_bind_group_layout: wgpu::BindGroupLayout,

    // Lights:
    pub light_bind_group: wgpu::BindGroup,
    pub light_buffer: wgpu::Buffer,
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

        // ---> Create light:
        let light_uniform = LightUniform::new([2.0, 10.0, 2.0], [1.0, 1.0, 1.0], 5.0);

        let light_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor{
            label: Some("Light Buffer"),
            contents: bytemuck::cast_slice(&[light_uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let light_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor { 
            label: Some("Light Bind Group Layout"), 
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer { 
                        ty: wgpu::BufferBindingType::Uniform, 
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
            ],
        });

        let light_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor { 
            label: Some("Light Bind Group"), 
            layout: &light_bind_group_layout, 
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: light_buffer.as_entire_binding(),
                },
            ],
        });

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

        let camera_controller = CameraController::new(5.0);

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

        // ---> Create Texture:
        let texture_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor { 
            label: Some("Texture Bind Group Layout"), 
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture { 
                        sample_type: wgpu::TextureSampleType::Float { filterable: true }, 
                        view_dimension: wgpu::TextureViewDimension::D2, 
                        multisampled: false, 
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ], 
        });

        // ---> Create a Mesh:
        let cube_mesh = Mesh::new(&device, mesh::CUBE_VERTICES, mesh::CUBE_INDICES);
        let instance_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Instance Buffer"),
            size: (std::mem::size_of::<InstanceRaw>() * 10000) as wgpu::BufferAddress,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        
        // ---> Configure render pipeline:
        let shader = device.create_shader_module(wgpu::include_wgsl!("triangle_shader.wgsl"));
        let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor { 
            label: Some("Render Pipeline Layout"), 
            bind_group_layouts: &[
                &camera_bind_group_layout,
                &texture_bind_group_layout,
                &light_bind_group_layout,
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
                buffers: &[Vertex::desc(), InstanceRaw::desc()], 
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
            cube_mesh,
            instance_buffer,
            camera,
            camera_bind_group,
            camera_buffer,
            camera_uniform,
            camera_controller,
            depth_texture,
            texture_bind_group_layout,
            light_bind_group,
            light_buffer,
        }
    }

    pub fn update(&mut self, input: &InputState, dt: f32) {
        self.camera_controller.update_camera(&mut self.camera, input, dt);
        self.camera_uniform.update_view_proj(&self.camera);
        self.queue.write_buffer(&self.camera_buffer, 0, bytemuck::cast_slice(&[self.camera_uniform]));
    }

    pub fn update_light(&mut self, position: glam::Vec3, color: glam::Vec3, radius: f32) {
        let light_uniform = LightUniform::new(position.into(), color.into(), radius);
        self.queue.write_buffer(&self.light_buffer, 0, bytemuck::cast_slice(&[light_uniform]));
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

    pub fn render(
        &mut self, 
        asset_manager: &AssetManager, 
        batches: &HashMap<(MeshHandle, TextureHandle), Vec<InstanceRaw>>,
    ) -> Result<(), wgpu::SurfaceError> {
        
        let output = self.surface.get_current_texture()?;
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
        
        // ---> Collect all instances and write to ONE buffer:
        let mut all_instances = Vec::new();
        let mut render_commands = Vec::new();

        for ((mesh_handle, tex_handle), instances) in batches {
            let start_idx = all_instances.len() as u32;
            all_instances.extend_from_slice(instances);
            let end_idx = all_instances.len() as u32;
            render_commands.push((*mesh_handle, *tex_handle, start_idx..end_idx));
        }

        self.queue.write_buffer(&self.instance_buffer, 0, bytemuck::cast_slice(&all_instances));

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
                            load: wgpu::LoadOp::Clear(wgpu::Color { r: 0.01, g: 0.01, b: 0.02, a: 1.0 }), // Background color...
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
            render_pass.set_bind_group(2, &self.light_bind_group, &[]);

            // ---> Bind Buffers:
            render_pass.set_vertex_buffer(1, self.instance_buffer.slice(..));

            // ---> Draw batched objects:
            for (mesh_handle, tex_handle, instance_range) in render_commands {
                if let (Some(mesh), Some(texture)) = (
                    asset_manager.get_mesh(mesh_handle), 
                    asset_manager.get_texture(tex_handle),
                ) {
                    // ---> Bind specific mesh (slot 0):
                    render_pass.set_vertex_buffer(0, mesh.vertex_buffer.slice(..));
                    render_pass.set_index_buffer(mesh.index_buffer.slice(..), wgpu::IndexFormat::Uint16);

                    // ---> Bind specific texture (slot 1):
                    render_pass.set_bind_group(1, &texture.bind_group, &[]);

                    // ---> DRAW!!!
                    render_pass.draw_indexed(0..mesh.num_indices, 0, instance_range);
                }
            }
        } // End of Render Pass

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();
        Ok(())
    }
}
//***** GFX STATE STRUCTURE ***********************************************************************

