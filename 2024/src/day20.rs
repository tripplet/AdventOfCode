use std::collections::{HashMap, HashSet};

use aoc_runner_derive::{aoc, aoc_generator};
use glam::{ivec2, IVec2};
use ndarray::Array2;

type ParseResult = Grid;

const DIRECTIONS: [IVec2; 4] = [IVec2::X, IVec2::NEG_X, IVec2::Y, IVec2::NEG_Y];

#[derive(Debug, Clone)]
#[allow(unused)]
pub struct Grid {
    grid: Array2<Tile>,
    start: IVec2,
    end: IVec2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tile {
    Empty,
    Wall,
    Start,
    End,
}

impl Tile {
    pub fn from_char(c: char) -> Self {
        match c {
            '.' => Self::Empty,
            '#' => Self::Wall,
            'S' => Self::Start,
            'E' => Self::End,
            _ => unreachable!(),
        }
    }
}

impl Grid {
    fn get(&self, pos: IVec2) -> Option<Tile> {
        if pos.x < 0 || pos.y < 0 || pos.x >= self.grid.ncols() as i32 || pos.y >= self.grid.nrows() as i32 {
            None
        } else {
            Some(self.grid[index(pos)])
        }
    }
}

fn index(index: IVec2) -> [usize; 2] {
    [index.y as usize, index.x as usize]
}

#[aoc_generator(day20)]
pub fn parse_input(input: &str) -> ParseResult {
    let mut start = None;
    let mut end = None;

    let array: Vec<Vec<Tile>> = input
        .trim()
        .lines()
        .map(|line| line.chars().map(Tile::from_char).collect())
        .collect();

    let grid = Array2::from_shape_vec((array.len(), array[0].len()), array.into_iter().flatten().collect()).unwrap();

    input.trim().lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| {
            if c == 'S' {
                start = Some(ivec2(x as i32, y as i32));
            } else if c == 'E' {
                end = Some(ivec2(x as i32, y as i32));
            }
        });
    });

    assert!(start.is_some());
    assert!(end.is_some());

    Grid {
        grid,
        start: start.unwrap(),
        end: end.unwrap(),
    }
}

/// Find the path and save the distance for each tile
fn find_path(input: &Grid) -> (Vec<IVec2>, Array2<u32>) {
    let mut distances = Array2::from_elem(input.grid.dim(), u32::MAX);
    distances[index(input.start)] = 0;

    let mut distance = 1;
    let mut current = input.start;

    let mut path = vec![current];

    'PATH: loop {
        for direction in DIRECTIONS {
            let next = current + direction;
            if let Some(tile) = input.get(next) {
                if distances[index(next)] != u32::MAX {
                    continue;
                }

                if tile == Tile::Empty {
                    distances[index(next)] = distance;
                    distance += 1;
                    current = next;
                    path.push(next);
                    continue 'PATH;
                } else if tile == Tile::End {
                    distances[index(next)] = distance;
                    path.push(next);
                    break 'PATH;
                }
            }
        }
    }

    (path, distances)
}

#[aoc(day20, part1)]
pub fn part1(input: &ParseResult) -> isize {
    let (path, distances) = find_path(input);
    let path_set: HashSet<IVec2> = HashSet::from_iter(path.clone());

    let cheat_distance = 2;
    let cheat_directions = DIRECTIONS.iter().map(|d| d * cheat_distance).collect::<Vec<IVec2>>();

    let mut cheated_distances = HashMap::new();

    // Go trough the whole track and try to cheat for each position
    for pos in &path {
        let current_distance = distances[index(*pos)];

        for direction in &cheat_directions {
            let next = pos + direction;

            if path_set.contains(&next) {
                let new_distance = distances[index(next)];
                if new_distance > current_distance + 2 {
                    // We skipped part of the path
                    let skipped = new_distance - current_distance - 2;
                    cheated_distances.entry(skipped).and_modify(|c| *c += 1).or_insert(1);
                }
            }
        }
    }



    // for d in cheated_distances.iter().sorted_unstable_by_key(|(&saved, _)| saved) {
    //     println!("{}: {}", d.0, d.1);
    // }

    // // Print the distance map
    // for y in 0..distances.nrows() {
    //     for x in 0..distances.ncols() {
    //         if distances[(y, x)] == u32::MAX {
    //             print!("..");
    //         } else {
    //             print!("{:<2}", distances[(y, x)]);
    //             assert!(path_set.contains(&ivec2(x as i32, y as i32)), "Path not found at {:?}", ivec2(x as i32, y as i32));
    //         }
    //     }
    //     println!();
    // }

    cheated_distances
        .iter()
        .filter(|(&cheated, _)| cheated >= 100)
        .map(|(_, &count)| count)
        .sum()
}

fn can_cheat_to(path: &[IVec2], pos: IVec2, max_distance: u32) -> impl Iterator<Item = (IVec2, u32)> + '_ {
    path.iter().copied().filter_map(move |possible_skip| {
        let distance = (possible_skip - pos).abs().element_sum() as u32;
        if distance <= max_distance {
            Some((possible_skip, distance))
        } else {
            None
        }
    })
}

#[aoc(day20, part2)]
pub fn part2(input: &ParseResult) -> isize {
    let (path, distances) = find_path(input);
    let mut cheated_distances = HashMap::new();

    // Go trough the whole track and try to cheat for each position
    for pos in &path {
        let current_distance = distances[index(*pos)];

        for (skipped_to, skipped_distance) in can_cheat_to(&path, *pos, 20) {
            let new_distance = distances[index(skipped_to)];

            if new_distance > current_distance + skipped_distance {
                // We skipped part of the path
                let skipped = new_distance - current_distance - skipped_distance;
                cheated_distances.entry(skipped).and_modify(|c| *c += 1).or_insert(1);
            }
        }
    }

    cheated_distances
        .iter()
        .filter(|(&cheated, _)| cheated >= 100)
        .map(|(_, &count)| count)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../input/2024/day20_example.txt");
    const INPUT: &str = include_str!("../input/2024/day20.txt");

    #[test]
    fn example_part1() {
        let input = parse_input(EXAMPLE);
        assert_eq!(part1(&input), 0);
    }

    #[test]
    fn input_part1() {
        let input = parse_input(INPUT);
        assert_eq!(part1(&input), 1426);
    }

    #[test]
    fn input_part2() {
        let input = parse_input(INPUT);
        assert_eq!(part2(&input), 1000697);
    }
}
