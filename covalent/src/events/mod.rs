//! This module contains commonly-used events in covalent.
//! You can create custom events by implementing the `covalent::scene::Event` trait, and then
//! creating an event handler for it with `covalent::scene::EventHandler::<YourEventType>::new()`.

use std::sync::{Arc, RwLock};

mod common;
pub use common::*;

mod event;
pub use event::*;

mod lock_data;

use crate::input::KeyboardEvent;

/// A manager for event handlers in a scene. This contains all the common event handlers.
#[derive(Default)]
pub struct EventHandlers {
    pub tick: Arc<RwLock<EventHandler<TickEvent>>>,
    pub key: Arc<RwLock<EventHandler<KeyboardEvent>>>,
}
