use covalent::graphics;
use covalent::graphics::{PipelinePhase, RenderSettings, RenderTarget, RenderVertex, Renderable};
use covalent::scene::Scene;
use covalent::{Context, DisplayHints};
use glium;
use glium::glutin;
use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::Arc;
use glium::backend::glutin::glutin::event::{VirtualKeyCode, ElementState};

/// Max vertices to store in a single VBO.
const MAX_VERTS: usize = 10_000;
/// Max indices to store in a single IBO.
const MAX_INDS: usize = 10_000;

struct MeshGL {
    vbo: glium::VertexBuffer<Vertex>,
    ibo: glium::IndexBuffer<u32>,
}

/// BackendGL is a rendering backend for Covalent, using OpenGL.
pub struct BackendGL {
    /// The backend owns the glium display.
    display: glium::Display,

    /// The glium event loop. This tells us when certain events happen
    /// (e.g. user resizes window, exits application) as well as when we can render
    /// a frame to the screen.
    event_loop: Option<glium::glutin::event_loop::EventLoop<()>>,

    /// This map stores the meshes currently on the GPU.
    meshes: RefCell<HashMap<i64, MeshGL>>,
}

impl BackendGL {
    pub fn new(dh: DisplayHints) -> BackendGL {
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

        BackendGL {
            display,
            event_loop: Some(event_loop),
            meshes: RefCell::from(HashMap::new()),
        }
    }
}

