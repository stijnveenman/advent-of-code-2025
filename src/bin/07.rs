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

fn ray(grid: &CharGrid, mut pos: Point) -> Option<Point> {
    loop {
        pos += Point::DOWN;

        if !grid.in_bounds(&pos) {
            return None;
        }

        if grid.get(&pos).is_some_and(|c| c == '^') {
            let a = grid.draw(|p, c| {
                if p == &pos {
                    'X'.to_string()
                } else {
                    c.unwrap().to_string()
                }
            });
            println!("{a}\n");

            return Some(pos);
        }
    }
}

pub fn part_two(input: &str) -> Option<u64> {
    let grid = parse_input(input);

    let start = grid.entries().find(|v| v.1 == 'S').unwrap().0;

    let next = ray(&grid, start).unwrap();
    let mut stack = vec![next];
    let mut splits = 1;

    while let Some(pos) = stack.pop() {
        if let Some(next) = ray(&grid, pos + Point::LEFT) {
            splits += 1;
            stack.push(next);
        }
        if let Some(next) = ray(&grid, pos + Point::RIGHT) {
            splits += 1;
            stack.push(next);
        }
    }

    // Not fully sure where this 1 comes from
    Some(splits + 1)
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
