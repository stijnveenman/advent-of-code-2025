advent_of_code::solution!(6);
#[allow(unused_imports)]
use advent_of_code::prelude::*;

fn parse_input(input: &str) -> Vec<(Vec<u64>, char)> {
    let lines = input
        .lines()
        .map(|line| line.split_whitespace().collect_vec())
        .collect_vec();

    (0..lines.first().unwrap().len())
        .map(|n| lines.iter().map(|l| l.get(n).unwrap()).collect_vec())
        .map(|mut v| {
            let symbol = v.pop().unwrap().chars().next().unwrap();

            let v = v
                .into_iter()
                .map(|v| v.parse::<u64>().unwrap())
                .collect_vec();

            (v, symbol)
        })
        .collect_vec()
}

pub fn part_one(input: &str) -> Option<u64> {
    let input = parse_input(input);

    Some(
        input
            .into_iter()
            .map(|(v, op)| match op {
                '+' => v.into_iter().reduce(|a, b| a + b).unwrap(),
                '*' => v.into_iter().reduce(|a, b| a * b).unwrap(),
                _ => panic!(),
            })
            .sum(),
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
        assert_eq!(result, Some(4277556));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
