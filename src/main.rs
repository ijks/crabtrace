extern crate cgmath;
extern crate image;

use std::fs::File;

use cgmath::prelude::*;
use cgmath::vec3;

use camera::Camera;
use color::rgb;
use light::Light;
use material::Material;
use math::*;
use primitive::Primitive;
use raytracer::Raytracer;
use scene::Scene;

mod camera;
mod color;
mod intersection;
mod light;
mod material;
mod math;
mod primitive;
mod ray;
mod raytracer;
mod scene;
mod texture;

fn main() {
    let mut scene = Scene::new(vec3(0.05, 0.05, 0.10).into());

    scene.add_primitive(Primitive::plane(Material::solid(rgb(1.0, 1.0, 1.0), 1.0),
                                         vec3(0.0, -1.0, 0.0),
                                         Vector::unit_y()));
    scene.add_primitive(Primitive::sphere(Material::solid(rgb(1.0, 0.0, 0.0), 1.0),
                                          vec3(0.0, 0.0, 2.0),
                                          1.0));

    scene.add_light(Light::point(vec3(1.5, 1.5, 1.5).into(), vec3(0.0, 1.0, 0.5)));

    let raytracer = Raytracer {
        scene: scene,
        camera: Camera::new(Point::zero(), Vector::unit_z(), 90.0f32.to_radians()),
    };

    let size = (512, 512);
    let mut buffer = image::ImageBuffer::new(size.0, size.1);
    for (x, y, pixel) in buffer.enumerate_pixels_mut() {
        let screen_coords = (x as f32 / size.0 as f32, y as f32 / size.1 as f32);
        let ray = raytracer.camera.primary_ray(screen_coords);
        let result = raytracer.trace(ray, 1);

        *pixel = result.color.into_pixel();
    }

    // TODO: add command line options to configure this.
    let mut output = File::create("output.png").expect("Couldn't create output file.");
    image::ImageRgb8(buffer).save(&mut output, image::PNG).expect("Couldn't write to output file.");
}
