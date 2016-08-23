use intersection::{Intersect, Intersection};
use math::*;
use ray::Ray;

#[derive(Debug)]
pub struct Triangle {
    pub vertices: [Point; 3],
}

impl Triangle {
    pub fn intersect(&self, ray: &Ray) -> Option<(f32, Point, Vector)> {
        unimplemented!()
    }
}
