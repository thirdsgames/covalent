mod backend;
pub use backend::Backend;

mod primitives;
pub use primitives::*;

mod render_target;
pub use render_target::RenderTarget;

mod pipeline;
pub use pipeline::Pipeline;
pub use pipeline::PipelinePhase;
pub use pipeline::RenderSettings;

use cgmath::Vector3;

/// Covalent uses batched rendering. Objects are rendered to a batch, which should populate the GPU
/// with the objects. When dropped, the batch should then tell the GPU to perform the render.
/// Rendering in batches is much faster than in some kind of "immediate mode" (see OpenGL 2 vs 3).
///
/// While this is the intended behaviour of the `RenderContext`, backends are free to in fact use an immediate
/// mode rendering method, where batched rendering is unsupported.
/// 
/// A new `RenderContext` should be created every frame. This does not imply that a new VBO, IBO etc. should be created
/// every frame by the graphics backend; this is just a convenient API.
pub trait RenderContext {
    fn render_tri(&mut self, a: &RenderVertex, b: &RenderVertex, c: &RenderVertex);
}

/// This is the trait for objects that can be rendered using a RenderContext.
pub trait Renderable {
    fn render(&self, rc: &mut impl RenderContext);
}

/// Contains all the necessary information to define a single vertex.
/// This includes its position in world space.
#[derive(Copy, Clone)]
pub struct RenderVertex {
    pub pos: Vector3<f32>
}
