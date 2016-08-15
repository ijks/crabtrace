use intersection::Intersection;
use ray::Ray;

mod sphere;
mod plane;

pub use self::sphere::*;
pub use self::plane::*;

pub trait Primitive {
    fn intersect(&self, ray: Ray) -> Option<Intersection>;
}
