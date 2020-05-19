use glium;
use glium::glutin;
use covalent::cgmath::Vector3;
use covalent::DisplayHints;
use covalent::graphics;
use covalent::graphics::{Pipeline, PipelinePhase, RenderTarget, RenderVertex, Renderable, Colour};

/// Max vertices to store in a single VBO.
const MAX_VERTS : usize = 10_000;
/// Max indices to store in a single IBO.
const MAX_INDS : usize = 10_000;

/// BackendGL is a rendering backend for Covalent, using OpenGL.
pub struct BackendGL;

impl BackendGL {
    pub fn new() -> BackendGL {
        BackendGL {}
    }
}

impl graphics::Backend for BackendGL {
    fn main_loop(self, dh: DisplayHints, pipeline: Pipeline) {
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

            in vec3 position;
            in uint col;
            
            out vec2 io_pos;
            out vec4 io_col;

            void main() {
                gl_Position = vec4(position, 1.0);
                io_pos = position.xy;
                io_col = vec4(
                    ((col & uint(0xFF000000)) >> 24) / 255.0f,
                    ((col & uint(0x00FF0000)) >> 16) / 255.0f,
                    ((col & uint(0x0000FF00)) >> 8) / 255.0f,
                    ((col & uint(0x000000FF))) / 255.0f
                );
            }
        "#;
        let fragment_shader_src = r#"
            #version 140

            in vec2 io_pos;
            in vec4 io_col;

            out vec4 color;

            void main() {
                //color = vec4(io_pos.x*0.5+0.5, io_pos.y*0.5+0.5, 1.0, 1.0);
                color = io_col;
            }
        "#;
        
        let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();

        let vbo = glium::VertexBuffer::dynamic(&display, &vec![Vertex {
            position: [0.0, 0.0, 0.0],
            col: 0xFFFFFFFF
        }; MAX_VERTS]).unwrap();
        let ibo = glium::index::IndexBuffer::dynamic(&display, glium::index::PrimitiveType::TrianglesList, &vec![0u32; MAX_INDS]).unwrap();
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
                    let mut frame = display.draw();

                    for (name, phase) in pipeline.iter() {
                        self.execute_phase(name, phase, &mut batch, &mut frame);
                    }
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

/// Convert a generic RenderVertex into an OpenGL-compatible vertex.
fn conv(v: &RenderVertex) -> Vertex {
    Vertex {
        position: [v.pos.x, v.pos.y, v.pos.z],
        col: v.col.packed()
    }
}

impl BackendGL {
    fn execute_phase(&self, _name: &str, phase: &PipelinePhase, batch: &mut BatchGL, frame: &mut glium::Frame) {
        match phase {
            PipelinePhase::Clear { target } => {
                // We need to clear the given target.
                let render_target = match target {
                    RenderTarget::Window => frame
                };

                self.clear(render_target);
            },
            PipelinePhase::Render { target } => {
                // We need to render to the given target.
                let render_target = match target {
                    RenderTarget::Window => frame
                };

                self.render(render_target, batch);
            }
        }
    }

    fn clear(&self, render_target: &mut impl glium::Surface) {
        render_target.clear_color(0.5, 0.5, 0.5, 1.0);
    }

    fn render(&self, render_target: &mut impl glium::Surface, batch: &mut BatchGL) {
        let mut scene = Vec::new();
        for i in (-100..100).map(|x| x as f32) {
            for j in (-100..100).map(|x| x as f32) {
                scene.push(Renderable::Triangle(
                    RenderVertex {
                        pos: Vector3 { x: i * 0.01, y: j * 0.01, z: 0.0 },
                        col: Colour::new(1.0, 1.0, 1.0)
                    },
                    RenderVertex {
                        pos: Vector3 { x: i * 0.01 + 0.008, y: j * 0.01, z: 0.0 },
                        col: Colour::new(1.0, 1.0, 1.0)
                    },
                    RenderVertex {
                        pos: Vector3 { x: i * 0.01, y: j * 0.01 + 0.008, z: 0.0 },
                        col: Colour::new(1.0, 1.0, 1.0)
                    }
                ));
                scene.push(Renderable::Triangle(
                    RenderVertex {
                        pos: Vector3 { x: i * 0.01 + 0.008, y: j * 0.01 + 0.008, z: 0.0 },
                        col: Colour::new(1.0, 1.0, 1.0)
                    },
                    RenderVertex {
                        pos: Vector3 { x: i * 0.01 + 0.008, y: j * 0.01, z: 0.0 },
                        col: Colour::new(1.0, 1.0, 1.0)
                    },
                    RenderVertex {
                        pos: Vector3 { x: i * 0.01, y: j * 0.01 + 0.008, z: 0.0 },
                        col: Colour::new(1.0, 1.0, 1.0)
                    }
                ));
            }
        }

        let mut it = scene.iter().peekable();

        while let Some(_) = it.peek() {
            let mut vbo = batch.vbo.map_write();
            let mut ibo = batch.ibo.map_write();
            let idx = render_lots(&mut it, &mut vbo, &mut ibo);
            drop(vbo);
            drop(ibo);

            if idx > 0 {
                let params = Default::default();
                render_target.draw(&batch.vbo, &batch.ibo.slice(0 .. idx).unwrap(), &batch.program, &glium::uniforms::EmptyUniforms, &params).unwrap();
            }
        }
    }
}

/// Render as many things from the given iterator as we can in the current batch, returning the (exclusive) max index we wrote to.
fn render_lots(
    it: &mut std::iter::Peekable<std::slice::Iter<'_, Renderable>>,
    vbo: &mut glium::buffer::WriteMapping<[Vertex]>,
    ibo: &mut glium::buffer::WriteMapping<[u32]>) -> usize {
    let mut current_vertex = 0;
    let mut current_index = 0;
    loop {
        match it.peek() {
            Some(r) => {
                match r {
                    Renderable::None => {
                        it.next();
                    },
                    Renderable::Triangle(v0, v1, v2) => {
                        if current_index + 3 >= MAX_INDS || current_vertex + 3 >= MAX_VERTS {
                            break  // Do not consume the triangle, leave it to the next call to render_lots.
                        }
                        vbo.set(current_vertex + 0, conv(&v0));
                        vbo.set(current_vertex + 1, conv(&v1));
                        vbo.set(current_vertex + 2, conv(&v2));
                        ibo.set(current_index + 0, (current_vertex + 0) as u32);
                        ibo.set(current_index + 1, (current_vertex + 1) as u32);
                        ibo.set(current_index + 2, (current_vertex + 2) as u32);
                        current_vertex += 3;
                        current_index += 3;
                        it.next();
                    }
                }
            },
            None => break
        }
    }
    current_index
}

#[derive(Copy, Clone)]
#[repr(C)]
struct Vertex {
    position: [f32; 3],
    col: u32
}
glium::implement_vertex!(Vertex, position, col);

struct BatchGL {
    vbo: glium::VertexBuffer<Vertex>,
    ibo: glium::IndexBuffer<u32>,
    program: glium::Program,
}
