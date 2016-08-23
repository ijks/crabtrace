use camera::Camera;
use color::Color;
use intersection::Intersect;
use ray::Ray;
use scene::Scene;

const MAX_DEPTH: f32 = 4.0;

pub struct Raytracer {
    pub camera: Camera,
    pub scene: Scene,
}

impl Raytracer {
    pub fn trace(&self, ray: Ray, max_depth: u32) -> TraceResult {
        if let Some(intersection) = self.scene.intersect(&ray) {
            let irradiance = self.scene.irradiance_at(&intersection);

            TraceResult { color: irradiance * intersection.material.texture.sample(0.0, 0.0) }
        } else {
            TraceResult { color: self.scene.ambient_color }
        }
    }
}

#[derive(Debug)]
pub struct TraceResult {
    pub color: Color,
}
