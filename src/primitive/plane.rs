use cgmath::dot;

use math::*;
use intersection::Intersection;
use primitive::Primitive;
use ray::Ray;

pub struct Plane {
    pub normal: Vector,
    pub offset: Point,
}

impl Plane {
    fn new(normal: Vector, offset: Point) -> Plane {
        Plane {
            normal: normal,
            offset: offset,
        }
    }
}

impl Primitive for Plane {
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
