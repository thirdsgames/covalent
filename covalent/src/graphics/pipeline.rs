use std::collections::BTreeMap;
use crate::graphics::RenderTarget;

/// The `Pipeline` is the way to tell covalent how to render your scene.
/// It contains a series of steps (`PipelinePhase`) which the graphics backend will execute sequentially.
/// 
/// These pipeline phases each have a `time` to execute, which is an `i32`. Phases with a small `time` are
/// executed before those with a large `time`. It is unsupported to insert multiple pipeline phases with the same `time`.
/// 
/// # Rules
/// Although programmable, pipelines must conform to certain rules.
/// - There must be at least one `Render` phase that targets the `Default` render target. This allows the user to see the result.
pub struct Pipeline {
    phases: BTreeMap<i32, (String, PipelinePhase)>
}

impl Pipeline {
    pub fn new() -> Pipeline {
        Pipeline {
            phases: BTreeMap::new()
        }
    }

    /// Register a phase in this pipeline.
    /// 
    /// # Panics
    /// If a phase with the given time already exists in the pipeline, it will panic.
    pub fn add_phase(&mut self, time: i32, name: String, phase: PipelinePhase) {
        if self.phases.contains_key(&time) {
            panic!("phase {} was already contained within this pipeline, conflicting phases were \"{}\"; \"{}\"", time, name, self.phases.get(&time).unwrap().0);
        }
        self.phases.insert(time, (name, phase));
    }

    /// Asserts that the pipeline conforms to the rules set out in the `Pipeline`'s documentation.
    fn check_phases(&self) {
        let mut contains_render_to_default = false;
        for (_, phase) in self.phases.values() {
            match phase {
                PipelinePhase::Render(_, target) => {
                    #[allow(irrefutable_let_patterns)]  // When we use framebuffers / other render targets, this will be needed, and probably turned into a match stmt.
                    if let RenderTarget::Default = target {
                        contains_render_to_default = true;
                    }
                }
            }
        }

        if !contains_render_to_default {
            panic!("pipeline was invalid! no phase was detected that renders to the user's display; this is disallowed behaviour!");
        }
    }
}

impl Pipeline {
    pub fn iter<'a>(&'a self) -> std::collections::btree_map::Values<'a, i32, (String, PipelinePhase)> {
        self.check_phases();
        self.phases.values()
    }
}

/// This struct defines how we should render a given scene.
pub struct RenderSettings {

}

/// A single render phase.
/// To render to the screen, construct a pipeline of these phases, which will be executed sequentially every frame by
/// the graphics backend.
pub enum PipelinePhase {
    /// Render a scene using specific settings, outputting the result to the given render target.
    Render(RenderSettings, RenderTarget)
}