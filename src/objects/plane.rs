use crate::color::Color;
use crate::objects::{Colorable, Distance, Normal, Object, Position};
use crate::vec::Vec3;

#[derive(Debug, Clone, Copy, Default)]
pub struct Plane {
    x: f32,
    y: f32,
    z: f32,
    color: Color,
}

impl Plane {
    pub fn new(position: Vec3, color: Color) -> Self {
        Self {
            x: position.0,
            y: position.1,
            z: position.2,
            color,
        }
    }
}

impl Distance for Plane {
    fn dist(&self, from: Vec3) -> f32 {
        Vec3(0.0, 1.0, 0.0) * (from - self.pos())
    }
}

impl Colorable for Plane {
    fn color(&self) -> Color {
        return self.color;
    }
}

impl Position for Plane {
    fn pos(&self) -> Vec3 {
        return Vec3(self.x, self.y, self.z);
    }
}

impl Normal for Plane {
    fn norm(&self, point: Vec3) -> Vec3 {
        return Vec3(0.0, 1.0, 0.0);
    }
}

impl Object for Plane {}
