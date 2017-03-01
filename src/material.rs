use std::fmt::{Debug, Formatter, Error};

use texture::Texture;

/// The kind of material a primitive has.
#[derive(Clone, Copy, Debug)]
pub enum MaterialType {
    /// A solid object that doesn't let light through, but does reflect it.
    Solid {
        specularity: f32,
    },

    /// A transparent object that refracts and reflects light according to
    /// Snell's law and Fresnel's law, respectively.
    Dielectric {
        ior: f32,
        opacity: f32,
    },
}

/// The material of a primitive.
pub struct Material {
    pub texture: Box<Texture>,
    pub material_type: MaterialType,
}

impl Material {
    /// Create a new material.
    pub fn new<T: Texture + 'static>(texture: T, material_type: MaterialType) -> Self {
        Material {
            texture: Box::new(texture),
            material_type: material_type,
        }
    }

    /// Create a new solid material.
    pub fn solid<T: Texture + 'static>(texture: T, specularity: f32) -> Self {
        Material::new(texture, MaterialType::Solid { specularity: specularity })
    }

    /// Create a new dielectric (i.e. transparent) material.
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
