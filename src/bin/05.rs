advent_of_code::solution!(5);
use std::{collections::HashSet, ops::RangeInclusive};

#[allow(unused_imports)]
use advent_of_code::prelude::*;

fn parse_input(input: &str) -> (Vec<std::ops::RangeInclusive<u64>>, Vec<u64>) {
    let (first, last) = input.split_once("\n\n").unwrap();

    let first = first
        .lines()
        .map(|line| {
            let (left, right) = line.split_once("-").unwrap();
            left.parse::<u64>().unwrap()..=right.parse::<u64>().unwrap()
        })
        .collect_vec();

    let last = last
        .lines()
        .map(|l| l.parse::<u64>().unwrap())
        .collect_vec();

    (first, last)
}

pub fn part_one(input: &str) -> Option<u64> {
    let (database, ingredients) = parse_input(input);

    Some(
        ingredients
            .iter()
            .filter(|n| database.iter().any(|d| d.contains(n)))
            .count() as u64,
    )
}

fn combine(left: &RangeInclusive<u64>, right: &RangeInclusive<u64>) -> Option<RangeInclusive<u64>> {
    ///// AAAA
    /////   BBBB

    if left.contains(right.start()) || left.contains(right.end()) {
        // overlap
        let start = left.start().min(right.start());
        let end = left.end().max(right.end());

        return Some(*start..=*end);
    }

    None
}

pub fn part_two(input: &str) -> Option<u64> {
    let (mut input, _) = parse_input(input);

    let mut count = input.len();
    loop {
        let mut collapsed = Vec::new();
        'outer: for left in input.into_iter() {
            for right in collapsed.iter_mut() {
                if let Some(new) = combine(&left, right) {
                    *right = new;
                    continue 'outer;
                }
            }

            collapsed.push(left);
        }
        input = collapsed;
        if count == input.len() {
            break;
        }
        count = input.len();
    }

    input.iter().for_each(|left| {
        let right = input
            .iter()
            .filter(|right| *right != left)
            .find(|right| left.contains(right.start()) || left.contains(right.end()));

        assert_eq!(right, None, "failed {left:?} {right:?}")
    });

    Some(input.iter().map(|c| 1 + c.end() - c.start()).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_combine() {
        assert_eq!(combine(&(12..=18), &(10..=14)), Some(10..=18));
        assert_eq!(combine(&(10..=18), &(16..=20)), Some(10..=20));
    }

    #[test]
    fn test_0_range() {
        assert_eq!(
            combine(
                &(354113252785914..=354113252785914),
                &(354113252785914..=359458697423182)
            ),
            Some(354113252785914..=359458697423182)
        );
        assert_eq!(
            combine(
                &(354113252785914..=359458697423182),
                &(354113252785914..=354113252785914),
            ),
            Some(354113252785914..=359458697423182)
        );
    }
}
