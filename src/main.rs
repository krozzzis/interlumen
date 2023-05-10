mod console;
mod engine;
mod gui;

use engine::Engine;

use interlumen_core::*;
use interlumen_render::*;

use std::sync::RwLock;

fn main() -> anyhow::Result<()> {
    let mut engine = Engine::new();

    let white_mat = BasicMaterial {
        albedo: Color::new_value(1.0, 1.0),
        roughness: 1.0,
    };

    let red_mat = BasicMaterial {
        albedo: Color::new(1.0, 0.0, 0.0, 1.0),
        roughness: 1.0,
    };

    let check_mat = CheckerMaterial {
        albedo1: Color::new(1.0, 0.0, 0.0, 1.0),
        albedo2: Color::new(1.0, 1.0, 0.0, 1.0),
    };

    let mut materials: Vec<Box<dyn Material>> = Vec::new();
    materials.push(Box::new(white_mat));
    materials.push(Box::new(check_mat));
    materials.push(Box::new(red_mat));

    engine.renderer_driver.materials = materials;

    let mut scene: Scene = Vec::new();

    let mut sphere = Sphere::new(Vec3(0.0, 0.0, 3.0), 1.0, 0);
    sphere.set_pos(Vec3((engine.time * 0.001).sin() * 2.0, 0.0, 3.0));
    scene.push(Box::new(sphere));

    let sphere = Sphere::new(Vec3(1.5, 0.0, 2.2), 0.3, 2);
    scene.push(Box::new(sphere));

    let plane = Plane::new(Vec3(0.0, -2.0, 0.0), 1);
    scene.push(Box::new(plane));

    engine.scene = scene;
    engine.renderer_driver.settings.max_iter = 110;

    let mut mode = 0;

    if let Some(m) = std::env::args().skip(1).next() {
        if m == "console" {
            mode = 0;
        } else if m == "gui" {
            mode = 1;
        }
    }

    match mode {
        #[cfg(feature = "console")]
        0 => console::run(RwLock::new(engine))?,

        #[cfg(feature = "gui")]
        1 => gui::run(RwLock::new(engine))?,

        _ => println!("Unknown mode"),
    }
    Ok(())
}
