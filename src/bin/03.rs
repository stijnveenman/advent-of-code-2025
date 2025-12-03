advent_of_code::solution!(3);

#[allow(unused_imports)]
use advent_of_code::prelude::*;

fn parse_input(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect_vec())
        .collect_vec()
}

#[allow(dead_code)]
fn max_two_slow(input: &[u32]) -> (u32, u32) {
    let mut max = u32::MIN;
    let mut max_v = (0, 0);
    for i in 0..input.len() {
        let left = input[i];

        for right in input.iter().skip(i + 1) {
            let number = left * 10 + *right;
            if number > max {
                max = number;
                max_v = (left, *right)
            }
        }
    }

    max_v
}

fn max_two(input: &[u32]) -> (u32, u32) {
    let mut left = input[0];
    let mut right = input[1];

    let mut iter = input.iter().skip(2).peekable();
    while let Some(current) = iter.next() {
        if *current > left && iter.peek().is_some() {
            left = *current;
            right = **iter.peek().unwrap();
        } else if *current > right {
            right = *current;
        }
    }

    (left, right)
}

pub fn part_one(input: &str) -> Option<u64> {
    let input = parse_input(input);

    Some(
        input
            .iter()
            .map(|i| {
                // assert_eq!(
                //     max_two(i),
                //     max_two_slow(i),
                //     "max_two is not equal for {i:?}"
                // );
                max_two(i)
            })
            .map(|i| i.0 * 10 + i.1)
            .sum::<u32>() as u64,
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
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
