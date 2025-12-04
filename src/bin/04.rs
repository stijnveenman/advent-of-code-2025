advent_of_code::solution!(4);
use advent_of_code::grid::{Grid, char_grid::CharGrid};
#[allow(unused_imports)]
use advent_of_code::prelude::*;

fn parse_input(input: &str) -> advent_of_code::grid::char_grid::CharGrid {
    CharGrid::new(input)
}

pub fn part_one(input: &str) -> Option<u64> {
    let grid = parse_input(input);

    Some(
        grid.keys()
            .filter(|p| grid.get(p).is_some_and(|c| c == '@'))
            .map(|p| {
                p.full_neighbours()
                    .iter()
                    .filter(|n| grid.get(n).is_some_and(|c| c == '@'))
                    .count()
            })
            .filter(|count| *count < 4)
            .count() as u64,
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut grid = parse_input(input);
    let mut removed = 0;

    loop {
        let rolls = grid
            .keys()
            .filter(|p| grid.get(p).is_some_and(|c| c == '@'))
            .filter(|p| {
                p.full_neighbours()
                    .iter()
                    .filter(|n| grid.get(n).is_some_and(|c| c == '@'))
                    .count()
                    < 4
            })
            .collect_vec();

        removed += rolls.len();
        if rolls.is_empty() {
            break;
        }

        for roll in rolls {
            grid.set(&roll, '.');
        }
    }

    Some(removed as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(43));
    }
}
