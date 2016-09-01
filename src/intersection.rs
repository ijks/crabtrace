use material::Material;
use math::*;
use primitive::Primitive;
use ray::Ray;

/// Represents an intersection between a `Ray` and another object.
#[derive(Clone, Copy, Debug)]
pub struct Intersection<'p> {
    pub primitive: &'p Primitive,
    pub distance: f32,
    pub position: Point,
    pub normal: Vector,
}

impl<'p> Intersection<'p> {
    pub fn nearest(lhs: Self, rhs: Self) -> Intersection<'p> {
        if lhs.distance < rhs.distance {
            lhs
        } else {
            rhs
        }
    }
}

/// Something that can be intersected by a ray.
pub trait Intersect {
    /// Intersect a ray with this object. Returns `None` if there is no intersection.
    fn intersect(&self, ray: &Ray) -> Option<Intersection>;
}
