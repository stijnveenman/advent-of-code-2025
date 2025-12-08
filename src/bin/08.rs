advent_of_code::solution!(8);

use std::{
    collections::{BTreeMap, HashMap, HashSet, btree_map::Entry},
    ops::AddAssign,
};

#[allow(unused_imports)]
use advent_of_code::prelude::*;

#[cfg(not(test))]
const CONNECTIONS: usize = 1000;
#[cfg(test)]
const CONNECTIONS: usize = 10;

fn parse_input(input: &str) -> Vec<[i64; 3]> {
    input
        .lines()
        .map(|line| {
            let [x, y, z] = line
                .split(",")
                .map(|s| s.parse::<i64>().unwrap())
                .collect_array()
                .unwrap();

            [x, y, z]
        })
        .collect_vec()
}

pub fn part_one(input: &str) -> Option<u64> {
    let input = parse_input(input);

    let mut distances = BTreeMap::new();
    let mut idx = 0;
    let mut circuits: HashMap<&[i64; 3], u64> = HashMap::new();

    for (left, right) in input.iter().tuple_combinations() {
        let distance: i64 = left
            .iter()
            .zip(right.iter())
            .map(|(p1, p2)| (p2 - p1).pow(2))
            .sum::<i64>()
            .isqrt();

        match distances.entry(distance) {
            Entry::Vacant(entry) => {
                entry.insert(vec![(left, right)]);
            }
            Entry::Occupied(mut entry) => {
                entry.get_mut().push((left, right));
            }
        }
    }

    let mut remaining = CONNECTIONS;
    let iter = distances.values().flatten();

    // probable issue, what if both left and right are in a different circuit
    for (left, right) in iter {
        let left_circuit = circuits.get(left);
        let right_circuit = circuits.get(right);

        match (left_circuit, right_circuit) {
            (Some(left_circuit), Some(right_circuit)) if left_circuit == right_circuit => {}
            (Some(left_circuit), Some(right_circuit)) => {
                let left_circuit = *left_circuit;
                let right_circuit = *right_circuit;
                circuits.iter_mut().for_each(|m| {
                    if *m.1 == right_circuit {
                        *m.1 = left_circuit;
                    }
                });
            }
            (Some(left_circuit), None) => {
                circuits.insert(right, *left_circuit);
            }
            (None, Some(right_circuit)) => {
                circuits.insert(left, *right_circuit);
            }
            (None, None) => {
                circuits.insert(left, idx);
                circuits.insert(right, idx);
                idx += 1;
            }
        }

        remaining -= 1;
        if remaining == 0 {
            break;
        }
    }

    let mut counts = circuits
        .iter()
        .counts_by(|v| v.1)
        .into_iter()
        .map(|v| v.1)
        .collect_vec();

    counts.sort();

    Some(
        counts
            .into_iter()
            .rev()
            .take(3)
            .reduce(|a, b| a * b)
            .unwrap() as u64,
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let input = parse_input(input);

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
