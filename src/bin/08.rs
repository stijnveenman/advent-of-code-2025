advent_of_code::solution!(8);

use std::collections::{BTreeMap, btree_map::Entry};

#[allow(unused_imports)]
use advent_of_code::prelude::*;

#[cfg(not(test))]
const CONNECTIONS: u64 = 10;
#[cfg(test)]
const CONNECTIONS: u64 = 10;

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

    dbg!(distances.pop_first());
    dbg!(distances.pop_first());

    None
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
