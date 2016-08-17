use cgmath::dot;

use math::*;
use intersection::{Intersect, Intersection};
use ray::Ray;

#[derive(Debug)]
pub struct Plane {
    pub normal: Vector,
    pub offset: Point,
}

impl Intersect for Plane {
    fn intersect(&self, ray: Ray) -> Option<Intersection> {
        let denom = dot(self.normal, ray.direction);

        if denom > EPSILON {
            let distance = dot(self.offset - ray.origin, self.normal) / denom;

            if distance < 0.0 {
                return None;
            }

            Some(Intersection {
                distance: distance,
                position: ray.evaluate(distance),
                normal: self.normal,
            })
        } else {
            None
        }
    }
}
