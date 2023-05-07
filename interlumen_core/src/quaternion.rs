use crate::vec::Vec3;

#[derive(Debug, Clone, Copy)]
pub struct Quaternion {
    pub re: f32,
    pub im: Vec3,
}
