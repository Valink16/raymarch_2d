use raymarch;
use vecmat::vec::Vec2;

fn main() {
    let c = raymarch::geom::Circle::new(Vec2::<f32>::from(400.0, 300.0), 10.0);
    let r = raymarch::geom::Rectangle::new(Vec2::<f32>::from(10.0, 300.0), Vec2::<f32>::from(10.0, 10.0));
    let ray = raymarch::ray::Ray::with_angle(Vec2::<f32>::from(0.0, 0.0), 0.0);

    println!("{}", ray.distance_from(&c));
    println!("{}", ray.distance_from(&r));

    let mut canvas = raymarch::render::Renderer::new(800, 600, None, vec![Box::new(c), Box::new(r)]);

    canvas.render_loop();
}
