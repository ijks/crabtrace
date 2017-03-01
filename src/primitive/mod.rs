use intersection::{Intersect, Intersection};
use material::Material;
use math::*;
use ray::Ray;

use self::plane::Plane;
use self::sphere::Sphere;
use self::triangle::Triangle;

mod plane;
mod sphere;
mod triangle;

/// The kind and parameters of the shape a primitive has.
#[derive(Clone, Debug)]
enum PrimitiveType {
    Plane(plane::Plane),
    Sphere(sphere::Sphere),
    Triangle(triangle::Triangle),
}

/// A basic shape from which we can build larger objects and scenes.
#[derive(Debug)]
pub struct Primitive {
    pub material: Material,

    primitive_type: PrimitiveType,
}

impl Primitive {
    fn new(material: Material, primitive_type: PrimitiveType) -> Self {
        Primitive {
            material: material,
            primitive_type: primitive_type,
        }
    }

    pub fn plane(material: Material, offset: Point, normal: Vector) -> Self {
        Primitive::new(material,
                       PrimitiveType::Plane(Plane {
                           offset: offset,
                           normal: normal,
                       }))
    }

    pub fn sphere(material: Material, position: Point, radius: f32) -> Self {
        Primitive::new(material,
                       PrimitiveType::Sphere(Sphere {
                           position: position,
                           radius: radius,
                       }))
    }

    /// Get the corresponding UV coordinates for a point on (or near) the surface
    /// of a primitive.
    pub fn texture_map(&self, position: Point) -> UVCoords {
        match self.primitive_type {
            PrimitiveType::Plane(ref plane) => plane.texture_map(position),
            PrimitiveType::Sphere(ref sphere) => sphere.texture_map(position),
            PrimitiveType::Triangle(ref triangle) => triangle.texture_map(position),
        }
    }
}

impl Intersect for Primitive {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        match self.primitive_type {
                PrimitiveType::Plane(ref plane) => plane.intersect(ray),
                PrimitiveType::Sphere(ref sphere) => sphere.intersect(ray),
                PrimitiveType::Triangle(ref triangle) => triangle.intersect(ray),
            }
            .map(|(distance, position, normal)| {
                Intersection {
                    primitive: self,
                    distance: distance,
                    position: position,
                    normal: normal,
                }
            })
    }
}
