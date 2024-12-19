use crate::field::{FieldElement64, MODULUS64};
use crate::vector::{i128_to_u64_matrix, i128_to_u64_vec, i64_to_i128_matrix, i64_to_i128_vec, MyVec};
use crate::r1cs::{witness_multiply, R1CS};

pub struct QAP {
    u: Vec<FieldElement64>,
    v: Vec<FieldElement64>,
    w: Vec<FieldElement64>,
    t: u64,
    h: Vec<FieldElement64>,
}

impl QAP {
    pub fn from_r1cs(r1cs: R1CS, witness: Vec<i64>) -> Self {
        let mut l_vec = witness_multiply(r1cs.left(), witness.clone());
        let mut r_vec = witness_multiply(r1cs.right(), witness.clone());
        let mut o_vec = witness_multiply(r1cs.output(), witness.clone());
        let t_val = o_vec.len();

        let t_poly = generate_t(t_val);
        let uv = multiply_polynomials(&l_vec, &r_vec);
        let uvw = subtract_polynomials(&uv, &o_vec);
        let (mut h, _) = divide_polynomials(&uvw, &t_poly);
                
        let left = FieldElement64::convert1d(&mut field_compatible_vector(&mut l_vec));
        let right = FieldElement64::convert1d(&mut field_compatible_vector(&mut r_vec));
        let output = FieldElement64::convert1d(&mut field_compatible_vector(&mut o_vec));
        let h_vec = FieldElement64::convert1d(&mut field_compatible_vector(&mut h));

        println!("QAP from R1CS done!");

        Self {
            u: left,
            v: right,
            w: output,
            t: t_val as u64,
            h: h_vec,
        }

    }

    // Prover
    pub fn evaluate(&self, srs_values: [Vec<FieldElement64>; 3]) -> [FieldElement64; 3] {
        let a1 = inner_product(&self.u, &srs_values[0]).unwrap();
        let b2 = inner_product(&self.v, &srs_values[1]).unwrap();
        let c1 =  inner_product(&self.w, &srs_values[0]).unwrap() + inner_product(&self.h, &srs_values[2]).unwrap();

        [a1, b2, c1]
    }

    // Verifier
    pub fn verify(prover: [FieldElement64; 3], srs_values: [Vec<FieldElement64>; 3]) -> () {
        assert_eq!(prover[0] * prover[1], prover[2] * srs_values[1][0]);
    }

    pub fn t_val(&self) -> u64 {
        self.t
    }
}

// Polynomial operations

fn generate_t(n: usize) -> Vec<i64> {
    let mut t = vec![1];
    for i in 1..=n {
        t = multiply_polynomials(&t, &vec![-(i as i64), 1]);
    }
    t
}

fn multiply_polynomials(a: &Vec<i64>, b: &Vec<i64>) -> Vec<i64> {
    let mut result = vec![0; a.len() + b.len() - 1];
    for (i, &coeff_a) in a.iter().enumerate() {
        for (j, &coeff_b) in b.iter().enumerate() {
            result[i + j] += coeff_a * coeff_b;
        }
    }
    result
}

fn subtract_polynomials(a: &Vec<i64>, b: &Vec<i64>) -> Vec<i64> {
    let max_len = a.len().max(b.len());
    let mut result = vec![0; max_len];
    for i in 0..max_len {
        let coeff_a = *a.get(i).unwrap_or(&0);
        let coeff_b = *b.get(i).unwrap_or(&0);
        result[i] = coeff_a - coeff_b;
    }
    result
}

fn divide_polynomials(dividend: &Vec<i64>, divisor: &Vec<i64>) -> (Vec<i64>, Vec<i64>) {
    let mut quotient = vec![0; dividend.len().saturating_sub(divisor.len()) + 1];
    let mut remainder = dividend.clone();

    for i in (0..=dividend.len().saturating_sub(divisor.len())).rev() {
        let lead_coeff = remainder[i + divisor.len() - 1] / divisor[divisor.len() - 1];
        quotient[i] = lead_coeff;

        for j in 0..divisor.len() {
            remainder[i + j] -= lead_coeff * divisor[j];
        }
    }

    (quotient, remainder)
}

pub fn falling_factorial(x: FieldElement64, n: u64) -> FieldElement64 {
    let mut value = FieldElement64::new(1);
    for i in 1..n+1 {
        value = value * (x - FieldElement64::new(i));
    }
    value
}

pub fn field_compatible_vector(input: &mut Vec<i64>) -> Vec<u64> {
    let mut vectorop = i64_to_i128_vec(input);
    vectorop = MyVec(vectorop) % (MODULUS64 as i128);
    vectorop = MyVec(vectorop) + (MODULUS64 as i128);
    i128_to_u64_vec(&mut vectorop)
}

pub fn field_compatible_matrix(input: &mut Vec<Vec<i64>>) -> Vec<Vec<u64>> {
    let mut matrixop = i64_to_i128_matrix(input);
    matrixop = MyVec(matrixop) % (MODULUS64 as i128);
    matrixop = MyVec(matrixop) + (MODULUS64 as i128);
    i128_to_u64_matrix(&mut matrixop)
}

pub fn inner_product(vec1: &Vec<FieldElement64>, vec2: &Vec<FieldElement64>) -> Option<FieldElement64> {
    assert_eq!(vec1.len(), vec2.len());
    let mut result = FieldElement64::new(0);

    for (a, b) in vec1.iter().zip(vec2.iter()) {
        result = result + (*a * *b);
    }
    Some(result)
}

pub fn lagrange_interpolation(x: FieldElement64, x_points: &Vec<FieldElement64>, y_points: &Vec<FieldElement64>) -> Option<FieldElement64> {
    if x_points.is_empty() {
        return None
    }

    let mut result = FieldElement64::new(0);
    for i in 0..x_points.len() {
        let mut term = y_points[i];
        for j in 0..x_points.len() {
            if i!=j {
                let num = x - x_points[j];
                let mut den = x_points[i] - x_points[j];
                den = FieldElement64::inverse(den);
                term = term * num * den;
            }
        }
        result = result + term;
    }

    Some(result)
}