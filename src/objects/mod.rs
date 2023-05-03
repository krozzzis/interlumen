mod plane;
mod sphere;

pub use plane::Plane;
pub use sphere::Sphere;

use crate::color::Color;
use crate::vec::Vec3;

pub trait Distance {
    fn dist(&self, from: Vec3) -> f32;
}

pub trait Colorable {
    fn color(&self) -> Color;
}

pub trait Position {
    fn pos(&self) -> Vec3;
}

pub trait Normal {
    fn norm(&self, point: Vec3) -> Vec3;
}

pub trait Object: Distance + Colorable + Position + Normal {}
