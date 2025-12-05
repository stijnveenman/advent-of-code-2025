advent_of_code::solution!(5);
#[allow(unused_imports)]
use advent_of_code::prelude::*;

fn parse_input(input: &str) -> (Vec<std::ops::RangeInclusive<u64>>, Vec<u64>) {
    let (first, last) = input.split_once("\n\n").unwrap();

    let first = first
        .lines()
        .map(|line| {
            let (left, right) = line.split_once("-").unwrap();
            (left.parse::<u64>().unwrap()..=right.parse::<u64>().unwrap())
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
            .filter(|n| database.iter().any(|d| d.contains(&n)))
            .count() as u64,
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
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
