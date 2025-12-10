advent_of_code::solution!(10);

use std::{
    collections::{HashMap, HashSet},
    sync::atomic::{AtomicUsize, Ordering},
};

#[allow(unused_imports)]
use advent_of_code::prelude::*;

#[allow(clippy::type_complexity)]
fn parse_input(input: &str) -> Vec<(Vec<bool>, Vec<Vec<usize>>, Vec<usize>)> {
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
                        .map(|b| b.parse::<usize>().unwrap())
                        .collect_vec()
                })
                .collect_vec();

            let joltage = buttons.pop().unwrap();

            (light_diagram, buttons, joltage)
        })
        .collect_vec()
}

fn apply_click(state: &[bool], button: &[usize]) -> Vec<bool> {
    let mut state = state.to_vec();
    for i in button {
        state[*i] = !state[*i];
    }
    state
}

fn count_pressed(lights: Vec<bool>, buttons: Vec<Vec<usize>>) -> u64 {
    let mut states = vec![lights.iter().map(|_| false).collect_vec()];
    let mut new_states = vec![];
    let mut visited = HashSet::new();
    let mut clicks = 0;

    loop {
        clicks += 1;
        for state in &states {
            for button in &buttons {
                let state = apply_click(state, button);

                if state == lights {
                    return clicks;
                }

                if !visited.contains(&state) {
                    visited.insert(state.clone());
                    new_states.push(state);
                }
            }
        }

        std::mem::swap(&mut states, &mut new_states);
        new_states.clear();
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let input = parse_input(input);

    Some(
        input
            .into_par_iter()
            .map(|(lights, buttons, _)| count_pressed(lights, buttons))
            .sum(),
    )
}

fn is_below(state: &[usize], target: &[usize]) -> bool {
    !state.iter().zip(target.iter()).any(|(a, b)| a > b)
}

fn press(state: &mut [usize], button: &[usize]) {
    for b in button {
        state[*b] += 1;
    }
}

fn unpress(state: &mut [usize], button: &[usize]) {
    for b in button {
        state[*b] -= 1;
    }
}

#[allow(clippy::collapsible_if)]
fn joltage_bfs(
    state: &mut [usize],
    target: &[usize],
    buttons: &[Vec<usize>],
    cache: &mut HashMap<Vec<usize>, Option<u64>>,
) -> Option<u64> {
    if let Some(value) = cache.get(state) {
        return value.to_owned();
    }

    let mut min = None;

    for button in buttons {
        press(state, button);

        if state == target {
            unpress(state, button);
            return Some(1);
        }

        if is_below(state, target) {
            if let Some(n) = joltage_bfs(state, target, buttons, cache) {
                let n = n + 1;

                if min.is_none_or(|min| n < min) {
                    min = Some(n);
                }
            }
        }

        unpress(state, button);
    }

    cache.insert(state.to_vec(), min);
    min
}

fn count_joltage(state: &mut [usize], target: &[usize], buttons: &[Vec<usize>]) -> Option<u64> {
    if buttons.is_empty() {
        return None;
    }

    let (button, buttons) = buttons.split_last().unwrap();
    let mut clicks = 0;

    while is_below(state, target) {
        if state == target {
            return Some(clicks);
        }

        press(state, button);
        clicks += 1;
    }

    clicks -= 1;
    unpress(state, button);

    loop {
        // println!("{button:?} - {clicks} - {state:?}");
        if let Some(next_clicks) = count_joltage(state, target, buttons) {
            return Some(clicks + next_clicks);
        }

        if clicks > 0 {
            unpress(state, button);
            clicks -= 1;
        } else {
            break;
        }
    }

    None
}

pub fn part_two(input: &str) -> Option<u64> {
    let input = parse_input(input);
    let count = input.len();
    let progress = AtomicUsize::new(0);

    Some(
        input
            .into_par_iter()
            .map(|(_, mut buttons, joltage)| {
                // println!("\n\n{joltage:?}\n");
                // assume it's more efficient to press large buttons first
                // this puts large buttons at the end
                buttons.sort_by_key(|s| s.len());

                let result = joltage_bfs(
                    &mut joltage.iter().map(|_| 0).collect_vec(),
                    &joltage,
                    &buttons,
                    &mut HashMap::new(),
                )
                .unwrap();
                let progress = progress.fetch_add(1, Ordering::Relaxed) + 1;
                println!("{}/{count} - {result}", progress);
                result
            })
            .sum(),
    )
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
        assert_eq!(result, Some(33));
    }
}
