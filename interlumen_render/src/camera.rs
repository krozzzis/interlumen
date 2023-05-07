use interlumen_core::{Ray, Vec3};

#[derive(Debug, Clone)]
pub struct Camera {
    pub pos: Vec3,
    pub dir: Vec3,
    pub fov: f32,
}

impl Camera {
    pub fn new(pos: Vec3, dir: Vec3, fov: f32) -> Self {
        Self { pos, dir, fov }
    }

    pub fn unit() -> Self {
        Self {
            pos: Vec3(0.0, 0.0, 0.0),
            dir: Vec3(0.0, 0.0, 1.0),
            fov: 90.0,
        }
    }

    pub fn get_pixel_ray(
        &self,
        x: usize,
        y: usize,
        screen_w: usize,
        screen_h: usize,
        pixel_ratio: f32,
    ) -> Ray {
        let cx = screen_w as f32 / 2.0;
        let cy = screen_h as f32 / 2.0;
        let ratio = screen_h as f32 / screen_w as f32;
        let hfov = (self.fov / 2.0).to_radians();
        let w = (x as f32 - cx) / cx;
        let h = -(y as f32 - cy) / cy * ratio * pixel_ratio;
        Ray::new(self.pos, Vec3(w, h, 1.0 / hfov.tan()).norm())
    }
}
