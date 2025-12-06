use aoc_runner_derive::{aoc, aoc_generator};

use crate::utils::NEIGHBORS_2D;

type Number = u32;
type ParseResult = Vec<Vec<bool>>;

const MAX_OCCUPIED_NEIGHBORS: Number = 4;

#[aoc_generator(day4)]
pub fn parse_input(input: &str) -> ParseResult {
    input
        .trim()
        .lines()
        .map(|line| line.trim().chars().map(|ch| ch == '@').collect::<Vec<_>>())
        .collect::<Vec<_>>()
}

#[aoc(day4, part1)]
pub fn part1(input: &ParseResult) -> Number {
    let mut moveable = 0;

    for y in 0..input.len() {
        for x in 0..input[y].len() {
            if input[y][x] == true
                && count_occupied_neighbors(input, (y, x)) < MAX_OCCUPIED_NEIGHBORS
            {
                moveable += 1;
            }
        }
    }

    moveable
}

#[aoc(day4, part2)]
pub fn part2(input: &ParseResult) -> Number {
    let mut grid = input.clone();
    let mut to_be_removed = vec![];
    let initial_occupied = count_occupied(&grid);

    loop {
        for y in 0..grid.len() {
            for x in 0..grid[y].len() {
                if grid[y][x] == true
                    && count_occupied_neighbors(&grid, (y, x)) < MAX_OCCUPIED_NEIGHBORS
                {
                    to_be_removed.push((y, x));
                }
            }
        }

        if to_be_removed.is_empty() {
            break;
        }

        for &(y, x) in &to_be_removed {
            *grid.get_mut(y).unwrap().get_mut(x).unwrap() = false;
        }

        to_be_removed.clear();
    }

    initial_occupied - count_occupied(&grid)
}

fn get<T>(array: &[T], index: i32) -> Option<&T> {
    if index < 0 || index >= array.len() as i32 {
        None
    } else {
        Some(&array[index as usize])
    }
}

fn count_occupied_neighbors(grid: &ParseResult, (y, x): (usize, usize)) -> Number {
    let mut occupied_neighbors = 0;

    for neighbor in NEIGHBORS_2D {
        if let Some(cell) =
            get(grid, y as i32 + neighbor.0).and_then(|row| get(row, x as i32 + neighbor.1))
            && *cell == true
        {
            occupied_neighbors += 1;
        }
    }
    occupied_neighbors
}

fn count_occupied(input: &ParseResult) -> Number {
    input
        .iter()
        .map(|row| row.iter().filter(|&cell| *cell == true).count())
        .sum::<usize>() as Number
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../input/2025/day4_example.txt");
    const INPUT: &str = include_str!("../input/2025/day4.txt");

    #[test]
    fn example_part1() {
        let input = parse_input(EXAMPLE);
        assert_eq!(part1(&input), 13);
    }

    #[test]
    fn input_part1() {
        let input = parse_input(INPUT);
        assert_eq!(part1(&input), 1564);
    }

    #[test]
    fn example_part2() {
        let input = parse_input(EXAMPLE);
        assert_eq!(part2(&input), 43);
    }

    #[test]
    fn input_part2() {
        let input = parse_input(INPUT);
        assert_eq!(part2(&input), 9401);
    }
}
