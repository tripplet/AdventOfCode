use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

type Number = u32;
type ParseResult = Vec<(Number, Number)>;

#[aoc_generator(day1)]
pub fn parse_input(input: &str) -> ParseResult {
    input
        .lines()
        .map(|line| line.split_once(' ').unwrap())
        .map(|pair| (pair.0.trim().parse().unwrap(), pair.1.trim().parse().unwrap()))
        .collect()
}

#[aoc(day1, part1)]
pub fn part1(input: &ParseResult) -> Number {
    let mut list1 = vec![];
    let mut list2 = vec![];

    for pair in input {
        list1.push(pair.0);
        list2.push(pair.1);
    }

    list1.sort_unstable();
    list2.sort_unstable();

    list1
        .iter()
        .zip(list2.iter())
        .map(|pair| pair.0.abs_diff(*pair.1))
        .sum()
}

#[aoc(day1, part2)]
pub fn part2(input: &ParseResult) -> Number {
    let mut list1 = vec![];
    let mut list2 = vec![];

    for pair in input {
        list1.push(pair.0);
        list2.push(pair.1);
    }

    let mut result = 0;
    for list1_nb in list1 {
        result += list1_nb * (list2.iter().filter(|nb| **nb == list1_nb).count()) as u32;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../input/2024/day1.txt");

    #[test]
    fn input_part1() {
        assert_eq!(1941353, part1(&parse_input(INPUT)));
    }

    #[test]
    fn input_part2() {
        assert_eq!(22539317, part2(&parse_input(INPUT)));
    }
}
