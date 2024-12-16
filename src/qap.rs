use std::process::Output;
use std::ops::{Add, Mul, Sub, Rem};
use crate::field::{FieldElement64, MODULUS64};
use crate::vector::{i128_to_u64_matrix, i128_to_u64_vec, i64_to_i128_matrix, i64_to_i128_vec, MyVec};

pub struct Polynomial {
    roots: FieldElement64,
    constant: FieldElement64,
}

pub enum VecVariant<T> {
    SingleVec(Vec<T>),
    MatrixVec(Vec<Vec<T>>),
}

pub struct QAP {
    A: Vec<FieldElement64>,
    B: Vec<FieldElement64>,
    C: Vec<FieldElement64>,
}

pub fn field_compatible<I64> (input: VecVariant<i64>) -> VecVariant<u64>{
    match input {
        VecVariant::SingleVec(vector) => {
            let mut vectorop = i64_to_i128_vec(&mut vector.clone());
            vectorop = MyVec(vectorop) % (MODULUS64 as i128);
            vectorop = MyVec(vectorop) + (MODULUS64 as i128);
            let vectorres = i128_to_u64_vec(&mut vectorop);
            VecVariant::SingleVec(vectorres)
        }
        VecVariant::MatrixVec(matrix) => {
            let mut matrixop = i64_to_i128_matrix(&mut matrix.clone());
            matrixop = MyVec(matrixop) % (MODULUS64 as i128);
            matrixop = MyVec(matrixop) + (MODULUS64 as i128);
            let matrixres = i128_to_u64_matrix(&mut matrixop);
            VecVariant::MatrixVec(matrixres)
        }
    }
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
                let mut num = x - x_points[j];
                let mut den = x_points[i] - x_points[j];
                den = FieldElement64::inverse(den);
                term = term * num * den;
            }
        }
        result = result + term;
    }

    Some(result)
}