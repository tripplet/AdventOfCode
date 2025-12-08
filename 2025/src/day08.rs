use std::{
    collections::{BTreeMap, HashSet},
    str::FromStr,
};

use aoc_runner_derive::{aoc, aoc_generator};

type Number = i64;
type ParseResult = Vec<Vec3>;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Vec3 {
    x: Number,
    y: Number,
    z: Number,
}

#[aoc_generator(day8)]
pub fn parse_input(input: &str) -> ParseResult {
    input
        .trim()
        .lines()
        .map(|line| line.parse::<Vec3>().unwrap())
        .collect()
}

impl FromStr for Vec3 {
    type Err = &'static str;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let parts: Vec<Number> = input
            .split(',')
            .map(|nb| nb.trim().parse::<Number>())
            .collect::<Result<Vec<_>, _>>()
            .map_err(|_| "Failed to parse integer")?;

        if parts.len() != 3 {
            return Err("Expected three comma-separated numbers");
        }

        Ok(Vec3 {
            x: parts[0],
            y: parts[1],
            z: parts[2],
        })
    }
}

impl Vec3 {
    fn dist(&self, other: &Vec3) -> Number {
        (other.x - self.x).pow(2) + (other.y - self.y).pow(2) + (other.z - self.z).pow(2)
    }
}

#[aoc(day8, part1)]
pub fn part1(input: &ParseResult) -> usize {
    let mut circuits = vec![];

    for vec3 in input {
        circuits.push(HashSet::from([vec3]));
    }

    let mut distances = BTreeMap::new();
    let mut calulated = HashSet::new();

    for idx in 0..input.len() {
        for idy in 0..input.len() {
            if idx == idy || calulated.contains(&(idx, idy)) || calulated.contains(&(idy, idx)) {
                continue;
            }
            let dist = input[idx].dist(&input[idy]);
            distances.insert(dist, (input[idx], input[idy]));
            calulated.insert((idx, idy));
        }
    }

    for idx in 0..1000 {
        let Some(shortest) = distances.iter().nth(idx) else {
            unreachable!()
        };

        // Find set which contains them
        let first = circuits
            .iter()
            .position(|ci| ci.contains(&shortest.1.0))
            .unwrap();

        let second = circuits
            .iter()
            .position(|ci| ci.contains(&shortest.1.1))
            .unwrap();

        if first == second {
            continue;
        }

        let second_ = circuits[second].iter().cloned().collect::<Vec<_>>();

        for entry in second_ {
            circuits.get_mut(first).unwrap().insert(entry);
        }
        circuits.remove(second);
    }

    let mut sizes = circuits.iter().map(|cir| cir.len()).collect::<Vec<_>>();

    sizes.sort_unstable();

    sizes.iter().rev().take(3).product()
}

#[aoc(day8, part2)]
pub fn part2(input: &ParseResult) -> usize {
    let mut circuits = vec![];

    for vec3 in input {
        circuits.push(HashSet::from([vec3]));
    }

    let mut distances = BTreeMap::new();
    let mut calulated = HashSet::new();

    for idx in 0..input.len() {
        for idy in 0..input.len() {
            if idx == idy || calulated.contains(&(idx, idy)) || calulated.contains(&(idy, idx)) {
                continue;
            }
            let dist = input[idx].dist(&input[idy]);
            distances.insert(dist, (input[idx], input[idy]));
            calulated.insert((idx, idy));
        }
    }

    for idx in 0.. {
        let Some(shortest) = distances.iter().nth(idx) else {
            unreachable!()
        };

        // Find set which contains them
        let first = circuits
            .iter()
            .position(|ci| ci.contains(&shortest.1.0))
            .unwrap();

        let second = circuits
            .iter()
            .position(|ci| ci.contains(&shortest.1.1))
            .unwrap();

        if first == second {
            continue;
        }

        if circuits.len() == 2 {
            return shortest.1.0.x as usize * shortest.1.1.x as usize;
        }

        let second_ = circuits[second].iter().cloned().collect::<Vec<_>>();

        for entry in second_ {
            circuits.get_mut(first).unwrap().insert(entry);
        }
        circuits.remove(second);
    }

    let mut sizes = circuits.iter().map(|cir| cir.len()).collect::<Vec<_>>();

    sizes.sort_unstable();

    sizes.iter().rev().take(3).product()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../input/2025/day8_example.txt");
    const INPUT: &str = include_str!("../input/2025/day8.txt");

    #[test]
    fn example_part1() {
        let input = parse_input(EXAMPLE);
        assert_eq!(part1(&input), 40);
    }

    #[test]
    fn input_part1() {
        let input = parse_input(INPUT);
        assert_eq!(part1(&input), 0);
    }

    #[test]
    fn example_part2() {
        let input = parse_input(EXAMPLE);
        assert_eq!(part2(&input), 25272);
    }

    #[test]
    fn input_part2() {
        let input = parse_input(INPUT);
        assert_eq!(part2(&input), 1131823407);
    }
}
