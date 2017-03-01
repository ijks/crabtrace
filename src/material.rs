use std::fmt::{Debug, Formatter, Error};

use texture::Texture;

#[derive(Clone, Copy, Debug)]
pub enum MaterialType {
    Solid {
        specularity: f32,
    },
    Dielectric {
        ior: f32,
        opacity: f32,
    },
}

pub struct Material {
    pub texture: Box<Texture>,
    pub material_type: MaterialType,
}

impl Material {
    pub fn new<T: Texture + 'static>(texture: T, material_type: MaterialType) -> Self {
        Material {
            texture: Box::new(texture),
            material_type: material_type,
        }
    }

    pub fn solid<T: Texture + 'static>(texture: T, specularity: f32) -> Self {
        Material::new(texture, MaterialType::Solid { specularity: specularity })
    }

    pub fn dielectric<T: Texture + 'static>(texture: T, ior: f32, opacity: f32) -> Self {
        Material::new(texture,
                      MaterialType::Dielectric {
                          ior: ior,
                          opacity: opacity,
                      })
    }
}

impl Debug for Material {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        f.debug_struct("Material")
            .field("material_type", &self.material_type)
            .field("texture", &"<trait object>")
            .finish()
    }
}
