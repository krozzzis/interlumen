use interlumen_core::{Color, Vec3};

pub trait Material: Sync {
    fn get_color(&self, uv: Vec3) -> Color;
}

#[derive(Debug, Clone)]
pub struct DiffuseMaterial {
    pub albedo: Color,
}

impl Material for DiffuseMaterial {
    fn get_color(&self, _uv: Vec3) -> Color {
        self.albedo
    }
}

#[derive(Debug, Clone)]
pub struct CheckerMaterial {
    pub albedo1: Color,
    pub albedo2: Color,
}

impl Material for CheckerMaterial {
    fn get_color(&self, uv: Vec3) -> Color {
        if (uv.0.ceil() + uv.1.ceil()) % 2.0 == 0.0 {
            self.albedo1
        } else {
            self.albedo2
        }
    }
}