fn winit_keycode_to_covalent_keycode(k: VirtualKeyCode) -> Option<covalent::input::VirtualKeyCode> {
    Some(match k {
        VirtualKeyCode::Key1 => { covalent::input::VirtualKeyCode::Key1 },
        VirtualKeyCode::Key2 => { covalent::input::VirtualKeyCode::Key2 },
        VirtualKeyCode::Key3 => { covalent::input::VirtualKeyCode::Key3 },
        VirtualKeyCode::Key4 => { covalent::input::VirtualKeyCode::Key4 },
        VirtualKeyCode::Key5 => { covalent::input::VirtualKeyCode::Key5 },
        VirtualKeyCode::Key6 => { covalent::input::VirtualKeyCode::Key6 },
        VirtualKeyCode::Key7 => { covalent::input::VirtualKeyCode::Key7 },
        VirtualKeyCode::Key8 => { covalent::input::VirtualKeyCode::Key8 },
        VirtualKeyCode::Key9 => { covalent::input::VirtualKeyCode::Key9 },
        VirtualKeyCode::Key0 => { covalent::input::VirtualKeyCode::Key0 },
        VirtualKeyCode::A => { covalent::input::VirtualKeyCode::A },
        VirtualKeyCode::B => { covalent::input::VirtualKeyCode::B },
        VirtualKeyCode::C => { covalent::input::VirtualKeyCode::C },
        VirtualKeyCode::D => { covalent::input::VirtualKeyCode::D },
        VirtualKeyCode::E => { covalent::input::VirtualKeyCode::E },
        VirtualKeyCode::F => { covalent::input::VirtualKeyCode::F },
        VirtualKeyCode::G => { covalent::input::VirtualKeyCode::G },
        VirtualKeyCode::H => { covalent::input::VirtualKeyCode::H },
        VirtualKeyCode::I => { covalent::input::VirtualKeyCode::I },
        VirtualKeyCode::J => { covalent::input::VirtualKeyCode::J },
        VirtualKeyCode::K => { covalent::input::VirtualKeyCode::K },
        VirtualKeyCode::L => { covalent::input::VirtualKeyCode::L },
        VirtualKeyCode::M => { covalent::input::VirtualKeyCode::M },
        VirtualKeyCode::N => { covalent::input::VirtualKeyCode::N },
        VirtualKeyCode::O => { covalent::input::VirtualKeyCode::O },
        VirtualKeyCode::P => { covalent::input::VirtualKeyCode::P },
        VirtualKeyCode::Q => { covalent::input::VirtualKeyCode::Q },
        VirtualKeyCode::R => { covalent::input::VirtualKeyCode::R },
        VirtualKeyCode::S => { covalent::input::VirtualKeyCode::S },
        VirtualKeyCode::T => { covalent::input::VirtualKeyCode::T },
        VirtualKeyCode::U => { covalent::input::VirtualKeyCode::U },
        VirtualKeyCode::V => { covalent::input::VirtualKeyCode::V },
        VirtualKeyCode::W => { covalent::input::VirtualKeyCode::W },
        VirtualKeyCode::X => { covalent::input::VirtualKeyCode::X },
        VirtualKeyCode::Y => { covalent::input::VirtualKeyCode::Y },
        VirtualKeyCode::Z => { covalent::input::VirtualKeyCode::Z },
        VirtualKeyCode::Escape => { covalent::input::VirtualKeyCode::Escape },
        VirtualKeyCode::F1 => { covalent::input::VirtualKeyCode::F1 },
        VirtualKeyCode::F2 => { covalent::input::VirtualKeyCode::F2 },
        VirtualKeyCode::F3 => { covalent::input::VirtualKeyCode::F3 },
        VirtualKeyCode::F4 => { covalent::input::VirtualKeyCode::F4 },
        VirtualKeyCode::F5 => { covalent::input::VirtualKeyCode::F5 },
        VirtualKeyCode::F6 => { covalent::input::VirtualKeyCode::F6 },
        VirtualKeyCode::F7 => { covalent::input::VirtualKeyCode::F7 },
        VirtualKeyCode::F8 => { covalent::input::VirtualKeyCode::F8 },
        VirtualKeyCode::F9 => { covalent::input::VirtualKeyCode::F9 },
        VirtualKeyCode::F10 => { covalent::input::VirtualKeyCode::F10 },
        VirtualKeyCode::F11 => { covalent::input::VirtualKeyCode::F11 },
        VirtualKeyCode::F12 => { covalent::input::VirtualKeyCode::F12 },
        VirtualKeyCode::F13 => { covalent::input::VirtualKeyCode::F13 },
        VirtualKeyCode::F14 => { covalent::input::VirtualKeyCode::F14 },
        VirtualKeyCode::F15 => { covalent::input::VirtualKeyCode::F15 },
        VirtualKeyCode::F16 => { covalent::input::VirtualKeyCode::F16 },
        VirtualKeyCode::F17 => { covalent::input::VirtualKeyCode::F17 },
        VirtualKeyCode::F18 => { covalent::input::VirtualKeyCode::F18 },
        VirtualKeyCode::F19 => { covalent::input::VirtualKeyCode::F19 },
        VirtualKeyCode::F20 => { covalent::input::VirtualKeyCode::F20 },
        VirtualKeyCode::F21 => { covalent::input::VirtualKeyCode::F21 },
        VirtualKeyCode::F22 => { covalent::input::VirtualKeyCode::F22 },
        VirtualKeyCode::F23 => { covalent::input::VirtualKeyCode::F23 },
        VirtualKeyCode::F24 => { covalent::input::VirtualKeyCode::F24 },
        VirtualKeyCode::Snapshot => { covalent::input::VirtualKeyCode::Snapshot },
        VirtualKeyCode::Scroll => { covalent::input::VirtualKeyCode::Scroll },
        VirtualKeyCode::Pause => { covalent::input::VirtualKeyCode::Pause },
        VirtualKeyCode::Insert => { covalent::input::VirtualKeyCode::Insert },
        VirtualKeyCode::Home => { covalent::input::VirtualKeyCode::Home },
        VirtualKeyCode::Delete => { covalent::input::VirtualKeyCode::Delete },
        VirtualKeyCode::End => { covalent::input::VirtualKeyCode::End },
        VirtualKeyCode::PageDown => { covalent::input::VirtualKeyCode::PageDown },
        VirtualKeyCode::PageUp => { covalent::input::VirtualKeyCode::PageUp },
        VirtualKeyCode::Left => { covalent::input::VirtualKeyCode::Left },
        VirtualKeyCode::Up => { covalent::input::VirtualKeyCode::Up },
        VirtualKeyCode::Right => { covalent::input::VirtualKeyCode::Right },
        VirtualKeyCode::Down => { covalent::input::VirtualKeyCode::Down },
        // `winit` suggested renaming this to Backspace, so covalent has done this
        VirtualKeyCode::Back => { covalent::input::VirtualKeyCode::Backspace },
        VirtualKeyCode::Return => { covalent::input::VirtualKeyCode::Return },
        VirtualKeyCode::Space => { covalent::input::VirtualKeyCode::Space },
        VirtualKeyCode::Compose => { covalent::input::VirtualKeyCode::Compose },
        VirtualKeyCode::Caret => { covalent::input::VirtualKeyCode::Caret },
        VirtualKeyCode::Numlock => { covalent::input::VirtualKeyCode::Numlock },
        VirtualKeyCode::Numpad0 => { covalent::input::VirtualKeyCode::Numpad0 },
        VirtualKeyCode::Numpad1 => { covalent::input::VirtualKeyCode::Numpad1 },
        VirtualKeyCode::Numpad2 => { covalent::input::VirtualKeyCode::Numpad2 },
        VirtualKeyCode::Numpad3 => { covalent::input::VirtualKeyCode::Numpad3 },
        VirtualKeyCode::Numpad4 => { covalent::input::VirtualKeyCode::Numpad4 },
        VirtualKeyCode::Numpad5 => { covalent::input::VirtualKeyCode::Numpad5 },
        VirtualKeyCode::Numpad6 => { covalent::input::VirtualKeyCode::Numpad6 },
        VirtualKeyCode::Numpad7 => { covalent::input::VirtualKeyCode::Numpad7 },
        VirtualKeyCode::Numpad8 => { covalent::input::VirtualKeyCode::Numpad8 },
        VirtualKeyCode::Numpad9 => { covalent::input::VirtualKeyCode::Numpad9 },
        VirtualKeyCode::AbntC1 => { covalent::input::VirtualKeyCode::AbntC1 },
        VirtualKeyCode::AbntC2 => { covalent::input::VirtualKeyCode::AbntC2 },
        VirtualKeyCode::Add => { covalent::input::VirtualKeyCode::Add },
        VirtualKeyCode::Apostrophe => { covalent::input::VirtualKeyCode::Apostrophe },
        VirtualKeyCode::Apps => { covalent::input::VirtualKeyCode::Apps },
        VirtualKeyCode::At => { covalent::input::VirtualKeyCode::At },
        VirtualKeyCode::Ax => { covalent::input::VirtualKeyCode::Ax },
        VirtualKeyCode::Backslash => { covalent::input::VirtualKeyCode::Backslash },
        VirtualKeyCode::Calculator => { covalent::input::VirtualKeyCode::Calculator },
        VirtualKeyCode::Capital => { covalent::input::VirtualKeyCode::Capital },
        VirtualKeyCode::Colon => { covalent::input::VirtualKeyCode::Colon },
        VirtualKeyCode::Comma => { covalent::input::VirtualKeyCode::Comma },
        VirtualKeyCode::Convert => { covalent::input::VirtualKeyCode::Convert },
        VirtualKeyCode::Decimal => { covalent::input::VirtualKeyCode::Decimal },
        VirtualKeyCode::Divide => { covalent::input::VirtualKeyCode::Divide },
        VirtualKeyCode::Equals => { covalent::input::VirtualKeyCode::Equals },
        VirtualKeyCode::Grave => { covalent::input::VirtualKeyCode::Grave },
        VirtualKeyCode::Kana => { covalent::input::VirtualKeyCode::Kana },
        VirtualKeyCode::Kanji => { covalent::input::VirtualKeyCode::Kanji },
        VirtualKeyCode::LAlt => { covalent::input::VirtualKeyCode::LAlt },
        VirtualKeyCode::LBracket => { covalent::input::VirtualKeyCode::LBracket },
        VirtualKeyCode::LControl => { covalent::input::VirtualKeyCode::LControl },
        VirtualKeyCode::LShift => { covalent::input::VirtualKeyCode::LShift },
        VirtualKeyCode::LWin => { covalent::input::VirtualKeyCode::LWin },
        VirtualKeyCode::Mail => { covalent::input::VirtualKeyCode::Mail },
        VirtualKeyCode::MediaSelect => { covalent::input::VirtualKeyCode::MediaSelect },
        VirtualKeyCode::MediaStop => { covalent::input::VirtualKeyCode::MediaStop },
        VirtualKeyCode::Minus => { covalent::input::VirtualKeyCode::Minus },
        VirtualKeyCode::Multiply => { covalent::input::VirtualKeyCode::Multiply },
        VirtualKeyCode::Mute => { covalent::input::VirtualKeyCode::Mute },
        VirtualKeyCode::MyComputer => { covalent::input::VirtualKeyCode::MyComputer },
        VirtualKeyCode::NavigateForward => { covalent::input::VirtualKeyCode::NavigateForward },
        VirtualKeyCode::NavigateBackward => { covalent::input::VirtualKeyCode::NavigateBackward },
        VirtualKeyCode::NextTrack => { covalent::input::VirtualKeyCode::NextTrack },
        VirtualKeyCode::NoConvert => { covalent::input::VirtualKeyCode::NoConvert },
        VirtualKeyCode::NumpadComma => { covalent::input::VirtualKeyCode::NumpadComma },
        VirtualKeyCode::NumpadEnter => { covalent::input::VirtualKeyCode::NumpadEnter },
        VirtualKeyCode::NumpadEquals => { covalent::input::VirtualKeyCode::NumpadEquals },
        VirtualKeyCode::OEM102 => { covalent::input::VirtualKeyCode::OEM102 },
        VirtualKeyCode::Period => { covalent::input::VirtualKeyCode::Period },
        VirtualKeyCode::PlayPause => { covalent::input::VirtualKeyCode::PlayPause },
        VirtualKeyCode::Power => { covalent::input::VirtualKeyCode::Power },
        VirtualKeyCode::PrevTrack => { covalent::input::VirtualKeyCode::PrevTrack },
        VirtualKeyCode::RAlt => { covalent::input::VirtualKeyCode::RAlt },
        VirtualKeyCode::RBracket => { covalent::input::VirtualKeyCode::RBracket },
        VirtualKeyCode::RControl => { covalent::input::VirtualKeyCode::RControl },
        VirtualKeyCode::RShift => { covalent::input::VirtualKeyCode::RShift },
        VirtualKeyCode::RWin => { covalent::input::VirtualKeyCode::RWin },
        VirtualKeyCode::Semicolon => { covalent::input::VirtualKeyCode::Semicolon },
        VirtualKeyCode::Slash => { covalent::input::VirtualKeyCode::Slash },
        VirtualKeyCode::Sleep => { covalent::input::VirtualKeyCode::Sleep },
        VirtualKeyCode::Stop => { covalent::input::VirtualKeyCode::Stop },
        VirtualKeyCode::Subtract => { covalent::input::VirtualKeyCode::Subtract },
        VirtualKeyCode::Sysrq => { covalent::input::VirtualKeyCode::Sysrq },
        VirtualKeyCode::Tab => { covalent::input::VirtualKeyCode::Tab },
        VirtualKeyCode::Underline => { covalent::input::VirtualKeyCode::Underline },
        VirtualKeyCode::Unlabeled => { covalent::input::VirtualKeyCode::Unlabeled },
        VirtualKeyCode::VolumeDown => { covalent::input::VirtualKeyCode::VolumeDown },
        VirtualKeyCode::VolumeUp => { covalent::input::VirtualKeyCode::VolumeUp },
        VirtualKeyCode::Wake => { covalent::input::VirtualKeyCode::Wake },
        VirtualKeyCode::WebBack => { covalent::input::VirtualKeyCode::WebBack },
        VirtualKeyCode::WebFavorites => { covalent::input::VirtualKeyCode::WebFavorites },
        VirtualKeyCode::WebForward => { covalent::input::VirtualKeyCode::WebForward },
        VirtualKeyCode::WebHome => { covalent::input::VirtualKeyCode::WebHome },
        VirtualKeyCode::WebRefresh => { covalent::input::VirtualKeyCode::WebRefresh },
        VirtualKeyCode::WebSearch => { covalent::input::VirtualKeyCode::WebSearch },
        VirtualKeyCode::WebStop => { covalent::input::VirtualKeyCode::WebStop },
        VirtualKeyCode::Yen => { covalent::input::VirtualKeyCode::Yen },
        VirtualKeyCode::Copy => { covalent::input::VirtualKeyCode::Copy },
        VirtualKeyCode::Paste => { covalent::input::VirtualKeyCode::Paste },
        VirtualKeyCode::Cut => { covalent::input::VirtualKeyCode::Cut },
    })
}

