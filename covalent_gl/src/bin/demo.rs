use covalent;
use covalent_gl;

fn main() {
    let mut hints = covalent::DisplayHints::new();
    hints.title = String::from("Covalent | Demo");

    let backend = covalent_gl::BackendGL::new();

    let mut pipeline = covalent::graphics::Pipeline::new();
    pipeline.add_phase(0, String::from("Clear"), covalent::graphics::PipelinePhase::Clear {
        target: covalent::graphics::RenderTarget::Window
    });
    pipeline.add_phase(100, String::from("Render"), covalent::graphics::PipelinePhase::Render {
        target: covalent::graphics::RenderTarget::Window
    });

    covalent::execute(hints, pipeline, backend);
}