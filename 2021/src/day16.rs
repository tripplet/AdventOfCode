use std::str::FromStr;

use bitvec::prelude::*;
use itertools::Itertools;
use num::FromPrimitive;
use num_derive::FromPrimitive;

const INPUT: &str = include_str!("../input/2021/day16.txt");

pub fn main() {
    let mut now = std::time::Instant::now();
    let data = INPUT.parse::<Packet>().unwrap();
    println!("Parsing [{}]\n", humantime::format_duration(now.elapsed()));

    now = std::time::Instant::now();
    println!("Part1: {} [{}]", part1(&data), humantime::format_duration(now.elapsed()));

    now = std::time::Instant::now();
    println!("Part2: {} [{}]", part2(&data), humantime::format_duration(now.elapsed()));
}

#[derive(Debug, PartialEq, Eq)]
enum Packet {
    Literal {
        version: u8,
        value: u64,
    },
    Operator {
        version: u8,
        packet_type: PacketType,
        subpackets: Vec<Packet>,
    },
}

#[derive(Debug, PartialEq, Eq, FromPrimitive)]
#[repr(u8)]
enum PacketType {
    Sum = 0,
    Product,
    Min,
    Max,
    Literal,
    GreaterThen,
    LessThen,
    EqualTo,
}

impl FromStr for Packet {
    type Err = Box<dyn std::error::Error>;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let bytes = input
            .trim()
            .chars()
            .tuples()
            .map(|hex: (char, char)| {
                u8::from_str_radix([hex.0, hex.1].iter().collect::<String>().as_str(), 16).unwrap()
            })
            .collect::<Vec<u8>>();

        let bits = bytes.view_bits::<Msb0>();
        Packet::from_bits(bits).map(|r| r.0)
    }
}

impl Packet {
    fn from_bits(bits: &BitSlice<bitvec::order::Msb0, u8>) -> Result<(Self, usize), Box<dyn std::error::Error>> {
        let version = bits[0..3].load_be::<u8>();
        let packet_type = PacketType::from_u8(bits[3..6].load_be::<u8>()).ok_or("invalid type")?;

        match packet_type {
            PacketType::Literal => {
                let mut pos = 6;
                let mut value_bits = bitvec![];
                loop {
                    value_bits.insert(0, bits[pos + 1]);
                    value_bits.insert(0, bits[pos + 2]);
                    value_bits.insert(0, bits[pos + 3]);
                    value_bits.insert(0, bits[pos + 4]);

                    if !bits[pos] {
                        break;
                    }
                    pos += 5;
                }

                Ok((
                    Packet::Literal {
                        version,
                        value: value_bits.load_be::<u64>(),
                    },
                    5 + pos,
                ))
            }
            _ => match bits[6] {
                false => {
                    let dst = bits![mut Msb0, u16; 0; 15];
                    dst.clone_from_bitslice(&bits[7..22]);
                    let sub_packet_length = dst.load_be::<u16>();

                    let mut pos = 22;
                    let mut read = 0;
                    let mut subpackets = vec![];
                    loop {
                        let (new_subpacket, pos2) =
                            Packet::from_bits(&bits[pos..pos + (sub_packet_length as usize - read)])?;
                        subpackets.push(new_subpacket);
                        pos += pos2;
                        read += pos2;
                        if pos >= 22 + sub_packet_length as usize {
                            break;
                        }
                    }
                    Ok((
                        Packet::Operator {
                            version,
                            packet_type,
                            subpackets,
                        },
                        pos,
                    ))
                }
                true => {
                    let dst = bits![mut Msb0, u16; 0; 11];
                    dst.clone_from_bitslice(&bits[7..18]);
                    let sub_packet_count = dst.load_be::<u16>();

                    let mut pos = 18;
                    let mut subpackets = vec![];
                    for _ in 0..sub_packet_count {
                        let (new_subpacket, pos2) = Packet::from_bits(&bits[pos..])?;
                        subpackets.push(new_subpacket);
                        pos += pos2
                    }

                    Ok((
                        Packet::Operator {
                            version,
                            packet_type,
                            subpackets,
                        },
                        pos,
                    ))
                }
            },
        }
    }

