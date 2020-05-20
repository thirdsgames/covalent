use cgmath::{Vector2, Vector3};

/// A camera is the lens through which your scene can be viewed. This tells covalent how to map the scene in 3D space
/// onto your screen, a 2D window. The two major types of camera are perspective and orthographic.
pub enum Camera {
    /// A perspective camera is used in a 3D setting. It emulates how our eyes or cameras work, making near things appear
    /// large and far things appear small.
    Perspective(pos: Vector3<f32>)
}