use std::ops::{Add, AddAssign, Div, Mul};

use cgmath::prelude::*;
use cgmath::vec3;
use image;

use math::*;

#[derive(Clone, Copy, Debug)]
pub struct Color {
    inner: Vector,
}

impl Color {
    pub fn new(inner: Vector) -> Self {
        Color { inner: inner }
    }

    pub fn greyscale(value: f32) -> Self {
        Color::new(Vector::from_value(value))
    }

    pub fn rgb(r: f32, g: f32, b: f32) -> Self {
        Color::new(vec3(r, g, b))
    }

    pub fn from_bytes(r: u8, g: u8, b: u8) -> Self {
        Color::rgb((r as f32) / 255.0, (g as f32) / 255.0, (b as f32) / 255.0)
    }

    pub fn from_u32(color: u32) -> Self {
        let r = (color >> 24) as u8;
        let g = (color >> 16) as u8;
        let b = (color >> 8) as u8;

        Color::from_bytes(r, g, b)
    }

    pub fn into_pixel(self) -> image::Rgb<u8> {
        image::Rgb([convert_component(self.inner.x),
                    convert_component(self.inner.y),
                    convert_component(self.inner.z)])
    }
}

impl Add for Color {
    type Output = Color;

    fn add(self, rhs: Self) -> Self::Output {
        Color { inner: self.inner.add_element_wise(rhs.inner) }
    }
}

impl AddAssign for Color {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Mul for Color {
    type Output = Color;

    fn mul(self, rhs: Self) -> Self::Output {
        Color { inner: self.inner.mul_element_wise(rhs.inner) }
    }
}

impl Mul<f32> for Color {
    type Output = Color;

    fn mul(self, rhs: f32) -> Self::Output {
        Color { inner: self.inner * rhs }
    }
}

impl Div for Color {
    type Output = Color;

    fn div(self, rhs: Self) -> Self::Output {
        Color { inner: self.inner.div_element_wise(rhs.inner) }
    }
}

impl Div<f32> for Color {
    type Output = Color;

    fn div(self, rhs: f32) -> Self::Output {
        Color { inner: self.inner / rhs }
    }
}

impl From<Vector> for Color {
    fn from(vector: Vector) -> Self {
        Color::new(vector)
    }
}

impl Into<Vector> for Color {
    fn into(self) -> Vector {
        self.inner
    }
}

/// Helper function, for less verbose construction of colors.
pub fn rgb(r: f32, g: f32, b: f32) -> Color {
    Color::rgb(r, g, b)
}

fn convert_component(component: f32) -> u8 {
    map_range(clamp(component, 0.0, 1.0), 0.0..1.0, 0.0..255.0) as u8
}
