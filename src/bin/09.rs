advent_of_code::solution!(9);
use std::u64;

use advent_of_code::components::Point;
#[allow(unused_imports)]
use advent_of_code::prelude::*;

fn parse_input(input: &str) -> Vec<Point> {
    input
        .lines()
        .map(|line| {
            let (left, right) = line.split_once(",").unwrap();

            Point::new(left.parse().unwrap(), right.parse().unwrap())
        })
        .collect_vec()
}

pub fn part_one(input: &str) -> Option<u64> {
    let input = parse_input(input);

    let mut max = u64::MIN;

    for l in 0..input.len() {
        for r in l + 1..input.len() {
            let left = input[l];
            let right = input[r];

            let area = (left - right).abs() + Point::new(1, 1);

            max = max.max((area.x.abs() * area.y.abs()) as u64);
        }
    }

    Some(max)
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
        assert_eq!(result, Some(50));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
