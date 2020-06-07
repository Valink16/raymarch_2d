use crate::geom::{Distance, Drawable};

pub trait Marchable: Drawable + Distance {}
impl <T: Drawable + Distance> Marchable for T {}

pub mod ray;
pub mod geom;
pub mod render;
pub mod vec_math;
pub use sfml;