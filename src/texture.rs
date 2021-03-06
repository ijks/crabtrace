use color::Color;
use math::*;

/// Something we can sample using UV coords to get a color.
pub trait Texture {
    fn sample(&self, uv_coords: UVCoords) -> Color;
}

/// A single color just means we always return it, no matter the coordinates
impl Texture for Color {
    fn sample(&self, _: UVCoords) -> Color {
        *self
    }
}

impl<F> Texture for F
    where F: Fn(UVCoords) -> Color
{
    fn sample(&self, uv_coords: UVCoords) -> Color {
        self(uv_coords)
    }
}

/// Create a function that procedurally generates a checkerboard texture.
pub fn checkerboard(color_a: Color, color_b: Color, scale: (f32, f32)) -> impl Fn(UVCoords) -> Color {
    let (scale_x, scale_y) = scale;

    move |(u, v)| {
        let index = (u / scale_x).floor() as i32 + (v / scale_y).floor() as i32;

        // n & 1 == 1 is equivalent to n % 2 == 0
        if index & 1 == 1 { color_a } else { color_b }
    }
}
