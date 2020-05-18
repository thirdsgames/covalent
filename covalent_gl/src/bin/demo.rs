use covalent;
use covalent_gl;

fn main() {
    let mut hints = covalent::DisplayHints::new();
    hints.title = String::from("Covalent | Demo");

    let backend = covalent_gl::BackendGL::new();

    let c = covalent::Covalent::new(hints, Box::new(backend));
    c.execute();
}