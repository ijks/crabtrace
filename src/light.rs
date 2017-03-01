use color::Color;
use math::*;

/// The kind of light and its parameters.
#[derive(Clone, Copy, Debug)]
pub enum LightType {
    /// Light radiates outwards from a single point.
    Point {
        position: Point,
    },

    /// All light comes from the same direction.
    Directional {
        direction: Vector,
    },
}

/// A light source in our scene.
pub struct Light {
    pub color: Color,

    light_type: LightType,
}

impl Light {
    fn new(color: Color, light_type: LightType) -> Self {
        Light {
            color: color,
            light_type: light_type,
        }
    }

    pub fn point(color: Color, position: Point) -> Self {
        Light::new(color, LightType::Point { position: position })
    }

    pub fn directional(color: Color, direction: Vector) -> Self {
        Light::new(color, LightType::Directional { direction: direction })
    }

    pub fn light_type(&self) -> LightType {
        self.light_type
    }
}
