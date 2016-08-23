use cgmath::dot;

use math::*;
use ray::Ray;

#[derive(Debug)]
pub struct Plane {
    pub normal: Vector,
    pub offset: Point,
}

impl Plane {
    pub fn intersect(&self, ray: &Ray) -> Option<(f32, Point, Vector)> {
        let denom = dot(self.normal, ray.direction);

        if denom.abs() > EPSILON {
            let distance = dot(self.offset - ray.origin, self.normal) / denom;

            if distance < 0.0 {
                return None;
            }

            Some((distance, ray.evaluate(distance), self.normal))
        } else {
            None
        }
    }
}
