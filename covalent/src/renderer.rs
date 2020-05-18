/// The Renderer is the struct that render calls and delegates them to the back end.
pub struct Renderer;

impl Renderer {
    /// This function will render the scene using the given batch.
    /// The function should be called every frame by the render backend.
    pub fn render(&self, b: &mut impl Batch) {
        b.begin();

        b.end();
    }
}

/// Covalent uses batched rendering. Objects are rendered to a batch, which should populate the GPU
/// with the objects. When `end` is called, the batch should then tell the GPU to perform the render.
/// Rendering in batches is much faster than in some kind of "immediate mode" (see OpenGL 2 vs 3).
///
/// While this is the intended behaviour of the Batch object, backends are free to in fact use an immediate
/// mode rendering method, where batched rendering is unsupported.
pub trait Batch {
    fn begin(&mut self);
    fn end(&mut self);
}