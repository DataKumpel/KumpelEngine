use bytemuck::{Pod, Zeroable};


//===== VERTEX STRUCTURE =====//
#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct Vertex {
    pub pos: [f32; 3],
    pub tex_coords: [f32; 2], // UV-coordinates...
    pub normal: [f32; 3],
}

impl Vertex {
    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,  // @location(0) in wgsl shader
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: 12,
                    shader_location: 1,  // @location(1) in wgsl shader
                    format: wgpu::VertexFormat::Float32x2,
                },
                wgpu::VertexAttribute {
                    offset: 20,
                    shader_location: 2,  // @location(2) in wgsl shader
                    format: wgpu::VertexFormat::Float32x3,
                }
            ],
        }
    }
}
//===== VERTEX STRUCTURE =====//
