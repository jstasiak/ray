pub mod image;
pub mod material;
pub mod render;
pub mod scene;
pub mod traits;

pub use crate::image::{image_to_file, Image};
pub use crate::material::Color;
pub use crate::render::render;
pub use crate::scene::{Camera, Radians, Ray, Sphere, Vector};
pub use crate::traits::AlmostEqual;
