use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

#[derive(Debug, Clone, Copy, Default)]
pub struct Color32 {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

#[allow(dead_code)]
impl Color32 {
    const BLACK: Self = Self::new(0, 0, 0, 255);
    const WHITE: Self = Self::new(255, 255, 255, 255);
    const GRAY: Self = Self::new(127, 127, 127, 255);
    const RED: Self = Self::new(255, 0, 0, 255);
    const GREEN: Self = Self::new(0, 255, 0, 255);
    const BLUE: Self = Self::new(0, 0, 255, 255);

    pub const fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }
}

impl Div<f32> for Color32 {
    type Output = Self;

    fn div(self, other: f32) -> Self {
        Self {
            r: self.r / other as u8,
            g: self.g / other as u8,
            b: self.b / other as u8,
            a: self.a / other as u8,
        }
    }
}

impl DivAssign<f32> for Color32 {
    fn div_assign(&mut self, other: f32) {
        *self = *self / other;
    }
}

impl Add<Color32> for Color32 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            r: self.r + other.r,
            g: self.g + other.g,
            b: self.b + other.b,
            a: self.a + other.a,
        }
    }
}

impl AddAssign<Color32> for Color32 {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

impl Sub<Color32> for Color32 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            r: self.r - other.r,
            g: self.g - other.g,
            b: self.b - other.b,
            a: self.a - other.a,
        }
    }
}

impl SubAssign<Color32> for Color32 {
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}

impl Mul<f32> for Color32 {
    type Output = Self;

    fn mul(self, other: f32) -> Self {
        Self {
            r: self.r * other as u8,
            g: self.g * other as u8,
            b: self.b * other as u8,
            a: self.a * other as u8,
        }
    }
}

impl MulAssign<f32> for Color32 {
    fn mul_assign(&mut self, other: f32) {
        *self = *self * other;
    }
}

impl PartialEq for Color32 {
    fn eq(&self, other: &Self) -> bool {
        self.r == other.r && self.g == other.g && self.b == other.b && self.a == other.a
    }
}

impl Eq for Color32 {}

#[derive(Debug, Clone, Copy, Default)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

#[allow(dead_code)]
impl Color {
    pub const BLACK: Self = Self::new(0.0, 0.0, 0.0, 1.0);
    pub const WHITE: Self = Self::new(1.0, 1.0, 1.0, 1.0);
    pub const GRAY: Self = Self::new(0.5, 0.5, 0.5, 1.0);
    pub const RED: Self = Self::new(1.0, 0.0, 0.0, 1.0);
    pub const GREEN: Self = Self::new(0.0, 1.0, 0.0, 1.0);
    pub const BLUE: Self = Self::new(0.0, 0.0, 1.0, 1.0);

    pub const fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }

    pub const fn new_value(value: f32, alpha: f32) -> Self {
        Self::new(value, value, value, alpha)
    }

    pub fn clamp(&self, min: f32, max: f32) -> Self {
        Self {
            r: self.r.clamp(min, max),
            g: self.g.clamp(min, max),
            b: self.b.clamp(min, max),
            a: self.a.clamp(min, max),
        }
    }

    pub fn map(&self, min: f32, max: f32, new_min: f32, new_max: f32) -> Self {
        let range = (max - min).abs();
        let new_range = (new_max - new_min).abs();
        Self {
            r: (self.r - min) * new_range / range + new_min,
            g: (self.g - min) * new_range / range + new_min,
            b: (self.b - min) * new_range / range + new_min,
            a: (self.a - min) * new_range / range + new_min,
        }
    }

    pub fn map_unit(&self, min: f32, max: f32) -> Self {
        self.map(min, max, 0.0, 1.0)
    }

    pub fn as_color32(&self) -> Color32 {
        let color = self.clamp(0.0, 1.0);
        Color32 {
            r: (color.r * 255.0) as u8,
            g: (color.g * 255.0) as u8,
            b: (color.b * 255.0) as u8,
            a: (color.a * 255.0) as u8,
        }
    }
}

impl Div<f32> for Color {
    type Output = Self;

    fn div(self, other: f32) -> Self {
        Self {
            r: self.r / other,
            g: self.g / other,
            b: self.b / other,
            a: self.a / other,
        }
    }
}

