use std::{
    collections::{HashMap, HashSet},
    hash::Hash
};

use aoc_runner_derive::{aoc, aoc_generator};

type ParseResult = HashMap<Node, HashSet<Node>>;

#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Node([char; 2]);

impl Node {
    fn new(name: &str) -> Self {
        Self([name.chars().next().unwrap(), name.chars().nth(1).unwrap()])
    }

    #[inline]
    fn starts_with(self, c: char) -> bool {
        self.0[0] == c
    }
}

impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.0[0], self.0[1])
    }
}

impl std::fmt::Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.0[0], self.0[1])
    }
}

#[aoc_generator(day23)]
pub fn parse_input(input: &str) -> ParseResult {
    let mut graph = HashMap::new();

    input
        .trim()
        .lines()
        .map(|line| {
            let parts = line.trim().split_once('-').unwrap();
            (Node::new(parts.0), Node::new(parts.1))
        })
        .for_each(|entry| {
            let node = graph.entry(entry.0).or_insert(HashSet::new());
            node.insert(entry.1);

            let node = graph.entry(entry.1).or_insert(HashSet::new());
            node.insert(entry.0);
        });

    graph
}

#[aoc(day23, part1)]
pub fn part1(input: &ParseResult) -> usize {
    let mut pairs_of_three = HashSet::new();

    for node in input.keys() {
        for neighbor in &input[node] {
            input[node].intersection(&input[neighbor]).for_each(|common| {
                let mut three_pair = [node, neighbor, common];
                three_pair.sort();
                pairs_of_three.insert(three_pair);
            });
        }
    }

    pairs_of_three
        .iter()
        .filter(|pair| pair.iter().any(|node| node.starts_with('t')))
        .count()
}

#[aoc(day23, part2)]
pub fn part2(input: &ParseResult) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../input/2024/day23_example.txt");
    const INPUT: &str = include_str!("../input/2024/day23.txt");

    #[test]
    fn example_part1() {
        let input = parse_input(EXAMPLE);
        assert_eq!(part1(&input), 7);
    }

    #[test]
    fn input_part1() {
        let input = parse_input(INPUT);
        assert_eq!(part1(&input), 1184);
    }

    #[test]
    fn example_part2() {
        let input = parse_input(EXAMPLE);
        assert_eq!(part2(&input), "co,de,ka,ta");
    }

    //#[test]
    fn input_part2() {
        let input = parse_input(INPUT);
        assert_eq!(part2(&input), "");
    }
}
