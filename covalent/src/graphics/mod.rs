mod backend;
pub use backend::*;

mod render_target;
pub use render_target::*;

mod pipeline;
pub use pipeline::*;

mod colour;
pub use colour::*;

mod camera;
pub use camera::*;

use cgmath::Vector3;

/// A renderable is an object that can be rendered and displayed on screen. The graphics backend will render these.
pub enum Renderable {
    /// A renderable that renders nothing. Used when a Renderable must be supplied but you don't want to render anything.
    None,
    /// A primitive triangle containing three vertices.
    Triangle(RenderVertex, RenderVertex, RenderVertex),
}

/// Contains all the necessary information to define a single vertex.
/// This includes its position in world space.
#[derive(Copy, Clone)]
pub struct RenderVertex {
    pub pos: Vector3<f32>,
    pub col: Colour
}
