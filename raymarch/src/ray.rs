use crate::geom;

use vecmat::vec::Vec2;

pub struct Ray {
    pub origin: Vec2::<f32>, // Position in the grid
    pub direction: Vec2::<f32>, // Normalized direction
    pub current: Vec2::<f32> // Current marching spot, should always be "on" the ray
}

impl Ray {
    pub fn with_angle(origin: Vec2::<f32>, a: f32) -> Self {
        let direction = Vec2::<f32>::from(a.acos(), a.asin());

        Self {
            origin,
            direction,
            current: origin
        }
    }

    pub fn new(origin: Vec2::<f32>, direction: Vec2::<f32>) -> Self {
        Self {
            origin,
            direction: direction.normalize(),
            current: origin
        }
    }

    pub fn distance_from<T: geom::Distance>(&self, from: &T) -> f32 {
        from.distance(self.current)
    }

    pub fn move_along(&mut self, d: f32) { // Move the current along the ray direction by d
        self.current += self.direction * d;
    }
}