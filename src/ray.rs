use math::{Point, Vector};

#[derive(Copy, Clone)]
pub struct Ray {
    pub origin: Point,
    pub direction: Vector,
}

impl Ray {
    pub fn evaluate(&self, distance: f32) -> Point {
        self.origin + self.direction * distance
    }
}
