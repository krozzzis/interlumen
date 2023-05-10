use crate::{Material, Scene, Camera, Renderer, RendererSettings};
use interlumen_core::Color;

use rayon::prelude::*;

pub struct RendererDriver {
    pub materials: Vec<Box<dyn Material>>,
    pub camera: Camera,
    pub settings: RendererSettings,
}

impl RendererDriver {
    pub fn new() -> Self {
        Self {
            materials: Vec::new(),
            camera: Camera::unit(),
            settings: RendererSettings::new(),
        }
    }

    pub fn draw_image(&self, width: usize, height: usize, scene: &Scene) -> Vec<Color> {
        (0..width*height).into_par_iter().map(move |pos| {
            let (x, y) = (pos % width, pos / width);
            Renderer::render_pixel(&self.settings, &scene, &self.materials, x, y, width, height, &self.camera)
        }).collect()
    }
}