use color::Color;

pub trait Texture {
    fn sample(&self, u: f32, v: f32) -> Color;
}

impl Texture for Color {
    fn sample(&self, _: f32, _: f32) -> Color {
        *self
    }
}

impl<F> Texture for F
    where F: Fn(f32, f32) -> Color
{
    fn sample(&self, u: f32, v: f32) -> Color {
        self(u, v)
    }
}
