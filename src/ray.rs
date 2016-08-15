use math::{Point, Vector};

use cgmath::prelude::*;
use cgmath::ApproxEq;

#[derive(Copy, Clone)]
pub struct Ray {
    pub origin: Point,
    pub direction: Vector,
}

impl Ray {
    pub fn new(origin: Point, direction: Vector) -> Ray {
        debug_assert!(direction.magnitude().approx_eq(&1.0),
                      "Ray direction is not normalized. (|{:?}| == {:?} != 1.0)",
                      direction,
                      direction.magnitude());

        Ray {
            origin: origin,
            direction: direction,
        }
    }

    pub fn evaluate(&self, distance: f32) -> Point {
        self.origin + self.direction * distance
    }
}
