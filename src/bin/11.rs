advent_of_code::solution!(11);
use std::collections::{HashMap, HashSet};

#[allow(unused_imports)]
use advent_of_code::prelude::*;

fn parse_input(input: &str) -> std::collections::HashMap<&str, std::vec::Vec<&str>> {
    input
        .lines()
        .map(|line| {
            let (from, to) = line.split_once(":").unwrap();
            let to = to.trim().split(" ").collect_vec();

            (from, to)
        })
        .collect::<HashMap<_, _>>()
}

fn path_count(connections: &HashMap<&str, Vec<&str>>, from: &str, to: &str) -> u64 {
    if from == to {
        return 1;
    }

    let Some(next) = connections.get(from) else {
        return 0;
    };

    next.iter()
        .map(|from| path_count(connections, from, to))
        .sum()
}

fn path_count_containing(
    connections: &HashMap<&str, Vec<&str>>,
    from: &str,
    to: &str,
    visited: &mut HashMap<String, u64>,
) -> u64 {
    if from == to {
        return 1;
    }

    let Some(next) = connections.get(from) else {
        return 0;
    };

    let count = next
        .iter()
        .map(|from| path_count_containing(connections, from, to, visited))
        .sum();
    visited.insert(from.to_string(), count);
    count
}

pub fn part_one(input: &str) -> Option<u64> {
    let input = parse_input(input);

    Some(path_count(&input, "you", "out"))
}

pub fn part_two(input: &str) -> Option<u64> {
    let input = parse_input(input);

    dbg!(path_count_containing(
        &input,
        "fft",
        "dac",
        &mut HashMap::new(),
    ));
    Some(path_count_containing(
        &input,
        "dac",
        "out",
        &mut HashMap::new(),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(2));
    }
}
