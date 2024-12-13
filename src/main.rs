use r1cs::{Constraint, Element, R1CS};

mod r1cs;
mod qap;
mod vector;
mod field;

fn main() {
    let c1 = Constraint::new(Element::new(0, 1), Element::new(1, 1), Element::new(2, 1));
    let c2 = Constraint::new(Element::new(2, 2), Element::new(3, 1), Element::new(4, 3));

    let r1cs = R1CS::new(vec![c1, c2], vec![1, 2, 3, 4, 5]);

    r1cs.verify(vec![1, 2, 3, 4, 5]);
}
