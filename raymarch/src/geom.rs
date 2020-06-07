use sfml::graphics::{RenderWindow, CircleShape, RectangleShape, Transformable, Shape, Color, RenderTarget};
use sfml::system::{Vector2f};

pub trait Distance { // Ability to compute the closest distance from a point
    fn distance(&self, from: Vector2f) -> f32;
}

pub trait Drawable {
    fn draw(&self, c: &mut RenderWindow);
}

pub struct Circle<'a> {
    pub origin: Vector2f,
    pub r: f32,
    shape: CircleShape<'a>
}

pub struct Rectangle<'a> {
    pub origin: Vector2f,
    pub size: Vector2f,
    shape: RectangleShape<'a>
}

impl<'a> Circle<'a> {
    pub fn new(origin: Vector2f, r: f32) -> Self {
        let mut shape = CircleShape::new(r, 30);
        shape.set_origin(Vector2f::new(r, r));
        shape.set_position(origin);
        shape.set_fill_color(Color::WHITE);
        Self {
            origin,
            r,
            shape
        }
    }
}

impl<'a> Rectangle<'a> {
    pub fn new(origin: Vector2f, size: Vector2f) -> Self {
        let mut shape = RectangleShape::with_size(size);
        shape.set_position(origin);
        shape.set_fill_color(Color::WHITE);
        Self {
            origin,
            size,
            shape
        }
    }
}

impl<'a> Distance for Circle<'a> {
    fn distance(&self, from: Vector2f) -> f32 {
        let of = self.origin - from;
        (of.x * of.x + of.y * of.y).sqrt() - self.r
    }
}

impl<'a> Distance for Rectangle<'a> {
    fn distance(&self, from: Vector2f) -> f32 {
        let cx = if from.x > self.origin.x + self.size.x * 0.5 {
            self.size.x
        } else { 0.0 };

        let cy = if from.y > self.origin.y + self.size.y * 0.5 {
            self.size.y
        } else { 0.0 };
        // Here, cx and cy can either be 0(so they don't change influence dx and dy) or the width

        let dx = from.x - self.origin.x - cx;
        let dy = from.y - self.origin.y - cy;

        (dx * dx + dy * dy).sqrt()
    }
}

impl<'a> Drawable for Circle<'a>  {
    fn draw(&self, c: &mut RenderWindow) {
        c.draw(&self.shape);
    }
}

impl<'a>  Drawable for Rectangle<'a>  {
    fn draw(&self, c: &mut RenderWindow) {
        c.draw(&self.shape);
    }
}