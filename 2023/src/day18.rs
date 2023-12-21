use std::collections::HashSet;
use std::error::Error;

use aoc_runner_derive::{aoc, aoc_generator};
use glam::{i64vec2, ivec2, I64Vec2, IVec2};
use strum_macros::{Display, EnumString};

type ParseResult = (Vec<DigInstruction>, Vec<DigInstruction>);

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

impl From<Direction> for IVec2 {
    fn from(value: Direction) -> Self {
        match value {
            Direction::Right => ivec2(1, 0),
            Direction::Down => ivec2(0, 1),
            Direction::Left => ivec2(-1, 0),
            Direction::Up => ivec2(0, -1),
        }
    }
}

impl From<Direction> for I64Vec2 {
    fn from(value: Direction) -> Self {
        match value {
            Direction::Right => i64vec2(1, 0),
            Direction::Down => i64vec2(0, 1),
            Direction::Left => i64vec2(-1, 0),
            Direction::Up => i64vec2(0, -1),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct DigInstruction {
    direction: Direction,
    steps: u32,
}

impl DigInstruction {
    fn from_str_part1(line: &str) -> Result<Self, Box<dyn Error>> {
        let mut parts = line.split(' ');

        Ok(Self {
            direction: parts.next().ok_or("invalid direction")?.parse()?,
            steps: parts.next().ok_or("invalid steps")?.parse()?,
        })
    }

    fn from_str_part2(line: &str) -> Result<Self, Box<dyn Error>> {
        let mut parts = line.split(' ');
        let hex = parts
            .nth(2)
            .and_then(|s| s.strip_prefix("(#"))
            .and_then(|s| s.strip_suffix(')'))
            .ok_or("invalid format")?;

        if hex.len() != 6 {
            return Err(format!("Invalid hex coding {hex}").into());
        }

        Ok(Self {
            steps: u32::from_str_radix(&hex[0..5], 16)?,
            direction: match &hex[5..] {
                "0" => Direction::Right,
                "1" => Direction::Down,
                "2" => Direction::Left,
                "3" => Direction::Up,
                invalid => return Err(format!("invalid direction value {invalid}").into()),
            },
        })
    }
}

#[aoc_generator(day18)]
pub fn parse_input(input: &str) -> ParseResult {
    input
        .trim()
        .lines()
        .map(|line| line.trim())
        .map(|line| {
            Ok::<(DigInstruction, DigInstruction), Box<dyn Error>>((
                DigInstruction::from_str_part1(line)?,
                DigInstruction::from_str_part2(line)?,
            ))
        })
        .collect::<Result<Vec<_>, _>>()
        .expect("AoC inputs are always valid")
        .into_iter()
        .unzip()
}

pub fn in_fill(instructions: &[DigInstruction]) -> usize {
    let mut pos = ivec2(0, 0);
    let mut dig_site = HashSet::<IVec2>::with_capacity(instructions.len());

    let mut min = None;
    let mut max = None;

    for instr in instructions {
        for _ in 1..=instr.steps as i32 {
            pos += IVec2::from(instr.direction);
            dig_site.insert(pos);
        }

        min = Some(min.unwrap_or(pos).min(pos));
        max = Some(max.unwrap_or(pos).max(pos));
    }

    let (min, max) = (min.unwrap(), max.unwrap());

    let mut content = 0;

    for y in min.y..=max.y {
        let mut inside = false;
        let mut above_at_line_start = None;

        for x in min.x..=max.x {
            if dig_site.contains(&ivec2(x, y)) {
                //print!("#");

                // Check above and below to find horizontal lines
                let above = dig_site.contains(&ivec2(x, y - 1));
                let below = dig_site.contains(&ivec2(x, y + 1));

                if above_at_line_start.is_none() && above == below {
                    inside = !inside;
                } else {
                    // Inside a horizontal line
                    if let Some(above_at_line_start) = above_at_line_start {
                        // Inside or at the end (no longer at the start)
                        if above != below && above_at_line_start != above {
                            inside = !inside;
                        }
                    } else {
                        // Start of horizontal line => remember above
                        above_at_line_start = Some(above);
                    }
                }
            } else {
                above_at_line_start = None;

                if inside {
                    content += 1;
                    //print!("x");
                } else {
                    //print!(" ");
                }
            }
        }

        //println!()
    }

    content + dig_site.len()
}

#[aoc(day18, part1, in_fill algorithm)]
pub fn part1(input: &ParseResult) -> usize {
    in_fill(&input.0)
}

#[aoc(day18, part1, polygon algorithm)]
pub fn part1_polygon(input: &ParseResult) -> usize {
    in_fill(&input.0)
}

pub fn calc_polygon(instructions: &[DigInstruction]) -> usize {
    let edges = instructions
        .iter()
        .scan(i64vec2(0, 0), |last_edge, instr| {
            *last_edge += I64Vec2::from(instr.direction) * instr.steps as i64;
            Some(*last_edge)
        })
        .collect::<Vec<_>>();

    // Using Gauss's Area Calculation Formula
    // https://en.wikipedia.org/wiki/Shoelace_formula
    let mut sum = edges
        .windows(2)
        .map(|edge| (edge[0].y + edge[1].y) * (edge[0].x - edge[1].x))
        .sum::<i64>();

    // Last windowd which overlaps to the end of the array
    sum += (edges[edges.len() - 1].y + edges[0].y) * (edges[edges.len() - 1].x - edges[0].x);

    let length_outer_trench = instructions.iter().fold(0, |acc, instr| {
        let vector = I64Vec2::from(instr.direction).abs() * instr.steps as i64;
        acc + vector.x + vector.y
    });

    // Not sure why (+ 1) but works
    (sum + length_outer_trench) as usize / 2 + 1
}

#[aoc(day18, part2)]
pub fn part2(input: &ParseResult) -> usize {
    calc_polygon(&input.1)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../input/2023/day18_example.txt");
    const INPUT: &str = include_str!("../input/2023/day18.txt");

    #[test]
    fn example_part1() {
        let input = parse_input(EXAMPLE);
        assert_eq!(part1(&input), 62);
    }

    #[test]
    fn example_part1_polygon_algorithm() {
        let input = parse_input(EXAMPLE);
        assert_eq!(part1_polygon(&input), 62);
    }

    #[test]
    fn input_part1() {
        let input = parse_input(INPUT);
        assert_eq!(part1(&input), 33491);
    }

    #[test]
    fn input_part1_polygon_algorithm() {
        let input = parse_input(INPUT);
        assert_eq!(part1_polygon(&input), 33491);
    }

    #[test]
    fn example_part2() {
        let input = parse_input(EXAMPLE);
        assert_eq!(part2(&input), 952408144115);
    }

    #[test]
    fn input_part2() {
        let input = parse_input(INPUT);
        assert_eq!(part2(&input), 87716969654406);
    }
}
