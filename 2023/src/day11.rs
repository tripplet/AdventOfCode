use std::collections::HashSet;
use std::usize;

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

type ParseResult = Universe;

#[derive(Debug, Clone)]
pub struct Universe {
    dim: (usize, usize),
    coordinates: HashSet<(usize, usize)>,
}

#[aoc_generator(day11)]
pub fn parse_input(input: &str) -> ParseResult {
    let mut coordinates = HashSet::new();

    let mut nrows = 0;
    let mut ncols = 0;

    input.trim().lines().enumerate().for_each(|line| {
        for symbol in line.1.chars().enumerate() {
            if symbol.1 == '#' {
                coordinates.insert((line.0, symbol.0));
                nrows = nrows.max(line.0);
                ncols = ncols.max(symbol.0);
            }
        }
    });

    Universe {
        dim: (nrows, ncols),
        coordinates,
    }
}

fn galaxy_distance(g1: &(usize, usize), g2: &(usize, usize)) -> usize {
    g1.0.abs_diff(g2.0) + g1.1.abs_diff(g2.1)
}

fn expand_universe(universe: &mut Universe, expansion_factor: usize) {
    let mut expand_x = vec![];
    let mut expand_y = vec![];

    // Find empty column
    'x_loop: for x in 0..=universe.dim.1 {
        for y in 0..=universe.dim.0 {
            if universe.coordinates.contains(&(y, x)) {
                continue 'x_loop;
            }
        }

        expand_x.push(x);
    }

    // Find empty rows
    'y_loop: for y in 0..=universe.dim.0 {
        for x in 0..=universe.dim.1 {
            if universe.coordinates.contains(&(y, x)) {
                continue 'y_loop;
            }
        }

        expand_y.push(y);
    }

    universe.dim = (universe.dim.0 + expand_y.len(), universe.dim.1 + expand_x.len());

    // Expand the universe in the x dimension
    for x in expand_x.iter().rev() {
        let move_right = universe
            .coordinates
            .iter()
            .filter(|&galaxy| galaxy.1 > *x)
            .cloned()
            .collect_vec();

        for galaxy in &move_right {
            universe.coordinates.remove(galaxy);
        }

        for galaxy in move_right {
            universe.coordinates.insert((galaxy.0, galaxy.1 + expansion_factor - 1));
        }
    }

    // Expand the universe in the y dimension
    for y in expand_y.iter().rev() {
        let move_down = universe
            .coordinates
            .iter()
            .filter(|galaxy| galaxy.0 > *y)
            .cloned()
            .collect_vec();

        for galaxy in &move_down {
            universe.coordinates.remove(galaxy);
        }

        for galaxy in move_down {
            universe.coordinates.insert((galaxy.0 + expansion_factor - 1, galaxy.1));
        }
    }
}

fn result_distance_sum(universe: &Universe) -> usize {
    universe
        .coordinates
        .iter()
        .map(|galaxy| {
            universe
                .coordinates
                .iter()
                .filter_map(|cur| {
                    if cur == galaxy {
                        None
                    } else {
                        Some(galaxy_distance(galaxy, cur))
                    }
                })
                .sum::<usize>()
        })
        .sum::<usize>()
        / 2
}

#[aoc(day11, part1)]
pub fn part1(universe: &ParseResult) -> usize {
    let mut universe = universe.clone();

    expand_universe(&mut universe, 2);
    result_distance_sum(&universe)
}

#[aoc(day11, part2)]
pub fn part2(universe: &ParseResult) -> usize {
    let mut universe = universe.clone();
    expand_universe(&mut universe, 1000000);
    result_distance_sum(&universe)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../input/2023/day11_example.txt");
    const INPUT: &str = include_str!("../input/2023/day11.txt");

    #[test]
    fn distance() {
        let _5 = (6, 1);
        let _9 = (11, 5);
        let _1 = (0, 4);
        let _7 = (10, 9);
        let _3 = (2, 0);
        let _6 = (7, 12);
        let _8 = (11, 0);

        assert_eq!(galaxy_distance(&_5, &_9), 9);
        assert_eq!(galaxy_distance(&_1, &_7), 15);
        assert_eq!(galaxy_distance(&_3, &_6), 17);
        assert_eq!(galaxy_distance(&_8, &_9), 5);
    }

    #[test]
    fn example_part1() {
        let input = parse_input(EXAMPLE);
        assert_eq!(part1(&input), 374);
    }

    #[test]
    fn input_part1() {
        let input = parse_input(INPUT);
        assert_eq!(part1(&input), 9214785);
    }

    #[test]
    fn input_part2() {
        let input = parse_input(INPUT);
        assert_eq!(part2(&input), 613686987427);
    }
}
