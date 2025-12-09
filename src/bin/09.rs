advent_of_code::solution!(9);

use std::{cmp::Ordering, net::ToSocketAddrs, ops::Bound};

#[allow(unused_imports)]
use advent_of_code::prelude::*;
use advent_of_code::{
    components::Point,
    grid::{Grid, hash_grid::HashGrid},
};
use itertools::Position;

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
    let mut direction = Ordering::Greater;

    for i in 0..points.len() {
        let l = points[i];
        let r = points[(i + 1) % points.len()];

        if l.x == r.x {
            let min = l.y.min(r.y);
            let mut max = l.y.max(r.y);

            let new_direction = l.y.cmp(&r.y);

            if direction == new_direction {
                max -= 1;
            }

            direction = new_direction;

            let line = (Point::new(l.x, min), Point::new(l.x, max));
            v.push(line);
        } else {
            v.push((l, r));
        }
    }
    v
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

fn abs_contains(from: isize, to: isize, y: isize) -> bool {
    let min = from.min(to);
    let max = from.max(to);
    (min..=max).contains(&y)
}

fn is_inside(vertical: &[&(Point, Point)], point: Point) -> bool {
    let mut crosses = 0;
    for line in vertical {
        // horizontal line
        if line.0.y == line.1.y {
            // println!("{line:?}");
            if line.0.y == point.y && abs_contains(line.0.x, line.1.x, point.x) {
                return true;
            } else {
                continue;
            }
        }

        if line.0.x == point.x && abs_contains(line.0.y, line.1.y, point.y) {
            return true;
        }

        if line.0.x > point.x {
            continue;
        }

        if abs_contains(line.0.y, line.1.y, point.y) {
            crosses += 1;
        }
    }

    crosses % 2 == 1
}

pub fn part_two(input: &str) -> Option<u64> {
    let input = parse_input(input);
    let lines = lines(&input);
    let vertical = lines.iter().collect_vec();

    let mut c = HashGrid::with_bounds(Point::new(0, 0), Point::new(13, 8));
    c.set(&Point::UP, 2);
    let f = c.draw(|p, c| {
        // if p == &Point::new(2, 5) {
        //     return "?".to_string();
        // }

        if lines.iter().any(|line| {
            line.0.x == p.x && abs_contains(line.0.y, line.1.y, p.y)
            // || line.0.y == p.y && abs_contains(line.0.x, line.1.x, p.x)
        }) {
            "#".to_string()
        } else if lines.iter().any(|line| {
            // line.0.x == p.x && abs_contains(line.0.y, line.1.y, p.y)
            line.0.y == p.y && abs_contains(line.0.x, line.1.x, p.x)
        }) {
            "O".to_string()
        } else {
            ".".to_string()
        }
    });
    println!("{f}");
    panic!();

    let mut max = u64::MIN;
    for l in 0..input.len() {
        for r in l + 1..input.len() {
            let left = input[l];
            let right = input[r];

            let o1 = Point::new(left.x, right.y);
            let o2 = Point::new(right.x, left.y);

            if !is_inside(&vertical, o1) || !is_inside(&vertical, o2) {
                continue;
            }

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
