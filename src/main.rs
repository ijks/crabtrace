extern crate cgmath;
extern crate image;

use std::io;

use cgmath::prelude::*;
use cgmath::vec3;

use camera::Camera;
use math::*;
use primitive::Sphere;
use ray::Ray;
use raytracer::Raytracer;
use scene::Scene;

mod camera;
mod color;
mod intersection;
mod math;
mod primitive;
mod ray;
mod raytracer;
mod scene;

fn main() {
    let mut scene = Scene::new();
    scene.add_primitive(Sphere::new(vec3(0.0, 0.0, 2.0), 1.0));
    let raytracer = Raytracer {
        scene: scene,
        camera: Camera::new(Point::zero(), Vector::unit_z(), 90.0),
    };

    let size = (512, 512);
    let mut buffer = image::ImageBuffer::new(size.0, size.1);
    for (x, y, pixel) in buffer.enumerate_pixels_mut() {
        let (top_left, top_right, bottom_left) = raytracer.camera.screen_corners();

        let basis_x = top_right - top_left;
        let basis_y = bottom_left - top_left;
        let (u, v) = (x as f32 / size.0 as f32, y as f32 / size.1 as f32);

        let ray = Ray {
            origin: raytracer.camera.position(),
            direction: top_left + u * basis_x + v * basis_y - raytracer.camera.position(),
        };
        let result = raytracer.trace(ray, 1);

        *pixel = result.color.into_pixel();
    }

    let mut output = io::stdout();
    image::ImageRgb8(buffer).save(&mut output, image::PNG);
}
