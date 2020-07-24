use std::sync::{RwLock, Arc};

mod node;
pub use node::*;

/// The scene contains everything that the user can see or hear, and anything that interacts with that.
/// Covalent will automatically render everything in this scene according to the active render pipeline.
pub struct Scene {
    nodes: Vec<Arc<RwLock<Node>>>,
    tick_handler: Arc<RwLock<EventHandler<TickEvent>>>
}

impl Scene {
    pub fn new() -> Scene {
        Scene {
            nodes: Vec::new(),
            tick_handler: Arc::new(RwLock::new(EventHandler::new()))
        }
    }

    /// Creates a new node and adds it to the scene.
    pub fn new_node(&mut self) -> Arc<RwLock<Node>> {
        let n = Node::default();
        self.nodes.push(Arc::clone(&n));
        n
    }

    pub fn tick_handler(&self) -> &Arc<RwLock<EventHandler<TickEvent>>> {
        &self.tick_handler
    }

    pub fn demo_squares_unoptimised() -> Scene {
        use crate::graphics::{Renderable, RenderVertex, Colour};
        use cgmath::vec3;

        let mut s = Scene::new();
        for i in (-10..10).map(|x| x as f32) {
            for j in (-10..10).map(|x| x as f32) {
                for k in (-10..10).map(|x| x as f32) {
                    s.new_node().write().unwrap().renderable = Some(Arc::new(Renderable::Triangle(
                        RenderVertex{ pos: vec3(0.1*i+0.01, 0.1*j+0.01, 0.02*k+0.0), col: Colour::new(0.1*i, 0.1*j, 0.1*k) },
                        RenderVertex{ pos: vec3(0.1*i+0.09, 0.1*j+0.01, 0.02*k+0.0), col: Colour::new(0.1*i, 0.1*j, 0.1*k) },
                        RenderVertex{ pos: vec3(0.1*i+0.09, 0.1*j+0.09, 0.02*k+0.0), col: Colour::new(0.1*i, 0.1*j, 0.1*k) }
                    )));
                    s.new_node().write().unwrap().renderable = Some(Arc::new(Renderable::Triangle(
                        RenderVertex{ pos: vec3(0.1*i+0.01, 0.1*j+0.01, 0.02*k+0.0), col: Colour::new(0.1*i, 0.1*j, 0.1*k) },
                        RenderVertex{ pos: vec3(0.1*i+0.01, 0.1*j+0.09, 0.02*k+0.0), col: Colour::new(0.1*i, 0.1*j, 0.1*k) },
                        RenderVertex{ pos: vec3(0.1*i+0.09, 0.1*j+0.09, 0.02*k+0.0), col: Colour::new(0.1*i, 0.1*j, 0.1*k) }
                    )));
                }
            }
        }
        s
    }

    pub fn demo_squares(gbackend: &impl crate::graphics::Backend) -> Scene {
        use crate::graphics::{RenderVertex, Colour};
        use cgmath::vec3;

        let mut s = Scene::new();
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
        s.new_node().write().unwrap().renderable = Some(Arc::new(gbackend.create_mesh(verts, inds)));
        let node = s.new_node();
        TickDebugBehaviour::new(&mut s, Arc::clone(&node));
        TickDebugBehaviour::new(&mut s, Arc::clone(&node));
        s
    }

    /// Iterates over all the 3D nodes in the scene.
    pub fn iter_3d(&self) -> impl Iterator<Item=&Arc<RwLock<Node>>> {
        self.nodes.iter()
    }
}