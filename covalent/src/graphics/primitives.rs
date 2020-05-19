use crate::graphics::{RenderVertex, RenderContext, Renderable};

pub struct PrimTriangle {
    verts: [RenderVertex; 3]
}

impl Renderable for PrimTriangle {
    fn render(&self, rc: &mut impl RenderContext) {
        rc.render_tri(&self.verts[0], &self.verts[1], &self.verts[2]);
    }
}