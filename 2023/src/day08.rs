use std::collections::HashMap;
use std::str::FromStr;

use aoc_runner_derive::{aoc, aoc_generator};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alphanumeric1, char, line_ending};
use nom::combinator::map_res;
use nom::multi::{many1, separated_list1};
use nom::sequence::{preceded, separated_pair, terminated};
use nom::IResult;
use rayon::iter::{IntoParallelRefMutIterator, ParallelIterator};

use crate::utils::ws;

type ParseResult = Network;

#[derive(Debug)]
pub struct Network {
    nav: Vec<Direction>,
    nodes: HashMap<String, (String, String)>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Direction {
    Left,
    Right,
}

impl FromStr for Direction {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "L" => Ok(Direction::Left),
            "R" => Ok(Direction::Right),
            _ => Err("Invalid char"),
        }
    }
}

#[aoc_generator(day8)]
pub fn parse_input(input: &str) -> ParseResult {
    let parser: IResult<_, _> = terminated(
        many1(map_res(alt((tag("L"), tag("R"))), Direction::from_str)),
        line_ending,
    )(input.trim());

    let (s, nav) = parser.unwrap();
    let parser: IResult<_, _> = line_ending(s);
    let (s, _) = parser.unwrap();

    let parser: IResult<_, _> = separated_list1(
        line_ending,
        separated_pair(
            alphanumeric1,
            ws(tag("=")),
            preceded(
                char('('),
                terminated(separated_pair(alphanumeric1, ws(char(',')), alphanumeric1), char(')')),
            ),
        ),
    )(s);

    let (_, node_strings) = parser.unwrap();

    let mut nodes = HashMap::with_capacity(node_strings.len());
    for node in &node_strings {
        nodes.insert(node.0.into(), (node.1 .0.into(), node.1 .1.into()));
    }

    Network { nav, nodes }
}

#[aoc(day8, part1)]
pub fn part1(input: &ParseResult) -> usize {
    let mut node_position = &String::from("AAA");
    let mut nav = input.nav.iter().cycle().enumerate();

    while node_position != "ZZZ" {
        match nav.next().unwrap().1 {
            Direction::Left => node_position = &input.nodes[node_position].0,
            Direction::Right => node_position = &input.nodes[node_position].1,
        }
    }

    nav.next().unwrap().0
}

#[inline]
fn is_start_part2<T: AsRef<str>>(node: T) -> bool {
    node.as_ref().ends_with('A')
}

#[inline]
fn is_end_part2<T: AsRef<str>>(node: T) -> bool {
    node.as_ref().ends_with('Z')
}

#[aoc(day8, part2)]
pub fn part2(input: &ParseResult) -> usize {
    // Find all start postions
    let mut positions = input.nodes.keys().filter(|&s| is_start_part2(s)).collect::<Vec<_>>();
    let get_nav = |idx: usize| input.nav[idx % input.nav.len()];

    // Find the increments where the first ..Z is found
    let increments = positions
        .par_iter_mut()
        .map(|pos| {
            let mut idx = 0;
            while !is_end_part2(*pos) {
                let cur = get_nav(idx);
                match cur {
                    Direction::Left => *pos = &input.nodes[*pos].0,
                    Direction::Right => *pos = &input.nodes[*pos].1,
                }

                idx += 1;
            }

            idx
        })
        .collect::<Vec<_>>();

    // Calculate smallest common multiple
    increments.into_iter().reduce(num::integer::lcm).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: &str = include_str!("../input/2023/day8_example1.txt");
    const EXAMPLE2: &str = include_str!("../input/2023/day8_example2.txt");
    const INPUT: &str = include_str!("../input/2023/day8.txt");

    #[test]
    fn example_part1() {
        let input = parse_input(EXAMPLE1);
        assert_eq!(part1(&input), 6);
    }

    #[test]
    fn example_part2() {
        let input = parse_input(EXAMPLE2);
        assert_eq!(part2(&input), 6);
    }

    #[test]
    fn input_part1() {
        let input = parse_input(INPUT);
        assert_eq!(part1(&input), 21797);
    }

    #[test]
    fn input_part2() {
        let input = parse_input(INPUT);
        assert_eq!(part2(&input), 23977527174353);
    }
}
