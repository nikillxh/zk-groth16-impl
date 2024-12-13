use std::process::Output;
use std::ops::{Add, Mul, Sub, Rem};
use crate::field::{FieldElement64, MODULUS64};
use crate::vector::{i128_to_u64_matrix, i128_to_u64_vec, i64_to_i128_matrix, i64_to_i128_vec, MyVec};

enum VecVariant<T> {
    SingleVec(Vec<T>),
    MatrixVec(Vec<Vec<T>>),
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