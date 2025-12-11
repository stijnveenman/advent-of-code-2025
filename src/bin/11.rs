advent_of_code::solution!(11);
use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::Write,
    process::{Command, Stdio},
};

#[allow(unused_imports)]
use advent_of_code::prelude::*;
use petgraph::{Graph, dot::Dot};

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

fn render_graph(connections: &HashMap<&str, Vec<&str>>) {
    let mut deps = Graph::<&str, &str>::new();
    let mut indexes = HashMap::new();

    for connection in connections {
        let from_index = *indexes
            .entry(connection.0)
            .or_insert_with(|| deps.add_node(connection.0));

        for target in connection.1 {
            let index = *indexes
                .entry(target)
                .or_insert_with(|| deps.add_node(target));

            deps.add_edge(from_index, index, "");
        }
    }

    let dot_string = Dot::with_config(&deps, &[]);

    let mut dot = Command::new("dot")
        .args(["-Tsvg"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    dot.stdin
        .take()
        .unwrap()
        .write_all(dot_string.to_string().as_bytes())
        .unwrap();

    let dot_output = dot.wait_with_output().unwrap();

    let mut file = File::create("output.svg").unwrap();
    file.write_all(&dot_output.stdout).unwrap();
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

    render_graph(&input);

    None
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
