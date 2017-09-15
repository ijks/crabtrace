use cgmath::prelude::*;
use cgmath::{dot, vec3};

use intersection::Intersection;
use material::Material;
use math::*;
use ray::Ray;

/// The shape a primitive has.
#[derive(Clone, Debug)]
pub enum Shape {
    Plane {
        normal: Vector,
        offset: Point,
    },
    Sphere {
        center: Point,
        radius: f32,
    },
    Triangle {
        vertices: [Point; 3],
    },
}

/// A basic object from which we can build larger objects and scenes.
#[derive(Debug)]
pub struct Primitive {
    pub material: Material,
    pub shape: Shape,
}

impl Primitive {
    fn new(material: Material, shape: Shape) -> Self {
        Primitive { material, shape }
    }

    pub fn plane(material: Material, offset: Point, normal: Vector) -> Self {
        Self::new(
            material,
            Shape::Plane {
                offset,
                normal,
            },
        )
    }

    pub fn sphere(material: Material, center: Point, radius: f32) -> Self {
        Self::new(
            material,
            Shape::Sphere {
                center,
                radius,
            },
        )
    }

    pub fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        match self.shape {
            Shape::Plane { normal, offset } =>
                intersect_plane(ray, normal, offset),
            Shape::Sphere { center, radius } =>
                intersect_sphere(ray, center, radius),
            Shape::Triangle { vertices } =>
                unimplemented!(),
        }
    }

    /// Get the corresponding UV coordinates for a point on (or near) the surface
    /// of a primitive.
    pub fn texture_map(&self, position: Point) -> UVCoords {
        match self.shape {
            Shape::Plane { offset, normal } => {
                // Create 2 vectors perpendicular to the plane normal.
                let basis_x = if normal.z == 0.0 {
                    vec3(-normal.y, normal.x, normal.z)
                } else {
                    vec3(-normal.z, normal.y, normal.z)
                };
                let basis_y = basis_x.cross(normal);

                let position = position - offset;

                (position.dot(basis_x), position.dot(basis_y))
            }
            Shape::Sphere { center, radius } => {
                // TODO
                (0.0, 0.0)
            }
            Shape::Triangle { vertices } => {
                // TODO
                unimplemented!()
            }
        }
    }
}

fn intersect_plane(ray: &Ray, normal: Vector, offset: Vector) -> Option<Intersection> {
    let denom = normal.dot(ray.direction);

    if denom.abs() > EPSILON {
        let distance = (offset - ray.origin).dot(normal) / denom;

        if distance < 0.0 {
            return None;
        }

        Some(Intersection {
            distance,
            position: ray.evaluate(distance),
            normal,
            inside: denom < 0.0,
        })
    } else {
        None
    }
}

fn intersect_sphere(ray: &Ray, center: Point, radius: f32) -> Option<Intersection> {
    let distance = center - ray.origin;
    let ray_length = distance.dot(ray.direction);
    let perpendicular = distance - ray_length * ray.direction;

    if perpendicular.magnitude2() > radius.powi(2) {
        return None;
    }

    let intersection_length =
        ray_length - (radius.powi(2) - perpendicular.magnitude2()).sqrt();

    if intersection_length > 0.0 {
        let position = ray.evaluate(intersection_length);
        let normal = (position - center).normalize();
        Some(Intersection {
            distance: intersection_length,
            position,
            normal,
            inside: normal.dot(ray.direction) < 0.0,
        })
    } else {
        None
    }
}

fn intersect_triangle(ray: &Ray, vertices: [Point; 3]) -> Option<Intersection> {
    unimplemented!()
}
