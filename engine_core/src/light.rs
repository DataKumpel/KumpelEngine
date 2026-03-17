use bytemuck::{Pod, Zeroable};


//===== LIGHT UNIFORM STRUCTURE =====//
#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub struct LightUniform {
    pub position: [f32; 4], // xyz + 1 x padding
    pub color: [f32; 4],    // rgba
}

impl LightUniform {
    pub fn new(position: [f32; 3], color: [f32; 3]) -> Self {
        Self { 
            position: [position[0], position[1], position[2], 1.0], 
            color: [color[0], color[1], color[2], 1.0], 
        }
    }
}
//===== LIGHT UNIFORM STRUCTURE =====//

