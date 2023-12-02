use std::collections::HashMap;

use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{char, multispace1, u32};
use nom::combinator::map;
use nom::multi::separated_list1;
use nom::sequence::{preceded, separated_pair, terminated};

use lazy_static::lazy_static;
use nom::IResult;

use crate::utils::ws;

type Number = u32;
type ParseResult = Vec<Game>;

#[derive(Debug)]
pub struct Game {
    number: Number,
    reveals: Vec<HashMap<Color, Number>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Color {
    Red,
    Green,
    Blue,
}

impl Game {
    fn parse(input: &str) -> IResult<&str, Self> {
        let (s, number) = preceded(ws(tag("Game")), terminated(u32, ws(char(':'))))(input)?;
        let (s, reveals) = separated_list1(
            ws(char(';')),
            separated_list1(
                ws(char(',')),
                separated_pair(
                    u32,
                    multispace1,
                    map(alt((tag("red"), tag("green"), tag("blue"))), |color| match color {
                        "red" => Color::Red,
                        "green" => Color::Green,
                        "blue" => Color::Blue,
                        _ => unreachable!(),
                    }),
                ),
            ),
        )(s)?;

        let reveals = reveals
            .iter()
            .map(|pairs| {
                let mut hash_map = HashMap::with_capacity(3);

                for pair in pairs.iter() {
                    hash_map.insert(pair.1, pair.0);
                }
                hash_map
            })
            .collect();

        Ok((s, Self { number, reveals }))
    }
}

lazy_static! {
    static ref VALID_PART1: HashMap<Color, Number> =
        HashMap::from([(Color::Red, 12), (Color::Green, 13), (Color::Blue, 14)]);
}

const ALL_COLORS: [Color; 3] = [Color::Red, Color::Green, Color::Blue];

#[aoc_generator(day2)]
pub fn parse_input(input: &str) -> ParseResult {
    input.lines().map(|line| Game::parse(line).unwrap().1).collect()
}

#[aoc(day2, part1)]
pub fn part1(input: &ParseResult) -> Number {
    input
        .iter()
        .filter(|&game| {
            game.reveals.iter().all(|game| {
                ALL_COLORS.iter().all(|color|
                    game.get(&color)
                    .map(|&b| b <= VALID_PART1[&color])
                    .unwrap_or(true)
                )
            })
        })
        .map(|game| game.number)
        .sum::<Number>()
}

#[aoc(day2, part2)]
pub fn part2(input: &ParseResult) -> Number {
    input
        .iter()
        .map(|game| {
            let mut max = HashMap::from([(Color::Red, 1), (Color::Green, 1), (Color::Blue, 1)]);

            for reveal in game.reveals.iter() {
                for color in ALL_COLORS.iter() {
                    *max.get_mut(color).unwrap() = std::cmp::max(max[color], *reveal.get(color).unwrap_or(&1));
                }
            }

            max.values().product::<Number>()
        })
        .sum::<Number>()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../input/2023/day2_example.txt");
    const INPUT: &str = include_str!("../input/2023/day2.txt");

    #[test]
    fn example_part1() {
        let input = parse_input(EXAMPLE);
        assert_eq!(part1(&input), 8);
    }

    #[test]
    fn example_part2() {
        let input = parse_input(EXAMPLE);
        assert_eq!(part2(&input), 2286);
    }

    #[test]
    fn input_part1() {
        let input = parse_input(INPUT);
        assert_eq!(part1(&input), 2505);
    }

    #[test]
    fn input_part2() {
        let input = parse_input(INPUT);
        assert_eq!(part2(&input), 70265);
    }
}
