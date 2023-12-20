use std::{str::FromStr, error::Error, collections::HashMap};

use aoc_runner_derive::{aoc, aoc_generator};
use glam::{IVec2, ivec2};
use itertools::Itertools;
use strum_macros::{Display, EnumString};

type Number = i32;
type ParseResult = Vec<DigInstruction>;

#[derive(Debug, Display, Copy, Clone, PartialEq, Eq, EnumString)]
pub enum Direction {
    #[strum(serialize = "R")]
    Right,

    #[strum(serialize = "D")]
    Down,

    #[strum(serialize = "L")]
    Left,

    #[strum(serialize = "U")]
    Up,
}

impl Direction {
    fn delta(&self) -> IVec2 {
        match self {
            Direction::Right => ivec2(1, 0),
            Direction::Down => ivec2(0, 1),
            Direction::Left => ivec2(-1, 0),
            Direction::Up => ivec2(0, -1),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DigInstruction {
    direction: Direction,
    steps: u8,
    color: Color,
}

impl FromStr for Color {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rawhex = s
            .strip_prefix("(#")
            .ok_or("missing prefix '(#'")?
            .strip_suffix(')')
            .ok_or("Missing suffix ')'")?;

        if rawhex.len() != 6 {
            return Err("Invalid hex coding");
        }

        Ok(Self {
            red: u8::from_str_radix(&rawhex[0..2], 16).or(Err("invalid hex"))?,
            green: u8::from_str_radix(&rawhex[2..4], 16).or(Err("invalid hex"))?,
            blue: u8::from_str_radix(&rawhex[4..6], 16).or(Err("invalid hex"))?,
        })
    }
}

#[aoc_generator(day18)]
pub fn parse_input(input: &str) -> ParseResult {
    input
        .trim()
        .lines()
        .map(|line| line.trim().split(' '))
        .map(|mut parts| {
            Ok::<DigInstruction, Box<dyn Error>>(DigInstruction {
                direction: parts.next().ok_or("invalid direction")?.parse()?,
                steps: parts.next().ok_or("invalid steps")?.parse()?,
                color: parts.next().ok_or("invalid color")?.parse()?,
            })
        })
        .collect::<Result<Vec<_>,_>>().expect("AoC inputs are always valid")
}

#[aoc(day18, part1)]
pub fn part1(input: &ParseResult) -> isize {

    let mut pos = ivec2(0, 0);
    let mut dig_site = HashMap::<IVec2, &DigInstruction>::with_capacity(input.len());

    let mut min = None;
    let mut max = None;

    for instr in input {
        dig_site.insert(pos, instr);
        pos += instr.direction.delta();

        min = Some(min.unwrap_or(pos).min(pos));
        max = Some(max.unwrap_or(pos).max(pos));
    }

    let (min, max) = (min.unwrap(), max.unwrap());

    for x in min.x..=max.x {
        for y in min.y..=max.y {
            if x==0 && y==0 {
                print!("O");
                continue;
            }

            if dig_site.contains_key(&ivec2(x, y)) {
                print!("#");
            }
            else {
                print!(" ");
            }
        }

        println!()
    }


    42
}

#[aoc(day18, part2)]
pub fn part2(input: &ParseResult) -> isize {
    42
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../input/2023/day18_example.txt");
    const INPUT: &str = include_str!("../input/2023/day18.txt");

    #[test]
    fn example_part1() {
        let input = parse_input(EXAMPLE);
        assert_eq!(part1(&input), todo!());
    }

    //#[test]
    fn input_part1() {
        let input = parse_input(INPUT);
        assert_eq!(part1(&input), todo!());
    }

    //#[test]
    fn example_part2() {
        let input = parse_input(EXAMPLE);
        assert_eq!(part2(&input), todo!());
    }

    //#[test]
    fn input_part2() {
        let input = parse_input(INPUT);
        assert_eq!(part2(&input), 42);
    }
}
