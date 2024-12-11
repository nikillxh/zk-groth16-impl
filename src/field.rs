use std::ops::Add;

const MODULUS256: [u64; 4] = [
    0x2523648240000001,
    0xBA344D8000000008,
    0x5A9320E033BA4E14,
    0x1B9FEFFFFFFFFAAAB,
];

pub struct FieldElement256 {
    value: [u64; 4],
}

const MODULUS64: u64 = 0xFFFFFFFFFFFFFFC5; 

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
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let value = rng.gen::<u64>() % MODULUS64;
        FieldElement64 { value }
    }

    pub fn value(&self) -> u64 {
        self.value
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
            let (sum, c1) = self.value[i].carrying_add(addend.value[i], carry);
            result[i] = sum;
            carry = c1;
        }

        FieldElement256::new(result)
    }

    pub fn subtract(self, subtrahend: FieldElement256) {
        let mut result = [0u64; 4];
        let mut borrow = 0;

        for i in 0..4 {
            let (diff, b1) = self.value[i].borrowing_sub(subtrahend.value[i], borrow);
            result[i] = diff;
            borrow = b1;
        }

        if borrow != 0 {
            let mut carry = 0;
            for i in 0..4 {
                let (sum, c1) = result[i].carrying_add(MODULUS256[i], carry);
                result[i] = sum;
                carry = c1;
            }
        }

        FieldElement256::new(result)
    }

    pub fn multiply(self, multiplier: FieldElement256) -> FieldElement256 {
        let mut result = [0u64; 4];

        for i in 0..4 {
            let mut carry = 0u64;
            for j in 0..4 {
                let (low, high) = self.value[i].overflowing_mul(other.value[j]);
                let (res_low, c1) = result[i + j].overflowing_add(low);
                let (res_high, c2) = result[i + j + 1].overflowing_add(high + carry + c1 as u64);
                result[i + j] = res_low;
                result[i + j + 1] = res_high;
                carry = c2;
            }
        }

        FieldElement256::new([result[0], result[1], result[2], result[3]])
    }

    pub fn inverse(self) -> FieldElement256 {
        let mut t = [0u64; 4];
        let mut new_t = [1u64; 4];
        let mut r = MODULUS256;
        let mut new_r = self.value;

        while !is_zero(&new_r) {
            let q = divide(&r, &new_r); // Divide r by new_r
            t = subtract(&t, &multiply(&q, &new_t));
            r = subtract(&r, &multiply(&q, &new_r));
            std::mem::swap(&mut t, &mut new_t);
            std::mem::swap(&mut r, &mut new_r);
        }

        assert!(r == [1u64, 0, 0, 0], "Element is not invertible");
        FieldElement256::new(t)
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

impl Add for FieldElement64 {
    type Output = Self;

    fn add(self, addend: Self) -> Self::Output {
        self.add(addend)
    }
}

