use qap::QAP;
use r1cs::{Constraint, Element, R1CS};
use trustedsetup::trustedsetup;

mod r1cs;
mod qap;
mod vector;
mod field;
mod trustedsetup;

fn main() {
    let c1 = Constraint::new(Element::new(0, 1), Element::new(1, 1), Element::new(2, 1));
    let c2 = Constraint::new(Element::new(2, 2), Element::new(3, 1), Element::new(4, 3));

    let r1cs = R1CS::new(vec![c1, c2], vec![1, 2, 3, 4, 5]);
    let qap = QAP::from_r1cs(r1cs,  vec![1, 2, 3, 4, 5]);
    let srs_values = trustedsetup(qap.t_val());
    let prover = qap.evaluate(srs_values.clone());
    QAP::verify(prover, srs_values);
}
