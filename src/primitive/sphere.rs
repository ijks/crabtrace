use cgmath::prelude::*;
use cgmath::dot;

use math::*;
use intersection::Intersection;
use primitive::Primitive;
use ray::Ray;

pub struct Sphere {
    pub position: Point,
    pub radius: f32,
}

impl Sphere {
    pub fn new(position: Point, radius: f32) -> Sphere {
        Sphere {
            position: position,
            radius: radius,
        }
    }
}

impl Primitive for Sphere {
    fn intersect(&self, ray: Ray) -> Option<Intersection> {
        let distance = self.position - ray.origin;
        let ray_length = dot(distance, ray.direction);
        let perpendicular = distance - ray_length * ray.direction;

        if perpendicular.magnitude2() > self.radius.powi(2) {
            return None;
        }

        let intersection_length = ray_length -
                                  (self.radius.powi(2) - perpendicular.magnitude2()).sqrt();

        if intersection_length <= 0.0 {
            return None;
        }

        let position = ray.evaluate(intersection_length);
        Some(Intersection {
            distance: intersection_length,
            position: position,
            normal: (position - self.position).normalize(),
        })
    }
}
