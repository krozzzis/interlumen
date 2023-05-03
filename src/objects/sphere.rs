use crate::color::Color;
use crate::objects::{Colorable, Distance, Normal, Object, Position};
use crate::vec::Vec3;

#[derive(Debug, Clone, Copy, Default)]
pub struct Sphere {
    x: f32,
    y: f32,
    z: f32,
    radius: f32,
    color: Color,
}

impl Sphere {
    pub fn new(position: Vec3, radius: f32, color: Color) -> Self {
        Self {
            x: position.0,
            y: position.1,
            z: position.2,
            radius,
            color,
        }
    }
}

impl Distance for Sphere {
    fn dist(&self, from: Vec3) -> f32 {
        ((from - self.pos()).len() - self.radius).abs()
    }
}

impl Colorable for Sphere {
    fn color(&self) -> Color {
        return self.color;
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
