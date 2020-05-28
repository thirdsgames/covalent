mod display_hints;
pub use display_hints::DisplayHints;

pub mod graphics;
pub mod scene;

pub use cgmath;
pub use cgmath::{vec1, vec2, vec3, vec4};
/// Convenience constructor for a one-dimensional point.
pub fn pt1<S>(x: S) -> cgmath::Point1<S> {
    cgmath::Point1::new(x)
}
/// Convenience constructor for a two-dimensional point.
pub fn pt2<S>(x: S, y: S) -> cgmath::Point2<S> {
    cgmath::Point2::new(x, y)
}
/// Convenience constructor for a three-dimensional point.
pub fn pt3<S>(x: S, y: S, z: S) -> cgmath::Point3<S> {
    cgmath::Point3::new(x, y, z)
}

/// A context that encapsulates the behaviour of an application run with covalent.
/// This contains all the functions that the graphics backend will execute when the given event occurs.
pub struct Context {
    graphics_pipeline: graphics::Pipeline,
    scene: scene::Scene
}

impl Context {
    /// Should be called by the graphics backend once every frame to retrieve the current graphics pipeline.
    pub fn render_phases<'a>(&'a self) -> (&scene::Scene, std::collections::btree_map::Values<'a, i32, (String, graphics::PipelinePhase)>) {
        (&self.scene, self.graphics_pipeline.iter())
    }
}

/// Construct a Covalent context from the given backend, then executes the application defined by this Covalent context.
/// Only create a single context during the lifetime of your application,
/// and only create this context on the main thread!
pub fn execute(hints: DisplayHints, pipeline: graphics::Pipeline, rb: impl graphics::Backend) {
    rb.main_loop(Context {
        graphics_pipeline: pipeline,
        scene: scene::Scene::demo_squares()
    }, hints);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
