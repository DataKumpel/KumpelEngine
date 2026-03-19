use bytemuck::{Pod, Zeroable};
use glam::{Mat4, Vec3};
use winit::keyboard::KeyCode;

use crate::input::InputState;


// ---> Correction matrix for WGPU (z in range 0 to 1), OpenGL (z in range -1 to 1)
pub const OGL_2_WGPU: Mat4 = Mat4::from_cols_array(&[
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0, 
    0.0, 0.0, 0.5, 0.0, 
    0.0, 0.0, 0.5, 1.0,
]);


//===== CAMERA STRUCTURE =====//
pub struct Camera {
    pub eye: Vec3,
    pub target: Vec3,
    pub up: Vec3,
    pub aspect: f32,
    pub fovy: f32,
    pub clip_near: f32,
    pub clip_far: f32,
}

impl Camera {
    pub fn build_view_projection_matrix(&self) -> Mat4 {
        let view = Mat4::look_at_rh(self.eye, self.target, self.up);
        let proj = Mat4::perspective_rh(self.fovy, self.aspect, self.clip_near, self.clip_far);
        OGL_2_WGPU * proj * view
    }
}
//===== CAMERA STRUCTURE =====//


//===== CAMERA UNIFORM STRUCTURE =====//
#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub struct CameraUniform {
    pub view_proj: [f32; 4*4],
}

impl CameraUniform {
    pub fn new() -> Self {
        Self { view_proj: Mat4::IDENTITY.to_cols_array() }
    }

    pub fn update_view_proj(&mut self, camera: &Camera) {
        self.view_proj = camera.build_view_projection_matrix().to_cols_array();
    }
}
//===== CAMERA UNIFORM STRUCTURE =====//


//===== CAMERA CONTROLLER STRUCTURE =====//
pub struct CameraController {
    speed: f32,
}

impl CameraController {
    pub fn new(speed: f32) -> Self {
        Self { speed }
    }

    pub fn update_camera(&self, camera: &mut Camera, input: &InputState, dt: f32) {
        // ---> Calculate forward_norm and right vector:
        let mut forward = camera.target - camera.eye;
        forward.y = 0.0;
        let forward_mag = forward.length();
        let forward_norm = if forward_mag > 0.0 {
            forward / forward_mag
        } else {
            forward
        };
        let right = forward_norm.cross(camera.up).normalize();
        
        // ---> Calculate velocity:
        let velocity = self.speed * dt;

        // ---> Handle inputs:
        if input.is_key_pressed(KeyCode::KeyW) {
            camera.eye += forward_norm * velocity;
            camera.target += forward_norm * velocity;
        }
        if input.is_key_pressed(KeyCode::KeyS) {
            camera.eye -= forward_norm * velocity;
            camera.target -= forward_norm * velocity;
        }
        if input.is_key_pressed(KeyCode::KeyA) {
            camera.eye -= right * velocity;
            camera.target -= right * velocity;
        }
        if input.is_key_pressed(KeyCode::KeyD) {
            camera.eye += right * velocity;
            camera.target += right * velocity;
        }
    }
}
//===== CAMERA CONTROLLER STRUCTURE =====//
