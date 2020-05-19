/// Rendering operations render to a `RenderTarget`. This could be as simple as the user's screen, or it could
/// be an off-screen framebuffer.
/// 
/// `RenderTargets` are used in the programmable graphics pipeline to tell covalent where to render data.
pub enum RenderTarget {
    /// The default render target is the user's screen. This is the window that covalent opens.
    Default,
}