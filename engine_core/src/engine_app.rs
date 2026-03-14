use std::sync::Arc;


//===== IMPORTS =====//
use crate::{gfx_state::GfxState, input::InputState};
use winit::{application::ApplicationHandler, event::WindowEvent, event_loop::EventLoop, keyboard::{KeyCode, PhysicalKey}, window::Window};
//===== IMPORTS =====//


//===== ENGINE APP STRUCTURE =====//
pub struct EngineApp {
    gfx_state: Option<GfxState>,
    input_state: InputState,
}

impl EngineApp {
    pub fn new() -> Self {
        Self { 
            gfx_state: None, 
            input_state: InputState::new(),
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

            // Block on initialising WebGPU because new is async:
            let state = pollster::block_on(GfxState::new(window));
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

                    match state.render() {
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

