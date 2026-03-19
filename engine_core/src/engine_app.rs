use std::{
    collections::HashMap, 
    sync::Arc,
    time::Instant,
};

//===== IMPORTS =====//
use crate::{
    assets::{
        AssetManager, 
        TextureHandle,
    }, 
    components::{
        InstanceRaw, 
        Material, 
        Transform,
        PointLight,
    }, 
    gfx_state::GfxState, 
    input::InputState,
};
use glam::{
    Quat, 
    Vec3,
};
use hecs::World;
use winit::{
    application::ApplicationHandler, 
    event::WindowEvent, 
    event_loop::EventLoop, 
    keyboard::{
        KeyCode, 
        PhysicalKey,
    }, 
    window::Window,
};
//===== IMPORTS =====//


//===== ENGINE APP STRUCTURE =====//
pub struct EngineApp {
    gfx_state: Option<GfxState>,
    input_state: InputState,
    world: World,
    asset_manager: AssetManager,
    start_time: Instant,
}

impl EngineApp {
    pub fn new() -> Self {
        Self { 
            gfx_state: None, 
            input_state: InputState::new(),
            world: World::new(),
            asset_manager: AssetManager::new(),
            start_time: Instant::now(),
        }
    }

    pub fn run(mut self) {
        env_logger::init();
        let event_loop = EventLoop::new().expect("Couldn't create event loop");
        event_loop.run_app(&mut self).expect("Error in event loop");
    }
}

// Binding winit and engine together
impl ApplicationHandler for EngineApp {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        if self.gfx_state.is_none() {
            let window_attibs = Window::default_attributes().with_title("Kumpel Engine v0.0.1");
            let window = Arc::new(event_loop.create_window(window_attibs).unwrap());

            // ---> Block on initialising WebGPU because new is async:
            let state = pollster::block_on(GfxState::new(window));

            // ---> Load Texture via Asset Manager:
            //let diffuse_bytes = include_bytes!("../../sample_texture.png");
            //let texture = crate::texture::DiffuseTexture::from_memory(
            //    &state.device,
            //    &state.queue,
            //    diffuse_bytes,
            //    "test_texture",
            //    &state.texture_bind_group_layout,
            //).unwrap();
            let texture = crate::texture::DiffuseTexture::from_path(
                &state.device, 
                &state.queue, 
                "./sample_texture.png", 
                "test_texture", 
                &state.texture_bind_group_layout,
            ).expect("Couldn't load texture from disk!");
            let handle = self.asset_manager.add_texture(texture);

            //let diffuse_bytes_2 = include_bytes!("../../sample_texture2.png");
            //let texture2 = crate::texture::DiffuseTexture::from_memory(
            //    &state.device, 
            //    &state.queue, 
            //    diffuse_bytes_2, 
            //    "test_texture_2", 
            //    &state.texture_bind_group_layout,
            //).unwrap();
            let texture2 = crate::texture::DiffuseTexture::from_path(
                &state.device, 
                &state.queue, 
                "./sample_texture2.png", 
                "test_texture", 
                &state.texture_bind_group_layout,
            ).expect("Couldn't load texture from disk!");
            let handle2 = self.asset_manager.add_texture(texture2);

            // ---> Spawn 100 cubes in a 10x10 grid:
            for x in 0..10 {
                for z in 0..10 {
                    let position = Vec3::new(x as f32 * 2.0 - 10.0, 0.0, z as f32 * 2.0 -10.0);
                    self.world.spawn((
                        Transform::new(position),
                        Material { 
                            diffuse_texture: if (x * 10 + z) % 2 == 0 {
                                handle
                            } else {
                                handle2
                            }
                        }
                    ));
                }
            }

            // ---> Spawn a point-light:
            self.world.spawn((
                Transform::new(glam::Vec3::new(0.0, 5.0, 0.0)),
                PointLight::new(glam::Vec3::new(1.0, 1.0, 1.0)),
            ));

            self.gfx_state = Some(state);
        }
    }

    fn window_event(
            &mut self,
            event_loop: &winit::event_loop::ActiveEventLoop,
            _window_id: winit::window::WindowId,
            event: winit::event::WindowEvent,
        ) {
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(phys_size) => {
                if let Some(state) = &mut self.gfx_state {
                    state.resize(phys_size);
                }
            }
            WindowEvent::RedrawRequested => {
                if let Some(state) = &mut self.gfx_state {
                    state.update(&self.input_state);

                    let elapsed_time = self.start_time.elapsed().as_secs_f32();

                    // ---> Light System:
                    let mut light_pos = glam::Vec3::ZERO;
                    let mut light_color = glam::Vec3::ONE;

                    for (transform, light) in self.world.query_mut::<(&mut Transform, &PointLight)>() {
                        transform.position.x = elapsed_time.cos() * 10.0;
                        transform.position.z = elapsed_time.sin() * 10.0;
                        transform.position.y = 5.0 + (elapsed_time * 2.0).sin() * 2.0;

                        light_pos = transform.position;
                        light_color = light.color;
                    }
                    state.update_light(light_pos, light_color);

                    // ---> Group instances by material handle:
                    let mut batches: HashMap<TextureHandle, Vec<InstanceRaw>> = HashMap::new();

                    // ---> Collect all transforms and make into matrices:
                    for (transform, material) in self.world.query_mut::<(&mut Transform, &Material)>() {
                        //transform.rotation *= Quat::from_rotation_y(0.001); // Spinning Cubes!!!
                        let instance = InstanceRaw {
                            model: transform.to_matrix().to_cols_array_2d(),
                        };

                        batches.entry(material.diffuse_texture).or_default().push(instance);
                    }

                    match state.render(&self.asset_manager, &batches) {
                        Ok(_) => {}
                        Err(wgpu::SurfaceError::Lost) => state.resize(state.size),
                        Err(wgpu::SurfaceError::OutOfMemory) => event_loop.exit(),
                        Err(e) => log::error!("Render Error: {:?}", e),
                    }
                }
            }
            WindowEvent::KeyboardInput { event: key_event, .. } => {
                if key_event.physical_key == PhysicalKey::Code(KeyCode::Escape) {
                    event_loop.exit();
                }
                self.input_state.process_keyboard_event(&key_event);
            }
            _ => (),
        }
    }

    fn about_to_wait(&mut self, _event_loop: &winit::event_loop::ActiveEventLoop) {
        if let Some(state) = &self.gfx_state {
            state.window.request_redraw();
        }
    }
}
//===== ENGINE APP STRUCTURE =====//

