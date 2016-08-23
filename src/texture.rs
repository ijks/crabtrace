use color::Color;

pub trait Texture {
    fn sample(&self, uv_coords: (f32, f32)) -> Color;
}

impl Texture for Color {
    fn sample(&self, _: (f32, f32)) -> Color {
        *self
    }
}

impl<F> Texture for F
    where F: Fn((f32, f32)) -> Color
{
    fn sample(&self, uv_coords: (f32, f32)) -> Color {
        self(uv_coords)
    }
}