impl graphics::Backend for BackendGL {
    fn main_loop(mut self, ctx: Context) {
        let vertex_shader_src = r#"
            #version 140

            uniform mat4 combined;

            in vec3 position;
            in uint col;
            
            out vec2 io_pos;
            out vec4 io_col;

            void main() {
                gl_Position = combined * vec4(position, 1.0);
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

        let program = glium::Program::from_source(
            &self.display,
            vertex_shader_src,
            fragment_shader_src,
            None,
        )
        .unwrap();

        let vbo = glium::VertexBuffer::dynamic(
            &self.display,
            &vec![
                Vertex {
                    position: [0.0, 0.0, 0.0],
                    col: 0xFFFFFFFF
                };
                MAX_VERTS
            ],
        )
        .unwrap();
        let ibo = glium::index::IndexBuffer::dynamic(
            &self.display,
            glium::index::PrimitiveType::TrianglesList,
            &vec![0u32; MAX_INDS],
        )
        .unwrap();
        let mut batch = BatchGL {
            vbo,
            ibo,
            program,
        };

        self.event_loop
            .take()
            .unwrap()
            .run(move |ev, _, control_flow| {
                *control_flow = glutin::event_loop::ControlFlow::Poll;

                match ev {
                    glutin::event::Event::WindowEvent { event, .. } => match event {
                        glutin::event::WindowEvent::CloseRequested => {
                            *control_flow = glutin::event_loop::ControlFlow::Exit;
                            return;
                        }
                        glutin::event::WindowEvent::KeyboardInput { device_id: _, input, is_synthetic: _ } => {
                            // echo this event through to covalent, so that event listeners can
                            // listen for it
                            let event = covalent::input::KeyboardEvent {
                                scan_code: input.scancode,
                                state: match input.state {
                                    ElementState::Pressed => { covalent::input::ElementState::Pressed },
                                    ElementState::Released => { covalent::input::ElementState::Released },
                                },
                                virtual_keycode: match input.virtual_keycode {
                                    None => { None },
                                    Some(k) => { winit_keycode_to_covalent_keycode(k) },
                                }
                            };
                            ctx.process_keyboard_event(event);
                        }
                        _ => return,
                    },

                    // All events have been successfully polled.
                    // We can now begin rendering the screen.
                    glutin::event::Event::MainEventsCleared => {
                        // For information about function invocation order,
                        // please see the documentation for `covalent::Context`.
                        ctx.begin_frame();

                        let mut frame = self.display.draw();

                        let (scene, phases) = ctx.render_phases();

                        for (name, phase) in phases {
                            self.execute_phase(name, &scene.read().unwrap(), phase, &mut batch, &mut frame);
                        }
                        if let Err(e) = frame.finish() {
                            eprintln!("Error caught when swapping buffers: {:?}", e);
                        }

                        ctx.end_frame();
                    }
                    _ => (),
                }
            });
    }

    fn create_mesh(&self, verts: Vec<RenderVertex>, inds: Vec<u32>) -> Renderable {
        println!(
            "Creating mesh with {} verts, {} inds",
            verts.len(),
            inds.len()
        );
        let verts1 = verts.iter().map(conv).collect::<Vec<_>>();
        let mesh = MeshGL {
            vbo: glium::VertexBuffer::new(&self.display, &verts1).unwrap(),
            ibo: glium::IndexBuffer::new(
                &self.display,
                glium::index::PrimitiveType::TrianglesList,
                &inds,
            )
            .unwrap(),
        };
        let idx = 1; // TODO create random index
        self.meshes.borrow_mut().insert(idx, mesh);
        Renderable::Mesh(idx)
    }
}

/// Convert a generic RenderVertex into an OpenGL-compatible vertex.
fn conv(v: &RenderVertex) -> Vertex {
    Vertex {
        position: [v.pos.x, v.pos.y, v.pos.z],
        col: v.col.packed(),
    }
}

static mut I: i32 = 0;

impl BackendGL {
    fn execute_phase(
        &self,
        _name: &str,
        scene: &Scene,
        phase: &PipelinePhase,
        batch: &mut BatchGL,
        frame: &mut glium::Frame,
    ) {
        match phase {
            PipelinePhase::Clear { target } => {
                // We need to clear the given target.
                let render_target = match target {
                    RenderTarget::Window => frame,
                };

                self.clear(render_target);
            }
            PipelinePhase::Render { settings, target } => {
                // We need to render to the given target.
                let render_target = match target {
                    RenderTarget::Window => frame,
                };

                self.render(settings, scene, render_target, batch);
            }
        }
    }

