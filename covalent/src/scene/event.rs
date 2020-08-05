use std::collections::HashMap;
use rayon::prelude::*;

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
    /// In the process of performing this action, we will unbox the node and the lock_data that are encapsulated in the listener.
    /// If either of the weak variables fail to upgrade to strong smart pointers (`Arc` variables), the node is considered deleted.
    /// In this case, the listener can never fire. The function then returns false without doing anything.
    /// Otherwise, the function will fire, and true will be returned.
    fn execute(&self, e: &E) -> bool {
        (*self.func)(e)
    }
}

/// A generic event. See `EventHandler` for more information.
pub trait Event: Send + Sync {}

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