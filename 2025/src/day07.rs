use std::collections::{HashMap, HashSet};

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug)]
pub struct ParseResult {
    start: usize,
    grid: Vec<Vec<bool>>,
}

#[aoc_generator(day7)]
pub fn parse_input(input: &str) -> ParseResult {
    let mut start = 0;

    let grid = input
        .trim()
        .lines()
        .map(|line| {
            line.trim()
                .chars()
                .enumerate()
                .map(|(x, ch)| match ch {
                    '^' => true,
                    'S' => {
                        start = x;
                        false
                    }
                    _ => false,
                })
                .collect()
        })
        .collect();

    ParseResult { start, grid }
}

#[aoc(day7, part1)]
pub fn part1(input: &ParseResult) -> usize {
    let mut beams = HashSet::from([input.start]);
    let mut beam_splited = 0;

    for y in (2..input.grid[0].len()).step_by(2) {
        // Project beams down
        let beams_cur = beams.clone();
        beams.clear();

        for beam_x in beams_cur {
            if input.grid[y][beam_x] {
                beams.insert(beam_x - 1);
                beams.insert(beam_x + 1);
                beam_splited += 1;
            } else {
                beams.insert(beam_x);
            }
        }
    }

    beam_splited
}

#[aoc(day7, part2)]
pub fn part2(input: &ParseResult) -> usize {
    let mut cache = HashMap::new();
    track_beam(2, input.start, &input.grid, &mut cache)
}

fn track_beam(
    y: usize,
    x: usize,
    grid: &Vec<Vec<bool>>,
    cache: &mut HashMap<(usize, usize), usize>,
) -> usize {
    if y >= grid.len() {
        return 1;
    }

    if let Some(cached_result) = cache.get(&(y, x)) {
        return *cached_result;
    }

    let result = if grid[y][x] {
        track_beam(y + 2, x - 1, grid, cache) + track_beam(y + 2, x + 1, grid, cache)
    } else {
        track_beam(y + 2, x, grid, cache)
    };

    cache.insert((y, x), result);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../input/2025/day7_example.txt");
    const INPUT: &str = include_str!("../input/2025/day7.txt");

    #[test]
    fn example_part1() {
        let input = parse_input(EXAMPLE);
        assert_eq!(part1(&input), 21);
    }

    #[test]
    fn input_part1() {
        let input = parse_input(INPUT);
        assert_eq!(part1(&input), 1675);
    }

    #[test]
    fn example_part2() {
        let input = parse_input(EXAMPLE);
        assert_eq!(part2(&input), 40);
    }

    #[test]
    fn input_part2() {
        let input = parse_input(INPUT);
        assert_eq!(part2(&input), 187987920774390);
    }
}
