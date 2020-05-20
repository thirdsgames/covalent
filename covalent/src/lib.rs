mod display_hints;
pub use display_hints::DisplayHints;

pub mod graphics;
pub mod scene;

pub use cgmath;

/// Construct a Covalent context from the given backend, then executes the application defined by this Covalent context.
/// Only create a single context during the lifetime of your application,
/// and only create this context on the main thread!
pub fn execute(hints: DisplayHints, pipeline: graphics::Pipeline, rb: impl graphics::Backend) {
    rb.main_loop(hints, pipeline);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
