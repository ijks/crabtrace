use math::*;
use ray::Ray;

#[derive(Clone, Debug)]
pub struct Triangle {
    pub vertices: [Point; 3],
}

impl Triangle {
    pub fn intersect(&self, ray: &Ray) -> Option<(f32, Point, Vector)> {
        unimplemented!()
    }

    pub fn texture_map(&self, position: Point) -> UVCoords {
        unimplemented!()
    }
}