impl DivAssign<f32> for Color {
    fn div_assign(&mut self, other: f32) {
        *self = *self / other;
    }
}

impl Add<Color> for Color {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            r: self.r + other.r,
            g: self.g + other.g,
            b: self.b + other.b,
            a: self.a + other.a,
        }
    }
}

impl AddAssign<Color> for Color {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

impl Sub<Color> for Color {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            r: self.r - other.r,
            g: self.g - other.g,
            b: self.b - other.b,
            a: self.a - other.a,
        }
    }
}

impl SubAssign<Color> for Color {
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}

impl Mul<f32> for Color {
    type Output = Self;

    fn mul(self, other: f32) -> Self {
        Self {
            r: self.r * other,
            g: self.g * other,
            b: self.b * other,
            a: self.a * other,
        }
    }
}

impl MulAssign<f32> for Color {
    fn mul_assign(&mut self, other: f32) {
        *self = *self * other;
    }
}

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        self.r == other.r && self.g == other.g && self.b == other.b && self.a == other.a
    }
}

mod tests {
    use super::*;

    #[test]
    fn add_color32() {
        let mut color1 = Color32::GRAY;
        let color2 = Color32::new(128, 128, 128, 0);
        color1 += color2;
        assert_eq!(Color32::new(255, 255, 255, 255), color1);
    }

    #[test]
    fn sub_color32() {
        let mut color1 = Color32::WHITE;
        let color2 = Color32::RED;
        color1 -= color2;
        assert_eq!(Color32::new(0, 255, 255, 0), color1);
    }

    #[test]
    fn multiply_color32() {
        let mut color = Color32::new(127, 127, 127, 2);
        color *= 2.0;
        assert_eq!(Color32::new(254, 254, 254, 4), color);
    }

    #[test]
    fn division_color32() {
        let mut color = Color32::new(127, 127, 127, 2);
        color /= 2.0;
        assert_eq!(Color32::new(63, 63, 63, 1), color);
    }

    #[test]
    fn add_color() {
        let mut color1 = Color::new_value(0.5, 1.0);
        let color2 = Color::new_value(0.5, 1.0);
        color1 += color2;
        assert_eq!(Color::new_value(1.0, 2.0), color1);
    }

    #[test]
    fn sub_color() {
        let mut color1 = Color::WHITE;
        let color2 = Color::RED;
        color1 -= color2;
        assert_eq!(Color::new(0.0, 1.0, 1.0, 0.0), color1);
    }

    #[test]
    fn multiply_color() {
        let mut color = Color::new(0.5, 0.5, 0.5, 2.0);
        color *= 2.0;
        assert_eq!(Color::new(1.0, 1.0, 1.0, 4.0), color);
    }

    #[test]
    fn division_color() {
        let mut color = Color::new_value(1.0, 2.0);
        color /= 2.0;
        assert_eq!(Color::new_value(0.5, 1.0), color);
    }

    #[test]
    fn clamp_color() {
        let color = Color::new(0.5, 1.0, 2.0, 3.5);
        let new_color = color.clamp(1.0, 2.0);
        assert_eq!(Color::new(1.0, 1.0, 2.0, 2.0), new_color);
    }

    #[test]
    fn map_color() {
        let color = Color::new(0.5, 1.0, 2.0, 3.5);
        let new_color = color.map(1.0, 2.0, 3.0, 4.0);
        assert_eq!(Color::new(2.5, 3.0, 4.0, 5.5), new_color);

        let color = Color::new(0.0, 1.0, 2.0, -1.0);
        let new_color = color.map(0.0, 1.0, 10.0, 20.0);
        assert_eq!(Color::new(10.0, 20.0, 30.0, 0.0), new_color);

        let color = Color::new(0.0, 1.0, 2.0, -1.0);
        let new_color = color.map(0.0, 1.0, -10.0, -20.0);
        assert_eq!(Color::new(-20.0, -10.0, -30.0, 0.0), new_color);
    }

    #[test]
    fn map_unit_color() {
        let color = Color::new(10.0, 9.0, 20.0, 2.0);
        let new_color = color.map_unit(10.0, 20.0);
        assert_eq!(Color::new(0.0, -0.1, 1.0, -0.8), new_color);
    }
}
