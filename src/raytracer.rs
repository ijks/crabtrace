use camera::Camera;
use color::Color;
use intersection::Intersect;
use ray::Ray;
use scene::Scene;

pub struct Raytracer {
    pub camera: Camera,
    pub scene: Scene,
}

impl Raytracer {
    pub fn trace(&self, ray: Ray, max_depth: u32) -> TraceResult {
        if let Some(intersection) = self.scene.intersect(&ray) {
            let primitive = intersection.primitive;
            let irradiance = self.scene.irradiance_at(&intersection);

            TraceResult {
                color: irradiance *
                       primitive.material
                    .texture
                    .sample(primitive.texture_map(intersection.position)),
            }
        } else {
            TraceResult { color: self.scene.ambient_color }
        }
    }
}

#[derive(Debug)]
pub struct TraceResult {
    pub color: Color,
}
