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