use crate::scene::Scene;
use std::collections::HashMap;
use std::sync::{RwLock, Arc, Weak};
use cgmath::{vec3, Vector3, Quaternion, Matrix4, Transform};
use crate::graphics::Renderable;
use rayon::prelude::*;

/// The node is the root of anything that is in the scene.
/// Nodes have a list of `Behaviour`s, which represent the functionality of the node.
pub struct Node {
    /// Refers to this node.
    self_ref: Weak<RwLock<Self>>,
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
        let node = Arc::new(RwLock::new(Node {
            self_ref: Weak::new(),
            pos: vec3(0.0, 0.0, 0.0),
            rot: Quaternion::new(1.0, 0.0, 0.0, 0.0),
            scl: vec3(1.0, 1.0, 1.0),
            xform: Matrix4::one(),

            behaviours: Vec::new(),

            renderable: None
        }));
        node.write().unwrap().self_ref = Arc::downgrade(&node);
        return node;
    }
}

type ListenerID = i64;

/// Listens for an event. Don't create these yourself, use the `lock_data` macro to automatically
/// create listeners.
///
/// Only really exists inside the `EventHandler`.
pub struct Listener<E>
    where E: Event {
    /// A uniquely generated ID for this listener, which allows the event handler to locate specific
    /// listeners if required.
    pub id: ListenerID,

    /// When the function first returns false (i.e. some elements of the listen function could not
    /// be upgraded to strong Arcs), this listener is deleted.
    pub func: Box<dyn Fn(&E) -> bool + Send + Sync>
}

impl<E> Listener<E>
    where E: Event {
    
    /// Tries to execute the listener's function.
    /// Returns false if the listener should be deleted from the event handler.
    /// 
    /// In the process of performing this action, we will unbox the node and the behaviour that are encapsulated in the listener.
    /// If either of the weak variables fail to upgrade to strong smart pointers (`Arc` variables), the node is considered deleted.
    /// In this case, the listener can never fire. The function then returns false without doing anything.
    /// Otherwise, the function will fire, and true will be returned.
    fn execute(&self, e: &E) -> bool {
        (*self.func)(e)
    }
}

/// A generic event. See `EventHandler` for more information.
pub trait Event: Send + Sync {}

