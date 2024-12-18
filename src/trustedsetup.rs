use crate::{field::{FieldElement64, MODULUS64}, qap::falling_factorial};


pub fn trustedsetup(x: u64, t: FieldElement64) -> [Vec<FieldElement64>; 3] {
    let seed = FieldElement64::random();
    let g1 = FieldElement64::new(generator().1);
    let g2 = FieldElement64::new(generator().1);
    let srs1 = srs_creator(x, seed, g1);
    let srs2 = srs_creator(x, seed, g2);
    let srs3 = srs_creator(x, seed, g1 * t);
    [srs1, srs2, srs3]
}

pub fn srs_creator(len: u64, tau: FieldElement64, g: FieldElement64) -> Vec<FieldElement64> {
    let mut srs: Vec<FieldElement64> = vec![g];
    for _ in 0..len {
        srs.push(srs.last().unwrap().clone() * tau);
    }
    srs
}

pub fn mod_exp(base: u64, exp: u64, modulus: u64) -> u64 {
    let mut result = 1;
    let mut base = base % modulus;
    let mut exp = exp;

    while exp > 0 {
        if exp % 2 == 1 {
            result = (result * base) % modulus;
        }
        base = (base * base) % modulus;
        exp /= 2;
    }
    result
}

pub fn generator() -> (u64, u64) {
    for x in 1..MODULUS64 {
        let rhs = (mod_exp(x, 3, MODULUS64) + 3) % MODULUS64;
        let y = mod_exp(rhs, (MODULUS64 + 1) / 4, MODULUS64);
        if (y * y) % MODULUS64 == rhs {
            return (x, y);
        }
    }
    panic!("No generator found");
}