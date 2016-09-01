use color::Color;
use math::*;

pub trait Texture {
    fn sample(&self, uv_coords: UVCoords) -> Color;
}

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

pub fn checkerboard(color_a: Color, color_b: Color, scale: (f32, f32)) -> impl Fn(UVCoords) -> Color {
    let (scale_x, scale_y) = scale;

    move |(u, v)| if ((u / scale_x).floor() as i32 + (v / scale_y).floor() as i32) & 1 == 1 { color_a } else { color_b }
    // n & 1 == 1 is equivalent to n % 2 == 0
}
