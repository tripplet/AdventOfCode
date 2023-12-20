use std::collections::{HashMap, HashSet};
use std::ops::{Add, AddAssign};
use std::str::FromStr;

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use lazy_static::lazy_static;
use ndarray::Array2;
use strum_macros::{EnumString, Display};
use tinyvec::{array_vec, ArrayVec};

type Number = i16;
type ParseResult = Input;

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Point {
    y: Number,
    x: Number,
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            y: self.y + rhs.y,
            x: self.x + rhs.x,
        }
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        self.y += rhs.y;
        self.x += rhs.x;
    }
}

#[derive(Debug, Clone)]
pub struct Input {
    start: Point,
    grid: Array2<Symbol>,
}

#[derive(Debug, Default, Display, Copy, Clone, PartialEq, Eq, Hash, EnumString)]
pub enum Symbol {
    #[default]
    #[strum(serialize = ".")]
    Ground,

    #[strum(serialize = "|")]
    VerticalPipe,

    #[strum(serialize = "-")]
    HorizontalPipe,

    #[strum(serialize = "L")]
    BendNorthEast,

    #[strum(serialize = "J")]
    BendNorthWest,

    #[strum(serialize = "7")]
    BendSouthWest,

    #[strum(serialize = "F")]
    BendSouthEast,

    #[strum(serialize = "S")]
    StartPoint,

    #[strum(serialize = "#")]
    FilledDone,

    #[strum(serialize = "o")]
    FilledInProgress,
}

#[aoc_generator(day10)]
pub fn parse_input(input: &str) -> ParseResult {
    let mut start = None;
    let array = input
        .trim()
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.trim()
                .chars()
                .enumerate()
                .map(|(x, c)| {
                    let shape = Symbol::from_str(c.to_string().as_str()).unwrap();
                    if matches!(shape, Symbol::StartPoint) {
                        start = Some(Point {
                            y: y as Number,
                            x: x as Number,
                        });
                    }
                    shape
                })
                .collect_vec()
        })
        .collect_vec();

    Input {
        grid: Array2::from_shape_vec((array.len(), array[0].len()), array.iter().flatten().cloned().collect()).unwrap(),
        start: start.unwrap(),
    }
}

#[inline(always)]
fn check(grid: &Array2<Symbol>, pos: Point, direction: Point) -> Option<(Symbol, Point)> {
    let new_pos = pos + direction;
    if new_pos.y < 0 || new_pos.x < 0 || new_pos.y >= grid.nrows() as Number || new_pos.x >= grid.ncols() as Number {
        return None;
    }

    let new_symbol = grid[[new_pos.y as usize, new_pos.x as usize]];

    if VALID_PIPE[&direction].contains(&new_symbol) {
        Some((new_symbol, new_pos))
    } else {
        None
    }
}

const UP: Point = Point { y: -1, x: 0 };
const DOWN: Point = Point { y: 1, x: 0 };
const LEFT: Point = Point { y: 0, x: -1 };
const RIGHT: Point = Point { y: 0, x: 1 };

lazy_static! {
    #[rustfmt::skip]
    static ref VALID_PIPE: HashMap<Point, [Symbol; 4]> = HashMap::from([
        (UP, [Symbol::VerticalPipe, Symbol::BendSouthEast, Symbol::BendSouthWest, Symbol::StartPoint]),
        (DOWN, [Symbol::VerticalPipe, Symbol::BendNorthEast, Symbol::BendNorthWest, Symbol::StartPoint]),
        (LEFT, [Symbol::HorizontalPipe, Symbol::BendNorthEast, Symbol::BendSouthEast, Symbol::StartPoint]),
        (RIGHT, [Symbol::HorizontalPipe, Symbol::BendNorthWest, Symbol::BendSouthWest, Symbol::StartPoint]),
    ]);
}

lazy_static! {
    #[rustfmt::skip]
    static ref VALID_MOVES: HashMap<Symbol, ArrayVec<[Point; 4]>> = HashMap::from([
        (Symbol::VerticalPipe, array_vec!([Point; 4] => UP, DOWN)),
        (Symbol::HorizontalPipe, array_vec!([Point; 4] => RIGHT, LEFT)),
        (Symbol::BendNorthEast, array_vec!([Point; 4] => UP, RIGHT)),
        (Symbol::BendNorthWest, array_vec!([Point; 4] => UP, LEFT)),
        (Symbol::BendSouthWest, array_vec!([Point; 4] => DOWN, LEFT)),
        (Symbol::BendSouthEast, array_vec!([Point; 4] => DOWN, RIGHT)),
        (Symbol::StartPoint, array_vec!([Point; 4] => UP, DOWN, LEFT, RIGHT)),
    ]);
}

fn follow_loop(input: &ParseResult, mut callback: Option<&mut dyn FnMut(&Point, &Point)>) -> usize {
    let mut prev_pos = None;
    let mut cur_pos = input.start;
    let mut cur_symbol = input.grid[[cur_pos.y as usize, cur_pos.x as usize]];
    let mut length = 0;

    'outer: loop {
        'inner: for direction in VALID_MOVES[&cur_symbol] {
            if let Some((new_symbol, new_pos)) = check(&input.grid, cur_pos, direction) {
                if let Some(prev_pos) = prev_pos {
                    if prev_pos == new_pos {
                        continue;
                    }
                }

                prev_pos = Some(cur_pos);
                cur_pos = new_pos;
                cur_symbol = new_symbol;
                length += 1;

                if cur_symbol == Symbol::StartPoint {
                    break 'outer;
                }

                if let Some(callback) = callback.as_mut() {
                    callback(&cur_pos, &direction);
                }

                break 'inner;
            }
        }
    }

    length / 2
}

#[aoc(day10, part1)]
pub fn part1(input: &ParseResult) -> usize {
    follow_loop(input, None)
}

#[aoc(day10, part2)]
pub fn part2(input: &ParseResult) -> isize {
    // Get the loop
    let mut loop_path = HashSet::new();
    let mut ff = |p: &Point, p2p: &Point| {
        loop_path.insert(*p);
    };

    _ = follow_loop(input, Some(&mut ff));

    let nrows = input.grid.nrows();
    let ncols = input.grid.ncols();

    // Create a new grid with a border around
    let mut grid = Array2::from_elem((nrows + 2, ncols + 2), Symbol::FilledInProgress);

    for y in  0..input.grid.nrows() {
        for x in 0..input.grid.ncols() {
            if !loop_path.contains(&Point {y: y as Number, x: x as Number}) {
                grid[[y + 1, x + 1]] = Symbol::Ground;
            }
            else {
                grid[[y + 1, x + 1]] = input.grid[[y, x]];
            }
        }
    }

    for y in  0..grid.nrows() {
        for x in 0..grid.ncols() {
            print!("{}", grid[[y, x]])
        }
        println!()
    }

    12
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../input/2023/day10_example.txt");
    const INPUT: &str = include_str!("../input/2023/day10.txt");

    #[test]
    fn example_part1() {
        let input = parse_input(EXAMPLE);
        assert_eq!(part1(&input), 8);
    }

    #[test]
    fn input_part1() {
        let input = parse_input(INPUT);
        assert_eq!(part1(&input), 6823);
    }

    #[test]
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
