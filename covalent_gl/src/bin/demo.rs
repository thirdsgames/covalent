use std::sync::{Arc, RwLock};
use covalent;
use covalent_gl;

fn main() {
    let mut hints = covalent::DisplayHints::new();
    hints.title = String::from("Covalent | Demo");

    let backend = covalent_gl::BackendGL::new();

    let cam = Arc::new(RwLock::new(covalent::graphics::PerspectiveCamera::new(
        covalent::pt3(1.1, 1.1, 0.0),
        covalent::vec3(-1.0, -1.0, -3.0),
        covalent::vec3(0.0, 0.0, 1.0),
    )));

    let mut pipeline = covalent::graphics::Pipeline::new();
    pipeline.add_phase(0, String::from("Clear"), covalent::graphics::PipelinePhase::Clear {
        target: covalent::graphics::RenderTarget::Window
    });
    pipeline.add_phase(100, String::from("Render"), covalent::graphics::PipelinePhase::Render {
        settings: covalent::graphics::RenderSettings::new(cam),
        target: covalent::graphics::RenderTarget::Window
    });

    covalent::execute(hints, pipeline, backend);
}