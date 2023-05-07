mod sphere;

pub use sphere::*;

use interlumen_core::Vec3;

pub trait Hittable {
    fn dist(&self, from: Vec3) -> f32;
}

pub trait Position {
    fn pos(&self) -> Vec3;
}

pub trait Normal {
    fn norm(&self, point: Vec3) -> Vec3;
}

pub trait Object: Hittable + Position + Normal + Sync {}
