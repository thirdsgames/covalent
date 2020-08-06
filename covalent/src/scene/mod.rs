//! A scene is essentially everything that the user can see or hear, and anything that interacts with that.

mod node;
pub use node::*;

use std::sync::{RwLock, Arc, Weak};
use crate::events::EventHandlers;

/// The scene contains everything that the user can see or hear, and anything that interacts with that.
/// Covalent will automatically render everything in this scene according to the active render pipeline.
///
/// The scene should mostly be borrowed immutably to allow for more concurrency, many of its fields
/// are internally mutable.
pub struct Scene {
    self_ref: Weak<RwLock<Scene>>,
    nodes: Vec<Arc<RwLock<Node>>>,
    pub events: EventHandlers
}

impl Scene {
    pub fn new() -> Arc<RwLock<Scene>> {
        let scene = Arc::new(RwLock::new(Scene {
            self_ref: Weak::new(),
            nodes: Vec::new(),
            events: EventHandlers::default()
        }));
        scene.write().unwrap().self_ref = Arc::downgrade(&scene);
        scene
    }

    /// Creates a new node and adds it to the scene.
    pub fn new_node(&mut self) -> Arc<RwLock<Node>> {
        let n = Node::default(Weak::upgrade(&self.self_ref).unwrap());
        self.nodes.push(Arc::clone(&n));
        n
    }

    pub fn demo_squares_unoptimised() -> Arc<RwLock<Scene>> {
        use crate::graphics::{Renderable, RenderVertex, Colour};
        use cgmath::vec3;

        let mut s = Scene::new();
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

    pub fn demo_squares(gbackend: &impl crate::graphics::Backend) -> Arc<RwLock<Scene>> {
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
        s.write().unwrap().new_node().write().unwrap().renderable = Some(Arc::new(gbackend.create_mesh(verts, inds)));
        let node = s.write().unwrap().new_node();
        TickDebugComponent::new(Arc::clone(&node));
        TickDebugComponent::new(Arc::clone(&node));
        s
    }

    /// Iterates over all the 3D nodes in the scene.
    pub fn iter_3d(&self) -> impl Iterator<Item=&Arc<RwLock<Node>>> {
        self.nodes.iter()
    }
}