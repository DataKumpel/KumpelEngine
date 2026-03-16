use crate::texture::DiffuseTexture;


//===== HANDLES =====//
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TextureHandle(pub usize);
//===== HANDLES =====//


//===== ASSET MANAGER STRUCTURE =====//
pub struct AssetManager {
    textures: Vec<DiffuseTexture>,
}

impl AssetManager {
    pub fn new() -> Self {
        Self { textures: Vec::new() }
    }

    pub fn add_texture(&mut self, texture: DiffuseTexture) -> TextureHandle {
        let id = self.textures.len();
        self.textures.push(texture);
        TextureHandle(id)
    }

    pub fn get_texture(&self, handle: TextureHandle) -> Option<&DiffuseTexture> {
        self.textures.get(handle.0)
    }
}
//===== ASSET MANAGER STRUCTURE =====//

