#[macro_use]
extern crate glium;
extern crate rand;
use rand::prelude::*;

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
}
implement_vertex!(Vertex, position);
type Colour = (f32, f32, f32, f32);
type Velocity = [f32; 2];
struct Particle {
    position: Vertex,
    velocity: Velocity,
    colour: Colour,
}

fn main() {
    use glium::glutin;

    let events_loop = glutin::EventsLoop::new();
    let wb = glutin::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &events_loop).unwrap();

    main_loop(display, events_loop);
}

struct State {
    width: f32,
    height: f32,
    num_particles: usize,
    particles: Vec<Particle>,
}
impl State {
    fn new(width: f32, height: f32, num_particles: usize) -> State {
        let mut particles = vec![];
        let mut rng = thread_rng();
        for _ in 0..num_particles {
            let speed = rng.gen_range(0.0, 1.0);
            let angle = rng.gen_range(0.0, 2.0 * std::f32::consts::PI);
            particles.push(Particle {
                position: Vertex { position: [rng.gen_range(0.0, width), rng.gen_range(0.0, height)] },
                velocity: [speed * angle.cos(), speed * angle.sin()],
                colour: (rng.gen_range(0.0, 1.0), rng.gen_range(0.0, 1.0), rng.gen_range(0.0, 1.0), 1.0)
            });
        }
        State { width, height, num_particles, particles }
    }
}

fn main_loop(display: glium::Display, mut events_loop: glium::glutin::EventsLoop) {
    use glium::glutin::{Event, WindowEvent};

    let mut closed = false;
    let (width, height) = (400.0, 600.0);
    let num_particles = 50;
    let mut state = State::new(width, height, num_particles);
    let mut last_time = std::time::Instant::now();
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
            let current_time = std::time::Instant::now();
            update_state(&mut state, (current_time - last_time).as_nanos() as f32 / 1000_000_000.0);
            last_time = current_time;
            render(&mut target, &state);
            target.finish().unwrap();
        });
    }
}

fn render(target: &mut glium::Frame, state: &State) {
    use glium::Surface;
    target.clear_color(0.0, 0.0, 0.0, 1.0);
    //https://github.com/glium/glium/blob/master/book/tuto-04-matrices.md
    //target.draw()
}

fn update_state(state: &mut State, delta: f32) {
    for p in &mut state.particles {
        p.position.position[0] += p.velocity[0] * delta;
        p.position.position[1] += p.velocity[1] * delta;
    }
}