    fn get_value(&self) -> usize {
        match self {
            Packet::Literal { value, .. } => *value as usize,
            Packet::Operator {
                packet_type,
                subpackets,
                ..
            } => match packet_type {
                PacketType::Sum => subpackets.iter().map(Packet::get_value).sum::<usize>() as usize,
                PacketType::Product => subpackets.iter().map(Packet::get_value).product::<usize>() as usize,
                PacketType::Min => subpackets.iter().map(Packet::get_value).min().unwrap() as usize,
                PacketType::Max => subpackets.iter().map(Packet::get_value).max().unwrap() as usize,
                PacketType::GreaterThen => {
                    if subpackets[0].get_value() > subpackets[1].get_value() {
                        1
                    } else {
                        0
                    }
                }
                PacketType::LessThen => {
                    if subpackets[0].get_value() < subpackets[1].get_value() {
                        1
                    } else {
                        0
                    }
                }
                PacketType::EqualTo => {
                    if subpackets[0].get_value() == subpackets[1].get_value() {
                        1
                    } else {
                        0
                    }
                }
                PacketType::Literal => panic!(),
            },
        }
    }
}

fn part1(packet: &Packet) -> usize {
    match packet {
        Packet::Literal { version, .. } => *version as usize,
        Packet::Operator {
            version, subpackets, ..
        } => *version as usize + subpackets.iter().map(part1).sum::<usize>() as usize,
    }
}

fn part2(packet: &Packet) -> usize {
    packet.get_value()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_example_1() {
        assert_eq!(
            Packet::Literal {
                version: 6,
                value: 2021
            },
            "D2FE28".parse().unwrap()
        );
    }

    #[test]
    fn parse_example_2() {
        let expected = Packet::Operator {
            version: 1,
            packet_type: PacketType::LessThen,
            subpackets: vec![
                Packet::Literal { version: 6, value: 10 },
                Packet::Literal { version: 2, value: 20 },
            ],
        };
        assert_eq!(expected, "38006F45291200".parse().unwrap());
    }

    #[test]
    fn parse_example_3() {
        let expected = Packet::Operator {
            version: 7,
            packet_type: PacketType::Max,
            subpackets: vec![
                Packet::Literal { version: 2, value: 1 },
                Packet::Literal { version: 4, value: 2 },
                Packet::Literal { version: 1, value: 3 },
            ],
        };
        assert_eq!(expected, "EE00D40C823060".parse().unwrap());
    }

    #[test]
    fn part1_example_1() {
        dbg!("8A004A801A8002F478".parse::<Packet>().unwrap());
        assert_eq!(16, part1(&"8A004A801A8002F478".parse().unwrap()));
    }

    #[test]
    fn part2_example_1() {
        assert_eq!(3, part2(&"C200B40A82".parse().unwrap()));
        assert_eq!(54, part2(&"04005AC33890".parse().unwrap()));
        assert_eq!(7, part2(&"880086C3E88112".parse().unwrap()));
        assert_eq!(9, part2(&"CE00C43D881120".parse().unwrap()));

        assert_eq!(1, part2(&"D8005AC2A8F0".parse().unwrap()));
        assert_eq!(0, part2(&"F600BC2D8F".parse().unwrap()));
        assert_eq!(0, part2(&"9C005AC2F8F0".parse().unwrap()));
        assert_eq!(1, part2(&"9C0141080250320F1802104A08".parse().unwrap()));
    }

    #[test]
    fn part1_on_input() {
        assert_eq!(1007, part1(&INPUT.parse().unwrap()));
    }

    #[test]
    fn part2_on_input() {
        assert_eq!(834151779165, part2(&INPUT.parse().unwrap()));
    }
}
