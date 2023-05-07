mod camera;
mod material;
mod objects;
mod renderer;

pub use camera::*;
pub use material::*;
pub use objects::*;
pub use renderer::*;

pub type Scene = Vec<Box<dyn Object>>;
