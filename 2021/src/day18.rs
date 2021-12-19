use std::{error::Error, str::FromStr};

#[derive(Debug, PartialEq)]
enum SnailfishNumber {
    Number(u8),
    Pair {
        left: Box<SnailfishNumber>,
        right: Box<SnailfishNumber>,
    },
}

const INPUT: &str = include_str!("../input/2021/day18.txt");

pub fn main() {
    let mut now = std::time::Instant::now();
    let data = parse_input(INPUT).unwrap();
    println!("Parsing [{}]\n", humantime::format_duration(now.elapsed()));

    now = std::time::Instant::now();
    println!(
        "Part1: {} [{}]",
        part1(&data),
        humantime::format_duration(now.elapsed())
    );

    now = std::time::Instant::now();
    println!(
        "Part2: {} [{}]",
        part2(&data),
        humantime::format_duration(now.elapsed())
    );
}

fn parse_input(input: &str) -> Result<Vec<SnailfishNumber>, Box<dyn Error>> {
    input.trim().lines().map(SnailfishNumber::from_str).collect()
}

fn part1(target: &[SnailfishNumber]) -> usize {
    42
}

fn part2(target: &[SnailfishNumber]) -> usize {
    23
}

impl FromStr for SnailfishNumber {
    type Err = Box<dyn Error>;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        SnailfishNumber::from_chars(&mut input.chars())
    }
}

impl SnailfishNumber {
    fn from_chars(chars: &mut dyn std::iter::Iterator<Item = char>) -> Result<SnailfishNumber, Box<dyn Error>> {
        if let Some(n) = chars.next() {
            if n == '[' {
                let left = SnailfishNumber::from_chars(chars)?;
                chars.next(); // skip ','
                let right = SnailfishNumber::from_chars(chars)?;
                chars.next(); // skip ']'
                return Ok(SnailfishNumber::Pair {
                    left: left.into(),
                    right: right.into(),
                });
            } else if n.is_digit(10) {
                return Ok(SnailfishNumber::Number(n.to_digit(10).unwrap() as u8));
            }
        }

        Err("invalid input".into())
    }

    fn add(left: SnailfishNumber, right: SnailfishNumber) -> SnailfishNumber {
        SnailfishNumber::Pair {
            left: left.into(),
            right: right.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse() {
        let expected = SnailfishNumber::Pair {
            left: SnailfishNumber::Number(1).into(),
            right: SnailfishNumber::Number(9).into(),
        };
        assert_eq!(expected, "[1,9]".parse::<SnailfishNumber>().unwrap());

        let expected = SnailfishNumber::Pair {
            left: SnailfishNumber::Pair {
                left: SnailfishNumber::Pair {
                    left: SnailfishNumber::Pair {
                        left: SnailfishNumber::Number(7).into(),
                        right: SnailfishNumber::Number(3).into(),
                    }
                    .into(),
                    right: SnailfishNumber::Number(1).into(),
                }
                .into(),
                right: SnailfishNumber::Pair {
                    left: SnailfishNumber::Pair {
                        left: SnailfishNumber::Number(4).into(),
                        right: SnailfishNumber::Number(6).into(),
                    }
                    .into(),
                    right: SnailfishNumber::Pair {
                        left: SnailfishNumber::Number(5).into(),
                        right: SnailfishNumber::Number(1).into(),
                    }
                    .into(),
                }
                .into(),
            }
            .into(),
            right: SnailfishNumber::Pair {
                left: SnailfishNumber::Pair {
                    left: SnailfishNumber::Pair {
                        left: SnailfishNumber::Number(4).into(),
                        right: SnailfishNumber::Number(7).into(),
                    }
                    .into(),
                    right: SnailfishNumber::Number(4).into(),
                }
                .into(),
                right: SnailfishNumber::Pair {
                    left: SnailfishNumber::Pair {
                        left: SnailfishNumber::Number(5).into(),
                        right: SnailfishNumber::Number(2).into(),
                    }
                    .into(),
                    right: SnailfishNumber::Pair {
                        left: SnailfishNumber::Number(3).into(),
                        right: SnailfishNumber::Number(7).into(),
                    }
                    .into(),
                }
                .into(),
            }
            .into(),
        };

        assert_eq!(
            expected,
            "[[[[7,3],1],[[4,6],[5,1]]],[[[4,7],4],[[5,2],[3,7]]]]"
                .parse::<SnailfishNumber>()
                .unwrap()
        );
    }
    #[test]
    fn add() {
        let expected = SnailfishNumber::Pair {
            left: SnailfishNumber::Pair {
                left: SnailfishNumber::Pair {
                    left: SnailfishNumber::Number(1).into(),
                    right: SnailfishNumber::Number(2).into(),
                }
                .into(),
                right: SnailfishNumber::Number(4).into(),
            }
            .into(),
            right: SnailfishNumber::Pair {
                left: SnailfishNumber::Number(7).into(),
                right: SnailfishNumber::Number(3).into(),
            }
            .into(),
        };

        assert_eq!(
            expected,
            SnailfishNumber::add("[[1,2],4]".parse().unwrap(), "[7,3]".parse().unwrap())
        );
    }

    #[test]
    fn check_input() {
        for (idx, line) in INPUT.trim().lines().enumerate() {
            assert!(
                line.parse::<SnailfishNumber>().is_ok(),
                "Error parsing line {}",
                idx + 1
            );
        }
    }
}
