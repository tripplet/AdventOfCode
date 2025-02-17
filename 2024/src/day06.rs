use std::{
    collections::HashSet,
    fmt::{self, Debug, Display},
};

use aoc_runner_derive::{aoc, aoc_generator};
use enumflags2::{make_bitflags, BitFlags};
use glam::{ivec2, IVec2};
use itertools::iproduct;
use ndarray::{Array, Array2, Ix2};
use rayon::prelude::*;

#[derive(Clone, Copy, PartialEq)]
pub enum Tile {
    Empty,
    Wall,
    Guard,
}

#[enumflags2::bitflags]
#[derive(Debug, Default, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum Direction {
    #[default]
    NONE,
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Guard {
    pos: IVec2,
    direction: Direction,
}

#[derive(Clone)]
pub struct ParseResult {
    grid: Array2<Tile>,
    guard_pos: Guard,
}

impl Display for ParseResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..self.grid.shape()[0] {
            for x in 0..self.grid.shape()[1] {
                write!(f, "{}", self.grid[(y, x)])?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Tile::Empty => " ",
                Tile::Wall => "#",
                Tile::Guard => "x",
            }
        )
    }
}

impl Tile {
    fn from_char(c: char) -> Self {
        match c {
            '.' => Self::Empty,
            '#' => Self::Wall,
            '^' => Self::Guard,
            _ => panic!("Invalid input"),
        }
    }
}

impl Direction {
    fn vec(self) -> IVec2 {
        match self {
            Self::UP => IVec2::NEG_Y,
            Self::DOWN => IVec2::Y,
            Self::LEFT => IVec2::NEG_X,
            Self::RIGHT => IVec2::X,
            Self::NONE => panic!(),
        }
    }

    fn rotate_right(self) -> Self {
        match self {
            Self::UP => Self::RIGHT,
            Self::DOWN => Self::LEFT,
            Self::LEFT => Self::UP,
            Self::RIGHT => Self::DOWN,
            Self::NONE => panic!(),
        }
    }
}

impl Debug for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Tile::Empty => ' ',
                Tile::Wall => '#',
                Tile::Guard => 'X',
            }
        )
    }
}

impl Guard {
    #[inline]
    fn pos_vec(&self) -> (usize, usize) {
        (self.pos.y as usize, self.pos.x as usize)
    }

    fn move_next(&self, grid: &Array2<Tile>) -> Option<(Guard, bool)> {
        let next = Guard {
            pos: self.pos + self.direction.vec(),
            direction: self.direction,
        };

        if next.pos.y < 0 || next.pos.x < 0 {
            return None;
        }

        if let Some(tile) = grid.get(next.pos_vec()) {
            match tile {
                Tile::Empty => {
                    return Some((next, false));
                }
                Tile::Wall => {
                    return Some((
                        Guard {
                            pos: self.pos,
                            direction: self.direction.rotate_right(),
                        },
                        true,
                    ));
                }
                Tile::Guard => unreachable!(),
            }
        }

        None
    }
}

#[aoc_generator(day6)]
pub fn parse_input(input: &str) -> ParseResult {
    let array: Vec<Vec<Tile>> = input
        .trim()
        .lines()
        .map(|line| line.chars().map(Tile::from_char).collect())
        .collect();

    let mut grid =
        Array2::from_shape_vec((array.len(), array[0].len()), array.into_iter().flatten().collect()).unwrap();

    let guard_pos = grid.indexed_iter().find(|&(_, &tile)| tile == Tile::Guard).unwrap().0;
    grid[guard_pos] = Tile::Empty;

    ParseResult {
        grid,
        guard_pos: Guard {
            pos: ivec2(guard_pos.1 as i32, guard_pos.0 as i32),
            direction: Direction::UP,
        },
    }
}

#[aoc(day6, part1)]
pub fn part1(input: &ParseResult) -> usize {
    visited(input).iter().filter(|&tile| *tile).count()
}

fn visited(input: &ParseResult) -> Array2<bool> {
    let mut visited = Array::from_elem(input.grid.shape(), false)
        .into_dimensionality::<Ix2>()
        .unwrap();

    let mut pos = input.guard_pos;

    loop {
        let Some((next, _)) = pos.move_next(&input.grid) else {
            break;
        };

        *visited.get_mut(next.pos_vec()).unwrap() = true;
        pos = next;
    }

    visited
}

fn is_loop(lab: &ParseResult, visited: &mut Array2<BitFlags<Direction>>) -> bool {
    let mut pos = lab.guard_pos;

    loop {
        let Some((next, _)) = pos.move_next(&lab.grid) else {
            return false;
        };

        let visited_tile = visited.get_mut(next.pos_vec()).unwrap();
        if visited_tile.contains(next.direction) {
            return true;
        }

        *visited_tile |= next.direction;
        pos = next;
    }
}

