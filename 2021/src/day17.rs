use std::error::Error;
use std::str::FromStr;

use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug, PartialEq)]
struct TargetArea {
    x: (isize, isize),
    y: (isize, isize),
}

const INPUT: &str = include_str!("../input/2021/day17.txt");

pub fn main() {
    let mut now = std::time::Instant::now();
    let data = INPUT.parse::<TargetArea>().unwrap();
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

fn part1(target: &TargetArea) -> usize {
    42
}

fn part2(target: &TargetArea) -> usize {
    23
}

lazy_static! {
    // target area: x=60..94, y=-171..-136
    static ref REGEX_TARGET: Regex = Regex::new(r"target area:\s*x=(?P<x1>-?\d+)\s*..\s*(?P<x2>-?\d+),\s*y=(?P<y1>-?\d+)..(?P<y2>-?\d+)").unwrap();
}

impl FromStr for TargetArea {
    type Err = Box<dyn Error>;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        if let Some(values) = REGEX_TARGET.captures(input.trim()) {
            return Ok(TargetArea {
                x: (
                    values.name("x1").unwrap().as_str().parse()?,
                    values.name("x2").unwrap().as_str().parse()?,
                ),
                y: (
                    values.name("y1").unwrap().as_str().parse()?,
                    values.name("y2").unwrap().as_str().parse()?,
                ),
            });
        }
        Err("input does not match valid format".into())
    }
}

impl TargetArea {
    fn contains(&self, point: (isize, isize)) -> bool {
        point.0 >= self.x.0 && point.0 <= self.x.1 && point.1 >= self.y.0 && point.1 <= self.y.1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse() {
        assert_eq!(
            TargetArea {
                x: (20, 30),
                y: (-10, -5)
            },
            "target area: x=20..30, y=-10..-5".parse().unwrap()
        );
    }

    #[test]
    fn contains() {
        let target = TargetArea {
            x: (20, 30),
            y: (-10, -5),
        };

        assert_eq!(false, target.contains((15, -7)));
        assert_eq!(false, target.contains((32, -7)));
        assert_eq!(false, target.contains((25, -11)));
        assert_eq!(false, target.contains((25, 2)));
        assert_eq!(true, target.contains((20, -10)));
        assert_eq!(true, target.contains((30, -5)));
    }
}
