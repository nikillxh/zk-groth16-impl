use std::process::Output;
use std::ops::{Add, Mul, Sub, Rem};
use crate::field::{FieldElement64, MODULUS64};
use crate::vector::{i128_to_u64_matrix, i128_to_u64_vec, i64_to_i128_matrix, i64_to_i128_vec, MyVec};
use crate::r1cs::{witness_multiply, R1CS};

pub struct Polynomial {
    roots: FieldElement64,
    constant: FieldElement64,
}

pub struct QAP {
    u: Vec<FieldElement64>,
    v: Vec<FieldElement64>,
    w: Vec<FieldElement64>,
    t: u64,
}

impl QAP {
    pub fn from_r1cs(r1cs: R1CS, witness: Vec<i64>) -> Self {
        let mut l_vec = witness_multiply(r1cs.left(), witness.clone());
        let mut r_vec = witness_multiply(r1cs.right(), witness.clone());
        let mut o_vec = witness_multiply(r1cs.output(), witness.clone());
        
        let left = FieldElement64::convert1D(&mut field_compatible_vector(&mut l_vec));
        let right = FieldElement64::convert1D(&mut field_compatible_vector(&mut r_vec));
        let output = FieldElement64::convert1D(&mut field_compatible_vector(&mut o_vec));
        let t_val = output.len() as u64;

        Self {
            u: left,
            v: right,
            w: output,
            t: t_val
        }
    }
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