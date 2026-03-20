use bytemuck::{Pod, Zeroable};
use glam::{Mat4, Quat, Vec3};

use crate::assets::TextureHandle;


//***** TRANSFORM STRUCTURE ***********************************************************************
pub struct Transform {
    pub position: Vec3,
    pub rotation: Quat,
    pub scale: Vec3,
}

impl Transform {
    pub fn new(position: Vec3) -> Self {
        Self { 
            position, 
            rotation: Quat::IDENTITY, 
            scale: Vec3::ONE, 
        }
    }

    pub fn to_matrix(&self) -> Mat4 {
        Mat4::from_scale_rotation_translation(self.scale, self.rotation, self.position)
    }
}
//***** TRANSFORM STRUCTURE ***********************************************************************


//***** INSTANCE RAW STRUCTURE ********************************************************************
#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct InstanceRaw {
    pub model: [[f32; 4]; 4], // 4x4-matrix
}

impl InstanceRaw {
    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout { 
            array_stride: std::mem::size_of::<InstanceRaw>() as wgpu::BufferAddress, 
            step_mode: wgpu::VertexStepMode::Instance, 
            attributes: &[
                wgpu::VertexAttribute { offset: 16 * 0, shader_location: 5, format: wgpu::VertexFormat::Float32x4 },
                wgpu::VertexAttribute { offset: 16 * 1, shader_location: 6, format: wgpu::VertexFormat::Float32x4 },
                wgpu::VertexAttribute { offset: 16 * 2, shader_location: 7, format: wgpu::VertexFormat::Float32x4 },
                wgpu::VertexAttribute { offset: 16 * 3, shader_location: 8, format: wgpu::VertexFormat::Float32x4 },
            ], 
        }
    }
}
//***** INSTANCE RAW STRUCTURE ********************************************************************


//***** MATERIAL STRUCTURE ************************************************************************
pub struct Material {
    pub diffuse_texture: TextureHandle
}
//***** MATERIAL STRUCTURE ************************************************************************


//***** POINT LIGHT STRUCTURE *********************************************************************
pub struct PointLight {
    pub color: glam::Vec3,
    pub radius: f32,
}

impl PointLight {
    pub fn new(color: glam::Vec3, radius: f32) -> Self {
        Self { color, radius }
    }
}
//***** POINT LIGHT STRUCTURE *********************************************************************


