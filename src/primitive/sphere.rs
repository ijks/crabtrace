use cgmath::prelude::*;
use cgmath::dot;

use math::*;
use ray::Ray;

#[derive(Debug)]
pub struct Sphere {
    pub position: Point,
    pub radius: f32,
}

impl Sphere {
    pub fn intersect(&self, ray: &Ray) -> Option<(f32, Point, Vector)> {
        let distance = self.position - ray.origin;
        let ray_length = dot(distance, ray.direction);
        let perpendicular = distance - ray_length * ray.direction;

        if perpendicular.magnitude2() > self.radius.powi(2) {
            return None;
        }

        let intersection_length = ray_length -
                                  (self.radius.powi(2) - perpendicular.magnitude2()).sqrt();

        if intersection_length > 0.0 {
            let position = ray.evaluate(intersection_length);
            Some((intersection_length, position, (position - self.position).normalize()))
        } else {
            None
        }
    }
}
