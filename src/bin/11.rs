advent_of_code::solution!(11);
use std::{
    collections::{HashMap, HashSet, hash_map::Entry},
    fs::File,
    io::Write,
    process::{Command, Stdio},
    vec,
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

#[allow(dead_code)]
fn render_graph(connections: &HashMap<&str, Vec<&str>>, filename: &str) {
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

    let mut file = File::create(filename).unwrap();
    file.write_all(&dot_output.stdout).unwrap();
}

pub fn part_one(input: &str) -> Option<u64> {
    let input = parse_input(input);

    Some(path_count(&input, "you", "out"))
}

fn flip_connections<'a>(
    connections: &HashMap<&'a str, Vec<&'a str>>,
) -> HashMap<&'a str, Vec<&'a str>> {
    let mut hm = HashMap::new();
    for (from, to) in connections {
        for to in to {
            match hm.entry(*to) {
                Entry::Occupied(mut occupied_entry) => {
                    let v: &mut Vec<&str> = occupied_entry.get_mut();
                    v.push(*from);
                }
                Entry::Vacant(vacant_entry) => {
                    vacant_entry.insert(vec![*from]);
                }
            }
        }
    }

    hm
}

fn next_layer<'a>(connections: &HashMap<&'a str, Vec<&'a str>>, from: &'a str) -> Vec<&'a str> {
    let mut stack = connections.get(from).unwrap().to_vec();
    let mut layer = HashSet::new();

    while let Some(node) = stack.pop() {
        if node == "fft" || node == "dac" || node == "svr" {
            return vec![node];
        }

        let out = connections.get(node).unwrap();

        if out.len() > 6 {
            layer.insert(node);
        } else {
            stack.extend(out);
        }
    }

    layer.into_iter().collect_vec()
}

fn layered_path_count(
    connections: &HashMap<&str, Vec<&str>>,
    from: &str,
    to: &str,
    layer: &Vec<&str>,
) -> u64 {
    if from == to {
        return 1;
    }

    if layer.contains(&from) {
        return 0;
    }

    let Some(next) = connections.get(from) else {
        return 0;
    };

    next.iter()
        .map(|from| layered_path_count(connections, from, to, layer))
        .sum()
}

pub fn part_two(input: &str) -> Option<u64> {
    let out_connections = parse_input(input);
    let in_connections = flip_connections(&out_connections);

    // render_graph(&out_connections, "test.svg");
    // render_graph(&in_connections, "in.svg");

    let mut layers = vec![vec!["out"]];
    loop {
        let from = layers.last().unwrap().first().unwrap();

        let layer = next_layer(&in_connections, from);

        if layer.contains(&"svr") {
            layers.push(layer);
            break;
        }
        layers.push(layer);
    }

    let mut routes = HashMap::from([("out", 1)]);

    for i in 0..layers.len() - 1 {
        let current = &layers[i];
        let next = &layers[i + 1];

        for to in next {
            let to_count = current
                .iter()
                .map(|from| {
                    let prev = routes.get(from).unwrap();

                    let next = if next.len() == 1 {
                        layers.get(i + 2).unwrap_or(next)
                    } else {
                        next
                    };

                    prev * layered_path_count(&in_connections, from, to, next)
                })
                .sum();

            routes.insert(to, to_count);
        }
    }

    Some(*routes.get("svr").unwrap())
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
