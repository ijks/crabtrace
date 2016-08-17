use intersection::{Intersect, Intersection};
use math::*;
use ray::Ray;

#[derive(Debug)]
pub struct Triangle {
    pub vertices: [Point; 3],
}

impl Intersect for Triangle {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        unimplemented!()
    }
}
