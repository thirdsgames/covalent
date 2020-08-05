use std::collections::HashMap;
use rayon::prelude::*;
use rayon::iter::Either;

/// `lock_data` structs can listen for events. When the event is fired, the `lock_data` will try
/// to get references to the required variables defined in the macro. This has the possibility to
/// fail if one or more of the required variables could not be locked.
pub enum ListenError {
    /// The required variables are stored as `Weak` references. If the `Weak` could not be upgraded
    /// to an `Arc`, the variable must have been deleted. In this case, `RequirementDeleted` will be
    /// returned, and in response, the listener will be deleted.
    RequirementDeleted,
    /// One of the variables could not be locked at this time. This could be, for example, because
    /// another `lock_data` currently has the lock. The listener will be retried later, hopefully
    /// after the lock is released by the other thread.
    LockUnavailable
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
    pub func: Box<dyn Fn(&E) -> Result<(), ListenError> + Send + Sync>
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
    fn execute(&self, e: &E) -> Result<(), ListenError> {
        (*self.func)(e)
    }
}

/// A generic event. See `EventHandler` for more information.
pub trait Event: Send + Sync {}

pub struct EventHandler<E: Event> {
    next_id: ListenerID,
    set: HashMap<ListenerID, Listener<E>>
}

impl<E> Default for EventHandler<E>
    where E: Event {
    fn default() -> Self {
        Self {
            next_id: 0,
            set: HashMap::new()
        }
    }
}

impl<E> EventHandler<E>
    where E: Event {
    pub fn new_id(&mut self) -> ListenerID {
        let id = self.next_id;
        self.next_id += 1;
        id
    }

    pub fn insert(&mut self, l: Listener<E>) {
        self.set.insert(l.id, l);
    }

    /// Returns a list of listeners to permanently remove from the event handler.
    fn handle_iter<'a>(e: E, to_try: impl rayon::iter::ParallelIterator<Item=(&'a ListenerID, &'a Listener<E>)>) -> Vec<i64> where E: 'a {
        let (to_retry, mut to_remove) : (Vec<(&'a ListenerID, &'a Listener<E>)>, Vec<ListenerID>) = to_try.filter_map(|(k, v)| {
            match v.execute(&e) {
                Ok(_) => {
                    None
                },
                Err(e) => {
                    // If there was some kind of error, we need to store the error with the listener.
                    Some((k, v, e))
                },
            }
        }).partition_map(|(k, v, e)| {
            // The Either::Left is for listeners that can retry. The Either::Right is for listeners that must now be deleted.
            match e {
                ListenError::LockUnavailable => {
                    Either::Left((k, v))
                },
                ListenError::RequirementDeleted => {
                    Either::Right(*k)
                }
            }
        });

        if !to_retry.is_empty() {
            to_remove.append(&mut EventHandler::handle_iter(e, to_retry.into_par_iter()));
        }
        to_remove
    }

    /// Handle the given event by passing it through all provided listeners.
    pub fn handle(&mut self, e: E) {
        for k in EventHandler::handle_iter(e, self.set.par_iter()) {
            self.set.remove(&k);
        }
    }
}