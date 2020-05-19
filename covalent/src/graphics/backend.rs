use crate::display_hints::DisplayHints;
use crate::graphics::Pipeline;

/// Covalent supports the use of "graphics backends", distinct rendering engines for use with covalent.
/// They all support the same rendering API, so similar code can run on multiple platforms
/// with limited, or zero, edits.
/// 
/// If implementing a custom backend for Covalent, please make implementations for the following traits:
/// - `graphics::Backend`
/// - `graphics::RenderContext`
pub trait Backend {
    /// This function will only be called once.
    /// Should create a render context, then enter a loop that will not be terminated until the application itself quits.
    /// Every loop iteration, the following steps must be taken.
    /// - Render a single frame on the back buffer.
    /// - Swap the back and front buffers.
    fn main_loop(self, dh: DisplayHints, pipeline: Pipeline);
}
