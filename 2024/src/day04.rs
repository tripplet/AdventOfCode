use aoc_runner_derive::{aoc, aoc_generator};

use ndarray::Array2;

type ParseResult = Array2<char>;

const DIRECTIONS: &[(isize, isize)] = &[(0, 1), (0, -1), (1, 0), (-1, 0), (1, 1), (-1, -1), (1, -1), (-1, 1)];

#[aoc_generator(day4)]
pub fn parse_input(input: &str) -> ParseResult {
    let array = input
        .trim()
        .lines()
        .map(|line| line.trim().chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    Array2::from_shape_vec((array.len(), array[0].len()), array.iter().flatten().copied().collect()).unwrap()
}

#[aoc(day4, part1)]
pub fn part1(input: &ParseResult) -> isize {
    let xmas = "XMAS".chars().collect::<Vec<_>>();

    let mut count = 0;

    for y in 0..input.shape()[0] {
        for x in 0..input.shape()[1] {
            'DIRECTIONS: for dir in DIRECTIONS {
                for (char_pos, xmas_char) in xmas.iter().enumerate() {
                    if let Some(pos) = get_pos(y, x, char_pos, dir) {
                        if let Some(map_char) = input.get(pos) {
                            if *map_char != *xmas_char {
                                continue 'DIRECTIONS;
                            }
                        } else {
                            continue 'DIRECTIONS;
                        }
                    } else {
                        continue 'DIRECTIONS;
                    }
                }

                count += 1;
            }
        }
    }

    count
}

fn get_pos(y: usize, x: usize, char_pos: usize, dir: &(isize, isize)) -> Option<(usize, usize)> {
    let y = y as isize + (char_pos as isize * dir.0);
    if y < 0 {
        return None;
    }

    let x = x as isize + (char_pos as isize * dir.1);
    if x < 0 {
        return None;
    }

    Some((y as usize, x as usize))
}

#[aoc(day4, part2)]
pub fn part2(input: &ParseResult) -> isize {
    let mut count = 0;

    for y in 1..input.shape()[0] - 1 {
        for x in 1..input.shape()[1] - 1 {
            // Check the X-MAX pattern
            if *input.get((y, x)).unwrap() != 'A' {
                continue;
            }

            let corners = [
                *input.get((y - 1, x - 1)).unwrap(),
                *input.get((y - 1, x + 1)).unwrap(),
                *input.get((y + 1, x + 1)).unwrap(),
                *input.get((y + 1, x - 1)).unwrap(),
            ]
            .iter()
            .collect::<String>();

            if corners == "MMSS" || corners == "SSMM" || corners == "MSSM" || corners == "SMMS" {
                count += 1;
            }
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../input/2024/day4_example.txt");
    const INPUT: &str = include_str!("../input/2024/day4.txt");

    #[test]
    fn example_part1() {
        let input = parse_input(EXAMPLE);
        assert_eq!(part1(&input), 18);
    }

    #[test]
    fn input_part1() {
        let input = parse_input(INPUT);
        assert_eq!(part1(&input), 2603);
    }

    #[test]
    fn example_part2() {
        let input = parse_input(EXAMPLE);
        assert_eq!(part2(&input), 9);
    }

    #[test]
    fn input_part2() {
        let input = parse_input(INPUT);
        assert_eq!(part2(&input), 1965);
    }
}
