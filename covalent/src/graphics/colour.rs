/// A colour structure that contains red, green, blue and alpha information.
/// These values must be between zero and one.
/// Whenever any value in this struct is updated, it computes a packed representation as a u32,
/// so retrieving this value is of zero cost.
#[derive(Copy, Clone)]
pub struct Colour {
    r: f32,
    g: f32,
    b: f32,
    a: f32,
    packed: u32
}

impl Colour {
    /// Creates a colour from the red, green and blue components.
    /// The colour is implicitly considered opaque, as in, it has an alpha value of one.
    pub fn new(r: f32, g: f32, b: f32) -> Colour {
        let mut c = Colour {
            r, g, b,
            a: 1.0,
            packed: 0
        };
        c.compute_packed();
        c
    }

    /// Computes the packed representation of the colour,
    /// storing it in self.
    /// This is automatically called by all Colour functions,
    /// so you should never need to call it.
    /// This results in undefined behaviour if any colour component is outside of the range [0, 1].
    fn compute_packed(&mut self) {
        let r = (self.r * 255.0) as u32;
        let g = (self.g * 255.0) as u32;
        let b = (self.b * 255.0) as u32;
        let a = (self.a * 255.0) as u32;
        self.packed = r << 24 | g << 16 | b << 8 | a;
    }

    /// Extracts the red component of the colour.
    pub fn r(&self) -> f32 {
        self.r
    }
    /// Extracts the green component of the colour.
    pub fn g(&self) -> f32 {
        self.g
    }
    /// Extracts the blue component of the colour.
    pub fn b(&self) -> f32 {
        self.b
    }
    /// Extracts the alpha (transparency) component of the colour.
    pub fn a(&self) -> f32 {
        self.a
    }

    /// Extracts the packed representation of the colour. The packed representation is defined as follows:
    /// ```plain
    /// Red      | Green    | Blue     | Alpha
    /// ---------+----------+----------+---------
    /// rrrrrrrr | gggggggg | bbbbbbbb | aaaaaaaa
    /// ```
    /// The highest eight bits contain the information about the red component of this colour, represented
    /// as a u8 (from 0 to 255). Likewise, the next eight bits contain the information about the green component, and so on.
    /// 
    /// # Examples
    /// To extract information from this, simply use a binary AND to mask the particular colour. For example,
    /// ```
    /// use covalent::graphics::Colour;
    /// let c = Colour::new(1.0, 0.5, 0.1);
    /// assert_eq!(255, (c.packed() & 0xFF000000) >> 24);   // Red
    /// assert_eq!(127, (c.packed() & 0x00FF0000) >> 16);   // Green
    /// assert_eq!(25, (c.packed() & 0x0000FF00) >> 8);     // Blue
    /// assert_eq!(255, (c.packed() & 0x000000FF) >> 0);    // Alpha
    /// ```
    pub fn packed(&self) -> u32 {
        self.packed
    }

    /// Sets the red component of the colour.
    pub fn set_r(&mut self, r: f32) {
        self.r = r;
        self.compute_packed();
    }
    /// Sets the green component of the colour.
    pub fn set_g(&mut self, g: f32) {
        self.g = g;
        self.compute_packed();
    }
    /// Sets the blue component of the colour.
    pub fn set_b(&mut self, b: f32) {
        self.b = b;
        self.compute_packed();
    }
    /// Sets the alpha (transparency) component of the colour.
    pub fn set_a(&mut self, a: f32) {
        self.a = a;
        self.compute_packed();
    }
}