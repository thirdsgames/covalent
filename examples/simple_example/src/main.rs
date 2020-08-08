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

fn create_scene(gbackend: &impl covalent::graphics::Backend, camera_matrices: Arc<RwLock<covalent::graphics::CameraMatrices>>) -> Arc<RwLock<covalent::scene::Scene>> {
    use covalent::graphics::{RenderVertex, Colour};
    use covalent::vec3;

    let s = covalent::scene::Scene::new();
    let mut verts = Vec::new();
    let mut inds = Vec::new();
    for i in (-10..10).map(|x| x as f32) {
        for j in (-10..10).map(|x| x as f32) {
            for k in (-10..10).map(|x| x as f32) {
                let v = verts.len() as u32;
                verts.push(RenderVertex{ pos: vec3(0.1*i+0.01, 0.1*j+0.01, 0.02*k+0.0), col: Colour::new(0.1*i, 0.1*j, 0.1*k) });
                verts.push(RenderVertex{ pos: vec3(0.1*i+0.09, 0.1*j+0.01, 0.02*k+0.0), col: Colour::new(0.1*i, 0.1*j, 0.1*k) });
                verts.push(RenderVertex{ pos: vec3(0.1*i+0.09, 0.1*j+0.09, 0.02*k+0.0), col: Colour::new(0.1*i, 0.1*j, 0.1*k) });
                verts.push(RenderVertex{ pos: vec3(0.1*i+0.01, 0.1*j+0.09, 0.02*k+0.0), col: Colour::new(0.1*i, 0.1*j, 0.1*k) });
                inds.push(v);
                inds.push(v+1);
                inds.push(v+2);
                inds.push(v);
                inds.push(v+2);
                inds.push(v+3);
            }
        }
    }
    s.write().unwrap().new_node().write().unwrap().renderable = Some(Arc::new(gbackend.create_mesh(verts, inds)));
    let node = s.write().unwrap().new_node();
    covalent::scene::TickDebugComponent::new(Arc::clone(&node));
    covalent::scene::TickDebugComponent::new(Arc::clone(&node));

    let cam = covalent::graphics::PerspectiveCamera::new(
        covalent::pt3(1.1, 1.1, 0.0),
        covalent::vec3(-1.0, -1.0, -3.0),
        covalent::vec3(0.0, 0.0, 1.0),
    );
    covalent::scene::CameraMotionComponent::new(Arc::clone(&node), cam, camera_matrices);

    s
}

pub fn create_scene_unoptimised() -> Arc<RwLock<covalent::scene::Scene>> {
    use covalent::graphics::{Renderable, RenderVertex, Colour};
    use covalent::vec3;

    let s = covalent::scene::Scene::new();
    for i in (-10..10).map(|x| x as f32) {
        for j in (-10..10).map(|x| x as f32) {
            for k in (-10..10).map(|x| x as f32) {
                s.write().unwrap().new_node().write().unwrap().renderable = Some(Arc::new(Renderable::Triangle(
                    RenderVertex{ pos: vec3(0.1*i+0.01, 0.1*j+0.01, 0.02*k+0.0), col: Colour::new(0.1*i, 0.1*j, 0.1*k) },
                    RenderVertex{ pos: vec3(0.1*i+0.09, 0.1*j+0.01, 0.02*k+0.0), col: Colour::new(0.1*i, 0.1*j, 0.1*k) },
                    RenderVertex{ pos: vec3(0.1*i+0.09, 0.1*j+0.09, 0.02*k+0.0), col: Colour::new(0.1*i, 0.1*j, 0.1*k) }
                )));
                s.write().unwrap().new_node().write().unwrap().renderable = Some(Arc::new(Renderable::Triangle(
                    RenderVertex{ pos: vec3(0.1*i+0.01, 0.1*j+0.01, 0.02*k+0.0), col: Colour::new(0.1*i, 0.1*j, 0.1*k) },
                    RenderVertex{ pos: vec3(0.1*i+0.01, 0.1*j+0.09, 0.02*k+0.0), col: Colour::new(0.1*i, 0.1*j, 0.1*k) },
                    RenderVertex{ pos: vec3(0.1*i+0.09, 0.1*j+0.09, 0.02*k+0.0), col: Colour::new(0.1*i, 0.1*j, 0.1*k) }
                )));
            }
        }
    }
    s
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

    let mut pipeline = covalent::graphics::Pipeline::new();

    pipeline.add_phase(
        0,
        "Clear".to_string(),
        covalent::graphics::PipelinePhase::Clear {
            target: covalent::graphics::RenderTarget::Window,
        },
    );

    let render_settings = covalent::graphics::RenderSettings::default();
    let render_camera_matrices = Arc::clone(&render_settings.camera_matrices);
    pipeline.add_phase(
        100,
        "Render".to_string(),
        covalent::graphics::PipelinePhase::Render {
            settings: render_settings,
            target: covalent::graphics::RenderTarget::Window,
        },
    );

    let scene = create_scene(&backend, render_camera_matrices);
    //let scene = create_scene_unoptimised();

    covalent::execute(scene, pipeline, backend);
}
