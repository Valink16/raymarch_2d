use crate::geom;
use crate::Marchable;
use crate::vec_math::VectorMath;
use crate::geom::{Drawable, Distance};
use sfml::system::{Vector2f};
use sfml::graphics::{RenderWindow, CircleShape, Color, Shape, Transformable, RenderTarget};
use sfml::graphics::{VertexArray, Vertex, PrimitiveType};

pub struct Ray {
    pub origin: Vector2f, // Position in the grid
    pub direction: Vector2f, // Normalized direction
    pub current: Vector2f, // Current marching spot, should always be "on" the ray
    pub steps: Vec<(Vector2f, f32)>,// Array contain the history of `current` values and radiuses
}

impl Ray {
    pub fn with_angle(origin: Vector2f, a: f32) -> Self {
        Self {
            origin,
            direction: Vector2f::from((a.cos(), a.sin())),
            current: origin,
            steps: Vec::<(Vector2f, f32)>::new()
        }
    }

    pub fn new(origin: Vector2f, direction: Vector2f) -> Self {
        Self {
            origin,
            direction: direction.normalized(),
            current: origin,
            steps: Vec::<(Vector2f, f32)>::new()
        }
    }

    pub fn distance_from<T: geom::Distance>(&self, from: &T) -> f32 {
        from.distance(self.current)
    }

    pub fn move_along(&mut self, d: f32) { // Move the current along the ray direction by d
        self.steps.push((self.current, d));
        self.current += self.direction * d;
    }

    pub fn step(&mut self, objects: &Vec<Box<dyn Marchable>>) { // Core raymarch functionality
        let mut min_distance = f32::MAX;

        for obj in objects {
            let d = obj.distance(self.current);
            if d < min_distance {
                min_distance = d;
            }
        }

        println!("Minimal distance: {}", min_distance);

        self.move_along(min_distance);
    }

    pub fn march(&mut self) {
        
    }
}

impl Drawable for Ray {
    fn draw(&self, c: &mut RenderWindow) {
        let mut circ = CircleShape::default();
        circ.set_fill_color(Color::TRANSPARENT);
        circ.set_outline_thickness(1.0);
        circ.set_outline_color(Color::CYAN);
        circ.set_point_count(50);

        let mut point = CircleShape::default();
        point.set_fill_color(Color::RED);
        point.set_radius(5.0);
        point.set_origin(Vector2f::new(point.radius(), point.radius()));

        let mut line = VertexArray::new(PrimitiveType::Lines, 2);
        line[0] = Vertex::with_pos_color(self.origin, Color::WHITE);
        let def = (self.origin, 0.0);
        let last_pos = self.steps.last().unwrap_or(&def);
        line[1] = Vertex::with_pos_color(last_pos.0, Color::WHITE);

        for step in &self.steps {
            circ.set_radius(step.1);
            circ.set_origin(Vector2f::new(step.1, step.1));
            circ.set_position(step.0);

            point.set_position(step.0);

            c.draw(&circ);
            c.draw(&point);
        }

        c.draw(&line);
    }
}