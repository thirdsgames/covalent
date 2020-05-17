use crate::render_backend::RenderBackend;
use crate::covalent::DisplayHints;
use glium;

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

    fn render_frame(&mut self) {
        let frame = self.ctx.as_ref().unwrap().display.draw();

        if let Err(e) = frame.finish() {
            eprintln!("Error caught when swapping buffers: {:?}", e);
        }
    }
}