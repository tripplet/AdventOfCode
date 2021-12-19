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
        part1(&data).unwrap(),
        humantime::format_duration(now.elapsed())
    );

    now = std::time::Instant::now();
    println!(
        "Part2: {} [{}]",
        part2(&data),
        humantime::format_duration(now.elapsed())
    );
}

fn part1(target: &TargetArea) -> Option<usize> {
    // Brute force the solution
    for dy in (0..=1000).rev() {
        for dx in target.minx() as isize..((target.x.1) + 1) {
            if target.gets_hit((dy, dx as isize)) {
                return Some(TargetArea::max_value(dy as usize));
            }
        }
    }
    None
}

fn part2(target: &TargetArea) -> usize {
    // Brute force the solution
    let mut count = 0;
    for dy in ((target.y.0)..=1000).rev() {
        for dx in target.minx() as isize..((target.x.1) + 1) {
            if target.gets_hit((dy, dx as isize)) {
                count += 1
            }
        }
    }
    count
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
        point.1 >= self.x.0 && point.1 <= self.x.1 && point.0 >= self.y.0 && point.0 <= self.y.1
    }

    fn gets_hit(&self, vector: (isize, isize)) -> bool {
        let mut x = 0;
        let mut y = 0;

        let mut vector = vector;

        loop {
            if self.contains((y, x)) {
                return true;
            }

            y += vector.0;
            x += vector.1;

            vector = (vector.0 - 1, std::cmp::max(0, vector.1 - 1));

            if x > self.x.1 || y < self.y.0 {
                return false;
            }
        }
    }

    fn minx(&self) -> usize {
        (((2.0 * self.x.0 as f64 + 0.25) as f64).sqrt() - 0.5).ceil() as usize
    }

    fn max_value(delta: usize) -> usize {
        ((delta * (delta + 1)) as f64 / 2.0) as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "target area: x=20..30, y=-10..-5";

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

        assert_eq!(false, target.contains((-7, 15)));
        assert_eq!(false, target.contains((-7, 32)));
        assert_eq!(false, target.contains((-11, 25)));
        assert_eq!(false, target.contains((2, 25)));
        assert_eq!(true, target.contains((-10, 20)));
        assert_eq!(true, target.contains((-5, 30)));
    }

    #[test]
    fn check_minx() {
        let target = TargetArea {
            x: (20, 30),
            y: (-10, -5),
        };

        assert_eq!(6, target.minx());
    }

    #[test]
    fn check_maxy() {
        assert_eq!(3, TargetArea::max_value(2));
        assert_eq!(6, TargetArea::max_value(3));
    }

    #[test]
    fn check_hits() {
        let target = EXAMPLE.parse::<TargetArea>().unwrap();

        assert!(target.gets_hit((2, 7)));
        assert!(target.gets_hit((3, 6)));
        assert!(target.gets_hit((0, 9)));

        assert_eq!(false, target.gets_hit((-4, 17)));
    }

    #[test]
    fn part1_on_example() {
        assert_eq!(45, part1(&EXAMPLE.parse().unwrap()).unwrap());
    }

    #[test]
    fn part2_on_example() {
        assert_eq!(112, part2(&EXAMPLE.parse().unwrap()));
    }

    #[test]
    fn part1_on_input() {
        assert_eq!(14535, part1(&INPUT.parse().unwrap()).unwrap());
    }

    #[test]
    fn part2_on_input() {
        assert_eq!(2270, part2(&INPUT.parse().unwrap()));
    }
}
