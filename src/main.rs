#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]

use std::time::Instant;

use glium::glutin;

mod app;
use app::App;
mod amogus;
mod amogus_manager;

fn main() {
    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new()
        .with_min_inner_size(glutin::dpi::PhysicalSize::new(200, 200))
        .with_title("AMOGUS");
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    let mut egui_glium = egui_glium::EguiGlium::new(&display, &event_loop);

    let mut app = App::new(&display);

    let mut last_update = Instant::now();

    event_loop.run(move |event, _, control_flow| {
        control_flow.set_poll();
        match event {
            glutin::event::Event::WindowEvent { event, .. } => {
                let event_response = egui_glium.on_event(&event);
                if event_response.consumed {
                    return;
                }
                let glium::glutin::dpi::PhysicalSize {
                    width: _,
                    height: screen_h,
                } = display.gl_window().window().inner_size();
                if app.input(&event, screen_h as f32) {
                    return;
                }
                match event {
                    glutin::event::WindowEvent::CloseRequested => {
                        control_flow.set_exit();
                    }
                    _ => {}
                }
            }
            glutin::event::Event::MainEventsCleared => {
                display.gl_window().window().request_redraw();

                egui_glium.run(&display, |ctx| {
                    app.ui(ctx);
                });

                let window_size = display.gl_window().window().inner_size();
                app.update(
                    Instant::now().duration_since(last_update).as_secs_f32(),
                    window_size.width as f32,
                    window_size.height as f32,
                );
                last_update = Instant::now();
            }
            glutin::event::Event::RedrawRequested(_) => {
                let mut target = display.draw();

                app.draw(&display, &mut target);
                egui_glium.paint(&display, &mut target);

                target.finish().unwrap();
            }
            _ => {}
        }
    });
}
