mod camera;
mod material;
mod objects;
mod renderer;
mod renderer_driver;

pub use camera::*;
pub use material::*;
pub use objects::*;
pub use renderer::*;
pub use renderer_driver::*;

pub type Scene = Vec<Box<dyn Object>>;
