use camera::Camera;
use color::Color;
use intersection::Intersect;
use material::{Material, MaterialType};
use ray::Ray;
use scene::Scene;

/// The main state of our raytracer. Keeps track of the scene and camera.
pub struct Raytracer {
    pub camera: Camera,
    pub scene: Scene,
}

impl Raytracer {
    /// Trace a single ray into the scene and return the color of whatever it hit.
    pub fn trace(&self, ray: Ray, max_depth: u32) -> Color {
        if max_depth <= 0 {
            return Color::greyscale(0.0);
        }

        if let Some(intersection) = self.scene.intersect(&ray) {
            let primitive = intersection.primitive;
            let surface_color =
                primitive.material.texture
                    .sample(primitive.texture_map(intersection.position));
            let irradiance = self.scene.irradiance_at(&intersection);

            match primitive.material.material_type {
                MaterialType::Solid { specularity } => {
                    // Seems I actually meant for these materials to also reflect
                    // based on specularity. Well, that's what you get for not
                    // commenting I guess. So, TODO.
                    irradiance * surface_color
                }
                MaterialType::Dielectric { ior, opacity } => {
                    // TODO: Implement Snell, Fresnel laws
                    // For we now we just do perfect reflections.
                    let reflection = self.trace(
                        ray.reflect(intersection.normal, intersection.position),
                        max_depth - 1,
                    );

                    reflection * surface_color
                }
            }
        } else {
            self.scene.ambient_color
        }
    }
}
