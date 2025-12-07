advent_of_code::solution!(7);
#[allow(unused_imports)]
use advent_of_code::prelude::*;
use advent_of_code::{
    components::Point,
    grid::{Grid, char_grid::CharGrid},
};

fn parse_input(input: &str) -> advent_of_code::grid::char_grid::CharGrid {
    CharGrid::new(input)
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut grid = parse_input(input);

    let start = grid.entries().find(|v| v.1 == 'S').unwrap().0;

    let mut stack = vec![start];
    let mut splits = 0;

    while let Some(mut pos) = stack.pop() {
        loop {
            pos += Point::DOWN;

            if grid.get(&pos).is_none_or(|c| c == '|') {
                break;
            }

            if grid.get(&pos).is_some_and(|c| c == '^') {
                stack.push(pos + Point::LEFT);
                stack.push(pos + Point::RIGHT);
                splits += 1;
                break;
            }
            grid.set(&pos, '|');
        }
    }

    Some(splits)
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
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
