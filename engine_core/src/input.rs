use std::collections::HashSet;

use winit::{event::{ElementState, KeyEvent}, keyboard::{KeyCode, PhysicalKey}};


//===== INPUT STATE STRUCTURE =====//
#[derive(Default)]
pub struct InputState {
    pressed_keys: HashSet<KeyCode>,
}

impl InputState {
    pub fn new() -> Self {
        Self { pressed_keys: HashSet::new() }
    }

    /// This function gets called by the event-loop to update status.
    pub fn process_keyboard_event(&mut self, event: &KeyEvent) {
        let is_pressed = event.state == ElementState::Pressed;

        if let PhysicalKey::Code(keycode) = event.physical_key {
            if is_pressed {
                self.pressed_keys.insert(keycode);
            } else {
                self.pressed_keys.remove(&keycode);
            }
        }
    }

    /// Other systems (like the camera) can ask if certain keys are pressed.
    pub fn is_key_pressed(&self, keycode: KeyCode) -> bool {
        self.pressed_keys.contains(&keycode)
    }
}
//===== INPUT STATE STRUCTURE =====//
