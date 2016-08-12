use math::*;
use ray::Ray;

#[derive(Copy, Clone)]
pub struct Intersection {
    pub distance: f32,
    pub position: Point,
    pub normal: Vector,
}

impl Intersection {
    pub fn nearest(lhs: Self, rhs: Self) -> Intersection {
        if lhs.distance > rhs.distance {
            lhs
        } else {
            rhs
        }
    }
}
