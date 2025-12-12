advent_of_code::solution!(10);

use std::collections::HashSet;

use advent_of_code::components::matrix::Matrix;
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

fn buttons_to_matrix(buttons: Vec<Vec<usize>>, m: usize) -> Matrix {
    (0..m)
        .map(|m| {
            (0..buttons.len())
                .map(|n| {
                    if buttons.get(n).unwrap().contains(&m) {
                        1
                    } else {
                        0
                    }
                })
                .collect_vec()
        })
        .collect_vec()
        .into()
}

pub fn part_two(input: &str) -> Option<u64> {
    let input = parse_input(input);

    input.into_iter().for_each(|(_, buttons, joltage)| {
        let matrix = Matrix::from(joltage.iter().map(|v| *v as isize).collect_vec());
        let mut buttons = buttons_to_matrix(buttons, joltage.len());
        buttons.append(matrix);
        println!("-----\n{}", buttons);
        buttons.row_echelon();
        println!("{}", buttons);
    });

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
        assert_eq!(result, Some(33));
    }
}
