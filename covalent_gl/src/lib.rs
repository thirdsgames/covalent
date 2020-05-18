use glium;
use glium::glutin;
use glium::Surface;
use covalent::cgmath::Vector3;
use covalent::DisplayHints;
use covalent::graphics;
use covalent::graphics::RenderContext;

/// BackendGL is a rendering backend for Covalent, using OpenGL.
pub struct BackendGL;

impl BackendGL {
    pub fn new() -> BackendGL {
        BackendGL {}
    }
}

impl graphics::Backend for BackendGL {
    fn main_loop(self, dh: DisplayHints) {
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

        let vbo = glium::VertexBuffer::dynamic(&display, &vec![Vertex { position: [0.0, 0.0] }; 100]).unwrap();
        let ibo = glium::index::IndexBuffer::dynamic(&display, glium::index::PrimitiveType::TrianglesList, &vec![0u32; 100]).unwrap();
        let mut batch = BatchGL {
            vbo: vbo,
            ibo: ibo,
            program: program,
        };

        event_loop.run(move |ev, _, control_flow| {
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

                    {
                        let mut frame = display.draw();

                        {
                            let mut rc = RenderContextGL {
                                vbo: batch.vbo.map_write(),
                                ibo: batch.ibo.map_write()
                            };
                            
                            rc.render_tri(
                                &graphics::RenderVertex { pos: Vector3 { x: -0.8, y: 1.0, z: 0.0 } },
                                &graphics::RenderVertex { pos: Vector3 { x: 0.9, y: -0.4, z: 0.0 } },
                                &graphics::RenderVertex { pos: Vector3 { x: 0.1, y: -0.8, z: 0.0 } }
                            );
                        }

                        let params = Default::default();
                        frame.draw(&batch.vbo, &batch.ibo.slice(0 .. 6).unwrap(), &batch.program, &glium::uniforms::EmptyUniforms, &params).unwrap();
                        
                        if let Err(e) = frame.finish() {
                            eprintln!("Error caught when swapping buffers: {:?}", e);
                        }
                    }

                    // Simulate vsync by waiting 1/60 of a second.
                    *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);
                }
                _ => (),
            }
        });
    }
}

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
}
glium::implement_vertex!(Vertex, position);

struct BatchGL {
    vbo: glium::VertexBuffer<Vertex>,
    ibo: glium::IndexBuffer<u32>,
    program: glium::Program,
}

struct RenderContextGL<'a> {
    vbo: glium::buffer::WriteMapping<'a, [Vertex]>,
    ibo: glium::buffer::WriteMapping<'a, [u32]>
}

impl graphics::RenderContext for RenderContextGL<'_> {
    fn render_tri(&mut self, a: &graphics::RenderVertex, b: &graphics::RenderVertex, c: &graphics::RenderVertex) {
        let vbo = &mut self.vbo;
        let ibo = &mut self.ibo;

        vbo.set(0, Vertex {
            position: [ a.pos.x, a.pos.y ]
        });
        vbo.set(1, Vertex {
            position: [ b.pos.x, b.pos.y ]
        });
        vbo.set(2, Vertex {
            position: [ c.pos.x, c.pos.y ]
        });

        ibo.set(0, 0u32);
        ibo.set(1, 1u32);
        ibo.set(2, 2u32);
    }
}
