use std::{
    io::stdout,
    sync::RwLock,
    time::{Duration, SystemTime},
};

use crossterm::{
    cursor::{DisableBlinking, EnableBlinking, Hide, MoveTo, SavePosition, Show},
    event::{poll, read, Event, KeyCode, KeyModifiers},
    execute,
    style::{self, Print},
    terminal::{
        self, disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
    },
};

use rayon::prelude::*;

use interlumen_core::{Color32, Vec3};
use interlumen_render::{Camera, Renderer};

use crate::engine::Engine;

fn draw(width: u16, height: u16, camera: &Camera, engine: &Engine) -> anyhow::Result<()> {
    for y in 0..height as usize {
        execute!(stdout(), SavePosition, MoveTo(0, y as u16),)?;
        let line: Vec<Color32> = (0..width as usize)
            .into_par_iter()
            .map(move |x| {
                let color = Renderer::render_pixel(
                    &engine.renderer_settings,
                    &engine.scene,
                    &engine.materials,
                    x,
                    y,
                    width as usize,
                    height as usize,
                    camera,
                );
                color.as_color32()
            })
            .collect();
        for color in line {
            execute!(
                stdout(),
                style::SetForegroundColor(style::Color::Rgb {
                    r: color.r,
                    g: color.g,
                    b: color.b
                }),
                Print("█")
            )?;
        }
    }
    Ok(())
}

pub fn run(engine: RwLock<Engine>) -> anyhow::Result<()> {
    let camera = Camera::unit();
    let mut update_time = true;
    execute!(stdout(), EnterAlternateScreen, DisableBlinking, Hide)?;
    enable_raw_mode()?;
    {
        let mut eng = engine.write().unwrap();
        eng.renderer_settings.pixel_ratio = 2.0;
    }
    loop {
        if poll(Duration::from_millis(1))? {
            match read()? {
                Event::Key(event)
                    if event.code == KeyCode::Esc
                        || event.code == KeyCode::Char('q')
                        || event.code == KeyCode::Char('c')
                            && event.modifiers.contains(KeyModifiers::CONTROL) =>
                {
                    break
                }
                Event::Key(event) if event.code == KeyCode::Char('p') => {
                    update_time = !update_time;
                    let mut eng = engine.write().unwrap();
                    eng.last_frame = SystemTime::now();
                }
                _ => {}
            }
        } else {
            // Update engine state
            {
                let mut eng = engine.write().unwrap();
                eng.next_frame();
                let time = eng.time;
                if let Some(obj) = eng.scene.get_mut(0) {
                    obj.set_pos(Vec3((time * 0.001).sin() * 2.0, 0.0, 3.0));
                }
            }
            {
                let size = terminal::size()?;
                let eng = engine.read().unwrap();
                draw(size.0, size.1, &camera, &eng)?;
            }
        }
    }
    execute!(stdout(), LeaveAlternateScreen, EnableBlinking, Show)?;
    disable_raw_mode()?;
    Ok(())
}
