#[macro_use]
extern crate glium;

fn main() {
    use glium::glutin;

    let events_loop = glutin::EventsLoop::new();
    let wb = glutin::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &events_loop).unwrap();

    main_loop(display, events_loop);
}

fn main_loop(display: glium::Display, mut events_loop: glium::glutin::EventsLoop) {
    use glium::glutin::{Event, WindowEvent};

    let mut closed = false;
    while !closed {
        // listing the events produced by application and waiting to be received
        events_loop.poll_events(|ev| {
            match ev {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => closed = true,
                    _ => (),
                },
                _ => (),
            }
            
            let mut target = display.draw();
            render(&mut target);
            target.finish().unwrap();
        });
    }
}

fn render(target: &mut glium::Frame) {
    use glium::Surface;
    target.clear_color(0.0, 0.0, 0.0, 1.0);
}