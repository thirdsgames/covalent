mod render_backend;
pub use render_backend::RenderBackend;

mod covalent;
pub use covalent::execute;
mod display_hints;
pub use display_hints::DisplayHints;
mod renderer;
pub use renderer::Renderer;
pub use renderer::Batch;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
