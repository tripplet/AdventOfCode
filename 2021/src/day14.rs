use std::error::Error;
use std::str::FromStr;
use itertools::Itertools;
use std::collections::HashMap;

const INPUT: &str = include_str!("../input/2021/day14.txt");
const EXAMPLE: &str = include_str!("../input/2021/day14_example.txt");

#[derive(Debug, PartialEq, Eq, Clone)]
struct Polymer {
    polymer: Vec<char>,
    rules: HashMap<(char, char), char>,
}

impl FromStr for Polymer {
    type Err = Box<dyn Error>;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let lines = input.trim().replace("\r", "");
        let mut parts = lines.split("\n\n");

        let polymer: Vec<_> = parts.next().ok_or("missing start polymer")?.chars().collect();
        let mut rules: HashMap<(char, char), char> = HashMap::new();

        for rule in parts.next().ok_or("missing insertion rules")?.lines() {
            let mut parts = rule.split(" -> ");
            let mut in_chars = parts.next().ok_or("missing inputs")?.chars();
            let a = in_chars.next().ok_or("missing char1")?;
            let b = in_chars.next().ok_or("missing char2")?;

            rules.insert((a, b), parts.next().ok_or("missing second part of insertion rule")?.chars().next().ok_or("missing char")?);
        }

        Ok(Polymer{polymer, rules})
    }
}

impl Polymer {
    fn polymerization(&mut self) {
        let new = self.polymer.windows(2).map(|x| self.rules[&(x[0], x[1])]).collect::<Vec<_>>();

        for (idx, n) in new.iter().enumerate() {
            self.polymer.insert((idx * 2) + 1 , *n)
        }
    }
}

pub fn main() {
    let polymer = INPUT.parse::<Polymer>().unwrap();

    println!("Part1: {}", part1(&polymer));
    //println!("Part2:\n{}", part2(&polymer));
}

fn part1(polymer: &Polymer) -> usize {
    let mut polymer = polymer.clone();

    for _ in 0..10 {
        polymer.polymerization();
    }

    let counts = polymer.polymer.iter().counts();
    let least = counts.values().min().unwrap();
    let most = counts.values().max().unwrap();

    most - least
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../input/2021/day14_example.txt");

    #[test]
    fn part1_example() {
        assert_eq!(1588, part1(&EXAMPLE.parse().unwrap()));
    }

    #[test]
    fn part1_on_input() {
        assert_eq!(2010, part1(&INPUT.parse().unwrap()));
    }
}
