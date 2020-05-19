mod backend;
pub use backend::Backend;

mod render_target;
pub use render_target::RenderTarget;

mod pipeline;
pub use pipeline::Pipeline;
pub use pipeline::PipelinePhase;

mod colour;
pub use colour::Colour;

use cgmath::Vector3;

/// This is the trait for objects that can be rendered using a RenderContext. The graphics backend will render these.
pub enum Renderable {
    /// A renderable that renders nothing. Used when a Renderable must be supplied but you don't want to render anything.
    None,
    /// A primitive triangle containing three vertices.
    Triangle(RenderVertex, RenderVertex, RenderVertex)
}

/// Contains all the necessary information to define a single vertex.
/// This includes its position in world space.
#[derive(Copy, Clone)]
pub struct RenderVertex {
    pub pos: Vector3<f32>,
    pub col: Colour
}
