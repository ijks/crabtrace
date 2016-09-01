use cgmath::prelude::*;
use cgmath::vec3;

use math::*;
use ray::Ray;

#[derive(Clone, Debug)]
pub struct Plane {
    pub normal: Vector,
    pub offset: Point,
}

impl Plane {
    pub fn intersect(&self, ray: &Ray) -> Option<(f32, Point, Vector)> {
        let denom = self.normal.dot(ray.direction);

        if denom.abs() > EPSILON {
            let distance = (self.offset - ray.origin).dot(self.normal) / denom;

            if distance < 0.0 {
                return None;
            }

            Some((distance, ray.evaluate(distance), self.normal))
        } else {
            None
        }
    }

    pub fn texture_map(&self, position: Point) -> UVCoords {
        // Create 2 vectors perpendicular to the plane normal.
        let basis_x = if self.normal.z == 0.0 {
            vec3(-self.normal.y, self.normal.x, self.normal.z)
        } else {
            vec3(-self.normal.z, self.normal.y, self.normal.z)
        };
        let basis_y = basis_x.cross(self.normal);

        let position = position - self.offset;

        (position.dot(basis_x), position.dot(basis_y))
    }
}
