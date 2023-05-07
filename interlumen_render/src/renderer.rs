use crate::{Camera, DiffuseMaterial, Material, Object, Scene};
use interlumen_core::{Color, Ray, Vec3};

pub struct RendererSettings {
    pub max_iter: usize,
    pub max_dist: f32,
    pub hit_thres: f32,
    pub pixel_ratio: f32,
}

impl RendererSettings {
    pub fn new() -> Self {
        Self {
            max_iter: 80,
            max_dist: 200.0,
            hit_thres: 0.01,
            pixel_ratio: 1.0,
        }
    }
}

pub struct Renderer {}

impl Renderer {
    pub fn render_pixel(
        settings: &RendererSettings,
        scene: &Scene,
        materials: &Vec<Box<dyn Material>>,
        x: usize,
        y: usize,
        screen_w: usize,
        screen_h: usize,
        camera: &Camera,
    ) -> Color {
        let pixel_ray = camera.get_pixel_ray(x, y, screen_w, screen_h, settings.pixel_ratio);

        if let Some((obj, hit)) = Renderer::closest_hit(settings, &pixel_ray, scene) {
            let light_ray = (Vec3(3.0, 5.0, 1.0) - hit).norm();
            let norm = obj.norm(hit);
            let mut light: f32 = light_ray * norm * 0.8 + 0.2;

            if let Some(_) = Renderer::closest_hit(
                settings,
                &Ray {
                    origin: hit,
                    dir: light_ray,
                },
                &scene,
            ) {
                light *= 0.1;
            }

            if let Some(a) = materials.get(obj.material()) {
                a.get_color(obj.uv(hit)) * light
            } else {
                DiffuseMaterial {
                    albedo: Color::new(1.0, 0.0, 1.0, 1.0),
                }
                .get_color(obj.uv(hit))
            }
        } else {
            Color::BLACK
        }
    }

    pub fn closest_hit<'a>(
        settings: &RendererSettings,
        ray: &Ray,
        scene: &'a Vec<Box<dyn Object>>,
    ) -> Option<(&'a Box<dyn Object>, Vec3)> {
        let mut t = 1.0;
        let mut hit = &scene[0];
        let mut i = 0;
        while i <= settings.max_iter {
            let mut dist: f32 = f32::MAX;
            for obj in scene {
                let d = obj.dist(ray.origin + ray.dir * t);
                if d <= dist {
                    dist = d;
                    hit = obj;
                }
            }
            i += 1;
            t += dist;
            if dist <= settings.hit_thres {
                return Some((hit, ray.origin + ray.dir * t));
            } else if dist > settings.max_dist {
                break;
            }
        }
        None
    }
}
