use crate::render_backend::RenderBackend;

/// The Covalent structure contains all the information required to render a scene.
pub struct Covalent {
    hints: DisplayHints,
    rb: Box<dyn RenderBackend>
}

/// Hints to use when constructing the display window.
pub struct DisplayHints {
    /// The title to show on the display window, if in windowed mode on a backend that supports this.
    pub title: String,
    /// The default width of the window, when this can be defined.
    pub width: i32,
    /// The default height of the window, when this can be defined.
    pub height: i32,
}

impl DisplayHints {
    /// Creates a DisplayHints object with default parameters.
    pub fn new() -> DisplayHints {
        DisplayHints {
            title: String::from("Covalent"),
            width: 1024,
            height: 768,
        }
    }
}

impl Covalent {
    /// Construct a Covalent context from the given backend.
    /// Only create a single context during the lifetime of your application,
    /// and only create this context on the main thread!
    pub fn new(hints: DisplayHints, rb: Box<dyn RenderBackend>) -> Covalent {
        Covalent {
            hints: hints,
            rb: rb
        }
    }

    /// Executes the application defined by this Covalent context.
    pub fn execute(mut self) {
        self.rb.create_window(&self.hints);
        self.rb.main_loop();
    }
}