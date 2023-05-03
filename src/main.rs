use std::{
    io::{stdout, Write},
    ops::{Add, Div, Mul, Sub},
    time::{SystemTime, UNIX_EPOCH},
};

use crossterm::{
    cursor::{DisableBlinking, Hide, MoveTo, SavePosition},
    execute,
    style::{self, Print},
    terminal::{self, EnterAlternateScreen, SetTitle},
};

#[derive(Debug, Clone, Copy)]
pub struct Color(pub u8, pub u8, pub u8);

impl Div<f32> for Color {
    type Output = Self;

    fn div(self, rhs: f32) -> Self {
        Self(
            (self.0 as f32 / rhs) as u8,
            (self.1 as f32 / rhs) as u8,
            (self.2 as f32 / rhs) as u8,
        )
    }
}

impl Mul<f32> for Color {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self {
        Self(
            (self.0 as f32 * rhs) as u8,
            (self.1 as f32 * rhs) as u8,
            (self.2 as f32 * rhs) as u8,
        )
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Vec3(pub f32, pub f32, pub f32);

impl Vec3 {
    pub fn len(&self) -> f32 {
        (*self * *self).sqrt()
    }

    pub fn norm(&self) -> Vec3 {
        *self / self.len()
    }

    pub fn normalize(&mut self) {
        *self = *self / self.len()
    }
}

impl Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self {
        Self(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = f32;

    fn mul(self, rhs: Self) -> f32 {
        self.0 * rhs.0 + self.1 * rhs.1 + self.2 * rhs.2
    }
}

impl Div<f32> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self {
        Self(self.0 / rhs, self.1 / rhs, self.2 / rhs)
    }
}

impl Add<Vec3> for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self(self.0 + rhs.1, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl Add<f32> for Vec3 {
    type Output = Self;

    fn add(self, rhs: f32) -> Self {
        Self(self.0 + rhs, self.1 + rhs, self.2 + rhs)
    }
}

impl Sub<Vec3> for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self(self.0 - rhs.1, self.1 - rhs.1, self.2 - rhs.2)
    }
}

pub trait Distance {
    fn dist(&self, from: Vec3) -> f32;
}

pub trait Colorable {
    fn color(&self) -> Color;
}

pub trait Position {
    fn pos(&self) -> Vec3;
}

#[derive(Debug, Clone, Copy)]
struct Sphere {
    x: f32,
    y: f32,
    z: f32,
    radius: f32,
    color: Color,
}

impl Distance for Sphere {
    fn dist(&self, from: Vec3) -> f32 {
        ((from - self.pos()).len() - self.radius).abs()
    }
}

impl Colorable for Sphere {
    fn color(&self) -> Color {
        return self.color;
    }
}

impl Position for Sphere {
    fn pos(&self) -> Vec3 {
        return Vec3(self.x, self.y, self.z);
    }
}

impl Object for Sphere {}

#[derive(Debug, Clone, Copy)]
struct Plane {
    x: f32,
    y: f32,
    z: f32,
    color: Color,
}

impl Distance for Plane {
    fn dist(&self, from: Vec3) -> f32 {
        Vec3(0.0, 1.0, 0.0) * (from - self.pos())
    }
}

impl Colorable for Plane {
    fn color(&self) -> Color {
        return self.color;
    }
}

impl Position for Plane {
    fn pos(&self) -> Vec3 {
        return Vec3(self.x, self.y, self.z);
    }
}

impl Object for Plane {}

trait Object: Distance + Colorable + Position {}

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
            for x in 0..self.width {
                let color = self.get_pixel(x, y).unwrap();
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

    pub fn get_pixel(&self, x: usize, y: usize) -> Option<Color> {
        let cx = self.width / 2;
        let cy = self.height / 2;

        let mut i = 0;
        let rd: Vec3 = self.get_vec_by_pixel(x, y).norm();
        let mut t = 1.0;
        let speed = 0.001;
        let light = Vec3(
            5.0 * ((self.time % 10000000) as f32 * speed).sin(),
            5.0 * ((self.time % 10000000) as f32 * speed).cos(),
            3.0,
        );
        // println!("{light:?}");
        let mut hit = &self.scene[0];
        while i <= 90 {
            let mut dist: f32 = 1000.0;
            for obj in &self.scene {
                let d = obj.dist(rd * t);
                if d <= dist {
                    dist = d;
                    hit = obj;
                }
            }
            i += 1;
            t += dist;
            if dist <= 0.01 {
                let n = (rd * t - hit.pos()).norm();
                let lux = ((n * (light - rd * t).norm()).max(0.0) + 0.1).min(1.0);
                return Some(hit.color() * lux);
            } else if dist > 2000.0 {
                break;
            }
        }
        Some(Color(0, 0, 0))
    }

    fn get_vec_by_pixel(&self, x: usize, y: usize) -> Vec3 {
        let cx = self.width as f32 / 2.0;
        let cy = self.height as f32 / 2.0;
        Vec3((x as f32 - cx) / cx, -(y as f32 - cy) / cy, 1.0)
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
fn main() {
    // execute!(stdout(), EnterAlternateScreen);
    let mut r = Renderer::new();
    r.add_obj(Box::new(Sphere {
        x: 0.0,
        y: 2.0,
        z: 5.0,
        radius: 1.0,
        color: Color(255, 0, 0),
    }));
    r.add_obj(Box::new(Sphere {
        x: 0.0,
        y: 1.5,
        z: 7.0,
        radius: 2.0,
        color: Color(255, 0, 128),
    }));
    r.add_obj(Box::new(Sphere {
        x: 0.0,
        y: 0.0,
        z: 8.0,
        radius: 2.0,
        color: Color(0, 255, 0),
    }));
    r.add_obj(Box::new(Plane {
        x: 0.0,
        y: -2.0,
        z: 0.0,
        color: Color(255, 255, 0),
    }));
    loop {
        r.update_size();
        r.update_time();
        r.draw();
    }
}
