use std::{sync::RwLock, time::SystemTime};

use interlumen_core::Vec3;
use interlumen_render::{Camera, Renderer};

use minifb::{Key, Window, WindowOptions};
use rayon::prelude::*;

use crate::engine::Engine;

const WIDTH: usize = 640;
const HEIGHT: usize = 480;

pub fn run(engine: RwLock<Engine>) -> anyhow::Result<()> {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let mut window = Window::new("Interlumen", WIDTH, HEIGHT, WindowOptions::default())?;

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));
    let camera = Camera::unit();

    let mut update_time = true;
    while window.is_open() && !window.is_key_down(Key::Q) {
        if window.is_key_released(Key::P) {
            update_time = !update_time;
            let mut eng = engine.write().unwrap();
            eng.last_frame = SystemTime::now();
        }
        // Update engine state
        {
            let mut eng = engine.write().unwrap();
            eng.next_frame();
            let time = eng.time;
            if let Some(obj) = eng.scene.get_mut(0) {
                obj.set_pos(Vec3((time * 0.001).sin() * 2.0, 0.0, 3.0));
            }
        }
        // Render
        {
            let eng = engine.read().unwrap();
            buffer.par_iter_mut().zip(eng.renderer_driver.draw_image(WIDTH, HEIGHT, &eng.scene).into_par_iter()).for_each(move |(i, color)| {
                let rgb = color.as_color32();
                *i = (rgb.b as u32) | (rgb.g as u32) << 8 | (rgb.r as u32) << 16;
            });
        }
        window.update_with_buffer(&buffer, WIDTH, HEIGHT)?;
    }
    Ok(())
}
