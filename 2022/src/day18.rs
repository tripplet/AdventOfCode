use std::{collections::HashSet, str::FromStr};

use ndarray::Array3;

use crate::utils::regex;

type Number = i8;
type ParseResult = Vec<Coordinate>;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Content {
    Empty,
    Lava,
    Water,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Coordinate {
    x: Number,
    y: Number,
    z: Number,
}

impl FromStr for Coordinate {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cap = regex!(r"(?P<x>\d+),(?P<y>\d+),(?P<z>\d+)")
            .captures(s)
            .ok_or("regex does not match")?;

        Ok(Coordinate {
            x: cap["x"].parse().unwrap(),
            y: cap["y"].parse().unwrap(),
            z: cap["z"].parse().unwrap(),
        })
    }
}

impl Coordinate {
    fn neighbors(&self) -> impl Iterator<Item = Self> + '_ {
        neighbors().map(|(dz, dy, dx)| Coordinate {
            x: (self.x as isize + dx) as Number,
            y: (self.y as isize + dy) as Number,
            z: (self.z as isize + dz) as Number,
        })
    }

    fn to_index(&self) -> (usize, usize, usize) {
        (self.z as usize, self.y as usize, self.x as usize)
    }
}

fn neighbors() -> impl Iterator<Item = (isize, isize, isize)> {
    [(0, 0, 1), (0, 0, -1), (0, 1, 0), (0, -1, 0), (1, 0, 0), (-1, 0, 0)].iter().copied()
}

pub fn parse_input(input: &str) -> ParseResult {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

pub fn part1(input: &ParseResult) -> isize {
    let mut faces = input.len() * 6;
    let coordinates: HashSet<&Coordinate> = HashSet::from_iter(input);

    for coordinate in input {
        for neighbor in coordinate.neighbors() {
            if coordinates.contains(&neighbor) {
                faces -= 1;
            }
        }
    }

    faces as isize
}

pub fn part2(input: &ParseResult) -> isize {
    let range = input.iter().fold(
        (0, 0, 0),
        |(x, y, z), coordinate| {
            (
                (x.max(coordinate.x)),
                (y.max(coordinate.y)),
                (z.max(coordinate.z)),
            )
        },
    );

    let mut space = Array3::from_elem(
        (
            (range.2 + 4) as usize,
            (range.1 + 4) as usize,
            (range.0 + 4) as usize,
        ),
        Content::Empty,
    );

    // move lava block to the center of the space to give room for sorounding water
    let input = input.iter().map(|c| {
        Coordinate {
            x: c.x + 2,
            y: c.y + 2,
            z: c.z + 2,
        }
    }).collect::<Vec<_>>();

    for coordinate in &input {
        space[coordinate.to_index()] = Content::Lava;
    }

    fill_with_water(&mut space);

    let mut faces = 0;
    for coordinate in &input {
        for neighbor in coordinate.neighbors() {
            if neighbor.x < 0 || neighbor.y < 0 || neighbor.z < 0 {
                continue;
            }

            if space.get(neighbor.to_index()) == Some(&Content::Water) {
                faces += 1;
            }
        }
    }

    faces
}

fn fill_with_water(water: &mut ndarray::ArrayBase<ndarray::OwnedRepr<Content>, ndarray::Dim<[usize; 3]>>) {
    let mut water_fill = vec![Coordinate { x: 0, y: 0, z: 0}];

    water[water_fill[0].to_index()] = Content::Water;

    while let Some(coord) = water_fill.pop() {
        for neighbor in coord.neighbors() {
            let position = neighbor.to_index();

            if water.get(position) == Some(&Content::Empty) {
                water[position] = Content::Water;
                water_fill.push(neighbor);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../input/2022/day18_example.txt");
    const INPUT: &str = include_str!("../input/2022/day18.txt");

    #[test]
    fn example_part1() {
        let input = parse_input(EXAMPLE);
        assert_eq!(part1(&input), 64);
    }

    #[test]
    fn example_part2() {
        let input = parse_input(EXAMPLE);
        assert_eq!(part2(&input), 58);
    }

    #[test]
    fn input_part1() {
        let input = parse_input(INPUT);
        assert_eq!(part1(&input), 4608);
    }

    #[test]
    fn input_part2() {
        let input = parse_input(INPUT);
        assert_eq!(part2(&input), 2652);
    }
}
