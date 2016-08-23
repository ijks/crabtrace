use std::ops::{Fn, Range};

use cgmath;

pub type Vector = cgmath::Vector3<f32>;
pub type Point = Vector;

pub const EPSILON: f32 = 10e-5;

pub trait VectorExt {
    fn map_element_wise<F: Fn(f32) -> f32>(&self, func: F) -> Self;
}

impl VectorExt for Vector {
    fn map_element_wise<F: Fn(f32) -> f32>(&self, func: F) -> Self {
        Vector {
            x: func(self.x),
            y: func(self.y),
            z: func(self.z),
        }
    }
}

pub fn clamp(value: f32, min: f32, max: f32) -> f32 {
    f32::max(min, f32::min(max, value))
}

pub fn map_range(value: f32, from: Range<f32>, to: Range<f32>) -> f32 {
    let slope = (to.end - to.start) / (from.end - from.start);

    to.start + slope * (value - from.start)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clamp() {
        assert_eq!(clamp(1.0, 0.0, 2.0), 1.0);
        assert_eq!(clamp(10.0, 0.0, 2.0), 2.0);
        assert_eq!(clamp(-1.0, 0.0, 2.0), 0.0);
    }

    #[test]
    fn test_map_range() {
        assert_eq!(map_range(5.0, (0.0..10.0), (0.0..2.0)), 1.0);
        assert_eq!(map_range(0.0, (0.0..10.0), (0.0..2.0)), 0.0);
        assert_eq!(map_range(10.0, (0.0..10.0), (0.0..2.0)), 2.0);
    }
}
