use aoc_runner_derive::{aoc, aoc_generator};
use itertools::{iproduct, Itertools};
use ndarray::{Array2, Axis};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

type Number = u16;
type ParseResult = Vec<Array2<bool>>;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum ColRow {
    Col(Number),
    Row(Number),
}

#[aoc_generator(day13)]
#[rustfmt::ignore]
pub fn parse_input(input: &str) -> ParseResult {
    input
        .trim()
        .replace("\r", "")
        .split("\n\n")
        .map(|puzzle| {
            let array = puzzle
                .lines()
                .map(|line| {
                    line.chars()
                        .map(|c| match c {
                            '.' => false,
                            '#' => true,
                            _ => panic!(),
                        })
                        .collect_vec()
                })
                .collect_vec();

            Array2::from_shape_vec((array.len(), array[0].len()), array.iter().flatten().cloned().collect()).unwrap()
        })
        .collect_vec()
}

fn get_next(split: Number, iteration: Number, max: Number) -> Option<(Number, Number)> {
    if iteration >= split || split + iteration + 1 > max {
        None
    } else {
        Some((split - iteration - 1, split + iteration))
    }
}

fn find_lines(array: &Array2<bool>, axis: usize) -> Vec<Number> {
    // Go over all columns
    let max = array.len_of(Axis(axis)) as Number;
    let mut splits = vec![];

    'try_lines: for split_idx in 1..max {
        // Go over all pairs to compare
        for iter_idx in (0 as Number)..max {
            if let Some(split) = get_next(split_idx, iter_idx, max) {
                if array.index_axis(Axis(axis), split.0 as usize) != array.index_axis(Axis(axis), split.1 as usize) {
                    // Columns do not mirror on the `split_idx` => try next
                    continue 'try_lines;
                }
            } else {
                break;
            }
        }

        splits.push(split_idx);
    }

    return splits;
}

fn solve_part1(array: &Array2<bool>) -> Option<Vec<ColRow>> {
    let columns = find_lines(array, 1).into_iter().map(|c| ColRow::Col(c));
    let rows = find_lines(array, 0).into_iter().map(|r| ColRow::Row(r));

    let found = columns.chain(rows).collect::<Vec<_>>();

    if found.len() == 0 {
        None
    } else {
        Some(found)
    }
}

#[aoc(day13, part1)]
pub fn part1(input: &ParseResult) -> u32 {
    input
        .iter()
        .map(solve_part1)
        .map(|ref res| match res.as_ref().map(|r| r.as_slice()) {
            Some([ColRow::Col(c)]) => *c as u32,
            Some([ColRow::Row(r)]) => (*r * 100) as u32,
            _ => unreachable!("Input data is design so this does not happen"),
        })
        .sum()
}

#[aoc(day13, part2)]
pub fn part2(input: &ParseResult) -> u32 {
    input
        .iter()
        //.par_iter()
        .map(|p| (p, solve_part1(p)))
        .map(|res| {
            let mut nn = res.0.clone();
            let mut last_pos: Option<(_, _)> = None;

            for (y, x) in iproduct!(0..res.0.nrows(), 0..res.0.ncols()) {
                nn[[y, x]] = !nn[[y, x]];

                if let Some(last_pos) = last_pos {
                    nn[[last_pos.0, last_pos.1]] = !nn[[last_pos.0, last_pos.1]];
                }

                last_pos = Some((y, x));

                let new_res = solve_part1(&nn);
                if new_res.is_none() {
                    continue;
                }

                let old = res.1.as_ref().unwrap()[0];

                if new_res.as_ref().unwrap() != res.1.as_ref().unwrap() {
                    // print!(
                    //     "Found alternative mirror with smudge at (y:{y}, x:{x}), new: {:?}, old: {:?}",
                    //     new_res, old
                    // );

                    if let Some(new) = new_res.unwrap().iter().find(|&e| *e != old) {
                        return *new;
                    }
                }
            }

            unreachable!("There must always be one alternative")
        })
        .map(|res| match res {
            ColRow::Col(c) => c as u32,
            ColRow::Row(r) => (r * 100) as u32,
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../input/2023/day13_example.txt");
    const INPUT: &str = include_str!("../input/2023/day13.txt");

    #[test]
    fn split_calc() {
        assert_eq!(get_next(1, 0, 100), Some((0, 1)));
        assert_eq!(get_next(1, 1, 100), None);

        assert_eq!(get_next(2, 0, 100), Some((1, 2)));
        assert_eq!(get_next(2, 1, 100), Some((0, 3)));
        assert_eq!(get_next(2, 2, 100), None);

        assert_eq!(get_next(3, 0, 5), Some((2, 3)));
        assert_eq!(get_next(3, 1, 5), Some((1, 4)));
        assert_eq!(get_next(3, 2, 5), None);
    }

    #[test]
    fn example_part1() {
        let input = parse_input(EXAMPLE);
        assert_eq!(part1(&input), 405);
    }

    #[test]
    fn input_part1() {
        let input = parse_input(INPUT);
        assert_eq!(part1(&input), 30802);
    }

    #[test]
    fn example_part2() {
        let input = parse_input(EXAMPLE);
        assert_eq!(part2(&input), 400);
    }

    #[test]
    fn input_part2() {
        let input = parse_input(INPUT);
        assert_eq!(part2(&input), 37876);
    }
}
