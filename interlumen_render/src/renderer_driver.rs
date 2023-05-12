use crate::{Material, Scene, Camera, Renderer, RendererSettings};
use interlumen_core::Color;

use rayon::prelude::*;

pub struct RendererDriver {
    pub materials: Vec<Box<dyn Material>>,
    pub camera: Camera,
    pub scene: Scene,
    pub settings: RendererSettings,
    pub accum_buffer: Vec<Color>,
    pub accum_steps: usize,
}

impl RendererDriver {
    pub fn new() -> Self {
        Self {
            materials: Vec::new(),
            scene: Vec::new(),
            camera: Camera::unit(),
            settings: RendererSettings::new(),
            accum_buffer: Vec::new(),
            accum_steps: 0,
        }
    }

    pub fn init_accum_buffer(&mut self, width: usize, height: usize) {
        self.accum_buffer.clear();
        self.accum_buffer.resize(width*height, Color::BLACK);
    }

    pub fn append_to_accum_buffer(&mut self, width: usize, height: usize) {
        self.accum_buffer.par_iter_mut().enumerate().for_each(|(pos, i)| {
            let (x, y) = (pos % width, pos / width);
            let color = Renderer::render_pixel(&self.settings, &self.scene, &self.materials, x, y, width, height, &self.camera);
            *i += color;
        });
        self.accum_steps += 1;
    }

    pub fn show_accum_buffer(&self) -> Vec<Color> {
        self.accum_buffer.par_iter().map(|i| {
            let color = *i / self.accum_steps as f32;
            color.pow(2.4).clamp(0.0, 1.0)
        }).collect()
    }

    pub fn draw_image(&self, width: usize, height: usize) -> Vec<Color> {
        (0..width*height).into_par_iter().map(move |pos| {
            let (x, y) = (pos % width, pos / width);
            let color = Renderer::render_pixel(&self.settings, &self.scene, &self.materials, x, y, width, height, &self.camera);
            color.pow(2.4)
        }).collect()
    }
}
