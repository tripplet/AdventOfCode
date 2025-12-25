use aoc_runner_derive::{aoc, aoc_generator};

use crate::utils::Vec2;

type Number = i32;

#[derive(Debug)]
pub struct ParseResult {
    shapes: Vec<Shape>,
    regions: Vec<Region>,
}

#[derive(Debug)]
struct Shape {
    size: Vec2<usize>,
}

#[derive(Debug)]
struct Region {
    size: Vec2<usize>,
    quantities: Vec<Number>,
}

#[aoc_generator(day12)]
pub fn parse_input(input: &str) -> ParseResult {
    let input = input.replace('\r', "");
    let mut parts = input.trim().split("\n\n").peekable();

    let mut shapes = vec![];
    let mut regions = vec![];

    loop {
        let part = parts.peek().unwrap();
        if part.contains('x') {
            break;
        }

        let mut shape: Vec<Vec<_>> = vec![];
        for line in parts.next().unwrap().lines().skip(1) {
            shape.push(line.chars().map(|ch| ch == '#').collect());
        }
        shapes.push(Shape {
            size: Vec2 {
                y: shape.len(),
                x: shape[0].len(),
            },
        });
    }

    for part in parts.next().unwrap().lines() {
        let (size, quantities) = part.split_once(':').unwrap();
        let size = size.trim().split_once('x').unwrap();

        regions.push(Region {
            size: Vec2 {
                y: size.1.parse().unwrap(),
                x: size.0.parse().unwrap(),
            },
            quantities: quantities
                .trim()
                .split_ascii_whitespace()
                .map(|nb| nb.parse().unwrap())
                .collect(),
        });
    }

    ParseResult { shapes, regions }
}

#[aoc(day12, part1)]
pub fn part1(input: &ParseResult) -> usize {
    let mut fiting_regions = 0;

    // Easy solution based on area, after reading reddit
    // Not neccessary to to full area packing like example suggests
    for region in &input.regions {
        let required_area = region
            .quantities
            .iter()
            .enumerate()
            .map(|(shape, &amount)| {
                input.shapes[shape].size.y * input.shapes[shape].size.x * amount as usize
            })
            .sum::<usize>();

        if required_area <= (region.size.y * region.size.x) {
            fiting_regions += 1;
        }
    }

    fiting_regions
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../input/2025/day12.txt");

    #[test]
    fn input_part1() {
        let input = parse_input(INPUT);
        assert_eq!(part1(&input), 555);
    }
}
