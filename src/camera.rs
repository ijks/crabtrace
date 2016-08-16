use cgmath::prelude::*;

use math::*;
use ray::Ray;

#[derive(Clone, Copy)]
pub struct Camera {
    position: Point,
    direction: Vector,
    fov: f32,

    // Order: top left, top right, bottom left.
    screen_corners: (Point, Point, Point),
}

impl Camera {
    pub fn new(position: Point, direction: Vector, fov: f32) -> Camera {
        let center = position + direction * (1.0 / (0.5 * fov).tan());
        let left = direction.cross(Vector::unit_y()).normalize();
        let up = direction.cross(left).normalize();

        Camera {
            position: position,
            direction: direction,
            fov: fov,

            screen_corners: (center + up + left, center + up - left, center - up + left),
        }
    }

    pub fn position(&self) -> Point {
        self.position
    }

    pub fn screen_corners(&self) -> (Point, Point, Point) {
        self.screen_corners
    }

    /// Given coordinates on the screen plane, create a ray passing
    /// through those coordinates from the camera's origin.
    pub fn primary_ray(&self, (u, v): (f32, f32)) -> Ray {
        let (top_left, top_right, bottom_left) = self.screen_corners();
        let basis_x = top_right - top_left;
        let basis_y = bottom_left - top_left;
        let direction = (top_left + u * basis_x + v * basis_y - self.position()).normalize();

        Ray::new(self.position, direction)
    }
}
