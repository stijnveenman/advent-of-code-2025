advent_of_code::solution!(1);
#[allow(unused_imports)]
use advent_of_code::prelude::*;

fn parse_input(input: &str) -> Vec<i64> {
    input
        .lines()
        .map(|line| {
            let mut chars = line.chars();
            let dir = chars.next().unwrap();
            let remaining: String = chars.collect();
            let remaining: i64 = remaining.parse().unwrap();

            match dir {
                'L' => -remaining,
                'R' => remaining,
                _ => todo!(),
            }
        })
        .collect_vec()
}

pub fn part_one(input: &str) -> Option<u64> {
    let input = parse_input(input);

    let mut dial = 50;
    let mut crossings = 0;

    for item in input {
        dial += item;

        dial = (dial + 100) % 100;
        if dial == 0 {
            crossings += 1;
        }
    }

    Some(crossings)
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
