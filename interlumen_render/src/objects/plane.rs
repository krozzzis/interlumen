use crate::Texturable;

use super::{Hittable, Normal, Object, Position};
use interlumen_core::Vec3;

#[derive(Debug, Clone, Copy, Default)]
pub struct Plane {
    pos: Vec3,
    norm: Vec3,
    material: usize,
}

impl Plane {
    pub fn new(pos: Vec3, norm: Vec3, material: usize) -> Self {
        Self { pos, norm, material }
    }
}

impl Hittable for Plane {
    fn dist(&self, from: Vec3) -> f32 {
        self.norm * (from - self.pos())
    }
}

impl Position for Plane {
    fn pos(&self) -> Vec3 {
        self.pos
    }

    fn set_pos(&mut self, pos: Vec3) {
        self.pos = pos
    }
}

impl Normal for Plane {
    fn norm(&self, _point: Vec3) -> Vec3 {
        self.norm
    }
}

impl Texturable for Plane {
    fn uv(&self, point: Vec3) -> Vec3 {
        let a = point - self.pos;
        Vec3(a.0, a.2, 0.0)
    }

    fn material(&self) -> usize {
        self.material
    }
}

impl Object for Plane {}
