pub struct Element {
    variable: usize,
    coeff: i64,
}

pub struct Constraint {
    x: Element,
    y: Element,
    z: Element,
}

pub struct R1CS {
    l: Vec<Vec<i64>>,
    r: Vec<Vec<i64>>,
    o: Vec<Vec<i64>>,
}

impl Element {
    pub fn new(var: usize, coeff: i64) -> Element{
        Self {
            variable: var,
            coeff: coeff,
        }
    }
}

impl Constraint {
    pub fn new(e1: Element, e2: Element, e3: Element) -> Constraint {
        Constraint {
            x: e1,
            y: e2,
            z: e3,
        }
    }
}

impl R1CS {
    pub fn blank(witness: Vec<i64>, eqn_count: Option<usize>) -> R1CS {
        let witnesses = witness.len();
        let eqn_count = eqn_count.unwrap_or(1);
        R1CS {
            l: vec![vec![0; witnesses]; eqn_count],
            r: vec![vec![0; witnesses]; eqn_count],
            o: vec![vec![0; witnesses]; eqn_count],
        }
    }

    pub fn new(constraints: Vec<Constraint>, witness: Vec<i64>) -> R1CS {
        let witnesses = witness.len();
        let eqn_count = constraints.len();
        let mut left: Vec<Vec<i64>> = vec![vec![0; witnesses]; eqn_count];
        let mut right: Vec<Vec<i64>> = vec![vec![0; witnesses]; eqn_count];
        let mut out: Vec<Vec<i64>> = vec![vec![0; witnesses]; eqn_count];
        let mut count: usize = 0;

        for constraint in constraints {
            left[count][constraint.x.variable] = constraint.x.coeff;
            right[count][constraint.y.variable] = constraint.y.coeff;
            out[count][constraint.z.variable] = constraint.z.coeff;
            count += 1;
        };

        Self {
            l: left,
            r: right,
            o: out,
        }
    }

    pub fn left(&self) -> Vec<Vec<i64>> {
        self.l.clone()
    }

    pub fn right(&self) -> Vec<Vec<i64>> {
        self.r.clone()
    }

    pub fn output(&self) -> Vec<Vec<i64>> {
        self.o.clone()
    }

    pub fn verify(&self, witness: Vec<i64>) {
        assert_eq!(self.l[0].len(), witness.len(), "Matrix L and witness size mismatch!");
        assert_eq!(self.r[0].len(), witness.len(), "Matrix R and witness size mismatch!");
        assert_eq!(self.o[0].len(), witness.len(), "Matrix O and witness size mismatch!");
    
        let l_mult = witness_multiply(self.l.clone(), witness.clone());
        let r_mult = witness_multiply(self.r.clone(), witness.clone());
        let o_mult = witness_multiply(self.o.clone(), witness.clone());
        let lhs = hadamard_multiply(l_mult, r_mult);
    
        assert_eq!(lhs, o_mult, "R1CS constraints not satisfied!");
        println!("R1CS constraints satisfied");
    }    
}

pub fn witness_multiply(matrix: Vec<Vec<i64>>, witness: Vec<i64>) -> Vec<i64>{
    let mut vector: Vec<i64> = vec![];
    for rows in matrix {
        let mut count = 0;
        let mut sum: Vec<i64> = vec![];
        while count < witness.len() {
            sum.push(rows[count] * witness[count]);
            count += 1;
        }
        vector.push(sum.into_iter().sum());
    }
    vector
}

fn hadamard_multiply(m1: Vec<i64>, m2: Vec<i64>) -> Vec<i64>{
    let mut vector: Vec<i64> = vec![];
    let mut count = 0;
    while count < m1.len() {
        vector.push(m1[count] * m2[count]);
        count += 1;
    }
    vector
}
