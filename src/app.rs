use glium::{self, Surface};

use crate::amogus::AmogusRenderer;
pub struct App {
    bg_color: [f32; 3],
    amogus_renderer: AmogusRenderer,
}

impl App {
    pub fn new(display: &glium::Display) -> Self {
        App {
            bg_color: [1.0, 0.0, 0.0],
            amogus_renderer: AmogusRenderer::new(display),
        }
    }

    pub fn draw(&self, frame: &mut glium::Frame) {
        frame.clear_color(self.bg_color[0], self.bg_color[1], self.bg_color[2], 1.0);
        self.amogus_renderer.draw(frame);
    }
}
