advent_of_code::solution!(7);
use std::collections::HashMap;

#[allow(unused_imports)]
use advent_of_code::prelude::*;
use advent_of_code::{
    components::Point,
    grid::{self, Grid, char_grid::CharGrid},
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

fn find_next(grid: &CharGrid, mut pos: Point) -> Option<Point> {
    loop {
        pos += Point::DOWN;

        if !grid.in_bounds(&pos) {
            return None;
        }

        if grid.get(&pos).is_some_and(|c| c == '^') {
            return Some(pos);
        }
    }
}

fn ray(grid: &CharGrid, pos: Point, cache: &mut HashMap<Point, u64>) -> u64 {
    if let Some(value) = cache.get(&pos) {
        return *value;
    }

    let left = find_next(grid, pos + Point::LEFT)
        .map(|p| ray(grid, p, cache))
        .unwrap_or(1);

    let right = find_next(grid, pos + Point::RIGHT)
        .map(|p| ray(grid, p, cache))
        .unwrap_or(1);

    cache.insert(pos, left + right);
    left + right
}

pub fn part_two(input: &str) -> Option<u64> {
    let grid = parse_input(input);

    let start = grid.entries().find(|v| v.1 == 'S').unwrap().0;

    let mut cache = HashMap::new();
    Some(ray(&grid, start, &mut cache))
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
        assert_eq!(result, Some(40));
    }
}