    fn clear(&self, render_target: &mut impl glium::Surface) {
        render_target.clear_color_and_depth((0.5, 0.5, 0.5, 1.0), std::f32::MAX);
    }

    fn render(
        &self,
        settings: &RenderSettings,
        scene: &Scene,
        render_target: &mut impl glium::Surface,
        batch: &mut BatchGL,
    ) {
        unsafe {
            I += 1;
        }

        let mut it = scene
            .iter_3d()
            .filter_map(|node| {
                node.read()
                    .unwrap()
                    .renderable
                    .as_ref()
                    .map(Arc::clone)
            })
            .peekable();

        use covalent::cgmath::Matrix;
        settings
            .cam
            .write()
            .unwrap()
            .as_perspective_camera()
            .unwrap()
            .set_pos(covalent::pt3(
                1.1,
                1.1,
                0.3 + 0.3 * ((unsafe { I } as f32) * 0.01).sin(),
            ));
        let c = settings
            .cam
            .read()
            .unwrap()
            .get_combined_matrix()
            .transpose();
        let combined = [
            [c.x.x, c.y.x, c.z.x, c.w.x],
            [c.x.y, c.y.y, c.z.y, c.w.y],
            [c.x.z, c.y.z, c.z.z, c.w.z],
            [c.x.w, c.y.w, c.z.w, c.w.w],
        ];
        let uniforms = glium::uniform! {
            combined: combined
        };

        let mut params: glium::DrawParameters = Default::default();
        params.depth.test = glium::DepthTest::IfLess;
        params.depth.write = true;

        while let Some(_) = it.peek() {
            let mut vbo = batch.vbo.map_write();
            let mut ibo = batch.ibo.map_write();
            let idx = self.render_lots(
                &mut it,
                &mut vbo,
                &mut ibo,
                render_target,
                &batch.program,
                &uniforms,
                &params,
            );
            drop(vbo);
            drop(ibo);

            if idx > 0 {
                render_target
                    .draw(
                        &batch.vbo,
                        &batch.ibo.slice(0..idx).unwrap(),
                        &batch.program,
                        &uniforms,
                        &params,
                    )
                    .unwrap();
            }
        }
    }

