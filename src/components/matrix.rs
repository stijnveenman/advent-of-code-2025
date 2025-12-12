use std::fmt::Display;

use itertools::Itertools;

pub struct Matrix(Vec<Vec<isize>>);

impl Matrix {
    pub fn empty(n: usize, m: usize) -> Self {
        Matrix(vec![vec![0; n]; m])
    }

    fn pivot_index(&self, n: usize) -> Option<usize> {
        self.0
            .iter()
            .enumerate()
            .skip(n)
            .find(|v| *v.1.get(n).unwrap() != 0)
            .map(|v| v.0)
    }

    pub fn append(&mut self, rhs: Matrix) {
        assert_eq!(self.0.len(), rhs.0.len());

        self.0.iter_mut().zip(rhs.0).for_each(|(l, r)| l.extend(r));
    }

    fn switch(&mut self, lhs: usize, rhs: usize) {
        if lhs == rhs {
            return;
        }
        let [first, second] = self.0.get_disjoint_mut([lhs, rhs]).unwrap();
        std::mem::swap(first, second);
    }

    fn sub(&mut self, lhs: usize, rhs: usize) {
        if lhs == rhs {
            return;
        }

        let [lhs, rhs] = self.0.get_disjoint_mut([lhs, rhs]).unwrap();

        lhs.iter_mut().zip(rhs).for_each(|(lhs, rhs)| {
            *lhs -= *rhs;
        });
    }

    fn add(&mut self, lhs: usize, rhs: usize) {
        if lhs == rhs {
            return;
        }

        let [lhs, rhs] = self.0.get_disjoint_mut([lhs, rhs]).unwrap();

        lhs.iter_mut().zip(rhs).for_each(|(lhs, rhs)| {
            *lhs += *rhs;
        });
    }

    fn flip_signs(&mut self, lhs: usize) {
        self.0
            .get_mut(lhs)
            .unwrap()
            .iter_mut()
            .for_each(|v| *v = -*v);
    }

    fn get(&self, n: usize, m: usize) -> isize {
        *self.0.get(m).unwrap().get(n).unwrap()
    }

    fn row_echelon_row(&mut self, m: usize) {
        if let Some(pivot) = self.pivot_index(m) {
            self.switch(m, pivot);
            if self.get(m, m) < 0 {
                self.flip_signs(m);
            }

            for i in m + 1..self.m() {
                while self.get(m, i) > 0 {
                    self.sub(i, m);
                }
                while self.get(m, i) < 0 {
                    self.add(i, m);
                }
            }
        }
    }

    fn m(&self) -> usize {
        self.0.len()
    }

    fn n(&self) -> usize {
        self.0.first().unwrap().len()
    }

    pub fn row_echelon(&mut self) {
        let nm = self.0.len().min(self.0.first().unwrap().len());
        for m in 0..nm {
            self.row_echelon_row(m);
            println!("{}", self);
        }
    }
}

impl From<Vec<isize>> for Matrix {
    fn from(value: Vec<isize>) -> Self {
        Matrix(value.iter().map(|v| Vec::from([*v])).collect_vec())
    }
}

impl From<Vec<Vec<isize>>> for Matrix {
    fn from(value: Vec<Vec<isize>>) -> Self {
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
