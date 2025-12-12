use std::{fmt::Display, ops::Rem, option::Option, vec::Vec};

use itertools::Itertools;

#[derive(Clone)]
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
            .find(|(_, v)| {
                let Some(first) = v.iter().find(|v| **v != 0) else {
                    return false;
                };

                if *v.get(n).unwrap() == 0 {
                    return false;
                }

                true
            })
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
        println!("{}", self);
        if let Some(pivot) = self.pivot_index(m) {
            self.switch(m, pivot);
            if self.get(m, m) < 0 {
                self.flip_signs(m);
            }
        };

        while self.get(m, m) > 1 {
            let Some(rhs) = self
                .0
                .iter()
                .enumerate()
                .skip(m + 1)
                .find(|(_, v)| v[m] != 0)
            else {
                break;
            };

            println!("{self}");
            dbg!(m, rhs.0);
            if rhs.1[m] > 0 {
                self.sub(m, rhs.0);
            } else {
                self.add(m, rhs.0);
            }
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

    fn m(&self) -> usize {
        self.0.len()
    }

    fn first_non_empty(&self) -> Option<&Vec<isize>> {
        self.0.iter().rev().find(|v| v.iter().any(|n| *n != 0))
    }

    pub fn solve(&mut self) -> Option<usize> {
        // println!("{}", self);

        let Some(row) = self.first_non_empty() else {
            return Some(0);
        };

        let unknowns = find_unknowns(row);
        // dbg!(&unknowns);

        if unknowns.len() == 1 {
            let unknown = *unknowns.first().unwrap();
            let x_count = *row.get(unknown).unwrap();
            let count = *row.last().unwrap() / x_count;
            if count < 0 {
                return None;
            }

            self.back_substitute(unknown, count);

            return self.solve().map(|v| v + (count as usize));
        }

        if unknowns.is_empty() {
            return None;
        }

        let mut min = None;

        for unknown in unknowns {
            let mut attempt = 0;
            loop {
                let mut matrix = self.clone();
                // println!("{unknown} {attempt}");

                matrix.back_substitute(unknown, attempt);

                if let Some(result) = matrix.solve() {
                    min = Some(min.unwrap_or(usize::MAX).min((attempt as usize) + result));
                };

                attempt += 1;

                if attempt > 100 {
                    break;
                }
            }
        }

        min
    }

    pub fn back_substitute(&mut self, n: usize, rhs: isize) {
        for row in &mut self.0 {
            let reff = row.get_mut(n).unwrap();
            let count = *reff;
            *reff = 0;

            *row.last_mut().unwrap() -= count * rhs;
        }
    }

    pub fn row_echelon(&mut self) {
        let nm = self.0.len().min(self.0.first().unwrap().len());
        for m in 0..nm {
            self.row_echelon_row(m);
        }
    }
}

fn find_unknowns(row: &[isize]) -> Vec<usize> {
    let last = row.len() - 1;
    row.iter()
        .enumerate()
        .filter(|v| *v.1 != 0 && v.0 != last)
        .map(|v| v.0)
        .collect_vec()
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
