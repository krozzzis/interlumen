use interlumen_core::{Color, Vec3};

pub struct PBRColor {
    pub albedo: Color,
    pub roughness: f32,
}

pub trait Material: Sync {
    fn get_color(&self, uv: Vec3) -> PBRColor;
}

#[derive(Debug, Clone)]
pub struct BasicMaterial {
    pub albedo: Color,
    pub roughness: f32,
}

impl Material for BasicMaterial {
    fn get_color(&self, _uv: Vec3) -> PBRColor {
        PBRColor {
            albedo: self.albedo,
            roughness: self.roughness,
        }
    }
}

#[derive(Debug, Clone)]
pub struct CheckerMaterial {
    pub albedo1: Color,
    pub albedo2: Color,
}

impl Material for CheckerMaterial {
    fn get_color(&self, uv: Vec3) -> PBRColor {
        PBRColor {
            albedo: if (uv.0.ceil() + uv.1.ceil()) % 2.0 == 0.0 {
                    self.albedo1
                } else {
                    self.albedo2
                },
            roughness: 1.0,
        }
    }
}
