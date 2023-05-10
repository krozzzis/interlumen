use std::time::SystemTime;

use interlumen_render::{RendererDriver, Scene};

pub struct Engine {
    pub scene: Scene,
    pub renderer_driver: RendererDriver,
    pub time: f32,
    pub last_frame: SystemTime,
}

impl Engine {
    pub fn new() -> Self {
        Self {
            renderer_driver: RendererDriver::new(),
            scene: Scene::new(),
            time: 0.0,
            last_frame: SystemTime::now(),
        }
    }

    pub fn next_frame(&mut self) {
        self.time += SystemTime::now()
            .duration_since(self.last_frame)
            .expect("Time went backwards")
            .as_millis() as f32;
        self.last_frame = SystemTime::now();
    }
}
