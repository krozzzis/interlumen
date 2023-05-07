use crate::Texturable;

use super::{Hittable, Normal, Object, Position};
use interlumen_core::Vec3;

#[derive(Debug, Clone, Copy, Default)]
pub struct Sphere {
    pos: Vec3,
    radius: f32,
    material: usize,
}

impl Sphere {
    pub fn new(pos: Vec3, radius: f32, material: usize) -> Self {
        Self {
            pos,
            radius,
            material,
        }
    }
}

impl Hittable for Sphere {
    fn dist(&self, from: Vec3) -> f32 {
        ((from - self.pos()).len() - self.radius).abs()
    }
}

impl Position for Sphere {
    fn pos(&self) -> Vec3 {
        self.pos
    }

    fn set_pos(&mut self, pos: Vec3) {
        self.pos = pos;
    }
}

impl Normal for Sphere {
    fn norm(&self, point: Vec3) -> Vec3 {
        (point - self.pos()).norm()
    }
}

impl Texturable for Sphere {
    fn uv(&self, point: Vec3) -> Vec3 {
        let d = self.pos - point;
        let u = 0.5 + d.2.atan2(d.0);
        let v = 0.5 + d.1.asin();
        Vec3(u, v, 0.0)
    }

    fn material(&self) -> usize {
        self.material
    }
}

impl Object for Sphere {}
