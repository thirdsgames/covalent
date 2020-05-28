use crate::display_hints::DisplayHints;

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
    /// - Render a single frame on the back buffer. To do this, call `ctx.render_phases` to retrieve the graphics pipeline's
    /// current list of phases.
    /// - Swap the back and front buffers.
    fn main_loop(self, ctx: crate::Context, dh: DisplayHints);
}
