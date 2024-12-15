use std::collections::HashSet;

use aoc_runner_derive::{aoc, aoc_generator};
use ndarray::Array2;

type ParseResult = Array2<u8>;

const NEIGHBORS: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

#[aoc_generator(day10)]
pub fn parse_input(input: &str) -> ParseResult {
    let array = input
        .trim()
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap() as u8).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    Array2::from_shape_vec((array.len(), array[0].len()), array.iter().flatten().copied().collect()).unwrap()
}

#[aoc(day10, part1)]
pub fn part1(input: &ParseResult) -> usize {
    calc(input, false)
}

fn calc(input: &ParseResult, count_paths: bool) -> usize {
    let mut result = 0;

    for start in input.indexed_iter().filter(|(_, &value)| value == 0) {
        let mut path = vec![start.0];
        let mut count = 0;
        let mut reached = HashSet::new();

        while let Some(current) = path.pop() {
            if input[current] == 9 {
                if count_paths || reached.insert(current) {
                    count += 1;
                }
                continue;
            }

            path.extend(get_next(input, current));
        }

        result += count;
    }

    result
}

fn get_next(input: &ParseResult, start: (usize, usize)) -> Vec<(usize, usize)> {
    let mut result = Vec::new();

    let next_value = input[start] + 1;

    for (dy, dx) in NEIGHBORS {
        let (ny, nx) = (start.0 as isize + dy, start.1 as isize + dx);
        if ny < 0 || ny >= input.ncols() as isize || nx < 0 || nx >= input.nrows() as isize {
            continue;
        }

        if input[[ny as usize, nx as usize]] == next_value {
            result.push((ny as usize, nx as usize));
        }
    }

    result
}

#[aoc(day10, part2)]
pub fn part2(input: &ParseResult) -> usize {
    calc(input, true)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../input/2024/day10_example.txt");
    const INPUT: &str = include_str!("../input/2024/day10.txt");

    #[test]
    fn example_part1() {
        let input = parse_input(EXAMPLE);
        assert_eq!(part1(&input), 36);
    }

    #[test]
    fn input_part1() {
        let input = parse_input(INPUT);
        assert_eq!(part1(&input), 667);
    }

    #[test]
    fn example_part2() {
        let input = parse_input(EXAMPLE);
        assert_eq!(part2(&input), 81);
    }

    #[test]
    fn input_part2() {
        let input = parse_input(INPUT);
        assert_eq!(part2(&input), 1344);
    }
}
