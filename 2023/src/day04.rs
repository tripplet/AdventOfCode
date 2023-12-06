use std::collections::HashSet;

use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;

use nom::bytes::complete::tag;
use nom::character::complete::{char, line_ending, space1, u16};
use nom::multi::{separated_list0, separated_list1};
use nom::sequence::{preceded, separated_pair, terminated};
use nom::IResult;

use crate::utils::ws;

type ParseResult = Vec<Card>;

#[derive(Debug, Clone)]
pub struct Card {
    number: u16,
    winning_numbers: Vec<u16>,
    own_numbers: Vec<u16>,
}

#[aoc_generator(day4)]
pub fn parse_input(input: &str) -> ParseResult {
    separated_list1(line_ending, Card::parse)(input.trim())
        .expect("aoc has only valid input data")
        .1
}

impl Card {
    fn parse(s: &str) -> IResult<&str, Self> {
        let (s, number) = preceded(ws(tag("Card")), terminated(u16, ws(char(':'))))(s)?;
        let (s, (winning_numbers, own_numbers)) = separated_pair(
            separated_list0(space1, u16),
            ws(char('|')),
            separated_list0(space1, u16),
        )(s)?;

        Ok((
            s,
            Self {
                number,
                winning_numbers,
                own_numbers,
            },
        ))
    }

    fn wins(&self) -> usize {
        let win = HashSet::<_>::from_iter(self.winning_numbers.iter());
        let own = HashSet::<_>::from_iter(self.own_numbers.iter());

        win.intersection(&own).count()
    }
}

#[aoc(day4, part1)]
pub fn part1(input: &ParseResult) -> u32 {
    input
        .iter()
        .filter_map(|card| {
            let wins = card.wins();

            if wins > 0 {
                Some(2_u32.pow(wins as u32 - 1))
            } else {
                None
            }
        })
        .sum()
}

#[aoc(day4, part2)]
pub fn part2(input: &ParseResult) -> u32 {
    let mut card_count = vec![1; input.len()];

    for card in input {
        let wins = card.wins();

        for pos in (card.number + 1)..(card.number + 1 + wins as u16) {
            card_count[pos as usize - 1] += card_count[card.number as usize - 1];
        }
    }

    card_count.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../input/2023/day4_example.txt");
    const INPUT: &str = include_str!("../input/2023/day4.txt");

    #[test]
    fn example_part1() {
        let input = parse_input(EXAMPLE);
        assert_eq!(part1(&input), 13);
    }

    #[test]
    fn example_part2() {
        let input = parse_input(EXAMPLE);
        assert_eq!(part2(&input), 30);
    }

    #[test]
    fn input_part1() {
        let input = parse_input(INPUT);
        assert_eq!(part1(&input), 21105);
    }

    #[test]
    fn input_part2() {
        let input = parse_input(INPUT);
        assert_eq!(part2(&input), 5329815);
    }
}
