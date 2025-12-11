use std::{str::FromStr, usize};

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

type Number = u16;

type ParseResult = Vec<Schematic>;

#[derive(Debug)]
struct Schematic {
    target_pattern: Vec<bool>,
    buttons: Vec<Button>,
    joltages: Vec<Number>,
}

#[derive(Debug)]
struct Button(Vec<Number>);

#[aoc_generator(day10)]
pub fn parse_input(input: &str) -> ParseResult {
    input
        .trim()
        .lines()
        .map(|line| line.parse())
        .collect::<Result<Vec<_>, _>>()
        .unwrap()
}

impl FromStr for Schematic {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.trim().split(' ');

        let target_pattern = parts.next().unwrap().trim_matches(['[', ']']);
        let target_pattern = target_pattern.chars().map(|ch| ch == '#').collect();

        let mut buttons = vec![];
        let mut joltages = None;
        for part in parts {
            if part.starts_with('(') {
                let button = Button(
                    part.trim_matches(['(', ')'])
                        .split(',')
                        .map(|nb| nb.parse())
                        .collect::<Result<Vec<_>, _>>()
                        .unwrap(),
                );

                buttons.push(button);
            } else {
                joltages = Some(
                    part.trim_matches(['{', '}'])
                        .split(',')
                        .map(|nb| nb.parse())
                        .collect::<Result<Vec<_>, _>>()
                        .unwrap(),
                );
            }
        }

        Ok(Self {
            target_pattern,
            buttons,
            joltages: joltages.unwrap(),
        })
    }
}

impl Schematic {
    fn solve(&self) -> usize {
        let mut target = 0u32;

        for bit in 0..self.target_pattern.len() {
            if self.target_pattern[bit] {
                target |= 1 << bit;
            }
        }

        let buttons = self
            .buttons
            .iter()
            .map(|btn| {
                let mut as_bit_pattern = 0u32;

                for nb in &btn.0 {
                    as_bit_pattern |= 1 << nb;
                }

                as_bit_pattern
            })
            .collect::<Vec<_>>();

        // Pressing a button more then once does not make sense
        // Find the pattern of buttons which need to be pressed
        for buttons_to_press in 1..=buttons.len() {
            for combo in (0..buttons.len())
                .combinations(buttons_to_press)
                .map(|positions| {
                    let mut arr = vec![false; buttons.len()];
                    for p in positions {
                        arr[p] = true;
                    }
                    arr
                })
            {
                let mut result = 0u32;
                for (idx, &pressed) in combo.iter().enumerate() {
                    if pressed {
                        result ^= buttons[idx];
                    }
                }

                if result == target {
                    return buttons_to_press;
                }
            }
        }

        unreachable!()
    }
}

#[aoc(day10, part1)]
pub fn part1(input: &ParseResult) -> usize {
    input
        .iter()
        .map(|schematic| schematic.solve())
        //.inspect(|nb| println!("{nb}"))
        .sum()
}

#[aoc(day10, part2)]
pub fn part2(input: &ParseResult) -> isize {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../input/2025/day10_example.txt");
    const INPUT: &str = include_str!("../input/2025/day10.txt");

    #[test]
    fn example_part1() {
        let input = parse_input(EXAMPLE);
        assert_eq!(part1(&input), 7);
    }

    #[test]
    fn input_part1() {
        let input = parse_input(INPUT);
        assert_eq!(part1(&input), 401);
    }

    #[test]
    fn example_part2() {
        let input = parse_input(EXAMPLE);
        assert_eq!(part2(&input), 33);
    }

    //#[test]
    fn input_part2() {
        let input = parse_input(INPUT);
        assert_eq!(part2(&input), todo!());
    }
}
