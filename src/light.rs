use color::Color;
use math::*;

#[derive(Clone, Copy, Debug)]
pub enum LightType {
    Point {
        position: Point,
    },
    Directional {
        direction: Vector,
    },
}

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
