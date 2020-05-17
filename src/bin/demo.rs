use covalent;

fn main() {
    let mut hints = covalent::DisplayHints::new();
    hints.title = String::from("Covalent | Demo");

    let backend = covalent::BackendGL::new();

    let c = covalent::Covalent::new(hints, Box::new(backend));
    c.execute();
}