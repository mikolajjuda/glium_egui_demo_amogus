use glium::Surface;
use std::time::Instant;

use crate::amogus::{AmogusData, AmogusRenderer};
use crate::amogus_manager::{
    AmogusManager, MAX_AMOGUS_SIZE, MAX_AMOGUS_TWERK, MIN_AMOGUS_SIZE, MIN_AMOGUS_TWERK,
};

struct SingleAmogus {
    pos: nalgebra::Vector2<f32>,
    velocity: nalgebra::Vector2<f32>,
    size: f32,
    color: [f32; 3],
    animation_frame: u8,
    twerks_per_second: f32,
    last_frame_switch: Instant,
}

const GRAVITY: f32 = 1000.0;
pub struct App {
    bg_color: [f32; 3],
    amogus_renderer: AmogusRenderer,
    amogsuses: Vec<SingleAmogus>,
    amogus_manager: AmogusManager,
    random_generator: oorandom::Rand32,
    mouse_pos: nalgebra::Vector2<f32>,
    mouse_down_point: Option<nalgebra::Vector2<f32>>,
}

impl App {
    pub fn new(display: &glium::Display) -> Self {
        App {
            bg_color: [1.0, 1.0, 1.0],
            amogus_renderer: AmogusRenderer::new(display),
            amogus_manager: AmogusManager::new(),
            amogsuses: vec![SingleAmogus {
                pos: nalgebra::Vector2::new(100.0, 200.0),
                velocity: nalgebra::Vector2::new(400.0, 20.0),
                size: 100.0,
                color: [1.0, 0.0, 1.0],
                animation_frame: 4,
                twerks_per_second: 6.0,
                last_frame_switch: Instant::now(),
            }],
            random_generator: oorandom::Rand32::new(666),
            mouse_pos: nalgebra::Vector2::new(0.0, 0.0),
            mouse_down_point: None,
        }
    }

    pub fn input(&mut self, event: &glium::glutin::event::WindowEvent, window_h: f32) -> bool {
        match event {
            glium::glutin::event::WindowEvent::KeyboardInput {
                device_id: _,
                input,
                is_synthetic,
            } => {
                if *is_synthetic {
                    return false;
                }
                match input {
                    #[allow(deprecated)]
                    glium::glutin::event::KeyboardInput {
                        scancode: _,
                        state,
                        virtual_keycode,
                        modifiers,
                    } => {
                        if modifiers.is_empty() {
                            if *state == glium::glutin::event::ElementState::Pressed {
                                if virtual_keycode.is_none() {
                                    return false;
                                }
                                if virtual_keycode.unwrap()
                                    == glium::glutin::event::VirtualKeyCode::Space
                                {
                                    self.bg_color = [
                                        self.random_generator.rand_float(),
                                        self.random_generator.rand_float(),
                                        self.random_generator.rand_float(),
                                    ];
                                    return true;
                                }
                            }
                        }
                    }
                }
            }
            glium::glutin::event::WindowEvent::CursorMoved { position, .. } => {
                self.mouse_pos = nalgebra::Vector2::new(position.x as f32, position.y as f32);
            }
            glium::glutin::event::WindowEvent::MouseInput { state, button, .. } => {
                if *button != glium::glutin::event::MouseButton::Left {
                    return false;
                }
                match *state {
                    glium::glutin::event::ElementState::Pressed => {
                        self.mouse_down_point = Some(self.mouse_pos);
                    }
                    glium::glutin::event::ElementState::Released => match self.mouse_down_point {
                        Some(mut pos) => {
                            pos.y = window_h - pos.y;
                            let mut dest_pos = self.mouse_pos;
                            dest_pos.y = window_h - dest_pos.y;
                            self.create_amogus(pos, dest_pos - pos);
                        }
                        _ => {}
                    },
                }
            }
            _ => {}
        }
        return false;
    }

    pub fn ui(&mut self, ctx: &egui::Context) {
        egui::Window::new("amogus manager").show(ctx, |ui| {
            self.amogus_manager.ui(ui);
        });
    }

    pub fn update(&mut self, delta: f32, screen_w: f32, screen_h: f32) {
        if self.amogus_manager.should_kill_all {
            self.amogus_manager.should_kill_all = false;
            self.amogsuses.clear();
        }
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

            let amog_half_size = amog.size / 2.0;
            amog.pos += amog.velocity * delta;
            amog.velocity.y += -GRAVITY * delta;

            if amog.pos.y - amog_half_size <= 0.0 {
                amog.pos.y = amog_half_size;
                amog.velocity.y *= -1.0;
            }
            if amog.pos.y >= screen_h - amog_half_size {
                amog.pos.y = screen_h - amog_half_size;
                amog.velocity.y *= -1.0;
            }
            if amog.pos.x - amog_half_size <= 0.0 {
                amog.pos.x = amog_half_size;
                amog.velocity.x *= -1.0;
            }
            if amog.pos.x >= screen_w - amog_half_size {
                amog.pos.x = screen_w - amog_half_size;
                amog.velocity.x *= -1.0;
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
                size: single_amogus.size,
                color: single_amogus.color,
                animation_frame: single_amogus.animation_frame,
            })
            .collect();
        self.amogus_renderer.draw(display, frame, amogsuses_data);
    }

    fn create_amogus(
        &mut self,
        location: nalgebra::Vector2<f32>,
        velocity: nalgebra::Vector2<f32>,
    ) {
        let size = if self.amogus_manager.random_size {
            self.random_generator.rand_float() * (MAX_AMOGUS_SIZE - MIN_AMOGUS_SIZE)
                + MIN_AMOGUS_SIZE
        } else {
            self.amogus_manager.size
        };
        let color = if self.amogus_manager.random_color {
            [
                self.random_generator.rand_float(),
                self.random_generator.rand_float(),
                self.random_generator.rand_float(),
            ]
        } else {
            self.amogus_manager.color
        };
        let twerks_per_second = if self.amogus_manager.random_twerk_speed {
            self.random_generator.rand_float() * (MAX_AMOGUS_TWERK - MIN_AMOGUS_TWERK)
                + MIN_AMOGUS_TWERK
        } else {
            self.amogus_manager.twerk_speed
        };
        self.amogsuses.push(SingleAmogus {
            pos: location,
            velocity,
            size,
            color,
            animation_frame: 0,
            twerks_per_second,
            last_frame_switch: Instant::now(),
        });
    }
}
