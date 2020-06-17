use crate::scene::Scene;
use std::collections::HashMap;
use std::sync::{RwLock, Arc, Weak};
use cgmath::{vec3, Vector3, Quaternion, Matrix4, Transform};
use crate::graphics::Renderable;

/// The node is the root of anything that is in the scene.
/// Nodes have a list of `Instance`s, which represent the functionality of the node.
pub trait Node: Send + Sync {
    /// This is an internal function. Do not call this yourself!
    /// Alternative: `Scene::new_node_3d`.
    /// 
    /// Creates a new node with default settings and no instances or renderable.
    fn new_default() -> Arc<RwLock<Self>>;

    /// Returns the ordered list of behaviours this node contains.
    /// Because this returns a vector without synchronisation, by acquiring a lock to read/write
    /// the node itself, you can read/write all of its behaviours.
    fn get_behaviours(&mut self) -> &mut Vec<Arc<RwLock<dyn Behaviour<Self>>>>;

    /// Retrieves a reference to the renderable that we are going to try to render with this instance, if we want to actually render something.
    fn get_renderable(&self) -> &Option<Arc<Renderable>>;
    /// Makes this node no longer render anything.
    fn clear_renderable(&mut self);
    /// Sets the renderable that this node will render.
    fn set_renderable(&mut self, renderable: Arc<Renderable>);
}

type ListenerID = i32;

/// Listens for an event. Only really exists inside the `EventHandler`.
struct Listener<N, B, E>
    where N: Node, B: Behaviour<N>, E: Event {
    id: ListenerID,
    n: Weak<RwLock<N>>,
    b: Weak<RwLock<B>>,
    /// Has the same lifetime as the node that owns the behaviour that created the listener.
    func: Box<dyn Fn(&mut N, &mut B, &E) + Send + Sync>
}

/// Listens for any events coming from the given event handler. When an event is found, calls the provided function with the given node, behaviour and function.
pub fn listen<N, B, E>(handler: &mut EventHandler<E>, n: Weak<RwLock<N>>, b: Weak<RwLock<B>>, func: &'static (impl Fn(&mut N, &mut B, &E) + Send + Sync))
    where N: Node + 'static, B: Behaviour<N> + 'static, E: Event + 'static {
    let l = Listener {
        id: handler.new_id(),
        n, b,
        func: Box::new(func)
    };
    handler.set.insert(l.id, Box::new(l));
}

trait AnyTypeListener<E: Event>: Send + Sync {
    fn execute(&self, e: &E) -> bool;
}

impl <N, B, E> AnyTypeListener<E> for Listener<N, B, E>
    where N: Node, B: Behaviour<N>, E: Event {
    
    /// Tries to execute the listener's function.
    /// Returns true if the listener should be deleted from the event handler.
    /// 
    /// In the process of performing this action, we will unbox the node and the behaviour that are encapsulated in the listener.
    /// If either of the weak variables fail to upgrade to strong smart pointers (`Arc` variables), the node is considered deleted.
    /// In this case, the listener can never fire. The function then returns true without doing anything.
    /// Otherwise, the function will fire, and false will be returned.
    fn execute(&self, e: &E) -> bool {
        let n1 = self.n.upgrade();
        if n1.is_none() {
            return true;
        }
        let b1 = self.b.upgrade();
        if b1.is_none() {
            return true;
        }

        (*self.func)(&mut n1.unwrap().write().unwrap(), &mut b1.unwrap().write().unwrap(), e);

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
pub trait Behaviour<N: Node>: Send + Sync {
}

pub struct TickEvent {
}

impl Event for TickEvent {
}

pub struct TickDebugBehaviour {
    tick_num: i32
}

impl TickDebugBehaviour {
    pub fn create<N: Node + 'static>(scene: &mut Scene, n: Weak<RwLock<N>>) -> Arc<RwLock<Self>> {
        let b = Arc::new(RwLock::new(TickDebugBehaviour { tick_num: 0 }));
        listen(&mut scene.tick_handler().write().unwrap(), n, Arc::downgrade(&b), &|n, b, e| {
            b.tick_num += 1;
            println!("Tick {}", b.tick_num);
        });
        b
    }
}

impl <N: Node> Behaviour<N> for TickDebugBehaviour {}

/// A `Node3D` is a `Node` that exists in a 3D setting. It may only have children that are also `Node3D`s.
pub struct Node3D {
    /// The position of the node.
    pos: Vector3<f32>,
    /// The rotation of the node.
    rot: Quaternion<f32>,
    /// The scale of the node (which can be different for each axis).
    scl: Vector3<f32>,
    /// The matrix that represents the transformation of this node.
    xform: Matrix4<f32>,

    /// The ordered list of behaviours this node contains.
    behaviours: Vec<Arc<RwLock<dyn Behaviour<Self>>>>,

    /// A reference to the renderable that we are going to try to render with this instance, if we want to actually render something.
    renderable: Option<Arc<Renderable>>,
}

impl Node for Node3D {
    fn new_default() -> Arc<RwLock<Node3D>> {
        Arc::new(RwLock::new(Node3D {
            pos: vec3(0.0, 0.0, 0.0),
            rot: Quaternion::new(1.0, 0.0, 0.0, 0.0),
            scl: vec3(1.0, 1.0, 1.0),
            xform: Matrix4::one(),

            behaviours: Vec::new(),
            
            renderable: None
        }))
    }

    fn get_behaviours(&mut self) -> &mut Vec<Arc<RwLock<dyn Behaviour<Self>>>> {
        &mut self.behaviours
    }

    fn get_renderable(&self) -> &Option<Arc<Renderable>> {
        &self.renderable
    }
    fn clear_renderable(&mut self) {
        self.renderable = None
    }
    fn set_renderable(&mut self, renderable: Arc<Renderable>) {
        self.renderable = Some(renderable)
    }
}
