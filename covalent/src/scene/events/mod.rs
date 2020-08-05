//! This module contains commonly-used events in covalent.
//! You can create custom events by implementing the `covalent::scene::Event` trait, and then
//! creating an event handler for it with `covalent::scene::EventHandler::<YourEventType>::new()`.

mod common;
pub use common::*;
