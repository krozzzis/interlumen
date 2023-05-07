use interlumen_core::*;
use interlumen_render::*;

use std::{
    io::stdout,
    time::{Duration, SystemTime},
};

use crossterm::{
    cursor::{DisableBlinking, Hide, MoveTo, SavePosition},
    event::{poll, read, Event, KeyCode, KeyModifiers},
    execute,
    style::{self, Print},
    terminal::{
        self, disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
        SetTitle,
    },
};

use rayon::prelude::*;

struct Screen {
    width: usize,
    height: usize,
    scene: Vec<Box<dyn Object>>,
    camera: Camera,
    time: usize,
    last_frame: SystemTime,
}

impl Screen {
    pub fn new() -> Self {
        Self {
            width: 0,
            height: 0,
            scene: Vec::new(),
            camera: Camera::unit(),
            time: 0,
            last_frame: SystemTime::now(),
        }
    }

    pub fn add_obj(&mut self, obj: Box<dyn Object>) {
        self.scene.push(obj);
    }

    pub fn draw(&self) {
        for y in 0..self.height {
            execute!(
                stdout(),
                SavePosition,
                MoveTo(0, y as u16),
                DisableBlinking,
                Hide
            );
            let settings = RendererSettings::new();
            let line: Vec<Color32> = (0..self.width)
                .into_par_iter()
                .map(move |x| {
                    let color = Renderer::render_pixel(
                        &settings,
                        &self.scene,
                        x,
                        y,
                        self.width,
                        self.height,
                        &self.camera,
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
                );
            }
        }
    }

    pub fn update_size(&mut self) {
        if let Ok((w, h)) = terminal::size() {
            self.width = w as usize;
            self.height = h as usize;
        }
    }

    pub fn update_time(&mut self) {
        self.time += SystemTime::now()
            .duration_since(self.last_frame)
            .expect("Time went backwards")
            .as_millis() as usize;
        self.last_frame = SystemTime::now();
    }
}
fn main() -> anyhow::Result<()> {
    execute!(stdout(), EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut r = Screen::new();
    r.add_obj(Box::new(Sphere::new(Vec3(0.0, 0.0, 3.0), 1.0)));
    let mut update_time = true;
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
                    r.last_frame = SystemTime::now();
                }
                _ => {}
            }
        } else {
            r.update_size();
            if update_time {
                r.update_time();
            }
            r.draw();
        }
    }
    execute!(stdout(), LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}
