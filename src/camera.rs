use cgmath::prelude::*;

use math::*;

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
}
