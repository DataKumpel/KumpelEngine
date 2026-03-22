use wgpu::util::DeviceExt;

use crate::vertex::Vertex;


//***** TEXTURED CUBE *****************************************************************************
pub const CUBE_VERTICES: &[Vertex] = &[
    // Front:
    Vertex { pos: [-0.5, -0.5,  0.5], tex_coords: [0.0, 1.0], normal: [ 0.0,  0.0,  1.0] }, // 0
    Vertex { pos: [ 0.5, -0.5,  0.5], tex_coords: [1.0, 1.0], normal: [ 0.0,  0.0,  1.0] }, // 1
    Vertex { pos: [ 0.5,  0.5,  0.5], tex_coords: [1.0, 0.0], normal: [ 0.0,  0.0,  1.0] }, // 2
    Vertex { pos: [-0.5,  0.5,  0.5], tex_coords: [0.0, 0.0], normal: [ 0.0,  0.0,  1.0] }, // 3
    
    // Back:
    Vertex { pos: [-0.5,  0.5, -0.5], tex_coords: [1.0, 0.0], normal: [ 0.0,  0.0, -1.0] }, // 4
    Vertex { pos: [ 0.5,  0.5, -0.5], tex_coords: [0.0, 0.0], normal: [ 0.0,  0.0, -1.0] }, // 5
    Vertex { pos: [ 0.5, -0.5, -0.5], tex_coords: [0.0, 1.0], normal: [ 0.0,  0.0, -1.0] }, // 6
    Vertex { pos: [-0.5, -0.5, -0.5], tex_coords: [1.0, 1.0], normal: [ 0.0,  0.0, -1.0] }, // 7
    
    // Right:
    Vertex { pos: [ 0.5, -0.5, -0.5], tex_coords: [1.0, 1.0], normal: [ 1.0,  0.0,  0.0] }, // 8
    Vertex { pos: [ 0.5,  0.5, -0.5], tex_coords: [1.0, 0.0], normal: [ 1.0,  0.0,  0.0] }, // 9
    Vertex { pos: [ 0.5,  0.5,  0.5], tex_coords: [0.0, 0.0], normal: [ 1.0,  0.0,  0.0] }, // 10
    Vertex { pos: [ 0.5, -0.5,  0.5], tex_coords: [0.0, 1.0], normal: [ 1.0,  0.0,  0.0] }, // 11
    
    // Left:
    Vertex { pos: [-0.5, -0.5,  0.5], tex_coords: [1.0, 1.0], normal: [-1.0,  0.0,  0.0] }, // 12
    Vertex { pos: [-0.5,  0.5,  0.5], tex_coords: [1.0, 0.0], normal: [-1.0,  0.0,  0.0] }, // 13
    Vertex { pos: [-0.5,  0.5, -0.5], tex_coords: [0.0, 0.0], normal: [-1.0,  0.0,  0.0] }, // 14
    Vertex { pos: [-0.5, -0.5, -0.5], tex_coords: [0.0, 1.0], normal: [-1.0,  0.0,  0.0] }, // 15
    
    // Top:
    Vertex { pos: [ 0.5,  0.5, -0.5], tex_coords: [1.0, 0.0], normal: [ 0.0,  1.0,  0.0] }, // 16
    Vertex { pos: [-0.5,  0.5, -0.5], tex_coords: [0.0, 0.0], normal: [ 0.0,  1.0,  0.0] }, // 17
    Vertex { pos: [-0.5,  0.5,  0.5], tex_coords: [0.0, 1.0], normal: [ 0.0,  1.0,  0.0] }, // 18
    Vertex { pos: [ 0.5,  0.5,  0.5], tex_coords: [1.0, 1.0], normal: [ 0.0,  1.0,  0.0] }, // 19
    
    // Bottom:
    Vertex { pos: [ 0.5, -0.5,  0.5], tex_coords: [1.0, 0.0], normal: [ 0.0, -1.0,  0.0] }, // 20
    Vertex { pos: [-0.5, -0.5,  0.5], tex_coords: [0.0, 0.0], normal: [ 0.0, -1.0,  0.0] }, // 21
    Vertex { pos: [-0.5, -0.5, -0.5], tex_coords: [0.0, 1.0], normal: [ 0.0, -1.0,  0.0] }, // 22
    Vertex { pos: [ 0.5, -0.5, -0.5], tex_coords: [1.0, 1.0], normal: [ 0.0, -1.0,  0.0] }, // 23
];

pub const CUBE_INDICES: &[u16] = &[
     0,  1,  2,  2,  3,  0, // Front
     4,  5,  6,  6,  7,  4, // Back
     8,  9, 10, 10, 11,  8, // Right
    12, 13, 14, 14, 15, 12, // Left
    16, 17, 18, 18, 19, 16, // Top
    20, 21, 22, 22, 23, 20, // Bottom
];
//***** TEXTURED CUBE *****************************************************************************


//***** MESH STRUCTURE ****************************************************************************
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

    pub fn from_assets(device: &wgpu::Device, mesh_filename: &str) -> Result<Self, tobj::LoadError> {
        let filepath = format!("assets/models/{mesh_filename}");
        Self::from_obj(device, filepath.as_str())
    }

    pub fn from_obj(device: &wgpu::Device, path: &str) -> Result<Self, tobj::LoadError> {
        // ---> Load OBJ file:
        let load_options = tobj::LoadOptions { 
            single_index: true, 
            triangulate: true, 
            ignore_points: true, 
            ignore_lines: true, 
        };
        let (models, _materials) = tobj::load_obj(path, &load_options)?;

        let mesh = &models[0].mesh;

        // ---> Interleaving (build vertex data):
        let mut vertices = Vec::new();
        let num_vertices = mesh.positions.len() / 3; // flat array of [x, y, z, x, y, z, ...]

        for i in 0..num_vertices {
            // ---> Positions:
            let pos = [
                mesh.positions[i * 3],
                mesh.positions[i * 3 + 1],
                mesh.positions[i * 3 + 2],
            ];
            
            // ---> UV coordinates:
            let tex_coords = if !mesh.texcoords.is_empty() {
                [
                    mesh.texcoords[i * 2],
                    1.0 - mesh.texcoords[i * 2 + 1],  // Invert V because tobj uses top-left coordinates,
                                                      // but WGPU uses bottom-left coordinates...
                ]
            } else {
                [0.0, 0.0]
            };

            // ---> Normals:
            let normal = if !mesh.normals.is_empty() {
                [
                    mesh.normals[i * 3],
                    mesh.normals[i * 3 + 1],
                    mesh.normals[i * 3 + 2],
                ]
            } else {
                [0.0, 1.0, 0.0] // Fallback...
            };

            vertices.push(Vertex { pos, tex_coords, normal });
        }

        // ---> Extract index data:
        let indices: Vec<u16> = mesh.indices.iter().map(|&index| index as u16).collect();

        Ok(Self::new(device, &vertices, &indices))
    }
}
//***** MESH STRUCTURE ****************************************************************************
