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

fn mm_range(lhs: isize, rhs: isize) -> std::ops::RangeInclusive<isize> {
    let min = lhs.min(rhs);
    let max = lhs.max(rhs);

    min..=max
}

fn line_contains(line: &(Point, Point), point: &Point) -> bool {
    let x_min = line.0.x.min(line.1.x);
    let x_max = line.0.x.max(line.1.x);

    let y_min = line.0.y.min(line.1.y);
    let y_max = line.0.y.max(line.1.y);

    (x_min..=x_max).contains(&point.x) && (y_min..=y_max).contains(&point.y)
}

pub fn draw(l_bound: Point, r_bound: Point, lines: &[(Point, Point)], points: &[Point]) {
    let grid: HashGrid<'_, ()> = HashGrid::with_bounds(l_bound, r_bound);

    let s = grid.draw(|point, _| {
        if points.contains(point) {
            return "O".into();
        }

        if lines
            .iter()
            .any(|line| line.0 == *point || line.1 == *point)
        {
            return "#".into();
        }

        if lines.iter().any(|line| line_contains(line, point)) {
            return "X".into();
        }

        ".".into()
    });

    println!("{s}");
}

fn into_draw_mode(
    mut l_bound: Point,
    mut r_bound: Point,
    lines: &[(Point, Point)],
    points: &[Point],
) {
    let stdin = std::io::stdin();

    loop {
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        draw(l_bound, r_bound, lines, points);

        let mut line = String::new();
        stdin.read_line(&mut line).unwrap();

        for c in line.chars() {
            match c {
                'h' => {
                    l_bound += Point::LEFT;
                    r_bound += Point::LEFT;
                }
                'j' => {
                    l_bound += Point::DOWN;
                    r_bound += Point::DOWN;
                }
                'k' => {
                    l_bound += Point::UP;
                    r_bound += Point::UP;
                }
                'l' => {
                    l_bound += Point::RIGHT;
                    r_bound += Point::RIGHT;
                }
                _ => {}
            }
        }
    }
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

fn is_vertical(line: &(Point, Point)) -> bool {
    line.0.x == line.1.x
}

fn is_horizontal(line: &(Point, Point)) -> bool {
    line.0.y == line.1.y
}

fn is_horizontal_intersection(lhs: &(Point, Point), rhs: &(Point, Point)) -> bool {
    assert!(is_horizontal(lhs));
    assert!(is_vertical(rhs));

    if !mm_range(lhs.0.x, lhs.1.x).contains(&rhs.0.x) {
        return false;
    }

    if !mm_range(rhs.0.y, rhs.1.y).contains(&lhs.0.y) {
        return false;
    }

    true
}

fn is_vertical_intersection(lhs: &(Point, Point), rhs: &(Point, Point)) -> bool {
    assert!(is_vertical(lhs));
    assert!(is_horizontal(rhs));

    if !mm_range(lhs.0.y, lhs.1.y).contains(&rhs.0.y) {
        return false;
    }

    if !mm_range(rhs.0.x, rhs.1.x).contains(&lhs.0.x) {
        return false;
    }

    true
}

fn is_valid(
    horizontal: &[(Point, Point)],
    vertical: &[(Point, Point)],
    square: (Point, Point),
) -> bool {
    let (mut left, mut right) = square;
    left += (right - left).normal();
    right += (left - right).normal();
    let o1 = Point::new(left.x, right.y);
    let o2 = Point::new(right.x, left.y);

    if horizontal.iter().any(|line| {
        is_horizontal_intersection(line, &(o2, right))
            || is_horizontal_intersection(line, &(left, o1))
    }) {
        return false;
    }

    if vertical.iter().any(|line| {
        is_vertical_intersection(line, &(left, o2)) || is_vertical_intersection(line, &(o1, right))
    }) {
        return false;
    }

    let ray = (Point::new(0, left.y), left + Point::LEFT);
    if vertical
        .iter()
        .filter(|line| is_vertical_intersection(line, &ray))
        .count()
        % 2
        == 0
    {
        return false;
    }

    true
}

pub fn part_two(input: &str) -> Option<u64> {
    let input = parse_input(input);
    let lines = lines(&input);
    let horizontal = lines.iter().cloned().filter(is_horizontal).collect_vec();
    let vertical = lines.iter().cloned().filter(is_vertical).collect_vec();

    draw(Point::ZERO, Point::new(13, 8), &lines, &[]);

    let mut max = u64::MIN;
    let mut max_points = (Point::ZERO, Point::ZERO);

    for l in 0..input.len() {
        for r in l + 1..input.len() {
            let left = input[l];
            let right = input[r];

            if !is_valid(&horizontal, &vertical, (left, right)) {
                continue;
            }

            let area = (left - right).abs() + Point::new(1, 1);
            let area = (area.x.abs() * area.y.abs()) as u64;
            if area > max {
                max = area;
                max_points = (left, right);
            }
        }
    }

    let (mut left, mut right) = (Point::new(9, 5), Point::new(2, 3));
    let ray = (Point::new(0, left.y), left + Point::LEFT);
    left += (right - left).normal();
    right += (left - right).normal();
    let o1 = Point::new(left.x, right.y);
    let o2 = Point::new(right.x, left.y);

    println!();
    let mut intersections = vertical
        .iter()
        .filter(|line| is_vertical_intersection(line, &ray))
        .cloned()
        .collect_vec();

    intersections.push(ray);

    draw(
        Point::ZERO,
        Point::new(13, 8),
        &intersections,
        &[left, right, o1, o2],
    );
    // into_draw_mode(
    //     Point::ZERO,
    //     Point::new(13, 8),
    //     &lines,
    //     &[max_points.0, max_points.1],
    // );

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
