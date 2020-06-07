use raymarch;
use raymarch::Marchable;
use raymarch::render::Renderer;
use raymarch::sfml::system::Vector2f;
use raymarch::sfml::window::Event;
use raymarch::geom::{Circle, Rectangle};
use raymarch::vec_math::VectorMath;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

fn main() {
    let mut objects: Vec<Box<dyn Marchable>> = vec![];

    // Generating objects
    for i in 0..5 {
        for j in 0..5 {
            let o = Vector2f::new(
                lerp(60.0, (WIDTH - 10) as f32, i as f32 / 5.0),
                lerp(50.0, (HEIGHT - 10) as f32, j as f32 / 5.0),
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
    
    canvas.render_loop(&mut |r: &mut Renderer, ev: Event| {
        match ev {
            Event::Closed => r.running = false,
            Event::MouseMoved { x, y } => {
                let mouse_vec = Vector2f::new(x as f32, y as f32);
                r.rays[0].direction = Vector2f::new(
                    mouse_vec.x - r.rays[0].origin.x,
                    mouse_vec.y - r.rays[0].origin.y
                ).normalized();
            },
            _ => ()
        }
    });
}



fn add_boxed<T: Marchable + 'static>(objects: &mut Vec<Box<dyn Marchable>>, obj: T) {
    let b = Box::from(obj);
    objects.push(b);
}

fn lerp(min: f32, max: f32, x: f32) -> f32 {
    min + (max - min) * x
}