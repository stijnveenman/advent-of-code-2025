advent_of_code::solution!(10);
#[allow(unused_imports)]
use advent_of_code::prelude::*;

fn parse_input(input: &str) -> Vec<(Vec<bool>, Vec<Vec<u64>>, Vec<u64>)> {
    input
        .lines()
        .map(|line| {
            let (light_diagram, line) = line.split_once(" ").unwrap();
            let light_diagram = light_diagram[1..light_diagram.len() - 1]
                .chars()
                .map(|c| c == '#')
                .collect_vec();

            let mut buttons = line
                .split(" ")
                .map(|button| {
                    button[1..button.len() - 1]
                        .split(",")
                        .map(|b| b.parse::<u64>().unwrap())
                        .collect_vec()
                })
                .collect_vec();

            let joltage = buttons.pop().unwrap();

            (light_diagram, buttons, joltage)
        })
        .collect_vec()
}

pub fn part_one(input: &str) -> Option<u64> {
    let input = parse_input(input);
    dbg!(input);

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
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
