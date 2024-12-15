use aoc_runner_derive::{aoc, aoc_generator};
use ndarray::Array2;

type ParseResult = Array2<char>;

const NEIGHBORS: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

#[aoc_generator(day12)]
pub fn parse_input(input: &str) -> ParseResult {
    let array = input
        .trim()
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    Array2::from_shape_vec((array.len(), array[0].len()), array.iter().flatten().copied().collect()).unwrap()
}

#[aoc(day12, part1)]
pub fn part1(input: &ParseResult) -> usize {
    let sections = find_sections(input);
    let mut result = 0;

    for section in sections {
        result += calculate_perimeter(&section, input) * section.len();
    }

    result
}

fn find_sections(input: &ParseResult) -> Vec<Vec<(usize, usize)>> {
    let mut filled = Array2::from_elem((input.ncols(), input.nrows()), false);

    let mut sections = vec![];

    for ((y, x), &start_char) in input.indexed_iter() {
        if filled[[y, x]] {
            continue;
        }

        let mut new_section = vec![(y, x)];
        let mut to_check = vec![(y, x)];

        while let Some((to_check_y, to_check_x)) = to_check.pop() {
            filled[[y, x]] = true;

            for (dy, dx) in NEIGHBORS {
                let new_y = to_check_y as isize + dy;
                let new_x = to_check_x as isize + dx;

                if new_y >= 0 && new_y < input.nrows() as isize && new_x >= 0 && new_x < input.ncols() as isize {
                    let new_char = input[[new_y as usize, new_x as usize]];
                    if new_char == start_char && !filled[[new_y as usize, new_x as usize]] {
                        filled[[new_y as usize, new_x as usize]] = true;
                        to_check.push((new_y as usize, new_x as usize));
                        new_section.push((new_y as usize, new_x as usize));
                    }
                }
            }
        }

        sections.push(new_section);
    }

    sections
}

fn calculate_perimeter(section: &[(usize, usize)], input: &ParseResult) -> usize {
    let section_char = input[section[0]];
    let mut result = 0;

    for (y, x) in section {
        for (dy, dx) in NEIGHBORS {
            let new_y = *y as isize + dy;
            let new_x = *x as isize + dx;

            if new_y < 0 || new_y >= input.nrows() as isize || new_x < 0 || new_x >= input.ncols() as isize {
                result += 1;
                continue;
            }

            if input[[new_y as usize, new_x as usize]] != section_char {
                result += 1;
            }
        }
    }

    result
}

fn find_perimeter_coords(section: &[(usize, usize)], input: &ParseResult) -> Vec<(usize, usize)> {
    let section_char = input[section[0]];
    let mut perimeter_coords = vec![];

    for (y, x) in section {
        for (dy, dx) in NEIGHBORS {
            let new_y = *y as isize + dy;
            let new_x = *x as isize + dx;

            if new_y < 0 || new_y >= input.nrows() as isize || new_x < 0 || new_x >= input.ncols() as isize {
                perimeter_coords.push((new_y as usize, new_x as usize));
                continue;
            }

            if input[[new_y as usize, new_x as usize]] != section_char {
                perimeter_coords.push((new_y as usize, new_x as usize));
            }
        }
    }

    perimeter_coords
}

#[aoc(day12, part2)]
pub fn part2(input: &ParseResult) -> usize {
    let sections = find_sections(input);
    let mut result = 0;

    for section in sections {
        let perimeter_coords = find_perimeter_coords(&section, input);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = include_str!("../input/2024/day12_example1.txt");
    const EXAMPLE_2: &str = include_str!("../input/2024/day12_example2.txt");
    const INPUT: &str = include_str!("../input/2024/day12.txt");

    #[test]
    fn example1_part1() {
        let input = parse_input(EXAMPLE_1);
        assert_eq!(part1(&input), 140);
    }

    #[test]
    fn example2_part1() {
        let input = parse_input(EXAMPLE_2);
        assert_eq!(part1(&input), 1930);
    }

    #[test]
    fn input_part1() {
        let input = parse_input(INPUT);
        assert_eq!(part1(&input), 123);
    }

    //#[test]
    fn example1_part2() {
        let input = parse_input(EXAMPLE_2);
        assert_eq!(part2(&input), 80);
    }

    //#[test]
    fn input_part2() {
        let input = parse_input(INPUT);
        assert_eq!(part2(&input), todo!());
    }
}
