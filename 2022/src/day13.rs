use std::fmt::{Debug, Formatter};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, line_ending, multispace0, newline},
    combinator::{map, map_res},
    multi::{separated_list0, separated_list1},
    sequence::{delimited, separated_pair, terminated, tuple},
    IResult,
};

type Number = i32;
type ParseResult = Vec<(Packet, Packet)>;

pub enum Packet {
    Number(Number),
    List(Vec<Packet>),
}

impl Packet {
    fn parse(s: &str) -> IResult<&str, Self> {
        let (s, packet) = alt((
            // Single number
            map_res(digit1, |nb: &str| nb.parse().map(Packet::Number)),
            // Or list of packets
            map(
                delimited(
                    tag("["),
                    // List of ',' separated packets (recursively)
                    separated_list0(tag(","), Packet::parse),
                    tag("]"),
                ),
                Packet::List,
            ),
        ))(s)?;

        Ok((s, packet))
    }

    fn parse_pair(s: &str) -> IResult<&str, (Self, Self)> {
        separated_pair(Packet::parse, line_ending, Packet::parse)(s)
    }
}

pub fn parse_input(input: &str) -> IResult<&str, Vec<(Packet, Packet)>> {
    separated_list1(tuple((line_ending, line_ending)), Packet::parse_pair)(input.trim())
}

impl Debug for Packet {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Packet::Number(nb) => write!(f, "{}", nb),
            Packet::List(packets) => {
                write!(f, "[")?;
                for (i, packet) in packets.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{:?}", packet)?;
                }
                write!(f, "]")
            }
        }
    }
}

pub fn part1(input: &ParseResult) -> isize {
    42
}

pub fn part2(input: &ParseResult) -> isize {
    42
}
