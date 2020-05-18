use crate::covalent::DisplayHints;

/// Covalent supports the use of "render backends", distinct rendering engines for use with covalent.
/// They all support the same rendering API, so similar code can run on multiple platforms
/// with limited, or zero, edits.
pub trait RenderBackend {
    /// Should create a render context.
    /// This function will only be called once.
    fn create_window(&mut self, dh: &DisplayHints);

    /// Enters a loop that will not be terminated until the application itself quits.
    /// Every loop iteration, the following steps must be taken.
    /// - Render a single frame on the back buffer.
    /// - Swap the back and front buffers.
    fn main_loop(&mut self);
}