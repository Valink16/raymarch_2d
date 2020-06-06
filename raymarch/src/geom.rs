use vecmat::vec::Vec2;
use sfml::graphics::{RenderWindow, CircleShape, RectangleShape, Transformable, Shape, Color, RenderTarget};
use sfml::system::{Vector2f};

pub trait Distance { // Ability to compute the closest distance from a point
    fn distance(&self, from: Vec2<f32>) -> f32;
}

pub trait Drawable {
    fn draw(&self, c: &mut RenderWindow);
}

pub struct Circle<'a> {
    pub origin: Vec2<f32>,
    pub r: f32,
    shape: CircleShape<'a>
}

pub struct Rectangle<'a> {
    pub origin: Vec2<f32>,
    pub size: Vec2<f32>,
    shape: RectangleShape<'a>
}

impl<'a> Circle<'a> {
    pub fn new(origin: Vec2<f32>, r: f32) -> Self {
        let mut shape = CircleShape::new(r, 30);
        shape.set_position((origin[0], origin[1]));
        shape.set_fill_color(Color::WHITE);
        Self {
            origin,
            r,
            shape
        }
    }
}

impl<'a> Rectangle<'a> {
    pub fn new(origin: Vec2<f32>, size: Vec2<f32>) -> Self {
        let mut shape = RectangleShape::with_size(Vector2f::from((size[0], size[1])));
        shape.set_position((origin[0], origin[1]));
        shape.set_fill_color(Color::WHITE);
        Self {
            origin,
            size,
            shape
        }
    }
}

impl<'a> Distance for Circle<'a> {
    fn distance(&self, from: Vec2<f32>) -> f32 {
        (self.origin - from).length() - self.r
    }
}

impl<'a> Distance for Rectangle<'a> {
    fn distance(&self, from: Vec2<f32>) -> f32 {
        let cx = if from[0] > self.origin[0] + self.size[0] * 0.5 {
            self.size[0]
        } else { 0.0 };

        let cy = if from[1] > self.origin[1] + self.size[1] * 0.5 {
            self.size[1]
        } else { 0.0 };
        // Here, cx and cy can either be 0(so they don't change influence dx and dy) or the width

        let dx = from[0] - self.origin[0] - cx;
        let dy = from[1] - self.origin[1] - cy;

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