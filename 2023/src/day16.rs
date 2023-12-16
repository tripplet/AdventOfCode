use std::collections::{HashMap, HashSet};
use std::str::FromStr;

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use ndarray::Array2;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use strum_macros::EnumString;

type Number = i16;
type ParseResult = Array2<Shape>;

const LEFT: (Number, Number) = (0, -1);
const RIGHT: (Number, Number) = (0, 1);
const DOWN: (Number, Number) = (1, 0);
const UP: (Number, Number) = (-1, 0);

#[derive(Debug, Copy, Clone, PartialEq, EnumString)]
pub enum Shape {
    #[strum(serialize = ".")]
    Empty,

    #[strum(serialize = "|")]
    SplitterVertical,

    #[strum(serialize = "-")]
    SplitterHorizontal,

    #[strum(serialize = "/")]
    MirrorDLUR,

    #[strum(serialize = r"\")]
    MirrorULDR,
}

#[aoc_generator(day16)]
pub fn parse_input(input: &str) -> ParseResult {
    let array = input
        .trim()
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| Shape::from_str(c.to_string().as_str()).unwrap())
                .collect_vec()
        })
        .collect_vec();

    Array2::from_shape_vec((array.len(), array[0].len()), array.iter().flatten().cloned().collect()).unwrap()
}

fn add_ray(
    rays: &mut HashMap<(Number, Number), HashSet<(Number, Number)>>,
    pos: (Number, Number),
    direction: (Number, Number),
) -> bool {
    if let Some(existing_rays) = rays.get_mut(&pos) {
        if existing_rays.contains(&direction) {
            return false;
        }

        existing_rays.insert(direction);
        return true;
    }

    rays.insert(pos, HashSet::from([direction]));
    true
}

fn propagate_ray(
    grid: &Array2<Shape>,
    rays: &mut HashMap<(Number, Number), HashSet<(Number, Number)>>,
    mut pos: (Number, Number),
    mut direction: (Number, Number),
) {
    loop {
        if pos.0 < 0 || pos.1 < 0 || pos.0 >= grid.nrows() as Number || pos.1 >= grid.ncols() as Number {
            return;
        }

        let cur = grid[[pos.0 as usize, pos.1 as usize]];
        if !add_ray(rays, pos, direction) {
            return;
        }

        match cur {
            Shape::Empty => {}
            Shape::MirrorDLUR => {
                direction = match direction {
                    RIGHT => UP,
                    LEFT => DOWN,
                    DOWN => LEFT,
                    UP => RIGHT,
                    _ => panic!("Should not appear"),
                };
            }
            Shape::MirrorULDR => {
                direction = match direction {
                    RIGHT => DOWN,
                    LEFT => UP,
                    DOWN => RIGHT,
                    UP => LEFT,
                    _ => panic!("Should not appear"),
                };
            }
            Shape::SplitterVertical => match direction {
                DOWN | UP => {}
                LEFT | RIGHT => {
                    propagate_ray(grid, rays, pos, UP);
                    propagate_ray(grid, rays, pos, DOWN);
                    return;
                }
                _ => panic!("Should not appear"),
            },
            Shape::SplitterHorizontal => match direction {
                LEFT | RIGHT => {}
                UP | DOWN => {
                    propagate_ray(grid, rays, pos, LEFT);
                    propagate_ray(grid, rays, pos, RIGHT);
                    return;
                }
                _ => panic!("Should not appear"),
            },
        }

        pos.0 += direction.0;
        pos.1 += direction.1;
    }
}

#[aoc(day16, part1)]
pub fn part1(grid: &ParseResult) -> usize {
    let mut rays: HashMap<(i16, i16), HashSet<(i16, i16)>> = HashMap::new();
    propagate_ray(grid, &mut rays, (0, 0), (0, 1));
    rays.keys().len()
}

#[aoc(day16, part2)]
pub fn part2(grid: &ParseResult) -> usize {
    let starts = (1..grid.nrows() - 1)
        .map(|x| vec![((0, x), DOWN), ((grid.ncols(), x), UP)])
        .chain((1..grid.ncols() - 1).map(|y| vec![((y, 0), RIGHT), ((y, grid.nrows()), LEFT)]))
        .flatten()
        .chain(vec![
            ((0, 0), RIGHT),
            ((0, 0), DOWN),
            ((grid.nrows(), 0), RIGHT),
            ((grid.nrows(), 0), UP),
            ((0, grid.ncols()), LEFT),
            ((0, grid.ncols()), DOWN),
            ((grid.nrows(), grid.ncols()), LEFT),
            ((grid.nrows(), grid.ncols()), UP),
        ])
        .collect_vec();

    starts
        .par_iter()
        .map(|start| {
            let mut rays: HashMap<(i16, i16), HashSet<(i16, i16)>> = HashMap::new();
            propagate_ray(grid, &mut rays, (start.0 .0 as Number, start.0 .1 as Number), start.1);
            rays.keys().len()
        })
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../input/2023/day16_example.txt");
    const INPUT: &str = include_str!("../input/2023/day16.txt");

    #[test]
    fn example_part1() {
        let input = parse_input(EXAMPLE);
        assert_eq!(part1(&input), 46);
    }

    #[test]
    fn input_part1() {
        let input = parse_input(INPUT);
        assert_eq!(part1(&input), 6361);
    }

    #[test]
    fn example_part2() {
        let input = parse_input(EXAMPLE);
        assert_eq!(part2(&input), 51);
    }

    #[test]
    fn input_part2() {
        let input = parse_input(INPUT);
        assert_eq!(part2(&input), 6701);
    }
}
