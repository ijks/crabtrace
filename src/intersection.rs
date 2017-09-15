use math::*;

/// Represents an intersection between a `Ray` and another object.
#[derive(Clone, Copy, Debug)]
pub struct Intersection {
    pub distance: f32,
    pub position: Point,
    pub normal: Vector,
    pub inside: bool,
}