    /// Render as many things from the given iterator as we can in the current batch, returning the (exclusive) max index we wrote to.
    fn render_lots(
        &self,
        it: &mut std::iter::Peekable<impl Iterator<Item = Arc<Renderable>>>,
        vbo: &mut glium::buffer::WriteMapping<[Vertex]>,
        ibo: &mut glium::buffer::WriteMapping<[u32]>,
        render_target: &mut impl glium::Surface,
        program: &glium::Program,
        uniforms: &impl glium::uniforms::Uniforms,
        params: &glium::DrawParameters,
    ) -> usize {
        let mut current_vertex = 0;
        let mut current_index = 0;
        loop {
            match it.peek() {
                Some(r) => {
                    match **r {
                        Renderable::None => {
                            it.next();
                        }
                        Renderable::Triangle(v0, v1, v2) => {
                            if current_index + 3 >= MAX_INDS || current_vertex + 3 >= MAX_VERTS {
                                break; // Do not consume the triangle, leave it to the next call to render_lots.
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
                        Renderable::Mesh(i) => {
                            let mesh = &self.meshes.borrow()[&i];
                            render_target
                                .draw(&mesh.vbo, &mesh.ibo, program, uniforms, params)
                                .unwrap();
                            it.next();
                        }
                    }
                }
                None => break,
            }
        }
        current_index
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
struct Vertex {
    position: [f32; 3],
    col: u32,
}
glium::implement_vertex!(Vertex, position, col);

struct BatchGL {
    vbo: glium::VertexBuffer<Vertex>,
    ibo: glium::IndexBuffer<u32>,
    program: glium::Program,
}
