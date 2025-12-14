advent_of_code::solution!(9);

#[allow(unused_imports)]
use advent_of_code::prelude::*;
#[allow(unused_imports)]
use advent_of_code::{
    components::Point,
    grid::{Grid, hash_grid::HashGrid},
};

fn parse_input(input: &str) -> Vec<Point> {
    input
        .lines()
        .map(|line| {
            let (left, right) = line.split_once(",").unwrap();

            Point::new(left.parse().unwrap(), right.parse().unwrap())
        })
        .collect_vec()
}

fn lines(points: &[Point]) -> Vec<(Point, Point)> {
    let mut v = vec![];

    for i in 0..points.len() {
        let l = points[i];
        let r = points[(i + 1) % points.len()];

        v.push((l, r));
    }
    v
}

fn line_contains(line: &(Point, Point), point: &Point) -> bool {
    let x_min = line.0.x.min(line.1.x);
    let x_max = line.0.x.max(line.1.x);

    let y_min = line.0.y.min(line.1.y);
    let y_max = line.0.y.max(line.1.y);

    (x_min..=x_max).contains(&point.x) && (y_min..=y_max).contains(&point.y)
}

pub fn draw(l_bound: Point, r_bound: Point, lines: &[(Point, Point)]) {
    let grid: HashGrid<'_, ()> = HashGrid::with_bounds(l_bound, r_bound);

    let s = grid.draw(|point, _| {
        if lines.iter().any(|line| line.0 == *point) {
            return "#".into();
        }

        if lines.iter().any(|line| line_contains(line, point)) {
            return "X".into();
        }

        ".".into()
    });

    println!("{s}");
}

pub fn part_one(input: &str) -> Option<u64> {
    let input = parse_input(input);

    let mut max = u64::MIN;

    for l in 0..input.len() {
        for r in l + 1..input.len() {
            let left = input[l];
            let right = input[r];

            let area = (left - right).abs() + Point::new(1, 1);

            max = max.max((area.x.abs() * area.y.abs()) as u64);
        }
    }

    Some(max)
}

pub fn part_two(input: &str) -> Option<u64> {
    let input = parse_input(input);
    let lines = lines(&input);

    draw(Point::ZERO, Point::new(13, 8), &lines);

    let mut max = u64::MIN;
    for l in 0..input.len() {
        for r in l + 1..input.len() {
            let left = input[l];
            let right = input[r];

            // let o1 = Point::new(left.x, right.y);
            // let o2 = Point::new(right.x, left.y);

            let area = (left - right).abs() + Point::new(1, 1);
            max = max.max((area.x.abs() * area.y.abs()) as u64);
        }
    }

    Some(max)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(50));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(24));
    }
}
