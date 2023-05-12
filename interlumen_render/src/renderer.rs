use crate::{Camera, BasicMaterial, Material, Object, Scene};
use interlumen_core::{Color, Ray, Vec3};

pub struct HitPayload<'a> {
    pub distance: f32,
    pub point: Vec3,
    pub object: &'a Box<dyn Object>,
}

pub struct RendererSettings {
    pub max_iter: usize,
    pub ray_depth: usize,
    pub max_dist: f32,
    pub hit_thres: f32,
    pub pixel_ratio: f32,
}

impl RendererSettings {
    pub fn new() -> Self {
        Self {
            max_iter: 80,
            ray_depth: 10,
            max_dist: 200.0,
            hit_thres: 0.01,
            pixel_ratio: 1.0,
        }
    }
}

pub struct Renderer {}

impl Renderer {
    pub const FALLBACK_MATERIAL: BasicMaterial = BasicMaterial { albedo: Color::new(1.0, 0.0, 1.0, 1.0), emit: Color::BLACK, roughness: 0.0};

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
        let mut incoming_color = Color::BLACK;
        let mut ray_color = Color::WHITE;

        let mut pixel_ray = camera.get_pixel_ray(x, y, screen_w, screen_h, settings.pixel_ratio);
        for _ in 0..settings.ray_depth {
            if let Some(payload) = Renderer::closest_hit(settings, &pixel_ray, scene) {
                let hit = payload.point;
                let obj = payload.object;

                let norm = obj.norm(hit);
                if let Some(a) = materials.get(obj.material()) {
                    let color = a.get_color(obj.uv(hit));
                    incoming_color += color.emit * ray_color;
                    ray_color *= color.albedo * (1.0 / (payload.distance*payload.distance));
                    let rand_dir = (Vec3(rand::random(), rand::random(), rand::random()) * color.roughness + norm).norm();
                    let new_dir = (rand_dir * (norm * rand_dir).signum()).norm();
                    pixel_ray = Ray {
                        origin: hit + norm*0.001,
                        dir: new_dir,
                    };
                }               

            } else {
                // incoming_color += Color::new(4.5, 4.5, 4.5, 1.0) * ray_color;
                break;
            }
        }
        incoming_color
    }

    pub fn closest_hit<'a>(
        settings: &RendererSettings,
        ray: &Ray,
        scene: &'a Vec<Box<dyn Object>>,
    ) -> Option<HitPayload<'a>> {
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
                return Some(HitPayload{distance: t, point: ray.origin + ray.dir * t, object: hit});
            } else if dist > settings.max_dist {
                break;
            }
        }
        None
    }
}
