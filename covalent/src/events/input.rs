use crate::events::Event;
use crate::input::*;

/// An event from the keyboard has been received.
/// Adapted from the `winit` crate, version 0.22.2.
#[derive(Debug)]
pub struct KeyboardEvent {
    /// Identifies the physical key pressed.
    ///
    /// This should not change if the user adjusts the host's keyboard map. Use when the physical location of the
    /// key is more important than the key's host GUI semantics, such as for movement controls in a first-person
    /// game.
    pub scan_code: ScanCode,

    /// Describes the input state of a key, i.e. pressed or released.
    pub state: ElementState,

    /// Identifies the semantic meaning of the key.
    ///
    /// Use when the semantics of the key are more important than the physical location of the key, such as when
    /// implementing appropriate behavior for "page up."
    pub virtual_keycode: Option<VirtualKeyCode>,
}
impl Event for KeyboardEvent {}

/// The mouse has been moved by a certain amount of pixels in the X and Y directions.
#[derive(Debug)]
pub struct MouseDeltaEvent {
    /// The difference in pixels between the location of the mouse last frame and this frame.
    pub delta: cgmath::Vector2<f64>
}
impl Event for MouseDeltaEvent {}

/// The window that covalent is running in has changed size.
/// This event is automatically emitted once at the start of running a scene.
#[derive(Debug)]
pub struct WindowResizeEvent {
    /// The new size of the window.
    pub new_size: cgmath::Vector2<u32>
}
impl Event for WindowResizeEvent {}

