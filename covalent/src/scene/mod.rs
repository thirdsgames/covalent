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

    /// Iterates over all the 3D nodes in the scene.
    pub fn iter_3d(&self) -> impl Iterator<Item=&Arc<RwLock<Node>>> {
        self.nodes.iter()
    }
}