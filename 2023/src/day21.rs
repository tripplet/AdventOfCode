use std::collections::HashSet;
use std::mem::swap;

use aoc_runner_derive::{aoc, aoc_generator};
use glam::{ivec2, IVec2};
use ndarray::Array2;
use strum_macros::EnumString;

type ParseResult = Input;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Input {
    start: IVec2,
    garden: Array2<Shape>,
}

#[derive(Debug, strum_macros::Display, Copy, Clone, PartialEq, Eq, EnumString)]
pub enum Shape {
    #[strum(serialize = ".")]
    Free,

    #[strum(serialize = "S")]
    Start,

    #[strum(serialize = "#")]
    Rock,
}

#[aoc_generator(day21)]
pub fn parse_input(input: &str) -> ParseResult {
    let array = input
        .trim()
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_string().parse::<Shape>())
                .collect::<Result<Vec<_>, _>>()
        })
        .collect::<Result<Vec<_>, _>>()
        .expect("AoC only has valid inputs");

    let garden =
        Array2::from_shape_vec((array.len(), array[0].len()), array.iter().flatten().cloned().collect()).unwrap();

    let start = garden
        .indexed_iter()
        .find(|(_, &shape)| shape == Shape::Start)
        .map(|((y, x), _)| ivec2(x as i32, y as i32))
        .expect("Input must have a start");

    Input { garden, start }
}

const LEFT1: IVec2 = ivec2(-1, 0);
const RIGHT1: IVec2 = ivec2(1, 0);
const DOWN1: IVec2 = ivec2(0, 1);
const UP1: IVec2 = ivec2(0, -1);

const ADJACENT: [IVec2; 4] = [UP1, DOWN1, LEFT1, RIGHT1];

fn valid_moves(pos: IVec2, garden: &Array2<Shape>) -> impl Iterator<Item = IVec2> + '_ {
    let ncols = garden.ncols() as i32;
    let nrows = garden.nrows() as i32;

    ADJACENT.iter().filter_map(move |direction| {
        let mut new_pos = (pos + *direction) % ivec2(ncols, nrows);

        if new_pos.x < 0 {
            new_pos.x += ncols;
        }

        if new_pos.y < 0 {
            new_pos.y += nrows;
        }

        if let Some(occ) = garden.get([new_pos.y as usize, new_pos.x as usize]) {
            if *occ != Shape::Rock {
                return Some(new_pos);
            }
        }
        None
    })
}

fn part1_solver(input: &ParseResult, steps: usize) -> usize {
    let mut queue = HashSet::from([input.start]);
    let mut new_queue = HashSet::new();

    for _ in 0..steps {
        for pos in queue.drain() {
            for new_pos in valid_moves(pos, &input.garden) {
                new_queue.insert(new_pos);
            }
        }

        swap(&mut queue, &mut new_queue);
    }

    queue.len()
}

#[aoc(day21, part1)]
pub fn part1(input: &ParseResult) -> usize {
    part1_solver(input, 64)
}

#[aoc(day21, part2)]
pub fn part2(_input: &ParseResult) -> usize {
    todo!()
    //part1_solver(input, 26501365)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../input/2023/day21_example.txt");
    const INPUT: &str = include_str!("../input/2023/day21.txt");

    #[test]
    fn example_part1() {
        let input = parse_input(EXAMPLE);
        assert_eq!(part1_solver(&input, 6), 16);
    }

    #[test]
    fn input_part1() {
        let input = parse_input(INPUT);
        assert_eq!(part1(&input), 3733);
    }

    //#[test]
    fn example_part2() {
        let input = parse_input(EXAMPLE);
        assert_eq!(part2(&input), todo!());
    }

    //#[test]
    fn input_part2() {
        let input = parse_input(INPUT);
        assert_eq!(part2(&input), 42);
    }
}
