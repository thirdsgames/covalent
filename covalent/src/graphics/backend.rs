use crate::graphics::{RenderVertex, Renderable};

/// Covalent supports the use of "graphics backends", distinct rendering engines for use with covalent.
/// They all support the same rendering API, so similar code can run on multiple platforms
/// with limited, or zero, edits.
/// 
/// Code for this backend should not be called directly by your application, due to potential synchronisation issues
/// between threads. Also, some graphics backends require that all graphics code be executed only on the main application
/// thread.
/// 
/// If implementing a custom backend for covalent, you will need to make an implementation for this trait.
pub trait Backend {
    /// This function will only be called once.
    /// Should create a render context, then enter a loop that will not be terminated until the application itself quits.
    /// Every loop iteration, the following steps must be taken.
    /// - Render a single frame on the back buffer. To do this, call `ctx.render_phases` to retrieve the graphics pipeline's
    /// current list of phases.
    /// - Swap the back and front buffers.
    fn main_loop(self, ctx: crate::Context);

    /// Groups a list of triangles together to form a mesh. This is an optimised rendering primitive where all of the data
    /// has been proactively sent to the GPU, so that the object can be rendered very quickly.
    /// 
    /// Use this for large, unchanging renderables. Do not use this for small, dynamic renderables or ones that will be
    /// quickly thrown away after at most a few frames.
    /// 
    /// The `verts` parameter is a list of vertices that the mesh uses.
    /// The `inds` parameter is a list of indices into the first parameter; each group of three entries in `inds` represents
    /// a single triangle represented by the given indexed vertices.
    fn create_mesh(&self, verts: Vec<RenderVertex>, inds: Vec<u32>) -> Renderable;
}
