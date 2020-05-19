/// Rendering operations render to a `RenderTarget`. This could be as simple as the user's screen, or it could
/// be an off-screen framebuffer.
/// 
/// `RenderTarget`s are used in the programmable graphics pipeline to tell covalent where to render data.
/// 
/// Render targets have multiple `RenderChannel`s. These are the specific output layers that covalent will render to.
/// See the `RenderChannel` documentation for more information.
pub enum RenderTarget {
    /// The default render target is the user's screen. This is the window that covalent opens.
    Window,
}

impl RenderTarget {
    /// Tests whether the given render channel is supported by the given render target.
    pub fn is_render_channel_supported(&self, rc: RenderChannel) -> bool {
        match self {
            RenderTarget::Window => {
                match rc {
                    RenderChannel::Colour(i) => i == 0,
                    RenderChannel::Depth => true
                }
            }
        }
    }
}

/// When covalent renders a scene, it outputs the result to several render channels of a render target.
/// Not all render targets will support all types of render channel.
/// 
/// The Screen render target always has the `Colour(0)` render channel.
pub enum RenderChannel {
    /// Render targets will always output to a colour channel.
    /// The default colour channel is `Colour(0)`.
    /// Writing to `Window`'s `Colour(0)` will output the result to the user's screen.
    /// 
    /// Up to eight colour channels are supported on a single render target, with the exception of the Window
    /// render target, which has only one colour channel, `Colour(0)`.
    /// 
    /// Even though the colour channels are named for their ability to store colours, please note that the channels
    /// may contain data of any kind. Perhaps the most famous use of this feature is a technique called "deferred
    /// rendering", in which the output is represented by two "colour" buffers, representing vertex positions and
    /// normals. In a later stage, this information is used to compute the final colour of the scene.
    Colour(i32),

    /// By using the `Depth` render channel, you can tell covalent to keep track of how far away objects are
    /// from the camera. This is useful for both 3D and 2D applications:
    /// - in 3D, the depth render channel will show how far away a given pixel was from the camera
    /// - in 2D, it represents which objects will be drawn on top of which
    /// 
    /// Using the depth render channel allows 3D scenes to be rendered accurately, as it ensures that close-up
    /// objects will be drawn in front of far-away objects. It is not essential to use the depth render channel
    /// for 2D scenes, but better visual control can be gained using it.
    Depth
}