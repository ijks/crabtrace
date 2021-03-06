#![feature(conservative_impl_trait)]

extern crate cgmath;
extern crate ordered_float;
extern crate png;

use std::fs::File;

use cgmath::prelude::*;
use cgmath::vec3;
use png::HasParameters;

use camera::Camera;
use light::Light;
use material::Material;
use math::*;
use primitive::Primitive;
use raytracer::Raytracer;
use scene::Scene;
use texture::checkerboard;

mod camera;
mod color;
mod intersection;
mod light;
mod material;
#[macro_use] mod math;
mod primitive;
mod ray;
mod raytracer;
mod scene;
mod texture;

fn main() {
    let raytracer = Raytracer {
        scene: example_scene(),
        camera: Camera::new(Point::zero(), Vector::unit_z(), 90.0f32.to_radians()),
    };

    let (w, h) = (1024, 1024);
    let mut output_data = Vec::with_capacity(w * h);
    let primary_rays = raytracer.camera.primary_rays();
    for y in 0 .. h {
        for x in 0 .. w {
            let screen_coords = (x as f32 / w as f32, y as f32 / h as f32);
            let ray = primary_rays.at(screen_coords);
            let result = raytracer.trace(ray, 10);

            let (r, g, b) = result.as_bytes();
            output_data.push(r);
            output_data.push(g);
            output_data.push(b);
        }
    }

    // TODO: add command line options to configure this.
    let output = File::create("output.png").expect("Couldn't create output file.");

    let mut encoder = png::Encoder::new(output, w as u32, h as u32);
    encoder.set(png::ColorType::RGB).set(png::BitDepth::Eight);
    encoder.write_header()
        .unwrap()
        .write_image_data(&output_data)
        .unwrap();
}

fn example_scene() -> Scene {
    let mut scene = Scene::new(color!(0.1, 0.1, 0.1));

    let white = color!(1.0, 1.0, 1.0);
    let red = color!(1.0, 0.0, 0.0);
    let green = color!(0.0, 1.0, 0.0);

    // floor
    scene.add_primitive(
        Primitive::plane(
            Material::solid(
                checkerboard(color!(1.0), color!(0.0), (0.5, 0.5)),
                0.0,
            ),
            vec3(0.0, -1.0, 0.0),
            Vector::unit_y(),
        )
    );

    // left
    scene.add_primitive(
        Primitive::plane(
            Material::solid(red, 0.0),
            vec3(-1.0, 0.0, 0.0),
            Vector::unit_x(),
        )
    );

    // right
    scene.add_primitive(
        Primitive::plane(
            Material::solid(green, 0.0),
            vec3(1.0, 0.0, 0.0),
            -Vector::unit_x(),
        )
    );

    // ceiling
    scene.add_primitive(
        Primitive::plane(
            Material::solid(white, 0.0),
            vec3(0.0, 1.0, 0.0),
            -Vector::unit_y(),
        )
    );

    // back
    scene.add_primitive(
        Primitive::plane(
            Material::solid(white, 0.0),
            vec3(0.0, 0.0, 2.0),
            -Vector::unit_z(),
        )
    );

    //front
    scene.add_primitive(
        Primitive::plane(
            Material::solid(white, 0.0),
            vec3(0.0, 0.0, -2.0),
            Vector::unit_z(),
        )
    );

    scene.add_primitive(
        Primitive::sphere(
            Material::solid(color!(0.1, 0.1, 0.1), 0.6),
            vec3(0.0, 0.0, 2.0),
            1.0,
        )
    );

    scene.add_light(
        Light::point(color!(4.0, 4.0, 4.0), vec3(0.0, 0.0, 0.0)),
    );

    scene
}
