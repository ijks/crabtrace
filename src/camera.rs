use cgmath::prelude::*;

use math::*;
use ray::Ray;

/// A camera in our scene, which will be the origin of our rays.
#[derive(Clone, Copy)]
pub struct Camera {
    pub position: Point,
    pub direction: Vector,
    pub fov: f32,
}

impl Camera {
    pub fn new(position: Point, direction: Vector, fov: f32) -> Camera {
        Camera { position, direction, fov }
    }

    pub fn primary_rays(&self) -> PrimaryRays {
        let center = self.position + self.direction * (1.0 / (0.5 * self.fov).tan());
        let left = self.direction.cross(Vector::unit_y()).normalize();
        let up = left.cross(self.direction).normalize();

        PrimaryRays {
            camera: self,
            screen_corners: (center + up + left, center + up - left, center - up + left),
        }
    }
}

pub struct PrimaryRays<'c> {
    camera: &'c Camera,
    // Order: top left, top right, bottom left.
    screen_corners: (Point, Point, Point),
}

impl<'c> PrimaryRays<'c> {
    pub fn screen_corners(&self) -> (Point, Point, Point) {
        self.screen_corners
    }

    pub fn at(&self, (u, v): (f32, f32)) -> Ray {
        let position = self.camera.position;
        let (top_left, top_right, bottom_left) = self.screen_corners;
        let basis_x = top_right - top_left;
        let basis_y = bottom_left - top_left;
        let direction = (top_left + u * basis_x + v * basis_y - position).normalize();

        Ray::new(position, direction)
    }
}
