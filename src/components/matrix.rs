use std::fmt::Display;

use itertools::Itertools;

pub struct Matrix(Vec<Vec<usize>>);

impl Matrix {
    pub fn empty(n: usize, m: usize) -> Self {
        Matrix(vec![vec![0; m]; n])
    }

    fn pivot_index(&self) -> usize {
        self.0
            .iter()
            .find_position(|v| *v.first().unwrap() == 1)
            .unwrap()
            .0
    }

    pub fn row_echelon(&mut self) {
        let pivot = self.pivot_index();

        if pivot != 0 {
            let [first, pivot] = self.0.get_disjoint_mut([0, pivot]).unwrap();
            std::mem::swap(first, pivot);
        }
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
