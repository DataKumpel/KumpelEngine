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
        InstanceRaw, Material, PointLight, Transform
    }, 
    gfx_state::GfxState, 
    input::InputState, systems::{animate_light_system, rotate_cubes_system},
};
use glam::{
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
    last_frame_time: Instant,
}

impl EngineApp {
    pub fn new() -> Self {
        let now = Instant::now();

        Self { 
            gfx_state: None, 
            input_state: InputState::new(),
            world: World::new(),
            asset_manager: AssetManager::new(),
            start_time: now,
            last_frame_time: now,
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
            let texture = crate::texture::DiffuseTexture::from_path(
                &state.device, 
                &state.queue, 
                "./sample_texture.png", 
                "test_texture", 
                &state.texture_bind_group_layout,
            ).expect("Couldn't load texture from disk!");
            let handle = self.asset_manager.add_texture(texture);

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
                // ---> Calculate time:
                let now = Instant::now();
                let dt = (now - self.last_frame_time).as_secs_f32();
                self.last_frame_time = now;
                let total_time = self.start_time.elapsed().as_secs_f32();

                // ---> Update logics (ECS systems):
                rotate_cubes_system(&mut self.world, dt);
                let (light_pos, light_color) = animate_light_system(&mut self.world, total_time);

                if let Some(state) = &mut self.gfx_state {
                    // ---> Update camera:
                    state.update(&self.input_state, dt);

                    // ---> Prepare GPU data:
                    state.update_light(light_pos, light_color);

                    // ---> Group instances by material handle:
                    let mut batches: HashMap<TextureHandle, Vec<InstanceRaw>> = HashMap::new();

                    // ---> Collect all transforms and make into matrices:
                    for (transform, material) in self.world.query_mut::<(&mut Transform, &Material)>() {
                        let instance = InstanceRaw {
                            model: transform.to_matrix().to_cols_array_2d(),
                        };
                        batches.entry(material.diffuse_texture).or_default().push(instance);
                    }

                    // ---> Finally render to screen:
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

