use std::cmp::Ordering;
use std::fmt::{Debug, Formatter};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, line_ending},
    combinator::{map, map_res},
    multi::{separated_list0, separated_list1},
    sequence::{delimited, separated_pair, tuple},
    IResult,
};

type Number = i32;
type ParseResult<'a> = Vec<(Packet, Packet)>;

#[derive(Clone, Eq)]
pub enum Packet {
    Number(Number),
    List(Vec<Packet>),
}

impl Packet {
    fn parse(s: &str) -> IResult<&str, Self> {
        alt((
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
        ))(s)
    }
}

pub fn parse_input(input: &str) -> Vec<(Packet, Packet)> {
    let parse_pair = separated_pair(Packet::parse, line_ending, Packet::parse);

    separated_list1(tuple((line_ending, line_ending)), parse_pair)(input.trim()).unwrap().1
}

impl PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Packet::Number(nb1), Packet::Number(nb2)) => nb1 == nb2,
            (Packet::List(packets1), Packet::List(packets2)) => {
                for (packet1, packet2) in packets1.iter().zip(packets2) {
                    if !packet1.eq(packet2) {
                        return false;
                    }
                }
                true
            }
            (Packet::Number(_), Packet::List(_)) => Packet::List(vec![self.clone()]).eq(other),
            (Packet::List(_), Packet::Number(_)) => self.eq(&Packet::List(vec![other.clone()])),
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Packet::Number(nb1), Packet::Number(nb2)) => nb1.partial_cmp(nb2),
            (Packet::List(packets1), Packet::List(packets2)) => {
                for idx in 0..packets1.len() {
                    if let Some(p2) = packets2.get(idx) {
                        if let Some(ordering) = packets1[idx].partial_cmp(p2) {
                            if ordering != Ordering::Equal {
                                return Some(ordering);
                            }
                        } else {
                            return Some(Ordering::Less);
                        }
                    } else {
                        return Some(Ordering::Greater);
                    }
                }

                if packets1.len() < packets2.len() {
                    return Some(Ordering::Less);
                }

                Some(Ordering::Equal)
            }
            (Packet::Number(_), Packet::List(_)) => Packet::List(vec![self.clone()]).partial_cmp(other),
            (Packet::List(_), Packet::Number(_)) => self.partial_cmp(&Packet::List(vec![other.clone()])),
        }
    }
}

impl Debug for Packet {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Packet::Number(nb) => write!(f, "{}", nb),
            Packet::List(packets) => {
                let strings = packets
                    .iter()
                    .map(|p| format!("{:?}", p))
                    .collect::<Vec<String>>()
                    .join(", ");
                write!(f, "[{}]", strings)
            }
        }
    }
}

pub fn part1(pairs: &ParseResult) -> usize {
    pairs
        .iter()
        .enumerate()
        .filter_map(|(idx, (p1, p2))| if p1 < p2 { Some(idx + 1) } else { None })
        .sum()
}

pub fn part2(pairs: &ParseResult) -> usize {
    let mut packets = Vec::with_capacity(pairs.len() * 2 + 2);

    let driver1 = Packet::parse("[[2]]").unwrap().1;
    let driver2 = Packet::parse("[[6]]").unwrap().1;

    packets.push((&driver1, true));
    packets.push((&driver2, true));

    for (p1, p2) in pairs {
        packets.push((p1, false));
        packets.push((p2, false));
    }

    packets.sort_unstable_by(|(p1, _), (p2, _)| p1.partial_cmp(p2).unwrap());

    packets
        .iter()
        .enumerate()
        .filter_map(|(idx, &(_, is_driver))| if is_driver { Some(idx + 1) } else { None })
        .product()
}
