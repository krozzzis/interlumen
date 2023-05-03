mod color;
mod objects;
mod vec;

use color::*;
use objects::*;
use vec::*;

use std::{
    io::stdout,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use crossterm::{
    cursor::{DisableBlinking, Hide, MoveTo, SavePosition},
    event::{poll, read, Event, KeyCode},
    execute,
    style::{self, Print},
    terminal::{
        self, disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
    },
};

use rayon::prelude::*;

struct Renderer {
    width: usize,
    height: usize,
    scene: Vec<Box<dyn Object>>,
    time: u128,
}

impl Renderer {
    pub fn new() -> Self {
        Self {
            width: 0,
            height: 0,
            scene: Vec::new(),
            time: 0,
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
            let line: Vec<Color> = (0..self.width)
                .into_iter()
                .map(move |x| self.get_pixel(x, y).unwrap())
                .collect();
            for color in line {
                execute!(
                    stdout(),
                    style::SetForegroundColor(style::Color::Rgb {
                        r: color.0,
                        g: color.1,
                        b: color.2
                    }),
                    Print("█")
                );
            }
        }
    }

    fn cast_ray(&self, origin: Vec3, ray: Vec3) -> Option<(&Box<dyn Object>, f32)> {
        let mut t = 1.0;
        let mut hit = &self.scene[0];
        let mut i = 0;
        while i <= 90 {
            let mut dist: f32 = 1000.0;
            for obj in &self.scene {
                let d = obj.dist(origin + ray * t);
                if d <= dist {
                    dist = d;
                    hit = obj;
                }
            }
            i += 1;
            t += dist;
            if dist <= 0.01 {
                return Some((hit, t));
            } else if dist > 2000.0 {
                break;
            }
        }
        None
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> Option<Color> {
        let cx = self.width / 2;
        let cy = self.height / 2;

        let speed = 0.0008;
        let light = Vec3(
            5.0 * ((self.time % 10000000) as f32 * speed).sin(),
            3.0 + 2.0 * ((self.time % 10000000) as f32 * speed).cos(),
            3.0 * ((self.time % 10000000) as f32 * speed).sin(),
        );

        let rd: Vec3 = self.get_vec_by_pixel(x, y).norm();
        let hit = self.cast_ray(Vec3(0.0, 0.0, 0.0), rd);
        if let Some((x, t)) = hit {
            let n = (rd * t - x.pos()).norm();
            let light_vec: Vec3 = (light - rd * t).norm();
            let mut lux: f32 = ((n * light_vec).max(0.0) + 0.1).min(1.0);

            let norm = x.norm(rd * t);
            let v = rd * t;
            let refl = (v - norm * ((v * norm) * 2.0)).norm();
            let light_hit = self.cast_ray(v, light_vec);

            if let Some(_) = light_hit {
                lux *= 0.1;
            }

            let ht = self.cast_ray(v, refl);

            if let Some((y, _)) = ht {
                return Some((y.color() * 0.8 + x.color() * 0.2) * lux);
            }

            return Some(x.color() * lux);
        }

        Some(Color(0, 0, 0))
    }

    fn get_vec_by_pixel(&self, x: usize, y: usize) -> Vec3 {
        let cx = self.width as f32 / 2.0;
        let cy = self.height as f32 / 2.0;
        let ratio = self.width as f32 / self.height as f32;
        let char_ratio = 2.0;
        Vec3(
            (x as f32 - cx) / cx,
            -(y as f32 - cy) / cy / ratio * char_ratio,
            1.0,
        )
    }

    pub fn update_size(&mut self) {
        if let Ok((w, h)) = terminal::size() {
            self.width = w as usize;
            self.height = h as usize;
        }
    }

    pub fn update_time(&mut self) {
        let start = SystemTime::now();
        self.time = start
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis();
    }
}
fn main() -> anyhow::Result<()> {
    execute!(stdout(), EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut r = Renderer::new();
    r.add_obj(Box::new(Sphere::new(
        Vec3(3.0, 3.5, 7.0),
        2.0,
        Color(255, 0, 128),
    )));
    r.add_obj(Box::new(Sphere::new(
        Vec3(0.0, 0.4, 8.0),
        2.0,
        Color(0, 255, 0),
    )));
    r.add_obj(Box::new(Plane::new(
        Vec3(0.0, -2.0, 0.0),
        Color(200, 200, 200),
    )));
    let mut running = true;
    while running {
        if poll(Duration::from_millis(1))? {
            match read()? {
                Event::Key(event)
                    if event.code == KeyCode::Esc || event.code == KeyCode::Char('q') =>
                {
                    running = false;
                    break;
                }
                _ => {}
            }
        } else {
            r.update_size();
            r.update_time();
            r.draw();
        }
    }
    execute!(stdout(), LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}
