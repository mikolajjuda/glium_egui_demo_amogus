use glium::{self, Surface};
pub struct App {
    bg_color: [f32; 3],
}

impl App {
    pub fn new() -> Self {
        App {
            bg_color: [1.0, 1.0, 1.0],
        }
    }

    pub fn draw(&self, frame: &mut glium::Frame) {
        frame.clear_color(self.bg_color[0], self.bg_color[1], self.bg_color[2], 1.0);
    }
}
