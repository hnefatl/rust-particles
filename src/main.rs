#[macro_use]
extern crate glium;
extern crate rand;
use rand::prelude::*;

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
}
implement_vertex!(Vertex, position);
type Colour = (f32, f32, f32);
type Velocity = [f32; 2];
struct Particle {
    position: Vertex,
    velocity: Velocity,
    colour: Colour,
}

struct Shaders {
    vertex: String,
    fragment: String,
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
        let min_velocity = 20.0;
        let max_velocity = 400.0;
        let mut particles = vec![];
        let mut rng = thread_rng();
        for _ in 0..num_particles {
            let speed = rng.gen_range(min_velocity, max_velocity);
            let angle = rng.gen_range(0.0, 2.0 * std::f32::consts::PI);
            particles.push(Particle {
                position: Vertex {
                    position: [
                        rng.gen_range(-width / 2.0, width / 2.0),
                        rng.gen_range(-height / 2.0, height / 2.0),
                    ],
                },
                velocity: [speed * angle.cos(), speed * angle.sin()],
                colour: (
                    rng.gen_range(0.0, 1.0),
                    rng.gen_range(0.0, 1.0),
                    rng.gen_range(0.0, 1.0),
                ),
            });
        }
        State {
            width,
            height,
            num_particles,
            particles,
        }
    }
}

fn main_loop(display: glium::Display, mut events_loop: glium::glutin::EventsLoop) {
    use glium::glutin::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent};

    let mut closed = false;
    let num_particles = 5000;
    let shaders = load_shaders().expect("Failed to load shaders");
    let shader_program =
        glium::program::Program::from_source(&display, &shaders.vertex, &shaders.fragment, None)
            .expect("Failed to create program");
    let mut state = State::new(1920.0, 1080.0, num_particles);
    let render_state = RenderState::new(&display, &shader_program);
    let mut last_time = std::time::Instant::now();
    let mut dpi = display.gl_window().window().get_hidpi_factor();
    while !closed {
        events_loop.poll_events(|ev| match ev {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => closed = true,
                WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            virtual_keycode: Some(VirtualKeyCode::Q),
                            state: ElementState::Pressed,
                            ..
                        },
                    ..
                } => closed = true,
                WindowEvent::Resized(logical_size) => {
                    let size = logical_size.to_physical(dpi);
                    state.width = size.width as f32;
                    state.height = size.height as f32;
                }
                WindowEvent::HiDpiFactorChanged(dpi_val) => {
                    dpi = dpi_val;
                }
                _ => (),
            },
            _ => (),
        });
        let mut target = display.draw();
        let current_time = std::time::Instant::now();
        let duration = (current_time - last_time).as_nanos() as f32 / 1000_000_000.0;
        update_state(&mut state, duration);
        last_time = current_time;
        render(&display, &mut target, &state, &render_state);
        target.finish().unwrap();
        println!("Loop took {}s", duration);
    }
}

struct RenderState<'a> {
    shape: Vec<Vertex>,
    vertex_buffer: glium::VertexBuffer<Vertex>,
    indices: glium::index::NoIndices,
    draw_params: glium::DrawParameters<'a>,
    shader_program: &'a glium::program::Program,
}
impl<'a> RenderState<'a> {
    fn new(display: &glium::Display, shader_program: &'a glium::program::Program) -> RenderState<'a> {
        let shape = vec![
            Vertex {
                position: [-1.0, -1.0],
            },
            Vertex {
                position: [-1.0, 1.0],
            },
            Vertex {
                position: [1.0, -1.0],
            },
            Vertex {
                position: [1.0, 1.0],
            },
        ];
        let mut result = RenderState {
            shape,
            vertex_buffer: glium::VertexBuffer::empty(display, 0).unwrap(),
            indices: glium::index::NoIndices(glium::index::PrimitiveType::TriangleStrip),
            draw_params: glium::DrawParameters {
                blend: glium::draw_parameters::Blend::alpha_blending(),
                ..Default::default()
            },
            shader_program
        };
        result.vertex_buffer = glium::VertexBuffer::new(display, &result.shape).unwrap();
        result
    }
}

fn render(
    display: &glium::Display,
    target: &mut glium::Frame,
    state: &State,
    render_state: &RenderState,
) {
    use glium::Surface;
    target.clear_color(0.0, 0.0, 0.0, 1.0);

    for particle in &state.particles {
        let uniforms = uniform! {
            resolution: [state.width, state.height],
            point_origin: particle.position.position,
            colour: particle.colour,
            radius: 100f32,
        };
        target
            .draw(
                &render_state.vertex_buffer,
                &render_state.indices,
                &render_state.shader_program,
                &uniforms,
                &render_state.draw_params,
            )
            .unwrap();
    }
}

fn load_shaders() -> Result<Shaders, std::io::Error> {
    let frag_path = "src/shader.frag";
    let vert_path = "src/shader.vert";
    std::fs::read_to_string(frag_path).and_then(|fragment| {
        std::fs::read_to_string(vert_path).map(|vertex| Shaders { vertex, fragment })
    })
}

fn update_state(state: &mut State, delta: f32) {
    for p in &mut state.particles {
        let ref mut x = p.position.position[0];
        let ref mut dx = p.velocity[0];
        *x += *dx * delta;
        if x.abs() > state.width {
            *x = x.signum() * state.width;
            *dx = -*dx;
        }
        let ref mut y = p.position.position[1];
        let ref mut dy = p.velocity[1];
        *y += *dy * delta;
        if y.abs() > state.height {
            *y = y.signum() * state.height;
            *dy = -*dy;
        }
    }
}
