advent_of_code::solution!(6);
#[allow(unused_imports)]
use advent_of_code::prelude::*;

fn rotate<T: Copy>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    let len = v.first().unwrap().len();

    (0..len)
        .map(|n| v.iter().map(|l| *l.get(n).unwrap()).collect_vec())
        .collect_vec()
}

pub fn part_one(input: &str) -> Option<u64> {
    let lines = input
        .lines()
        .map(|line| line.split_whitespace().collect_vec())
        .collect_vec();

    let input = rotate(lines);

    let input = input
        .into_iter()
        .map(|mut v| {
            let symbol = v.pop().unwrap().chars().next().unwrap();

            let v = v
                .into_iter()
                .map(|v| v.parse::<u64>().unwrap())
                .collect_vec();

            (v, symbol)
        })
        .collect_vec();

    Some(
        input
            .into_iter()
            .map(|(v, op)| match op {
                '+' => v.into_iter().reduce(|a, b| a + b).unwrap(),
                '*' => v.into_iter().reduce(|a, b| a * b).unwrap(),
                _ => panic!(),
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let input = input.lines().map(|l| l.chars().collect_vec()).collect_vec();
    let input = rotate(input);

    let result = input
        .split(|v| v.iter().all(|c| *c == ' '))
        .map(|v| v.iter().map(|s| s.iter().join("")).collect_vec())
        .map(|mut v| {
            let op = v.first_mut().unwrap().pop().unwrap();
            let v = v
                .iter()
                .map(|v| v.trim().parse::<u64>().unwrap())
                .collect_vec();

            match op {
                '+' => v.into_iter().reduce(|a, b| a + b).unwrap(),
                '*' => v.into_iter().reduce(|a, b| a * b).unwrap(),
                _ => panic!(),
            }
        })
        .sum();

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4277556));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3263827));
    }
}
