use std::time;

use glium::glutin::{self, dpi, event, event_loop, window};
use glium::Surface;

fn main() {
    let event_loop = event_loop::EventLoop::new();
    let wb = window::WindowBuilder::new()
        .with_inner_size(dpi::LogicalSize::new(1024.0, 768.0))
        .with_title("Hello, world!");
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    event_loop.run(move |event, _, control_flow| {
        let next_frame_time = time::Instant::now() + time::Duration::from_nanos(16_666_667);
        *control_flow = event_loop::ControlFlow::WaitUntil(next_frame_time);

        match event {
            event::Event::WindowEvent { event, .. } => match event {
                event::WindowEvent::CloseRequested => {
                    *control_flow = event_loop::ControlFlow::Exit;
                    return;
                }
                _ => return,
            },
            event::Event::NewEvents(cause) => match cause {
                event::StartCause::ResumeTimeReached { .. } => (),
                event::StartCause::Init => (),
                _ => return,
            },
            _ => return,
        }

        let mut target = display.draw();
        target.clear_color(255.0, 255.0, 255.0, 1.0);
        target.finish().unwrap();
    });
}
