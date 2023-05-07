use super::{Hittable, Normal, Object, Position};
use interlumen_core::Vec3;

#[derive(Debug, Clone, Copy, Default)]
pub struct Sphere {
    x: f32,
    y: f32,
    z: f32,
    radius: f32,
}

impl Sphere {
    pub fn new(position: Vec3, radius: f32) -> Self {
        Self {
            x: position.0,
            y: position.1,
            z: position.2,
            radius,
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
        return Vec3(self.x, self.y, self.z);
    }
}

impl Normal for Sphere {
    fn norm(&self, point: Vec3) -> Vec3 {
        return (point - self.pos()).norm();
    }
}

impl Object for Sphere {}