fn clear_array<T>(array: &mut Array2<T>, value: T)
where
    T: Copy,
{
    array.iter_mut().for_each(|x| *x = value);
}

#[aoc(day6, part2, BruteForce)]
pub fn part2(input: &ParseResult) -> usize {
    let mut loops = 0;

    let visted_in_part1 = visited(input);

    let mut visited = Array::from_elem(input.grid.shape(), make_bitflags!(Direction::{NONE}))
        .into_dimensionality::<Ix2>()
        .unwrap();

    for (y, x) in iproduct!(0..input.grid.shape()[0], 0..input.grid.shape()[1]) {
        if input.grid[(y, x)] == Tile::Wall || (y, x) == input.guard_pos.pos_vec() || !visted_in_part1[(y, x)] {
            continue;
        }

        clear_array(&mut visited, make_bitflags!(Direction::{NONE}));

        let mut lab = input.clone();
        *lab.grid.get_mut((y, x)).unwrap() = Tile::Wall;

        if is_loop(&lab, &mut visited) {
            loops += 1;
        }
    }

    loops
}

#[aoc(day6, part2, Cleverer)]
pub fn part2_clever(input: &ParseResult) -> usize {
    let mut guard = input.guard_pos;

    let mut placed_walls = HashSet::<IVec2>::new();

    let mut visited = Array::from_elem(input.grid.shape(), make_bitflags!(Direction::{NONE}))
        .into_dimensionality::<Ix2>()
        .unwrap();

    loop {
        // Next possible position of the guard
        let Some((infront, was_already_blocked)) = guard.move_next(&input.grid) else {
            break;
        };

        if was_already_blocked {
            // Position already blocked by a wall
            guard = infront;
            continue;
        }

        if placed_walls.contains(&infront.pos) || infront.pos == input.guard_pos.pos {
            guard = infront;
            continue;
        }

        // Test this loop with the wall infront
        let mut lab = input.clone();
        lab.guard_pos = guard;
        *lab.grid.get_mut(infront.pos_vec()).unwrap() = Tile::Wall;

        clear_array(&mut visited, make_bitflags!(Direction::{NONE}));

        if is_loop(&lab, &mut visited) {
            placed_walls.insert(infront.pos);
        }

        guard = infront;
    }

    let mut dbg_lab = input.clone();
    for placed_wall in &placed_walls {
        *dbg_lab
            .grid
            .get_mut((placed_wall.y as usize, placed_wall.x as usize))
            .unwrap() = Tile::Guard;
    }

    placed_walls.len()
}

#[aoc(day6, part2, BruteForceParallel)]
pub fn part2_parallel(input: &ParseResult) -> usize {
    let visted_in_part1 = visited(input);

    iproduct!(0..input.grid.shape()[0], 0..input.grid.shape()[1])
        .collect::<Vec<_>>()
        .into_par_iter()
        .map(|(y, x)| {
            if input.grid[(y, x)] == Tile::Wall || (y, x) == input.guard_pos.pos_vec() || !visted_in_part1[(y, x)] {
                return 0;
            }

            let mut lab = input.clone();
            *lab.grid.get_mut((y, x)).unwrap() = Tile::Wall;

            let mut visited = Array::from_elem(input.grid.shape(), make_bitflags!(Direction::{NONE}))
                .into_dimensionality::<Ix2>()
                .unwrap();

            if is_loop(&lab, &mut visited) {
                return 1;
            }

            0
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../input/2024/day6_example.txt");
    const INPUT: &str = include_str!("../input/2024/day6.txt");

    #[test]
    fn example_part1() {
        let input = parse_input(EXAMPLE);
        assert_eq!(part1(&input), 41);
    }

    #[test]
    fn input_part1() {
        let input = parse_input(INPUT);
        assert_eq!(part1(&input), 4602);
    }

    #[test]
    fn example_part2() {
        let input = parse_input(EXAMPLE);
        assert_eq!(part2(&input), 6);
    }

    #[test]
    fn example_part2_cleverer() {
        let input = parse_input(EXAMPLE);
        assert_eq!(part2_clever(&input), 6);
    }

    #[test]
    fn input_part2() {
        let input = parse_input(INPUT);
        assert_eq!(part2(&input), 1703);
    }

    #[test]
    fn input_part2_cleverer() {
        let input = parse_input(INPUT);
        assert_eq!(part2_clever(&input), 1703);
    }

    #[test]
    fn input_part2_parallel() {
        let input = parse_input(INPUT);
        assert_eq!(part2_parallel(&input), 1703);
    }
}
