use std::sync::{RwLock, Arc};

mod node;
pub use node::*;

/// The scene contains everything that the user can see or hear, and anything that interacts with that.
/// Covalent will automatically render everything in this scene according to the active render pipeline.
/// 
/// The scene is represented as a node graph, where each node inherits the transformation of its parent.
/// This way, nodes with children can be treated as "groups" of nodes that can act as one coherent unit.
pub struct Scene {
    root_nodes_3d: Vec<Arc<RwLock<Node3D>>>
}

/// This is used to iterate over the children of a specific node, depth first.
/// E.g. if node N has children `[A, B]`; and A has children `[A1, A2]`; and B has children `[B1, B2]`; the iteration
/// order on N is: `[N, A, A1, A2, B, B1, B2]`.
pub struct NodeIterator<'a, N: Node> {
    node: Option<Arc<RwLock<N>>>,
    children: std::slice::Iter<'a, Arc<RwLock<N>>>,
    child_iter: Option<Box<NodeIterator<'a, N>>>
}
impl<N: Node> Iterator for NodeIterator<'_, N> {
    type Item = Arc<RwLock<N>>;

    fn next(&mut self) -> Option<Arc<RwLock<N>>> {
        match &mut self.child_iter {
            Some(it) => {
                let result = it.next();
                if let None = result {
                    self.child_iter = None;
                }
                result
            },
            None => match &self.node {
                Some(_) => {
                    self.node.take()
                },
                None => self.children.next().map(Arc::clone)
            }
        }
    }
}

impl Scene {
    pub fn new() -> Scene {
        Scene {
            root_nodes_3d: Vec::new()
        }
    }

    /// Creates a new 3D node and adds it directly to the root of the scene.
    pub fn new_root_node_3d(&mut self) -> Arc<RwLock<Node3D>> {
        let n = Node3D::new_default();
        self.root_nodes_3d.push(Arc::clone(&n));
        n
    }

    pub fn demo_squares() -> Scene {
        use crate::graphics::{Renderable, RenderVertex, Colour};
        use std::rc::Rc;
        use cgmath::vec3;

        let mut s = Scene::new();
        s.new_root_node_3d().write().unwrap().set_renderable(Rc::new(Renderable::Triangle(
            RenderVertex{ pos: vec3(0.1, 0.1, 0.0), col: Colour::new(1.0, 0.3, 0.9) },
            RenderVertex{ pos: vec3(0.9, 0.1, 0.0), col: Colour::new(0.0, 1.0, 0.9) },
            RenderVertex{ pos: vec3(0.7, 0.9, 0.0), col: Colour::new(0.5, 0.8, 0.1) }
        )));
        s
    }

    /// Iterates over all the 3D nodes in the scene.
    pub fn iter_3d(&self) -> NodeIterator<Node3D> {
        NodeIterator {
            node: None,
            children: self.root_nodes_3d.iter(),
            child_iter: None
        }
    }
}