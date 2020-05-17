use crate::render_backend::RenderBackend;

/// BackendGL is a rendering backend for Covalent.
pub struct BackendGL {

}

impl BackendGL {
    pub fn new() -> BackendGL {
        BackendGL {}
    }
}

impl RenderBackend for BackendGL {

}