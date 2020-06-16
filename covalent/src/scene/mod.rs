use std::sync::{RwLock, Arc};

mod node;
pub use node::*;

/// The scene contains everything that the user can see or hear, and anything that interacts with that.
/// Covalent will automatically render everything in this scene according to the active render pipeline.
pub struct Scene {
    nodes_3d: Vec<Arc<RwLock<Node3D>>>,
    tick_handler: Arc<RwLock<EventHandler<TickEvent>>>
}

impl Scene {
    pub fn new() -> Scene {
        Scene {
            nodes_3d: Vec::new(),
            tick_handler: Arc::new(RwLock::new(EventHandler::new()))
        }
    }

    /// Creates a new 3D node and adds it to the scene.
    pub fn new_node_3d(&mut self) -> Arc<RwLock<Node3D>> {
        let n = Node3D::new_default();
        self.nodes_3d.push(Arc::clone(&n));
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
                    s.new_node_3d().write().unwrap().set_renderable(Arc::new(Renderable::Triangle(
                        RenderVertex{ pos: vec3(0.1*i+0.01, 0.1*j+0.01, 0.02*k+0.0), col: Colour::new(0.1*i, 0.1*j, 0.1*k) },
                        RenderVertex{ pos: vec3(0.1*i+0.09, 0.1*j+0.01, 0.02*k+0.0), col: Colour::new(0.1*i, 0.1*j, 0.1*k) },
                        RenderVertex{ pos: vec3(0.1*i+0.09, 0.1*j+0.09, 0.02*k+0.0), col: Colour::new(0.1*i, 0.1*j, 0.1*k) }
                    )));
                    s.new_node_3d().write().unwrap().set_renderable(Arc::new(Renderable::Triangle(
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
        s.new_node_3d().write().unwrap().set_renderable(Arc::new(gbackend.create_mesh(verts, inds)));
        let node = s.new_node_3d();
        node.write().unwrap().get_behaviours().push(TickDebugBehaviour::create(&mut s, Arc::downgrade(&node)));
        s
    }

    /// Iterates over all the 3D nodes in the scene.
    pub fn iter_3d(&self) -> impl Iterator<Item=&Arc<RwLock<Node3D>>> {
        self.nodes_3d.iter()
    }
}