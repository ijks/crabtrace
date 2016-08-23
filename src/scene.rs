use cgmath::prelude::*;
use cgmath::dot;

use color::Color;
use intersection::{Intersect, Intersection};
use light::{Light, LightType};
use math::*;
use primitive::Primitive;
use ray::Ray;

pub struct Scene {
    primitives: Vec<Primitive>,
    lights: Vec<Light>,

    pub ambient_color: Color,
}

impl Scene {
    pub fn new(ambient_color: Color) -> Scene {
        Scene {
            primitives: Vec::new(),
            lights: Vec::new(),
            ambient_color: ambient_color,
        }
    }

    pub fn add_primitive(&mut self, primitive: Primitive) {
        self.primitives.push(primitive);
    }

    pub fn add_light(&mut self, light: Light) {
        self.lights.push(light);
    }

    pub fn irradiance_at(&self, intersection: &Intersection) -> Color {
        let mut radiance = self.ambient_color;

        for light in self.lights.iter() {
            let direction = match light.light_type() {
                LightType::Point { position } => position - intersection.position,
                LightType::Directional { direction } => direction,
            };

            if dot(intersection.normal, direction) < 0.0 {
                continue; // Ignore lights that are on the other side of the surface.
            }

            let distance = direction.magnitude();
            let direction = direction / distance; // Normalize the direction.

            // We offset the ray origin a tiny amount towards the light, to prevent 'shadow acne.'
            let shadow_ray = Ray::new(intersection.position + EPSILON * direction, direction);

            if !self.is_occluded(&shadow_ray, distance) {
                radiance += light.color / distance.powi(2) * dot(intersection.normal, direction);
            }
        }

        radiance
    }

    pub fn is_occluded(&self, ray: &Ray, max_distance: f32) -> bool {
        self.primitives
            .iter()
            .filter_map(|p| p.intersect(ray))
            .any(|i| i.distance < max_distance)
    }
}

impl Intersect for Scene {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        let mut nearest = None;

        for intersection in self.primitives.iter().filter_map(|p| p.intersect(ray)) {
            nearest =
                Some(nearest.map_or(intersection, |i| Intersection::nearest(i, intersection)));
        }

        nearest
    }
}
