use crate::{field::{FieldElement64, MODULUS64}, qap::falling_factorial};


pub fn trustedsetup(x: u64) -> [Vec<FieldElement64>; 3] {
    let seed = FieldElement64::random();
    let g1 = FieldElement64::new(generator().1);
    let g2 = FieldElement64::new(generator().1);
    let t_tau = falling_factorial(seed, x);
    let srs1 = srs_creator(x, seed, g1);
    let srs2 = srs_creator(x, seed, g2);
    let srs3 = srs_creator(x, seed, g1 * t_tau);
    println!("Trusted setup done!");
    [srs1, srs2, srs3]
}

pub fn srs_creator(len: u64, tau: FieldElement64, g: FieldElement64) -> Vec<FieldElement64> {
    let mut srs: Vec<FieldElement64> = vec![g];
    for _ in 0..len {
        srs.push(srs.last().unwrap().clone() * tau);
    }
    srs
}


pub fn mod_exp(base: u64, mut exp: u64, modulus: u64) -> u64 {
    let mut result = 1u64;
    let mut base = base % modulus; 
    
    while exp > 0 {
        if exp % 2 == 1 {
            result = ((result as u128 * base as u128) % modulus as u128) as u64;
        }

        base = ((base as u128 * base as u128) % modulus as u128) as u64;
        exp /= 2;
    }
    result
}

pub fn mod_sqrt(a: u64, p: u64) -> Option<u64> {
    if a == 0 {
        return Some(0);
    }

    if mod_exp(a, (p - 1) / 2, p) != 1 {
        return None;
    }

    // Tonelli-Shanks algorithm
    let mut q = p - 1;
    let mut s = 0;
    while q % 2 == 0 {
        q /= 2;
        s += 1;
    }
    let mut z = 2;
    while mod_exp(z, (p - 1) / 2, p) == 1 {
        z += 1;
    }

    let mut m = s;
    let mut c = mod_exp(z, q, p);
    let mut t = mod_exp(a, q, p);
    let mut r = mod_exp(a, (q + 1) / 2, p);

    while t != 0 && t != 1 {
        let mut i = 0;
        let mut temp = t;
        while temp != 1 {
            temp = mod_exp(temp, 2, p);
            i += 1;
        }

        let b = mod_exp(c, 2u64.pow((m - i - 1) as u32), p);
        m = i;
        c = mod_exp(b, 2, p);

        t = mod_exp(t, 2u64.pow(i as u32), p);
        
        r = ((r as u128 * b as u128) % p as u128) as u64;
    }

    if t == 0 {
        None
    } else {
        Some(r)
    }
}

// y^2 = x^3 + 3 (mod p)
pub fn generator() -> (u64, u64) {
    for x in 1..MODULUS64 {
        let rhs = (mod_exp(x, 3, MODULUS64) + 3) % MODULUS64;

        if let Some(y) = mod_sqrt(rhs, MODULUS64) {
            let y_squared = ((y as u128 * y as u128) % MODULUS64 as u128) as u64;

            if y_squared == rhs {
                return (x, y);
            }
        }
        // println!("x = {}", x);
    }

    panic!("No generator found");
}
