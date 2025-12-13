advent_of_code::solution!(12);
#[allow(unused_imports)]
use advent_of_code::prelude::*;

fn parse_input(input: &str) -> (std::vec::Vec<u64>, std::vec::Vec<(u64, std::vec::Vec<u64>)>) {
    let blocks = input
        .split("\n\n")
        .take_while(|s| !s.contains("x"))
        .map(|block| block.chars().filter(|c| *c == '#').count() as u64)
        .collect_vec();
    let grids = input
        .split("\n\n")
        .last()
        .unwrap()
        .lines()
        .map(|line| {
            let (grid, blocks) = line.split_once(":").unwrap();

            let (width, height) = grid.split_once("x").unwrap();
            let blocks = blocks
                .trim()
                .split(" ")
                .map(|b| b.parse::<u64>().unwrap())
                .collect_vec();

            (
                width.parse::<u64>().unwrap() * height.parse::<u64>().unwrap(),
                blocks,
            )
        })
        .collect_vec();

    (blocks, grids)
}

pub fn part_one(input: &str) -> Option<u64> {
    let (blocks, grids) = parse_input(input);

    let result = grids
        .iter()
        .filter(|(area, block_counts)| {
            let min_area: u64 = block_counts
                .iter()
                .enumerate()
                .map(|(idx, count)| blocks[idx] * count)
                .sum();

            min_area <= *area
        })
        .count();

    Some(result as u64)
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
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
