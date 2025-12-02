advent_of_code::solution!(2);
#[allow(unused_imports)]
use advent_of_code::prelude::*;

fn parse_input(input: &str) -> Vec<std::ops::RangeInclusive<usize>> {
    input
        .split(",")
        .map(|input| {
            let (left, right) = input.trim().split_once("-").unwrap();

            let left: usize = left.parse().unwrap();
            let right: usize = right.parse().unwrap();

            left..=right
        })
        .collect_vec()
}

pub fn part_one(input: &str) -> Option<u64> {
    let input = parse_input(input);

    Some(
        input
            .into_iter()
            .map(|input| {
                input
                    .filter(|input| {
                        let s = input.to_string();
                        if s.len() % 2 == 1 {
                            return false;
                        }

                        let (left, right) = s.split_at(s.len() / 2);

                        left == right
                    })
                    .sum::<usize>()
            })
            .sum::<usize>() as u64,
    )
}

fn is_invalid_id(input: &str) -> bool {
    let len = input.len();
    for i in 1..len {
        if !len.is_multiple_of(i) {
            continue;
        }

        let repeats = len / i;
        let compare = input[0..i].repeat(repeats);

        if input == compare {
            return true;
        }
    }
    false
}

pub fn part_two(input: &str) -> Option<u64> {
    let input = parse_input(input);

    Some(
        input
            .into_iter()
            .map(|input| {
                input
                    .filter(|input| {
                        let s = input.to_string();

                        is_invalid_id(&s)
                    })
                    .sum::<usize>()
            })
            .sum::<usize>() as u64,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first() {
        assert_eq!(part_one("11-22"), Some(33));
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }
}
