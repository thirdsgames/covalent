use crate::render_backend::RenderBackend;
use crate::covalent::DisplayHints;
use glium;
use glium::glutin;
use glium::Surface;

/// BackendGL is a rendering backend for Covalent, using OpenGL.
pub struct BackendGL {
    ctx: Option<Context>
}

/// The render context, containing all glium information.
/// This is kept separate from the BackendGL struct so that we can create the window *after*
/// creating the backend struct.
struct Context {
    event_loop: glium::glutin::event_loop::EventLoop<()>,
    display: glium::Display
}

impl BackendGL {
    pub fn new() -> BackendGL {
        BackendGL {
            ctx: None
        }
    }
}

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
}
glium::implement_vertex!(Vertex, position);

impl RenderBackend for BackendGL {
    fn create_window(&mut self, dh: &DisplayHints) {
        // 1. The **winit::EventsLoop** for handling events.
        let event_loop = glium::glutin::event_loop::EventLoop::new();
        // 2. Parameters for building the Window.
        let wb = glium::glutin::window::WindowBuilder::new()
            .with_inner_size(glium::glutin::dpi::LogicalSize::new(dh.width, dh.height))
            .with_title(dh.title.clone());
        // 3. Parameters for building the OpenGL context.
        let cb = glium::glutin::ContextBuilder::new();
        // 4. Build the Display with the given window and OpenGL context parameters and register the
        //    window with the events_loop.
        let display = glium::Display::new(wb, cb, &event_loop).unwrap();

        self.ctx = Some(Context {
            event_loop: event_loop,
            display: display
        });
    }

    fn main_loop(&mut self) {
        let ctx = self.ctx.take().unwrap();
        let display = ctx.display;
        
        let shape = vec![
            Vertex { position: [ -0.5, -0.5 ] },
            Vertex { position: [  0.0,  0.5 ] },
            Vertex { position: [  0.5, -0.5 ] },
        ];
        let vbo = glium::VertexBuffer::new(&display, &shape).unwrap();
        let ibo = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

        let vertex_shader_src = r#"
            #version 140

            in vec2 position;

            void main() {
                gl_Position = vec4(position, 0.0, 1.0);
            }
        "#;
        let fragment_shader_src = r#"
            #version 140

            out vec4 color;

            void main() {
                color = vec4(1.0, 0.0, 0.0, 1.0);
            }
        "#;
        let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();

        ctx.event_loop.run(move |ev, _, control_flow| {
            *control_flow = glutin::event_loop::ControlFlow::Poll;

            match ev {
                glutin::event::Event::WindowEvent { event, .. } => match event {
                    glutin::event::WindowEvent::CloseRequested => {
                        *control_flow = glutin::event_loop::ControlFlow::Exit;
                        return;
                    },
                    _ => return,
                },

                // All events have been successfully polled.
                // We can now begin rendering the screen.
                glutin::event::Event::MainEventsCleared => {
                    let next_frame_time = std::time::Instant::now() + std::time::Duration::from_nanos(16_666_667);

                    let mut frame = display.draw();

                    frame.draw(&vbo, &ibo, &program, &glium::uniforms::EmptyUniforms,
                        &Default::default()).unwrap();

                    if let Err(e) = frame.finish() {
                        eprintln!("Error caught when swapping buffers: {:?}", e);
                    }

                    // Simulate vsync by waiting 1/60 of a second.
                    *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);
                }
                _ => (),
            }
        });
    }
}