use std::sync::{RwLock, Arc, Weak};
use std::rc::Rc;
use cgmath::{vec3, Vector3, Quaternion, Matrix4, Transform};
use crate::graphics::Renderable;

/// The node is the root of anything that is in the scene.
/// Nodes have a list of `Instance`s, which represent the functionality of the node.
pub trait Node {
    /// This is an internal function. Do not call this yourself!
    /// Alternatives include: `Node::new_child`, `Scene::new_root_node_3d`.
    /// 
    /// Creates a new node with default settings and no parent, children, instances or renderable.
    /// You must give the node a weak reference to itself so that `get_ref` can work correctly.
    fn new_default() -> Arc<RwLock<Self>>;

    /// Returns the reference to the lock on the current node.
    fn get_ref(&self) -> Weak<RwLock<Self>>;

    /// Returns the parent of this node, if one exists.
    fn get_parent(&self) -> Weak<RwLock<Self>>;
    /// Sets the current parent of this node.
    fn set_parent(&mut self, parent: Weak<RwLock<Self>>);

    /// Returns the ordered list of children this node contains.
    /// Any changes to the transformation of this node are also visible in the child nodes.
    fn get_children(&self) -> &Vec<Arc<RwLock<Self>>>;
    
    /// Returns the ordered list of instances this node contains.
    /// Because this returns a vector without synchronisation, by acquiring a lock to read/write
    /// the node itself, you can read/write all of its instances.
    fn get_instances(&mut self) -> &mut Vec<Box<dyn Instance<NodeType=Self>>>;

    /// Retrieves a reference to the renderable that we are going to try to render with this instance, if we want to actually render something.
    fn get_renderable(&self) -> &Option<Rc<Renderable>>;
    /// Makes this node no longer render anything.
    fn clear_renderable(&mut self);
    /// Sets the renderable that this node will render.
    fn set_renderable(&mut self, renderable: Rc<Renderable>);

    /// Adds a new child to this node. Use this instead of making new nodes manually.
    fn new_child(&mut self) -> Arc<RwLock<Self>> {
        let child = Self::new_default();
        child.write().unwrap().set_parent(self.get_ref());
        child
    }
}

/// `Instance`s supply functionality to `Node`s.
pub trait Instance {
    type NodeType: Node;
}

/// A `Node3D` is a `Node` that exists in a 3D setting. It may only have children that are also `Node3D`s.
pub struct Node3D {
    /// References this node. Useful for reparenting etc.
    self_ref: Weak<RwLock<Node3D>>,
    /// What index child is this node inside the parent?
    /// If there is no parent, this value is irrelevant.
    idx_in_parent: usize,

    /// The position of the node, relative to its parent.
    pos: Vector3<f32>,
    /// The rotation of the node, relative to its parent.
    rot: Quaternion<f32>,
    /// The scale of the node (which can be different for each axis), relative to its parent.
    scl: Vector3<f32>,
    /// The matrix that represents the transformation of this node, relative to its parent.
    xform: Matrix4<f32>,

    /// The parent of this node, if one exists.
    parent: Weak<RwLock<Node3D>>,
    /// The ordered list of children this node contains.
    /// Any changes to the transformation of this node are also visible in the child nodes.
    children: Vec<Arc<RwLock<Node3D>>>,
    /// The ordered list of instances this node contains.
    instances: Vec<Box<dyn Instance<NodeType=Self>>>,
    /// A reference to the renderable that we are going to try to render with this instance, if we want to actually render something.
    renderable: Option<Rc<Renderable>>,
}

impl Node for Node3D {
    fn new_default() -> Arc<RwLock<Node3D>> {
        let arc = Arc::new(RwLock::new(Node3D {
            self_ref: Weak::new(),
            idx_in_parent: 0,

            pos: vec3(0.0, 0.0, 0.0),
            rot: Quaternion::new(1.0, 0.0, 0.0, 0.0),
            scl: vec3(1.0, 1.0, 1.0),
            xform: Matrix4::one(),

            parent: Weak::new(),
            children: Vec::new(),
            instances: Vec::new(),
            renderable: None
        }));
        arc.write().unwrap().self_ref = Arc::downgrade(&arc);
        arc
    }

    fn get_ref(&self) -> Weak<RwLock<Node3D>> {
        Weak::clone(&self.self_ref)
    }

    fn get_parent(&self) -> Weak<RwLock<Node3D>> {
        Weak::clone(&self.parent)
    }
    fn set_parent(&mut self, parent: Weak<RwLock<Self>>) {
        if let Some(p) = self.parent.upgrade() {
            // Remove this node from the parent's list of children.
            let mut write = p.write().unwrap();
            let parent_children = &mut write.children;
            parent_children.remove(self.idx_in_parent);
            // Update the idx_in_parent values for the other children,
            // so that their references are not broken.
            for i in self.idx_in_parent+1 .. parent_children.len() {
                parent_children[i].write().unwrap().idx_in_parent -= 1;
            }
        }
        self.parent = parent;
        if let Some(p) = self.parent.upgrade() {
            let mut write = p.write().unwrap();
            let parent_children = &mut write.children;
            self.idx_in_parent = parent_children.len();
            parent_children.push(self.get_ref().upgrade().unwrap());
        }
    }

    fn get_children(&self) -> &Vec<Arc<RwLock<Node3D>>> {
        &self.children
    }

    fn get_instances(&mut self) -> &mut Vec<Box<dyn Instance<NodeType=Node3D>>> {
        &mut self.instances
    }

    fn get_renderable(&self) -> &Option<Rc<Renderable>> {
        &self.renderable
    }
    fn clear_renderable(&mut self) {
        self.renderable = None
    }
    fn set_renderable(&mut self, renderable: Rc<Renderable>) {
        self.renderable = Some(renderable)
    }
}
