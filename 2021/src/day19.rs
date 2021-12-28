use lazy_static::*;
use regex::Regex;
use std::{error::Error, str::FromStr};
use itertools::Itertools;
use std::collections::HashSet;

const INPUT: &str = include_str!("../input/2021/day19.txt");
const EXAMPLE: &str = include_str!("../input/2021/day19_example.txt");

pub fn main() {
    let scanners = parse_input(EXAMPLE).unwrap();
    let mut deltas: Vec<HashSet<i32>> = vec![];

    for s in scanners {
        deltas.push(HashSet::from_iter(s.point_delta().iter().cloned()));
    }

    //dbg!(&deltas);

    deltas.iter().enumerate().tuple_combinations().for_each(|(d1, d2)| {
        let common = d1.1.intersection(d2.1);

        if common.count() > 0 {
            println!("{} to {}: {} common", d1.0, d2.0, d1.1.intersection(d2.1).count());
        }
    });
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

#[derive(Debug)]
struct Scanner {
    id: u8,
    points: Vec<Point>,
}

impl FromStr for Point {
    type Err = Box<dyn Error>;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut numbers = input.split(",");

        Ok(Point {
            x: numbers.next().ok_or("missing x")?.parse()?,
            y: numbers.next().ok_or("missing y")?.parse()?,
            z: numbers.next().ok_or("missing z")?.parse()?,
        })
    }
}

impl Point {
    fn distance_to(&self, other: &Point) -> i32 {
        (other.x - self.x).pow(2) + (other.y - self.y).pow(2) + (other.z - self.z).pow(2)
    }
}

impl Scanner {
    fn point_delta(&self) -> Vec<i32> {
        self.points.iter().tuple_combinations().map(|(p1, p2)| p1.distance_to(p2)).collect()
    }
}

lazy_static! {
    static ref REGEX_SCANNER: Regex = Regex::new(r"--- scanner (?P<id>\d+) ---").unwrap();
}


fn parse_input(input: &str) -> Result<Vec<Scanner>, Box<dyn Error>> {
    let sanatized_input = input.trim().replace("\r", "");
    let sections = sanatized_input.split("\n\n");
    let mut scanners = vec![];

    for section in sections {
        let mut lines = section.lines();
        let id = REGEX_SCANNER
            .captures(lines.next().ok_or("missing header")?)
            .ok_or("invalid scanner header")?
            .name("id")
            .ok_or("missing scanner id")?
            .as_str()
            .parse()?;

        scanners.push(Scanner {
            id,
            points: lines.map(|l| l.parse()).collect::<Result<_, _>>()?,
        });
    }

    Ok(scanners)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn name() {
        unimplemented!();
    }
}
