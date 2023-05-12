mod console;
mod engine;
mod gui;

use engine::Engine;

use interlumen_core::*;
use interlumen_render::*;

use std::sync::RwLock;

fn main() -> anyhow::Result<()> {
    let mut engine = Engine::new();

    let white_mat1 = BasicMaterial {
        albedo: Color::new(0.8, 0.8, 0.8, 1.0),
        emit: Color::new_value(0.0, 1.0),
        roughness: 0.0,
    };

    let white_mat2 = BasicMaterial {
        albedo: Color::new(1.0, 1.0, 1.0, 1.0),
        emit: Color::new_value(0.0, 1.0),
        roughness: 0.5,
    };

    let white_mat3 = BasicMaterial {
        albedo: Color::new(0.8, 0.8, 0.8, 1.0),
        emit: Color::new_value(0.0, 1.0),
        roughness: 1.0,
    };

    let red_mat = BasicMaterial {
        albedo: Color::new(0.0, 0.0, 0.0, 1.0),
        emit: Color::new(1.0, 0.1, 0.1, 1.0) * 6.0,
        roughness: 1.0,
    };

    let green_mat = BasicMaterial {
        albedo: Color::new(0.0, 0.0, 0.0, 1.0),
        emit: Color::new(0.1, 1.0, 0.1, 1.0) * 6.0,
        roughness: 1.0,
    };

    let white_light_mat = BasicMaterial {
        albedo: Color::new(0.0, 0.0, 0.0, 1.0),
        emit: Color::new_value(1.0, 1.0) * 3.0,
        roughness: 1.0,
    };

    let floor_mat = CheckerMaterial {
        albedo1: Color::new(0.1, 0.8, 0.1, 1.0),
        albedo2: Color::new(0.8, 0.8, 0.1, 1.0),
    };

    let mut materials: Vec<Box<dyn Material>> = Vec::new();
    materials.push(Box::new(white_mat1));
    materials.push(Box::new(floor_mat));
    materials.push(Box::new(red_mat));
    materials.push(Box::new(green_mat));
    materials.push(Box::new(white_light_mat));
    materials.push(Box::new(white_mat2));
    materials.push(Box::new(white_mat3));

    engine.renderer_driver.materials = materials;

    let mut scene: Scene = Vec::new();

    let sphere = Sphere::new(Vec3(-1.1, 0.0, 2.0), 0.5, 0);
    scene.push(Box::new(sphere));

    let sphere = Sphere::new(Vec3(0.0, 0.0, 2.0), 0.5, 5);
    scene.push(Box::new(sphere));

    let sphere = Sphere::new(Vec3(1.1, 0.1, 2.0), 0.5, 6);
    scene.push(Box::new(sphere));

    let sphere = Sphere::new(Vec3(3.0, 0.0, 1.2), 0.8, 2);
    scene.push(Box::new(sphere));

    let sphere = Sphere::new(Vec3(-3.0, 0.0, 1.2), 0.8, 3);
    scene.push(Box::new(sphere));

    let sphere = Sphere::new(Vec3(0.0, 9.0, 5.8), 5.0, 4);
    scene.push(Box::new(sphere));

    let sphere = Sphere::new(Vec3(0.0, 9.0, -1.8), 5.0, 4);
    scene.push(Box::new(sphere));

    let plane = Plane::new(Vec3(0.0, -0.5, 0.0), Vec3(0.0, 1.0, 0.0), 1);
    scene.push(Box::new(plane));

    engine.renderer_driver.scene = scene;
    engine.renderer_driver.settings.max_iter = 100;
    engine.renderer_driver.settings.ray_depth = 10;

    engine.renderer_driver.camera.fov = 120.0;

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
