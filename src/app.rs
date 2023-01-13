use std::time::Instant;

use glium::{self, Surface};

use crate::amogus::{AmogusData, AmogusRenderer};

struct SingleAmogus {
    pos: nalgebra::Vector2<f32>,
    color: [f32; 3],
    animation_frame: u8,
    twerks_per_second: f32,
    last_frame_switch: Instant,
}

pub struct App {
    bg_color: [f32; 3],
    amogus_renderer: AmogusRenderer,
    amogsuses: Vec<SingleAmogus>,
}

impl App {
    pub fn new(display: &glium::Display) -> Self {
        App {
            bg_color: [1.0, 1.0, 1.0],
            amogus_renderer: AmogusRenderer::new(display),
            amogsuses: vec![SingleAmogus {
                pos: nalgebra::Vector2::new(100.0, 100.0),
                color: [1.0, 0.0, 1.0],
                animation_frame: 4,
                twerks_per_second: 6.0,
                last_frame_switch: Instant::now(),
            }],
        }
    }

    pub fn update(&mut self) {
        for amog in self.amogsuses.iter_mut() {
            let seconds_per_twerk = 1.0 / amog.twerks_per_second;
            if Instant::now()
                .duration_since(amog.last_frame_switch)
                .as_secs_f32()
                > seconds_per_twerk / self.amogus_renderer.frames_number as f32
            {
                amog.animation_frame =
                    (amog.animation_frame + 1) % self.amogus_renderer.frames_number;
                amog.last_frame_switch = Instant::now();
            }
        }
    }

    pub fn draw(&self, display: &glium::Display, frame: &mut glium::Frame) {
        frame.clear_color(self.bg_color[0], self.bg_color[1], self.bg_color[2], 1.0);
        let amogsuses_data: Vec<AmogusData> = self
            .amogsuses
            .iter()
            .map(|single_amogus| AmogusData {
                world_position: single_amogus.pos,
                size: 100.0,
                color: single_amogus.color,
                animation_frame: single_amogus.animation_frame,
            })
            .collect();
        self.amogus_renderer.draw(display, frame, amogsuses_data);
    }
}
