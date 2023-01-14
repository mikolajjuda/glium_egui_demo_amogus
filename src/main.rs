use std::time::Instant;

use glium::glutin;

mod app;
use app::App;
mod amogus;

fn main() {
    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new()
        .with_min_inner_size(glutin::dpi::PhysicalSize::new(200, 200));
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    let mut app = App::new(&display);

    let mut last_update = Instant::now();

    event_loop.run(move |event, _, control_flow| {
        control_flow.set_poll();
        match event {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    control_flow.set_exit();
                }
                _ => {}
            },
            glutin::event::Event::MainEventsCleared => {
                display.gl_window().window().request_redraw();
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

                target.finish().unwrap();
            }
            _ => {}
        }
    });
}
