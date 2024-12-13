use std::ops::{Add, Mul, Sub, Rem};

pub struct MyVec<T>(pub Vec<T>);

impl<T: Add<Output = T> + Clone> Add for MyVec<T> {
    type Output = Vec<T>;

    fn add(self, vector: MyVec<T>) -> Self::Output {
        self.0.into_iter()
            .zip(vector.0.into_iter())
            .map(|(a, b)| a + b)
            .collect()
    }
}

// Scalar Vector Operations

impl<T: Add<Output = T> + Clone> Add<T> for MyVec<T> {
    type Output = Vec<T>;

    fn add(self, value: T) -> Self::Output {
        self.0.into_iter().map(|x| x + value.clone()).collect()
    }
}

impl<T: Sub<Output = T> + Clone> Sub<T> for MyVec<T> {
    type Output = Vec<T>;

    fn sub(self, value: T) -> Self::Output {
        self.0.into_iter().map(|x| x - value.clone()).collect()
    }
}

impl<T: Mul<Output = T> + Clone> Mul<T> for MyVec<T> {
    type Output = Vec<T>;

    fn mul(self, value: T) -> Self::Output {
        self.0.into_iter().map(|x| x * value.clone()).collect()
    }
}

impl<T: Rem<Output = T> + Clone> Rem<T> for MyVec<T> {
    type Output = Vec<T>;

    fn rem(self, value: T) -> Self::Output {
        self.0.into_iter().map(|x| x % value.clone()).collect()
    }
}


impl<T: Add<Output = T> + Clone> Add<T> for MyVec<Vec<T>> {
    type Output = Vec<Vec<T>>;

    fn add(self, value: T) -> Self::Output {
        self.0.into_iter()
            .map(|vector| {
                vector
                    .into_iter()
                    .map(|x| x + value.clone())
                    .collect()
            })
            .collect()
    }
}

impl<T: Sub<Output = T> + Clone> Sub<T> for MyVec<Vec<T>> {
    type Output = Vec<Vec<T>>;

    fn sub(self, value: T) -> Self::Output {
        self.0.into_iter()
            .map(|vector| {
                vector
                    .into_iter()
                    .map(|x| x - value.clone())
                    .collect()
            })
            .collect()
    }
}

impl<T: Mul<Output = T> + Clone> Mul<T> for MyVec<Vec<T>> {
    type Output = Vec<Vec<T>>;

    fn mul(self, value: T) -> Self::Output {
        self.0.into_iter()
            .map(|vector| {
                vector
                    .into_iter()
                    .map(|x| x * value.clone())
                    .collect()
            })
            .collect()
    }
}

impl<T: Rem<Output = T> + Clone> Rem<T> for MyVec<Vec<T>> {
    type Output = Vec<Vec<T>>;

    fn rem(self, value: T) -> Self::Output {
        self.0.into_iter()
            .map(|vector| {
                vector
                    .into_iter()
                    .map(|x| x % value.clone())
                    .collect()
            })
            .collect()
    }
}

pub fn i64_to_i128_vec(vector: &mut Vec<i64>) -> Vec<i128> {
    vector.iter().map(|&x| x as i128).collect()
}

pub fn i128_to_u64_vec(vector: &mut Vec<i128>) -> Vec<u64> {
    vector.iter().map(|&x| x as u64).collect()
}

pub fn i64_to_i128_matrix(matrix: &mut Vec<Vec<i64>>) -> Vec<Vec<i128>> {
    matrix.iter().map(|row| row.iter().map(|&x| x as i128).collect()).collect()
}

pub fn i128_to_u64_matrix(matrix: &mut Vec<Vec<i128>>) -> Vec<Vec<u64>> {
    matrix.iter().map(|row| row.iter().map(|&x| x as u64).collect()).collect()
}
