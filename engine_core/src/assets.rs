use crate::{mesh::Mesh, texture::DiffuseTexture};


//***** HANDLES ***********************************************************************************
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TextureHandle(pub usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MeshHandle(pub usize);
//***** HANDLES ***********************************************************************************


//***** ASSET MANAGER STRUCTURE *******************************************************************
pub struct AssetManager {
    textures: Vec<DiffuseTexture>,
    meshes: Vec<Mesh>,
}

impl AssetManager {
    pub fn new() -> Self {
        Self { 
            textures: Vec::new(),
            meshes: Vec::new(),
        }
    }

    pub fn add_texture(&mut self, texture: DiffuseTexture) -> TextureHandle {
        let id = self.textures.len();
        self.textures.push(texture);
        TextureHandle(id)
    }

    pub fn get_texture(&self, handle: TextureHandle) -> Option<&DiffuseTexture> {
        self.textures.get(handle.0)
    }

    pub fn add_mesh(&mut self, mesh: Mesh) -> MeshHandle {
        let id = self.meshes.len();
        self.meshes.push(mesh);
        MeshHandle(id)
    }

    pub fn get_mesh(&self, handle: MeshHandle) -> Option<&Mesh> {
        self.meshes.get(handle.0)
    }
}
//***** ASSET MANAGER STRUCTURE *******************************************************************

