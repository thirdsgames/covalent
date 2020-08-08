use crate::events::Event;

/// An event automatically fired every frame.
pub struct TickEvent {
    /// The time that has passed between this frame and the last frame, in seconds.
    pub(crate) delta: f64,
}
impl Event for TickEvent {}
