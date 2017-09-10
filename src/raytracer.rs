use camera::Camera;
use color::Color;
use material::MaterialType;
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

        if let Some((intersection, object)) = self.scene.intersect(&ray) {
            let surface_color =
                object.material.texture
                    .sample(object.texture_map(intersection.position));
            let irradiance = self.scene.irradiance_at(&intersection);

            match object.material.material_type {
                MaterialType::Solid { specularity } => {
                    let reflection = if specularity > 0.0 {
                        self.trace(
                            ray.reflect(intersection.normal, intersection.position),
                            max_depth - 1,
                        ) * surface_color
                    } else {
                        color!(0.0)
                    };

                    let diffuse = irradiance * surface_color;

                    reflection * specularity + diffuse * (1.0 - specularity)
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
