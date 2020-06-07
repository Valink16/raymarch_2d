use crate::{Marchable, Drawable};
use crate::ray::Ray;
use sfml;
use sfml::graphics::{RenderWindow, Color, RenderTarget, Font, Text, Transformable};
use sfml::window::{VideoMode, Style, Event};
use sfml::system::{Vector2f, Clock};

pub struct Renderer {
    canvas: RenderWindow,
    pub objects: Vec<Box<dyn Marchable>>,
    pub rays: Vec<Ray>,
    pub running: bool
}

impl Renderer {
    pub fn new(w: u32, h: u32, title: Option<&str>, objects: Vec<Box<dyn Marchable>>) -> Self {
        let mut canvas = RenderWindow::new(
            VideoMode::from((w, h)),
            title.unwrap_or("Raymarch"),
            Style::CLOSE,
            &Default::default()
        );

        canvas.set_vertical_sync_enabled(true);

        Self {
            canvas,
            objects,
            rays: Vec::<Ray>::new(),
            running: false
        }
    }

    pub fn render_loop<F: FnMut(&mut Self, Event)>(&mut self, event_handle: &mut F) {

        let trace_font = Font::from_file("ARIAL.ttf").unwrap();
        let mut fps_text = Text::new(&0.to_string(), &trace_font, 14);
        fps_text.set_position((0.0, 0.0));

        let mut clock = Clock::start();
        let mut dt = 0.0_f32;
        self.running = true;
        'running: loop {
            clock.restart();
            while let Some(ev) = self.canvas.poll_event() {
                event_handle(self, ev);
            }

            if !self.running {
                break 'running;
            }

            self.rays[0].reset();
            self.rays[0].march(&self.objects);

            fps_text.set_string(&(1_f32 / dt).round().to_string());

            self.canvas.clear(Color::BLACK);

            for o in &self.objects {
                o.draw(&mut self.canvas);
            }

            for r in &self.rays {
                r.draw(&mut self.canvas);
            }


            self.canvas.draw(&fps_text);
            self.canvas.display();

            dt = clock.elapsed_time().as_seconds();
        }
    } 
}