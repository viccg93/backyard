use crate::garden::vegetables::Asparagus;

pub mod garden; // le indica al compilador que deben incluir este codigo tambien

fn main() {
    let plant = Asparagus{};
    dbg!(plant);
}
