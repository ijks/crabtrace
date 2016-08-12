use camera::Camera;
use intersection::Intersection;
use primitive::Primitive;
use ray::Ray;

pub struct Scene {
    primitives: Vec<Box<Primitive>>,
}

impl Scene {
    pub fn new() -> Scene {
        Scene { primitives: Vec::new() }
    }

    pub fn intersect(&self, ray: Ray) -> Option<Intersection> {
        let mut nearest = None;

        for intersection in self.primitives.iter().filter_map(|p| p.intersect(ray)) {
            nearest =
                Some(nearest.map_or(intersection, |i| Intersection::nearest(i, intersection)));
        }

        nearest
    }

    pub fn add_primitive<P: Primitive + 'static>(&mut self, primitive: P) {
        self.primitives.push(Box::new(primitive));
    }
}
