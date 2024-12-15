use std::collections::{HashMap, HashSet};

use aoc_runner_derive::{aoc, aoc_generator};
use glam::{ivec2, IVec2};
use itertools::Itertools;

#[derive(Debug)]
pub struct ParseResult {
    antennas: HashMap<char, Vec<IVec2>>,
    dimensions: IVec2,
}

#[aoc_generator(day8)]
pub fn parse_input(input: &str) -> ParseResult {
    let mut result = ParseResult {
        antennas: HashMap::new(),
        dimensions: ivec2(
            input.lines().count() as i32,
            input.trim().lines().next().unwrap().len() as i32,
        ),
    };

    input
        .trim()
        .lines()
        .enumerate()
        .map(|(y, line)| (y, line.char_indices()))
        .for_each(|(y, chars)| {
            for (x, char_value) in chars.filter(|&(_, c)| c != '.') {
                let coord = ivec2(x as i32, y as i32);

                if let Some(existing) = result.antennas.get_mut(&char_value) {
                    existing.push(coord);
                } else {
                    result.antennas.insert(char_value, vec![coord]);
                }
            }
        });

    result
}

impl ParseResult {
    fn is_inside(&self, pos: IVec2) -> bool {
        pos.y >= 0 && pos.x >= 0 && pos.y < self.dimensions.y && pos.x < self.dimensions.x
    }

    fn add_if_inside(&self, pos: IVec2, dest: &mut HashSet<IVec2>) -> bool {
        if self.is_inside(pos) {
            dest.insert(pos);
            true
        } else {
            false
        }
    }
}

#[aoc(day8, part1)]
pub fn part1(input: &ParseResult) -> usize {
    let mut antinodes = HashSet::new();

    for antennas in input.antennas.values() {
        for (antenna1, antenna2) in antennas.iter().tuple_combinations() {
            let delta = *antenna1 - *antenna2;
            input.add_if_inside(*antenna1 + delta, &mut antinodes);
            input.add_if_inside(*antenna2 - delta, &mut antinodes);
        }
    }

    antinodes.len()
}

#[aoc(day8, part2)]
pub fn part2(input: &ParseResult) -> usize {
    let mut antinodes = HashSet::new();

    for antennas in input.antennas.values() {
        for (antenna1, antenna2) in antennas.iter().tuple_combinations() {
            let delta = *antenna1 - *antenna2;
            for times in 0.. {
                if !(input.add_if_inside(*antenna1 + delta * times, &mut antinodes)
                    | input.add_if_inside(*antenna1 - delta * times, &mut antinodes))
                {
                    break;
                }
            }
        }
    }

    antinodes.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../input/2024/day8_example.txt");
    const INPUT: &str = include_str!("../input/2024/day8.txt");

    #[test]
    fn example_part1() {
        let input = parse_input(EXAMPLE);
        assert_eq!(part1(&input), 14);
    }

    #[test]
    fn input_part1() {
        let input = parse_input(INPUT);
        assert_eq!(part1(&input), 409);
    }

    #[test]
    fn example_part2() {
        let input = parse_input(EXAMPLE);
        assert_eq!(part2(&input), 34);
    }

    #[test]
    fn input_part2() {
        let input = parse_input(INPUT);
        assert_eq!(part2(&input), 1308);
    }
}
