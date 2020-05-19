use covalent;
use covalent_gl;

fn main() {
    let mut hints = covalent::DisplayHints::new();
    hints.title = String::from("Covalent | Demo");

    let backend = covalent_gl::BackendGL::new();

    let mut pipeline = covalent::graphics::Pipeline::new();
    pipeline.add_phase(0, String::from("Render"), covalent::graphics::PipelinePhase::Render(
        covalent::graphics::RenderSettings {},
        covalent::graphics::RenderTarget::Default
    ));

    covalent::execute(hints, pipeline, backend);
}