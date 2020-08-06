use crate::scene::*;
use std::sync::{RwLock, Arc, Weak};
use cgmath::{vec3, Vector3, Quaternion, Matrix4, Transform};
use crate::graphics::Renderable;

/// The node is the root of anything that is in the scene.
/// Nodes have a list of `Behaviour`s, which represent the functionality of the node.
pub struct Node {
    /// Refers to this node.
    self_ref: Weak<RwLock<Self>>,
    /// A reference to the scene that contains this node.
    scene: Weak<RwLock<Scene>>,
    /// The position of the node.
    pos: Vector3<f32>,
    /// The rotation of the node.
    rot: Quaternion<f32>,
    /// The scale of the node (which can be different for each axis).
    scl: Vector3<f32>,
    /// The matrix that represents the transformation of this node.
    xform: Matrix4<f32>,

    /// The ordered list of components this node contains.
    pub components: Vec<Arc<RwLock<dyn Component>>>,

    /// A reference to the renderable that we are going to try to render with this instance, if we want to actually render something.
    pub renderable: Option<Arc<Renderable>>,
}

impl Node {
    /// This is an internal function. Do not call this yourself!
    /// Alternative: `Scene::new_node`.
    /// 
    /// Creates a new node with default settings and no instances or renderable.
    /// Does not implement `Default`: we want to encapsulate every node in an `Arc<RwLock<>>`.
    pub(crate) fn default(scene: Arc<RwLock<Scene>>) -> Arc<RwLock<Self>> {
        let node = Arc::new(RwLock::new(Node {
            self_ref: Weak::new(),
            scene: Arc::downgrade(&scene),
            pos: vec3(0.0, 0.0, 0.0),
            rot: Quaternion::new(1.0, 0.0, 0.0, 0.0),
            scl: vec3(1.0, 1.0, 1.0),
            xform: Matrix4::one(),

            components: Vec::new(),

            renderable: None
        }));
        node.write().unwrap().self_ref = Arc::downgrade(&node);
        return node;
    }

    pub fn scene(&self) -> &Weak<RwLock<Scene>> {
        &self.scene
    }
}

/// Components listen for events to execute event-driven code.
pub trait Component: Send + Sync {}

pub struct TickDebugComponent {
    node: Weak<RwLock<Node>>,
    tick_num: i32
}

use crate::lock_data;
lock_data! {
    TickDebugData

    component: write TickDebugComponent
}

impl TickDebugComponent {
    pub fn new(node: Arc<RwLock<Node>>) {
        let component = Arc::new(RwLock::new(TickDebugComponent {
            node: Arc::downgrade(&node),
            tick_num: 0
        }));

        let data = Arc::new(RwLock::new(TickDebugData {
            component: Arc::downgrade(&component),
        }));

        if let Some(scene) = node.read().unwrap().scene.upgrade() {
            TickDebugData::listen(&data, &scene.read().unwrap().events.tick, |_event, component| {
                component.tick_num += 1;
                //println!("Tick {}", component.tick_num);
            });

            TickDebugData::listen(&data, &scene.read().unwrap().events.key, |event, component| {
                println!("{:?}", event);
            });
        }

        node.write().unwrap().components.push(component);
    }
}

impl Component for TickDebugComponent {}
