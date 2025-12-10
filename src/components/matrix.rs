use std::fmt::Display;

use itertools::Itertools;

pub struct Matrix(Vec<Vec<usize>>);

impl Matrix {
    pub fn empty(n: usize, m: usize) -> Self {
        Matrix(vec![vec![0; m]; n])
    }
}

impl From<&[usize]> for Matrix {
    fn from(value: &[usize]) -> Self {
        Matrix(value.iter().map(|v| Vec::from([*v])).collect_vec())
    }
}

impl From<Vec<Vec<usize>>> for Matrix {
    fn from(value: Vec<Vec<usize>>) -> Self {
        let first_len = value.first().unwrap().len();
        assert!(value.iter().all(|v| v.len() == first_len));

        Matrix(value)
    }
}

impl Display for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in &self.0 {
            writeln!(f, "{:?}", line)?;
        }

        Ok(())
    }
}
