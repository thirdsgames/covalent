use crate::covalent::DisplayHints;

/// Covalent supports the use of "render backends", distinct rendering engines for use with covalent.
/// They all support the same rendering API, so similar code can run on multiple platforms
/// with limited, or zero, edits.
pub trait RenderBackend {
    /// Should create a render context.
    /// This function will only be called once.
    fn create_window(&mut self, dh: &DisplayHints);

    /// Renders a single frame on the back buffer, and swaps the back buffer to the front.
    fn render_frame(&mut self);
}