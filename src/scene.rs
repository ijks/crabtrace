use intersection::{Intersect, Intersection};
use primitive::Primitive;
use ray::Ray;

pub struct Scene {
    primitives: Vec<Primitive>,
}

impl Scene {
    pub fn new() -> Scene {
        Scene { primitives: Vec::new() }
    }

    pub fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        let mut nearest = None;

        for intersection in self.primitives.iter().filter_map(|p| p.intersect(ray)) {
            nearest =
                Some(nearest.map_or(intersection, |i| Intersection::nearest(i, intersection)));
        }

        nearest
    }

    pub fn add_primitive(&mut self, primitive: Primitive) {
        self.primitives.push(primitive);
    }
}
