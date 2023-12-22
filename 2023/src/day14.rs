use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::{iproduct, Itertools};
use ndarray::Array2;

type ParseResult = Array2<Shape>;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Shape {
    RoundRock,
    CubeRock,
    Empty,
}

impl std::fmt::Display for Shape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Shape::RoundRock => write!(f, "O"),
            Shape::CubeRock => write!(f, "#"),
            Shape::Empty => write!(f, "."),
        }
    }
}

#[aoc_generator(day14)]
pub fn parse_input(input: &str) -> ParseResult {
    let array = input
        .trim()
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => Shape::Empty,
                    '#' => Shape::CubeRock,
                    'O' => Shape::RoundRock,
                    _ => panic!(),
                })
                .collect_vec()
        })
        .collect_vec();

    Array2::from_shape_vec((array.len(), array[0].len()), array.iter().flatten().cloned().collect()).unwrap()
}

#[inline(always)]
fn roll(array: &mut Array2<Shape>, direction: (isize, isize)) {
    let ncols = array.ncols();
    let nrows = array.nrows();

    loop {
        let mut move_occured = false;

        for y in 0..nrows {
            for x in 0..ncols {
                let target = (y as isize + direction.0, x as isize + direction.1);
                if target.0 < 0 || target.1 < 0 || target.0 >= nrows as isize || target.1 >= ncols as isize {
                    continue;
                }

                let target = (target.0 as usize, target.1 as usize);
                if array[target] != Shape::Empty {
                    continue;
                }

                let existing = array.get_mut([y, x]).unwrap();
                if *existing == Shape::RoundRock {
                    *existing = Shape::Empty;
                    array[target] = Shape::RoundRock;
                    move_occured = true;
                }
            }
        }

        if !move_occured {
            break;
        }
    }
}

fn calculate_load(array: &Array2<Shape>, shape: Shape) -> usize {
    let mut acc = 0;
    let nrows = array.nrows();
    let ncols = array.ncols();

    for (y, x) in iproduct!(0..nrows, 0..ncols) {
        if array[[y, x]] == shape {
            acc += nrows - y;
        }
    }

    acc
}

#[aoc(day14, part1)]
pub fn part1(input: &ParseResult) -> usize {
    let mut dish = input.clone();
    roll(&mut dish, (-1, 0));
    calculate_load(&dish, Shape::RoundRock)
}

#[inline(always)]
fn hash_dish(dish: &Array2<Shape>) -> u64 {
    let mut hasher = DefaultHasher::new();
    dish.hash(&mut hasher);
    hasher.finish()
}

#[aoc(day14, part2)]
pub fn part2(input: &ParseResult) -> usize {
    let mut dish = input.clone();
    let mut hashes = HashMap::new();
    let iterations = 1_000_000_000_usize;
    //let iterations = 30;

    let mut idx = 0;
    let mut alread_skipped_ahead = false;
    while idx < iterations {
        roll(&mut dish, (-1, 0));
        roll(&mut dish, (0, -1));
        roll(&mut dish, (1, 0));
        roll(&mut dish, (0, 1));

        idx += 1;

        if !alread_skipped_ahead {
            let hash = hash_dish(&dish);

            if let Some(v) = hashes.get(&hash) {
                let loop_start = *v;
                let loops_to_skip = (iterations - idx) / dbg!(idx - loop_start);

                // Skip ahead
                idx += loops_to_skip * (idx - loop_start);
                alread_skipped_ahead = true;
                continue;
            } else {
                hashes.insert(hash, idx);
            }
        }
    }

    calculate_load(&dish, Shape::RoundRock)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../input/2023/day14_example.txt");
    const INPUT: &str = include_str!("../input/2023/day14.txt");

    #[test]
    fn example_part1() {
        let input = parse_input(EXAMPLE);
        assert_eq!(part1(&input), 136);
    }

    #[test]
    fn input_part1() {
        let input = parse_input(INPUT);
        assert_eq!(part1(&input), 110821);
    }

    #[test]
    fn example_part2() {
        let input = parse_input(EXAMPLE);
        assert_eq!(part2(&input), 64);
    }

    #[test]
    fn input_part2() {
        let input = parse_input(INPUT);
        assert_eq!(part2(&input), 83516);
    }
}
