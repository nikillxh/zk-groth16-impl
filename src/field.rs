use std::ops::{Add, Mul, Sub};
use rand::{RngCore, Rng};

const MODULUS256: [u64; 4] = [
    0xffffffffffffffff, // 0xFFFFFFFFFFFFFFFF
    0xffffffffffffffff, // 0xFFFFFFFFFFFFFFFF
    0xffffffffffffffff, // 0xFFFFFFFFFFFFFFFF
    0xffffffffffffffff, // 0xFFFFFFFFFFFFFFFF
];

pub struct FieldElement256 {
    value: [u64; 4],
}

pub const MODULUS64: u64 = 0xFFFFFFFFFFFFFFC5;

#[derive(Clone, Debug, PartialEq, Eq, Copy)]
pub struct FieldElement64 {
    value: u64,
}

impl FieldElement64 {
    pub fn new(mut value: u64) -> Self {
        value %= MODULUS64;
        FieldElement64 { value }
    }

    pub fn add(self, addend: FieldElement64) -> FieldElement64 {
        let sum = self.value.wrapping_add(addend.value);
        FieldElement64::new(sum % MODULUS64)
    }

    pub fn subtract(self, subtrahend: FieldElement64) -> FieldElement64 {
        let mut result = self.value.wrapping_sub(subtrahend.value);
        if result >= MODULUS64 {
            result = result.wrapping_add(MODULUS64);
        }
        FieldElement64 { value: result }
    }

    pub fn multiply(self, multiplier: FieldElement64) -> FieldElement64 {
        let product = (self.value as u128).wrapping_mul(multiplier.value as u128);
        FieldElement64::new((product % MODULUS64 as u128) as u64)
    }

    pub fn inverse(self) -> FieldElement64 {
        let mut t = 0u64;
        let mut new_t = 1u64;
        let mut r = MODULUS64;
        let mut new_r = self.value;

        while new_r != 0 {
            let quotient = r / new_r;

            let temp_t = t;
            t = new_t;
            new_t = temp_t.wrapping_sub(quotient.wrapping_mul(new_t));

            let temp_r = r;
            r = new_r;
            new_r = temp_r.wrapping_sub(quotient.wrapping_mul(new_r));
        }

        assert_eq!(r, 1, "Element is not invertible");

        FieldElement64::new(t as u64)
    }

    pub fn random() -> Self {
        let mut rng = rand::thread_rng();
        let value = rng.gen::<u64>() % MODULUS64;
        FieldElement64 { value }
    }

    pub fn value(&self) -> u64 {
        self.value
    }

    pub fn convert1D(matrix: &mut Vec<u64>) -> Vec<FieldElement64> {
        matrix.iter().map(|&x| FieldElement64::new(x)).collect()
    }

    pub fn convert2D(matrix: &mut Vec<Vec<u64>>) -> Vec<Vec<FieldElement64>> {
        matrix.iter()
        .map(|row| row.iter().map(|&x| FieldElement64::new(x)).collect())
        .collect()
    }
}

impl Add for FieldElement64 {
    type Output = Self;

    fn add(self, addend: Self) -> Self::Output {
        self.add(addend)
    }
}

impl Mul for FieldElement64 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        self.multiply(rhs)
    }
}

impl Sub for FieldElement64 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self.subtract(rhs)
    }
}

impl FieldElement256 {
    pub fn new(mut value: [u64; 4]) -> Self {
        FieldElement256::reduce(&mut value);
        FieldElement256 {value}
    }

    pub fn reduce(value: &mut [u64; 4]) {
        if greater_equal(value, &MODULUS256) {
            wrapper(value, &MODULUS256);
        }

        fn greater_equal(a: &[u64; 4], b: &[u64; 4]) -> bool {
            for (x,y) in a.iter().rev().zip(b.iter().rev()) {
                if x != y {
                    return x > y;
                }
            }
            true
        }

        fn wrapper(a: &mut [u64; 4], b: &[u64; 4]) {
            let mut borrow = 0u64;
            for i in 0..4 {
                let (res, overflow) = a[i].overflowing_sub(b[i] + borrow);
                a[i] = res;
                borrow = if overflow { 1 } else { 0 };
            }
        }
    }

    pub fn add(self, addend: FieldElement256) -> FieldElement256 {
        let mut result = [0u64; 4];
        let mut carry = 0;
    
        for i in 0..4 {
            let (sum, overflow1) = self.value[i].overflowing_add(addend.value[i]);
            let (sum_with_carry, overflow2) = sum.overflowing_add(carry);
    
            result[i] = sum_with_carry;
            carry = (overflow1 as u64) + (overflow2 as u64);
        }
    
        FieldElement256::new(result)
    }
    

    pub fn subtract(self, subtrahend: FieldElement256) -> FieldElement256 {
        let mut result = [0u64; 4];
        let mut borrow = 0;
    
        for i in 0..4 {
            let (diff, overflow) = self.value[i].overflowing_sub(subtrahend.value[i] + borrow);
            result[i] = diff;
            borrow = overflow as u64;
        }
    
        if borrow != 0 {
            let mut carry = 0;
            for i in 0..4 {
                let (sum, overflow) = result[i].overflowing_add(MODULUS256[i] + carry);
                result[i] = sum;
                carry = overflow as u64;
            }
        }
    
        FieldElement256::new(result)
    }
    
    pub fn multiply(self, multiplier: FieldElement256) -> FieldElement256 {
        let mut result = [0u64; 8]; 
        for i in 0..4 {
            let mut carry = 0u64;
            for j in 0..4 {
                let (low, high) = self.value[i].overflowing_mul(multiplier.value[j]);
                
                let (res_low, c1) = result[i + j].overflowing_add(low);
                result[i + j] = res_low;
                carry = c1 as u64;
    
                let (res_high, c2) = result[i + j + 1].overflowing_add(carry);
                result[i + j + 1] = res_high;
                carry = c2 as u64;
            }
        }
    
        let mut reduced = [0u64; 4];
        let mut carry = 0u64;
        for i in 0..4 {
            let (sum, overflow) = result[i].overflowing_add(carry);
            reduced[i] = sum;
            carry = if overflow { 1 } else { 0 };
        }
    
        FieldElement256::new(reduced)
    }

    // pub fn inverse(self) -> FieldElement256 {
    //     let mut t = [0u64; 4];
    //     let mut new_t = [1u64; 4];
    //     let mut r = MODULUS256;
    //     let mut new_r = self.value;
    
    //     while !is_zero(&new_r) {
    //         let q = divide(&r, &new_r); // Divide r by new_r
    //         t = subtract(&t, &multiply(&q, &new_t));
    //         r = subtract(&r, &multiply(&q, &new_r));
    //         std::mem::swap(&mut t, &mut new_t);
    //         std::mem::swap(&mut r, &mut new_r);
    //     }
    
    //     assert!(r == [1u64, 0, 0, 0], "Element is not invertible");
    //     FieldElement256::new(t)
    // }
    
    fn is_zero(value: &[u64; 4]) -> bool {
        value.iter().all(|&x| x == 0)
    }
    
    pub fn random() -> Self {
        let mut rng = rand::thread_rng();
        let mut value = [0u64; 4];
        for limb in value.iter_mut() {
            *limb = rng.next_u64();
        }
    
        FieldElement256::new(value)
    }
    
}