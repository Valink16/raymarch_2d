use sfml::system::Vector2;
use std::ops::{Mul, Div, Add, Sub};

// Extends the Vector2 class from SFML using traits
pub trait VectorMath<T> {
    fn length(&self) -> f32;
    fn normalized(&self) -> Self;
    fn normalize(&mut self);
}

impl<T> VectorMath<T> for Vector2<T> 
    where T: Mul<Output = T> + Add<Output = T> + Div<f32, Output = T> + Into<f32> + Copy
{
    fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y).into().sqrt()
    }

    fn normalized(&self) -> Self {
        let l = self.length();
        Self::new(self.x / l, self.y / l)
    }

    fn normalize(&mut self) {
        let l = self.length();
        self.x = self.x / l; 
        self.y = self.y / l;
    }
}