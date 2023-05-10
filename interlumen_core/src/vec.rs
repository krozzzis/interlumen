use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

use crate::quaternion::Quaternion;

#[derive(Debug, Clone, Copy, Default)]
pub struct Vec3(pub f32, pub f32, pub f32);

impl Vec3 {
    pub fn len(&self) -> f32 {
        (*self * *self).sqrt()
    }

    pub fn norm(&self) -> Self {
        *self / self.len()
    }

    pub fn abs(&self) -> Self {
        Self(self.0.abs(), self.1.abs(), self.2.abs())
    }

    pub fn clamp_ceil(&self, min: f32) -> Self {
        Self(self.0.max(min), self.1.max(min), self.2.max(min))
    }

    pub fn cross(&self, other: Self) -> Self {
        Self(
            self.1 * other.2 - self.2 * other.1,
            self.2 * other.0 - self.0 * other.2,
            self.0 * other.1 - self.1 * other.0,
        )
    }

    pub fn normalize(&mut self) {
        *self = *self / self.len()
    }

    pub fn reflect(&self, normal: Self) -> Self {
        *self - normal * (*self * normal * 2.0)
    }
}

impl Add<Vec3> for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

impl AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

impl Add<f32> for Vec3 {
    type Output = Self;

    fn add(self, other: f32) -> Self {
        Self(self.0 + other, self.1 + other, self.2 + other)
    }
}

impl AddAssign<f32> for Vec3 {
    fn add_assign(&mut self, other: f32) {
        *self = *self + other;
    }
}

impl Sub<Vec3> for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
}

impl SubAssign<Vec3> for Vec3 {
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}

impl Sub<f32> for Vec3 {
    type Output = Self;

    fn sub(self, other: f32) -> Self {
        Self(self.0 - other, self.1 - other, self.2 - other)
    }
}

impl SubAssign<f32> for Vec3 {
    fn sub_assign(&mut self, other: f32) {
        *self = *self - other;
    }
}

impl Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(self, other: f32) -> Self {
        Self(self.0 * other, self.1 * other, self.2 * other)
    }
}

impl MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, other: f32) {
        *self = *self * other;
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = f32;

    fn mul(self, other: Self) -> f32 {
        self.0 * other.0 + self.1 * other.1 + self.2 * other.2
    }
}

impl Div<f32> for Vec3 {
    type Output = Self;

    fn div(self, other: f32) -> Self {
        Self(self.0 / other, self.1 / other, self.2 / other)
    }
}

impl DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, other: f32) {
        *self = *self / other;
    }
}

impl Mul<Quaternion> for Vec3 {
    type Output = Self;

    fn mul(self, other: Quaternion) -> Self {
        (self.cross(other.im) - other.im.cross(self)) / 2.0
    }
}
