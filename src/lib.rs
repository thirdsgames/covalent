mod render_backend;
mod backend_gl;

pub use render_backend::RenderBackend;
pub use backend_gl::BackendGL;

/// The Covalent structure contains all the information required to render a scene.
pub struct Covalent {
    rb: Box<dyn RenderBackend>
}

impl Covalent {
    /// Construct a Covalent context from the given backend.
    /// Only create a single context during the lifetime of your application,
    /// and only create this context on the main thread!
    pub fn new(rb: Box<dyn RenderBackend>) -> Covalent {
        Covalent {
            rb: rb
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
