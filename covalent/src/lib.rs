use std::time;
use std::cell::RefCell;

mod display_hints;
pub use display_hints::DisplayHints;

pub mod graphics;
pub mod scene;

pub use cgmath;
pub use cgmath::{vec1, vec2, vec3, vec4};
/// Convenience constructor for a one-dimensional point.
pub fn pt1<S>(x: S) -> cgmath::Point1<S> {
    cgmath::Point1::new(x)
}
/// Convenience constructor for a two-dimensional point.
pub fn pt2<S>(x: S, y: S) -> cgmath::Point2<S> {
    cgmath::Point2::new(x, y)
}
/// Convenience constructor for a three-dimensional point.
pub fn pt3<S>(x: S, y: S, z: S) -> cgmath::Point3<S> {
    cgmath::Point3::new(x, y, z)
}

/// A stopwatch (in covalent) is an object that counts the time between events.
/// An interpolated stopwatch counts the time between successive events, and calculates the average
/// time between those events, by storing the times of the last `n` events, where `n` is some arbitrary
/// constant specified in the stopwatch constructor.
pub struct InterpolatedStopwatch {
    times: Vec<time::Instant>,
    offset: usize
}

impl InterpolatedStopwatch {
    pub fn new(interpolation_amount: usize) -> InterpolatedStopwatch {
        let mut vec = Vec::with_capacity(interpolation_amount);
        for _ in 0..interpolation_amount {
            vec.push(time::Instant::now());
        }
        InterpolatedStopwatch {
            times: vec,
            offset: 0
        }
    }

    /// Call this function every time the given event happens.
    /// You will be able to retrieve the average time between calls to `tick`
    /// using the `average_time` function.
    pub fn tick(&mut self) {
        self.times[self.offset] = time::Instant::now();
        self.offset = (self.offset + 1) % self.times.len();
    }

    pub fn average_time(&self) -> time::Duration {
        let prev_offset = match self.offset {
            0 => self.times.len() - 1,
            _ => self.offset - 1
        };
        self.times[prev_offset].duration_since(self.times[self.offset]).div_f64(self.times.len() as f64)
    }
}

/// A context that encapsulates the behaviour of an application run with covalent.
/// This contains all the functions that the graphics backend will execute when the given event occurs.
/// 
/// The render thread runs concurrently with the update/behaviour threads. A single frame is calculated
/// by the update threads while a single frame is rendered by the render thread. This allows code to ensure
/// that certain outside things (e.g. the position of the mouse) will not change during calculation of a
/// frame. The graphics backend is required to call certain functions at intervals in the event loop
/// to tell covalent what it is allowed to do. This is the execution order:
/// 
/// ```notrust
/// Render thread       | Update threads
/// --------------------+--------------------
/// Event handling      |
/// Call begin_frame  ----> Run pre-frame actions,
///                     |   then process frame asynchronously
/// Render frame        | (Still processing frame)
/// Call end_frame    ----> Wait until processing frame is done,
///                     |   then run post-frame actions
/// ```
/// 
/// Pre/post-frame actions are therefore run while the render thread is idle. This means that they should only
/// be used sparingly, where it is absolutely necessary to synchronise certain actions with respect to their
/// rendering. For example, moving large amounts of nodes as a contiguous unit may require a post-frame action
/// to make sure that all nodes are actually moved the same amount per frame.
pub struct Context {
    frame_stopwatch: RefCell<InterpolatedStopwatch>,
    graphics_pipeline: graphics::Pipeline,
    scene: scene::Scene
}

impl Context {
    fn new(pipeline: graphics::Pipeline, scene: scene::Scene) -> Context {
        Context {
            frame_stopwatch: RefCell::from(InterpolatedStopwatch::new(512)),
            graphics_pipeline: pipeline,
            scene
        }
    }

    /// Should be called by the graphics backend as soon as event handling has been completed.
    /// This signals to covalent that it can start to process a frame.
    pub fn begin_frame(&self) {
        // Execute pre-frame actions.

        // Asynchronously process frame.
        self.scene.tick_handler().write().unwrap().handle(scene::TickEvent {});
    }
    
    /// Should be called by the graphics backend as soon as rendering the frame is complete.
    pub fn end_frame(&self) {
        // Execute post-frame actions.
    }

    /// Should be called by the graphics backend once every frame to retrieve the current graphics pipeline.
    pub fn render_phases<'a>(&'a self) -> (&scene::Scene, std::collections::btree_map::Values<'a, i32, (String, graphics::PipelinePhase)>) {
        self.frame_stopwatch.borrow_mut().tick();
        println!("{:.1} FPS", 1.0 / self.frame_stopwatch.borrow().average_time().as_secs_f64());
        (&self.scene, self.graphics_pipeline.iter())
    }
}

/// Construct a covalent context from the given backend, then executes the application defined by this Covalent context.
/// Only create a single context during the lifetime of your application,
/// and only create this context on the main thread!
/// 
/// You should never need to interact with the context manually - it is all handled by the active graphics backend.
pub fn execute(pipeline: graphics::Pipeline, gback: impl graphics::Backend) {
    let scene = scene::Scene::demo_squares(&gback);
    gback.main_loop(Context::new(pipeline, scene));
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
