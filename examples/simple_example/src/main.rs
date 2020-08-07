use std::sync::{Arc, RwLock};
use log::{info};

fn setup_logger() -> Result<(), fern::InitError> {
    if !std::path::Path::new("logs").is_dir() {
        std::fs::create_dir("logs")?;
    }
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{} {} [{}] {}",
                record.level(),
                chrono::Local::now().format("[%Y-%m-%d %H:%M:%S.%3f]"),
                record.target(),
                message
            ))
        })
        //.level(log::LevelFilter::Debug)
        .chain(std::io::stdout())
        .chain(std::fs::OpenOptions::new().write(true).create(true).truncate(true).open("logs/output.log")?)
        .apply()?;
    Ok(())
}

fn main() {
    match setup_logger() {
        Err(e) => {
            eprintln!("Could not instantiate fern logger: {:?}", e);
        }
        Ok(_) => {
            info!("Instantiated fern logger");
        }
    }

    let mut hints = covalent::DisplayHints::new();
    hints.title = String::from("Covalent | Simple Example");

    let backend = covalent_gl::BackendGL::new(hints);

    let cam = Arc::new(RwLock::new(covalent::graphics::PerspectiveCamera::new(
        covalent::pt3(1.1, 1.1, 0.0),
        covalent::vec3(-1.0, -1.0, -3.0),
        covalent::vec3(0.0, 0.0, 1.0),
    )));

    let mut pipeline = covalent::graphics::Pipeline::new();
    pipeline.add_phase(
        0,
        "Clear".to_string(),
        covalent::graphics::PipelinePhase::Clear {
            target: covalent::graphics::RenderTarget::Window,
        },
    );
    pipeline.add_phase(
        100,
        "Render".to_string(),
        covalent::graphics::PipelinePhase::Render {
            settings: covalent::graphics::RenderSettings::new(cam),
            target: covalent::graphics::RenderTarget::Window,
        },
    );

    covalent::execute(pipeline, backend);
}