/// A macro to generate a struct containing a list of fields that may be locked.
/// This allows for more intuitive concurrency, by abstracting away the lock-unlock logic
/// and potential lock mishaps.
///
/// ```
/// struct HelloWorldObject {
///     message: &'static str
/// }
///
/// struct Output {
///     message: Option<&'static str>
/// }
///
/// use covalent::lock_data;
/// lock_data! {
///     HelloWorldData
///
///     hello_world: read HelloWorldObject,
///     output: write Output
/// }
///
/// fn create_locks() {
///     use std::sync::*;
///     use covalent::scene::{Event, EventHandler};
///
///     let hello_world = Arc::new(RwLock::new(HelloWorldObject { message: "Hello, world!" }));
///     let output = Arc::new(RwLock::new(Output { message: None }));
///
///     let data = Arc::new(RwLock::new(HelloWorldData {
///         hello_world: Arc::downgrade(&hello_world),
///         output: Arc::downgrade(&output)
///     }));
///
///     struct HelloWorldEvent {}
///     impl Event for HelloWorldEvent {}
///     let event_handler: Arc<RwLock<EventHandler<HelloWorldEvent>>> = Arc::new(RwLock::new(EventHandler::new()));
///
///     HelloWorldData::listen(&data, &event_handler, |event, hello_world, output| {
///         output.message = Some(hello_world.message);
///     });
/// }
/// ```
#[macro_export]
macro_rules! lock_data {
    ($struct_name:ident $($name:ident : $mutability:ident $data_type:ty),+) => {
        struct $struct_name {
        // Populate the fields of the struct.
            $(
                $name : std::sync::Weak<std::sync::RwLock<$data_type>>,
            )*
        }

        // Implement the listen function.
        impl $struct_name {
            fn listen<'a, E, F>(data: &std::sync::Arc<std::sync::RwLock<Self>>, handler: &std::sync::Arc<std::sync::RwLock<$crate::scene::EventHandler<E>>>, func: F)
                where E: $crate::scene::Event,
                      F: Fn(&E
                          $(
                          , lock_data!(@ generate parameter $mutability $data_type)
                          )*
                      ),
                      F: Send + Sync + 'static {
                let copy = std::sync::Arc::clone(data);
                let l = $crate::scene::Listener {
                    id: handler.write().unwrap().new_id(),
                    func: Box::new(move |event| {
                        let self_var = copy.read().unwrap();
                        lock_data!{ @ generate locks self_var, func, event, $($name, $mutability, $data_type),+ }
                    })
                };
                handler.write().unwrap().insert(l);
            }
        }
    };

    (@ generate parameter read $data_type:ty) => { &$data_type };
    (@ generate parameter write $data_type:ty) => { &mut $data_type };
    (@ generate parameter $mutability:ident $data_type:ty) => {
        compile_error!("This macro requires the mutability of a variable to be 'read' or 'write'")
    };

    (@ generate locks $s:ident, $f:ident, $e:ident,
        $name0:ident, $mutability0:ident, $data_type0: ty | $($mutability1:ident)* | $($guard1:ident)*
        ) => {

        if let Some(arc) = std::sync::Weak::upgrade(&$s.$name0) {
            match lock_data!( @ generate try mutability $mutability0 arc ) {
                Ok(lock_data!( @ generate mutability $mutability0 guard )) => {
                    $f($e, $(lock_data!( @ generate mutability $mutability1 &$guard1)),*, lock_data!( @ generate mutability $mutability0 &guard));
                    true
                },
                _ => false
            }
        } else {
            false
        }
    };

    // A single-argument version
    (@ generate locks $s:ident, $f:ident, $e:ident,
        $name0:ident, $mutability0:ident, $data_type0: ty
        ) => {

        if let Some(arc) = std::sync::Weak::upgrade(&$s.$name0) {
            match lock_data!( @ generate try mutability $mutability0 arc ) {
                Ok(lock_data!( @ generate mutability $mutability0 guard )) => {
                    $f($e, lock_data!( @ generate mutability $mutability0 &guard));
                    true
                },
                _ => false
            }
        } else {
            false
        }
    };

    // We need to capture the `self` variable due to Rust's variable hygiene.
    // It's in the variable `s` here.
    // We also need to capture the `func` that must be called, and the `event`.
    // We'll also need to pass a reference to the `guard` we made, for the same reason.
    (@ generate locks $s:ident, $f:ident, $e:ident,
        $name0:ident, $mutability0:ident, $data_type0: ty,
        $($tail:tt)* | $($mutability1:ident),* | $($guard1:ident),*
        ) => {

        if let Some(arc) = std::sync::Weak::upgrade(&$s.$name0) {
            match lock_data!( @ generate try mutability $mutability0 arc ) {
                Ok(lock_data!( @ generate mutability $mutability0 guard)) => {
                    lock_data!{ @ generate locks $s, $f, $e, $($tail)* | $($mutability1),*, $mutability0 | $($guard1),*, guard }
                },
                _ => false
            }
        } else {
            false
        }
    };

    // Another copy of the above function that doesn't have the guard variables at the end.
    (@ generate locks $s:ident, $f:ident, $e:ident,
        $name0:ident, $mutability0:ident, $data_type0: ty,
        $($tail:tt)*
        ) => {

        if let Some(arc) = std::sync::Weak::upgrade(&$s.$name0) {
            match lock_data!( @ generate try mutability $mutability0 arc ) {
                Ok(lock_data!( @ generate mutability $mutability0 guard)) => {
                    lock_data!{ @ generate locks $s, $f, $e, $($tail)* | $mutability0 | guard }
                },
                _ => false
            }
        } else {
            false
        }
    };

    (@ generate mutability read $thing:ident) => { $thing };
    (@ generate mutability write $thing:ident) => { mut $thing };
    (@ generate mutability read &$thing:ident) => { &$thing };
    (@ generate mutability write &$thing:ident) => { &mut $thing };
    (@ generate try mutability read $rwlock:ident) => { $rwlock.try_read() };
    (@ generate try mutability write $rwlock:ident) => { $rwlock.try_write() };
}

pub struct EventHandler<E: Event> {
    next_id: ListenerID,
    set: HashMap<ListenerID, Listener<E>>
}

impl<E> EventHandler<E>
    where E: Event {
    pub fn new() -> EventHandler<E> {
        EventHandler {
            next_id: 0,
            set: HashMap::new()
        }
    }

    pub fn new_id(&mut self) -> ListenerID {
        let id = self.next_id;
        self.next_id += 1;
        id
    }

    pub fn insert(&mut self, l: Listener<E>) {
        self.set.insert(l.id, l);
    }

    /// Handle the given event by passing it through all provided listeners.
    pub fn handle(&mut self, e: E) {
        let to_remove: Vec<ListenerID> = self.set.par_iter().filter_map(|(k, v)| {
            let result = v.execute(&e);
            if result {
                None
            } else {
                Some(*k)
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

#[derive(Copy, Clone)]
pub struct TickEvent {
}

impl Event for TickEvent {
}

pub struct TickDebugBehaviour {
    node: Weak<RwLock<Node>>,
    tick_num: i32
}

lock_data! {
    TickDebugData

    behaviour: write TickDebugBehaviour
}

impl TickDebugBehaviour {
    pub fn new(scene: &mut Scene, node: Arc<RwLock<Node>>) {
        let behaviour = Arc::new(RwLock::new(TickDebugBehaviour {
            node: Arc::downgrade(&node),
            tick_num: 0
        }));

        let data = Arc::new(RwLock::new(TickDebugData {
            behaviour: Arc::downgrade(&behaviour),
        }));

        TickDebugData::listen(&data, &scene.tick_handler, |e, behaviour| {
            behaviour.tick_num += 1;
            println!("Tick {}", behaviour.tick_num);
        });

        node.write().unwrap().behaviours.push(behaviour);
    }
}

impl Behaviour for TickDebugBehaviour {
    fn node(&self) -> &Weak<RwLock<Node>> {
        &self.node
    }
}

