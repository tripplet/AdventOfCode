use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};
use ndarray::Array2;

type Neighbor = (isize, isize);
type ParseResult = Array2<char>;

const NEIGHBORS: [Neighbor; 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

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

#[aoc(day12, part2)]
pub fn part2(input: &ParseResult) -> usize {
    let sections = find_sections(input);
    let mut result = 0;

    for section in sections {
        let perimeter_coords = find_perimeter_coords(&section, input);

        result += section.len() * find_sides(&perimeter_coords);
    }

    result
}

#[derive(Debug)]
struct FrequencyMap<T>(HashMap<T, usize>);

impl<T: Eq + std::hash::Hash> FromIterator<T> for FrequencyMap<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut map = HashMap::new();
        for item in iter {
            *map.entry(item).or_insert(0) += 1;
        }

        Self(map)
    }
}

impl<T> FrequencyMap<T> {
    fn take(&mut self, key: &T) -> Option<T>
    where
        T: Eq + Clone + std::hash::Hash,
    {
        if let Some(value) = self.0.get_mut(key) {
            *value -= 1;
            if *value == 0 {
                self.0.remove(key);
            }
            Some(key.clone())
        } else {
            None
        }
    }

    fn take_next(&mut self) -> Option<T>
    where
        T: Eq + Clone + std::hash::Hash,
    {
        self.0.keys().next().cloned().and_then(|key| self.take(&key))
    }
}

fn find_sections(input: &ParseResult) -> Vec<Vec<(usize, usize)>> {
    let mut filled = Array2::from_elem((input.nrows(), input.ncols()), false);
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

    for &(y, x) in section {
        for (dy, dx) in NEIGHBORS {
            let new_y = y as isize + dy;
            let new_x = x as isize + dx;

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

fn find_sides(perimeter: &[(isize, isize, Neighbor)]) -> usize {
    let mut perimeter = perimeter.iter().copied().collect::<FrequencyMap<_>>();
    let mut total_sides = 0;

    while let Some(start) = perimeter.take_next() {
        let mut current = start;

        // Find all horizontal neighbors
        while let Some(right) = perimeter.take(&(current.0, current.1 + 1, start.2)) {
            current = right;
        }

        current = start;
        while let Some(left) = perimeter.take(&(current.0, current.1 - 1, start.2)) {
            current = left;
        }

        if current != start {
            total_sides += 1;
            continue;
        }

        // Find all vertical neighbors
        current = start;

        while let Some(down) = perimeter.take(&(current.0 + 1, current.1, start.2)) {
            current = down;
        }

        current = start;
        while let Some(up) = perimeter.take(&(current.0 - 1, current.1, start.2)) {
            current = up;
        }

        total_sides += 1;
    }

    total_sides
}

fn find_perimeter_coords(section: &[(usize, usize)], input: &ParseResult) -> Vec<(isize, isize, Neighbor)> {
    let section_char = input[section[0]];
    let mut perimeter_coords = vec![];

    for &(y, x) in section {
        for (dy, dx) in NEIGHBORS {
            let new_y = y as isize + dy;
            let new_x = x as isize + dx;

            if new_y < 0 || new_y >= input.nrows() as isize || new_x < 0 || new_x >= input.ncols() as isize {
                perimeter_coords.push((new_y, new_x, (dy, dx)));
                continue;
            }

            if input[[new_y as usize, new_x as usize]] != section_char {
                perimeter_coords.push((new_y, new_x, (dy, dx)));
            }
        }
    }

    perimeter_coords
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = include_str!("../input/2024/day12_example1.txt");
    const EXAMPLE_2: &str = include_str!("../input/2024/day12_example2.txt");
    const EXAMPLE_3: &str = include_str!("../input/2024/day12_example3.txt");
    const EXAMPLE_4: &str = include_str!("../input/2024/day12_example4.txt");
    const EXAMPLE_5: &str = include_str!("../input/2024/day12_example5.txt");
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
        assert_eq!(part1(&input), 1473620);
    }

    #[test]
    fn examples_part2() {
        assert_eq!(part2(&parse_input(EXAMPLE_1)), 80);
        assert_eq!(part2(&parse_input(EXAMPLE_2)), 1206);
        assert_eq!(part2(&parse_input(EXAMPLE_3)), 236);
        assert_eq!(part2(&parse_input(EXAMPLE_4)), 368);
        assert_eq!(part2(&parse_input(EXAMPLE_5)), 436);
    }

    #[test]
    fn input_part2() {
        let input = parse_input(INPUT);
        assert_eq!(part2(&input), 902620);
    }
}
