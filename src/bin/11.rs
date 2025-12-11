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

fn routes_between(
    in_connections: &HashMap<&str, Vec<&str>>,
    out_connections: &HashMap<&str, Vec<&str>>,
    from: &str,
    to: &str,
) -> usize {
    dbg!(from);
    let mut stack = out_connections.get(from).unwrap().to_vec();

    let mut routes = HashMap::from([(from, 1)]);
    for first in &stack {
        for inc in in_connections.get(first).unwrap() {
            routes.entry(inc).or_insert(0);
        }
    }

    loop {
        let mut next_stack = HashSet::new();
        dbg!(&stack);
        assert!(!stack.is_empty());
        for current in stack {
            let Some(in_routes) = in_connections
                .get(current)
                .unwrap()
                .iter()
                // i think this unwrap_or is wrong in the real input
                .map(|inc| routes.get(inc))
                .sum::<Option<_>>()
            else {
                // previous routes not fully calculated yet, skip it. another item will trigger it
                // again
                dbg!(current);
                continue;
            };

            routes.insert(current, in_routes);

            if current == to {
                return in_routes;
            }

            for next in out_connections.get(current).unwrap() {
                next_stack.insert(*next);
            }
        }
        stack = next_stack.into_iter().collect_vec();
    }
}

fn color_graph<'a>(
    in_connections: &HashMap<&'a str, Vec<&'a str>>,
    from: &'a str,
) -> HashSet<&'a str> {
    let mut stack = vec![from];
    let mut visited = HashSet::from([from]);

    while let Some(node) = stack.pop() {
        for n in in_connections.get(node).unwrap_or(&Vec::new()) {
            if visited.contains(n) {
                continue;
            }

            visited.insert(n);
            stack.push(n);
        }
    }

    visited
}

// test assumption:
// any node is at an exact depth, ie; nodes don't skip over layers
// false
#[allow(dead_code)]
fn find_depth<'a>(
    out_connections: &HashMap<&'a str, Vec<&'a str>>,
    node: &'a str,
    depth: usize,
    visited: &mut HashMap<&'a str, usize>,
) {
    dbg!(depth);
    for connection in out_connections.get(dbg!(node)).unwrap_or(&Vec::new()) {
        match visited.entry(connection) {
            Entry::Occupied(occupied_entry) => {
                assert_eq!(*occupied_entry.get(), depth)
            }
            Entry::Vacant(vacant_entry) => {
                vacant_entry.insert(depth);
                find_depth(out_connections, connection, depth + 1, visited);
            }
        }
    }
}

pub fn part_two(input: &str) -> Option<u64> {
    let out_connections = parse_input(input);
    let in_connections = flip_connections(&out_connections);

    // render_graph(&out_connections, "test.svg");
    // render_graph(&in_connections, "in.svg");

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
