use std::{cmp::Ordering, collections::HashSet};

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

type Number = u16;

#[derive(Debug)]
pub struct ParseResult {
    rules: HashSet<(Number, Number)>,
    updates: Vec<Vec<Number>>,
}

#[aoc_generator(day5)]
pub fn parse_input(input: &str) -> ParseResult {
    let input = input.trim().replace('\r', "");
    let (rules, updates) = input.split_once("\n\n").unwrap();

    let rules = rules
        .lines()
        .map(|line| {
            line.split_once('|')
                .map(|(a, b)| (a.parse::<Number>().unwrap(), b.parse::<Number>().unwrap()))
                .unwrap()
        })
        .collect::<HashSet<_>>();

    let updates = updates
        .lines()
        .map(|line| {
            line.split(',')
                .map(|nb| nb.trim().parse::<Number>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    ParseResult { rules, updates }
}

#[aoc(day5, part1)]
pub fn part1(input: &ParseResult) -> Number {
    input
        .updates
        .iter()
        .filter(|update| is_sorted(update, &input.rules))
        .map(|update| update[update.len() / 2])
        .sum()
}

#[aoc(day5, part2)]
pub fn part2(input: &ParseResult) -> Number {
    input
        .updates
        .iter()
        .filter(|update| !is_sorted(update, &input.rules))
        .map(|update| sort(update, &input.rules))
        .sum()
}

fn is_sorted(update: &[Number], rules: &HashSet<(Number, Number)>) -> bool {
    for (a, b) in update.iter().tuple_windows() {
        if !rules.contains(&(*a, *b)) {
            return false;
        }
    }
    true
}

fn sort(update: &[Number], rules: &HashSet<(Number, Number)>) -> Number {
    let mut update = update.to_vec();

    update.sort_unstable_by(|a, b| {
        if rules.contains(&(*a, *b)) {
            Ordering::Greater
        } else {
            Ordering::Less
        }
    });

    update[update.len() / 2]
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../input/2024/day5_example.txt");
    const INPUT: &str = include_str!("../input/2024/day5.txt");

    #[test]
    fn example_part1() {
        let input = parse_input(EXAMPLE);
        assert_eq!(part1(&input), 143);
    }

    #[test]
    fn input_part1() {
        let input = parse_input(INPUT);
        assert_eq!(part1(&input), 5087);
    }

    #[test]
    fn example_part2() {
        let input = parse_input(EXAMPLE);
        assert_eq!(part2(&input), 123);
    }

    #[test]
    fn input_part2() {
        let input = parse_input(INPUT);
        assert_eq!(part2(&input), 4971);
    }
}
