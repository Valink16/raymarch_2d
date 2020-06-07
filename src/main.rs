use raymarch;
use raymarch::Marchable;
use raymarch::sfml::system::Vector2f;
use raymarch::geom::{Circle, Rectangle};

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

fn main() {
    let mut objects: Vec<Box<dyn Marchable>> = vec![];

    // Generating objects
    for i in 0..10 {
        for j in 0..10 {
            let o = Vector2f::new(
                lerp(100.0, (WIDTH - 10) as f32, i as f32 / 10.0),
                lerp(100.0, (HEIGHT - 10) as f32, j as f32 / 10.0),
            );

            if i * j %  2 == 0 {
                add_boxed(&mut objects, Circle::new(o, 20.0));
            } else {
                let size = Vector2f::new(30.0, 30.0);
                add_boxed(&mut objects, Rectangle::new(o - size / 2.0, size));
            }
        }
    }

    let mut canvas = raymarch::render::Renderer::new(WIDTH, HEIGHT, None, objects);

    canvas.rays.push(raymarch::ray::Ray::with_angle(Vector2f::new(10.0, 200.0), 0.0));

    for i in 0..5 {
        canvas.rays[0].step(&canvas.objects);

    }
    
    canvas.render_loop();
}

fn add_boxed<T: Marchable + 'static>(objects: &mut Vec<Box<dyn Marchable>>, obj: T) {
    let b = Box::from(obj);
    objects.push(b);
}

fn lerp(min: f32, max: f32, x: f32) -> f32 {
    min + (max - min) * x
}