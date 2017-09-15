use math::{Point, Vector};

use cgmath::prelude::*;
use cgmath::{ApproxEq, dot};

/// A ray of light traveling through our scene.
#[derive(Copy, Clone, Debug, PartialEq)]
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

    /// Get the position of traveling `distance` along the ray.
    pub fn evaluate(&self, distance: f32) -> Point {
        self.origin + self.direction * distance
    }

    /// Calculates the ray we get by reflecting it from `intersection` with the
    /// specified normal.
    pub fn reflect(&self, normal: Vector, intersection: Point) -> Ray {
        let new_direction =
            self.direction - 2f32 * dot(self.direction, normal) * normal;

        Ray {
            origin: intersection,
            direction: new_direction,
        }
    }

    pub fn refract(&self, normal: Vector, intersection: Point, ior_from: f32, ior_to: f32)
        -> Option<Ray> {
        let ratio = ior_from / ior_to;
        let cos_in = self.direction.dot(normal);
        let k = 1.0 - ratio.powi(2) - (1.0 - cos_in.powi(2));

        if k < 0.0 {
            return None;
        }

        let new_direction = ratio * self.direction + normal * (ratio * cos_in * k.sqrt());

        Some(Ray {
            origin: intersection,
            direction: new_direction,
        })
    }
}

#[cfg(test)]
mod tests {
    use cgmath::vec3;
    use super::*;

    #[test]
    fn test_reflect() {
        let ray = Ray::new(vec3(-1.0, 1.0, 0.0), vec3(1.0, -1.0, 0.0).normalize());
        let normal = vec3(0.0, 1.0, 0.0);
        let intersection = vec3(0.0, 0.0, 0.0);

        let result = Ray::new(vec3(0.0, 0.0, 0.0), vec3(1.0, 1.0, 0.0).normalize());
        assert_eq!(ray.reflect(normal, intersection), result);
    }
}
