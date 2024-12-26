use std::collections::{HashMap, HashSet};

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Clone)]
pub struct ParseResult {
    towls: HashSet<String>,
    designs: Vec<String>,
}

#[aoc_generator(day19)]
pub fn parse_input(input: &str) -> ParseResult {
    let input = input.trim().replace('\r', "");
    let parts = input.split_once("\n\n").unwrap();

    let towls = parts.0.split(',').map(|s| s.trim().to_string()).collect::<HashSet<_>>();
    let designs = parts.1.split('\n').map(|s| s.trim().to_string()).collect::<Vec<_>>();

    ParseResult { towls, designs }
}

fn is_design_possible(towls: &HashSet<String>, max_towl_len: usize, design: &str) -> bool {
    for cur in 1..(design.len() + 1).min(max_towl_len + 1) {
        let sub = &design[0..cur];
        if towls.contains(sub) && (design[cur..].is_empty() || is_design_possible(towls, max_towl_len, &design[cur..]))
        {
            return true;
        }
    }

    false
}

fn count_possible_combinations(
    towls: &HashSet<String>,
    max_towl_len: usize,
    design: &str,
    cache: &mut HashMap<String, usize>,
) -> usize {
    if let Some(cached_count) = cache.get(design) {
        return *cached_count;
    }

    let mut combinations = 0;
    for cur in 1..(design.len() + 1).min(max_towl_len + 1) {
        let sub = &design[0..cur];

        if towls.contains(sub) {
            if design[cur..].is_empty() {
                combinations += 1;
            } else {
                combinations += count_possible_combinations(towls, max_towl_len, &design[cur..], cache);
            }
        }
    }

    cache.insert(design.to_string(), combinations);
    combinations
}

#[aoc(day19, part1)]
pub fn part1(input: &ParseResult) -> usize {
    let max_towl_len = input.towls.iter().map(String::len).max().unwrap();
    input
        .designs
        .iter()
        .filter(|d| is_design_possible(&input.towls, max_towl_len, d))
        .count()
}

#[aoc(day19, part2)]
pub fn part2(input: &ParseResult) -> usize {
    let max_towl_len = input.towls.iter().map(String::len).max().unwrap();
    let mut cache = HashMap::new();
    input
        .designs
        .iter()
        .map(|d| count_possible_combinations(&input.towls, max_towl_len, d, &mut cache))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../input/2024/day19_example.txt");
    const INPUT: &str = include_str!("../input/2024/day19.txt");

    #[test]
    fn example_part1() {
        let input = parse_input(EXAMPLE);
        assert_eq!(part1(&input), 6);
    }

    #[test]
    fn input_part1() {
        let input = parse_input(INPUT);
        assert_eq!(part1(&input), 233);
    }

    #[test]
    fn example_part2() {
        let input = parse_input(EXAMPLE);
        assert_eq!(part2(&input), 16);
    }

    #[test]
    fn input_part2() {
        let input = parse_input(INPUT);
        assert_eq!(part2(&input), 691316989225259);
    }
}
