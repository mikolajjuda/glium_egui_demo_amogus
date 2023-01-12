use glium::glutin;

mod app;
use app::App;
mod amogus;

fn main() {
    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    let app = App::new(&display);

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
            }
            glutin::event::Event::RedrawRequested(_) => {
                let mut target = display.draw();

                app.draw(&mut target);

                target.finish().unwrap();
            }
            _ => {}
        }
    });
}
