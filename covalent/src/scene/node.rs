use crate::scene::Scene;
use std::collections::HashMap;
use std::sync::{RwLock, Arc, Weak};
use cgmath::{vec3, Vector3, Quaternion, Matrix4, Transform};
use crate::graphics::Renderable;
use std::borrow::{BorrowMut, Borrow};

/// The node is the root of anything that is in the scene.
/// Nodes have a list of `Behaviour`s, which represent the functionality of the node.
pub struct Node {
    /// The position of the node.
    pos: Vector3<f32>,
    /// The rotation of the node.
    rot: Quaternion<f32>,
    /// The scale of the node (which can be different for each axis).
    scl: Vector3<f32>,
    /// The matrix that represents the transformation of this node.
    xform: Matrix4<f32>,

    /// The ordered list of behaviours this node contains.
    pub behaviours: Vec<Arc<RwLock<dyn Behaviour>>>,

    /// A reference to the renderable that we are going to try to render with this instance, if we want to actually render something.
    pub renderable: Option<Arc<Renderable>>,
}

impl Node {
    /// This is an internal function. Do not call this yourself!
    /// Alternative: `Scene::new_node`.
    /// 
    /// Creates a new node with default settings and no instances or renderable.
    /// Does not implement `Default`: we want to encapsulate every node in an `Arc<RwLock<>>`.
    pub(crate) fn default() -> Arc<RwLock<Self>> {
        Arc::new(RwLock::new(Node {
            pos: vec3(0.0, 0.0, 0.0),
            rot: Quaternion::new(1.0, 0.0, 0.0, 0.0),
            scl: vec3(1.0, 1.0, 1.0),
            xform: Matrix4::one(),

            behaviours: Vec::new(),

            renderable: None
        }))
    }
}

type ListenerID = i32;

/// Listens for an event. Only really exists inside the `EventHandler`.
struct Listener<B, E>
    where B: Behaviour, E: Event {
    id: ListenerID,
    b: RwLock<B>,
    /// Has the same lifetime as the node that owns the behaviour that created the listener.
    func: Box<dyn Fn(&mut B, &E) + Send + Sync>
}

impl Node {
    /// Listens for any events coming from the given event handler. When an event is found, it calls the
    /// provided function with the given behaviour, and the event that was detected.
    pub fn listen<B, E>(&mut self, handler: &Arc<RwLock<EventHandler<E>>>, b: B, func: &'static (impl Fn(&mut B, &E) + Send + Sync))
        where B: Behaviour + 'static, E: Event + 'static {

        let l = Listener {
            id: handler.write().unwrap().new_id(),
            b: RwLock::new(b),
            func: Box::new(func)
        };
        handler.write().unwrap().set.insert(l.id, Box::new(l));
    }
}

trait AnyTypeListener<E: Event>: Send + Sync {
    fn execute(&self, e: &E) -> bool;
}

impl <B, E> AnyTypeListener<E> for Listener<B, E>
    where B: Behaviour, E: Event {
    
    /// Tries to execute the listener's function.
    /// Returns true if the listener should be deleted from the event handler.
    /// 
    /// In the process of performing this action, we will unbox the node and the behaviour that are encapsulated in the listener.
    /// If either of the weak variables fail to upgrade to strong smart pointers (`Arc` variables), the node is considered deleted.
    /// In this case, the listener can never fire. The function then returns true without doing anything.
    /// Otherwise, the function will fire, and false will be returned.
    fn execute(&self, e: &E) -> bool {
        (*self.func)(&mut self.b.write().unwrap(), e);
        false
    }
}

/// A generic event. See `EventHandler` for more information.
pub trait Event {}

pub struct EventHandler<E: Event> {
    next_id: ListenerID,
    set: HashMap<ListenerID, Box<dyn AnyTypeListener<E>>>
}

impl <E: Event> EventHandler<E> {
    pub fn new() -> EventHandler<E> {
        EventHandler {
            next_id: 0,
            set: HashMap::new()
        }
    }

    fn new_id(&mut self) -> ListenerID {
        let id = self.next_id;
        self.next_id += 1;
        id
    }

    /// Handle the given event by passing it through all provided listeners.
    pub fn handle(&mut self, e: E) {
        let to_remove: Vec<ListenerID> = self.set.iter().filter_map(|(k, v)| {
            let result = v.execute(&e);
            if result {
                Some(*k)
            } else {
                None
            }
        }).collect();
        
        for k in to_remove {
            self.set.remove(&k);
        }
    }
}

/// Behaviours listen for events to execute event-driven code.
pub trait Behaviour: Send + Sync {
    fn node(&self) -> &Weak<RwLock<Node>>;
}

pub struct TickEvent {
}

impl Event for TickEvent {
}

pub struct TickDebugBehaviour {
    node: Weak<RwLock<Node>>,
    tick_num: i32
}

impl TickDebugBehaviour {
    pub fn new(scene: &mut Scene, node: Arc<RwLock<Node>>) {
        let b = TickDebugBehaviour { node: Arc::downgrade(&node), tick_num: 0 };

        node.write().unwrap().borrow_mut().listen(scene.tick_handler(), b, &|b2, e| {
            b2.tick_num += 1;
            println!("Tick {}", b2.tick_num);
        });
    }
}

impl Behaviour for TickDebugBehaviour {
    fn node(&self) -> &Weak<RwLock<Node>> {
        &self.node
    }
}

