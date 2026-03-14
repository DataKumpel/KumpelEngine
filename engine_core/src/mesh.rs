use wgpu::util::DeviceExt;

use crate::vertex::Vertex;


//===== HELLO WORLD TRIANGLE =====//
#[allow(dead_code)]
pub const TRIANGLE_VERTICES: &[Vertex] = &[
    Vertex { pos: [ 0.0,  0.5, 0.0], color: [1.0, 0.0, 0.0] },
    Vertex { pos: [-0.5, -0.5, 0.0], color: [0.0, 1.0, 0.0] },
    Vertex { pos: [ 0.5, -0.5, 0.0], color: [0.0, 0.0, 1.0] },
];

#[allow(dead_code)]
pub const TRIANGLE_INDICES: &[u16] = &[0, 1, 2];
//===== HELLO WORLD TRIANGLE =====//


//===== HELLO WORLD CUBE =====//
#[allow(dead_code)]
pub const CUBE_VERTICES: &[Vertex] = &[
    Vertex { pos: [-0.5, -0.5,  0.5], color: [1.0, 0.0, 0.0] }, // 0
    Vertex { pos: [ 0.5, -0.5,  0.5], color: [0.0, 1.0, 0.0] }, // 1
    Vertex { pos: [ 0.5,  0.5,  0.5], color: [0.0, 0.0, 1.0] }, // 2
    Vertex { pos: [-0.5,  0.5,  0.5], color: [1.0, 1.0, 0.0] }, // 3
    Vertex { pos: [-0.5, -0.5, -0.5], color: [1.0, 0.0, 1.0] }, // 4
    Vertex { pos: [ 0.5, -0.5, -0.5], color: [0.0, 1.0, 1.0] }, // 5
    Vertex { pos: [ 0.5,  0.5, -0.5], color: [1.0, 1.0, 1.0] }, // 6
    Vertex { pos: [-0.5,  0.5, -0.5], color: [0.0, 0.0, 0.0] }, // 7
];

#[allow(dead_code)]
pub const CUBE_INDICES: &[u16] = &[
    0, 1, 2, 2, 3, 0, // front
    1, 5, 6, 6, 2, 1, // right
    7, 6, 5, 5, 4, 7, // back
    4, 0, 3, 3, 7, 4, // left
    4, 5, 1, 1, 0, 4, // bottom
    3, 2, 6, 6, 7, 3, // top
];
//===== HELLO WORLD CUBE =====//


//===== MESH STRUCTURE =====//
pub struct Mesh {
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub num_indices: u32,
}

impl Mesh {
    pub fn new(device: &wgpu::Device, vertices: &[Vertex], indices: &[u16]) -> Self {
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Mesh Vertex Buffer"),
            contents: bytemuck::cast_slice(vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Mesh Index Buffer"),
            contents: bytemuck::cast_slice(indices),
            usage: wgpu::BufferUsages::INDEX,
        });

        Self { vertex_buffer, index_buffer, num_indices: indices.len() as u32 }
    }
}
//===== MESH STRUCTURE =====//
