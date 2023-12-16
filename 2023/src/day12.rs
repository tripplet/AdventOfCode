use aoc_runner_derive::{aoc, aoc_generator};

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

type ParseResult = Vec<(Vec<SpringState>, Vec<u8>)>;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum SpringState {
    Operational,
    Broken,
    Unknown,
}

impl SpringState {
    fn from_char(c: char) -> Result<Self, String> {
        match c {
            '.' => Ok(SpringState::Operational),
            '#' => Ok(SpringState::Broken),
            '?' => Ok(SpringState::Unknown),
            c => Err(format!("Invalid char {c}")),
        }
    }
}

impl std::fmt::Display for SpringState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SpringState::Operational => write!(f, "."),
            SpringState::Broken => write!(f, "#"),
            SpringState::Unknown => write!(f, "?"),
        }
    }
}

#[aoc_generator(day12)]
#[rustfmt::skip]
pub fn parse_input(input: &str) -> ParseResult {
    input.trim().lines()
        .map(|line| {
            let parts = line.split_once(' ').unwrap();
            (
                parts.0.chars().map(SpringState::from_char).collect::<Result<_, _>>().unwrap(),
                parts.1.split(',').map(|nb| nb.parse::<u8>()).collect::<Result<Vec<_>, _>>().unwrap(),
            )
        })
        .collect()
}

fn possibilities(conditions: &[SpringState], data: &[u8]) -> u16 {
    //print_conditions(conditions);

    if let Some(idx) = conditions.iter().position(|c| *c == SpringState::Unknown) {
        let mut sum_possibilites = 0;
        let mut variant: Vec<SpringState> = Vec::from(conditions);

        variant[idx] = SpringState::Broken;
        sum_possibilites += possibilities(&variant, data);

        variant[idx] = SpringState::Operational;
        sum_possibilites += possibilities(&variant, data);

        return sum_possibilites;
    }

    // Reached recursion end because there are no unknowns
    if is_match(conditions, data) {
        return 1;
    }

    // Valid possibility
    // println!("Found possibility");
    // println!();

    0
}

fn is_match(conditions: &[SpringState], damage_data: &[u8]) -> bool {
    let mut damage_parts = conditions
        .split(|c| *c == SpringState::Operational)
        .filter(|p| !p.is_empty());

    for damage_len in damage_data {
        match (damage_parts.next(), *damage_len) {
            (Some(part), len) if part.len() == (len as usize) && part.iter().all(|p| *p == SpringState::Broken) => {
                continue;
            }
            _ => {
                return false;
            }
        }
    }

    if damage_parts.next().is_some() {
        return false;
    }

    true
}

fn print_conditions(conditions: &[SpringState]) {
    for c in conditions {
        print!("{}", c);
    }
    println!();
}

#[aoc(day12, part1)]
pub fn part1(input: &ParseResult) -> usize {
    input
        .par_iter()
        .map(|(conditions, damage_data)| possibilities(conditions, damage_data) as usize)
        .sum::<usize>()
}

fn five_times<T: Copy>(input: &[T], sep: Option<T>) -> Vec<T> {
    let len = input.len();

    let mut new: Vec<T> = Vec::with_capacity(len * 5 + 4);
    new.extend(input);
    if let Some(sep) = sep {
        new.push(sep);
    }
    new.extend(input);
    if let Some(sep) = sep {
        new.push(sep);
    }
    new.extend(input);
    if let Some(sep) = sep {
        new.push(sep);
    }
    new.extend(input);
    if let Some(sep) = sep {
        new.push(sep);
    }
    new.extend(input);

    new
}

#[aoc(day12, part2)]
pub fn part2(input: &ParseResult) -> usize {
    input
        .par_iter()
        //.inspect(|x| println!("{x:?}"))
        .map(|(conditions, damage_data)| {
            possibilities(
                &five_times(conditions, Some(SpringState::Unknown)),
                &five_times(damage_data, None),
            ) as usize
        })
        //.inspect(|x| println!("{x}"))
        .sum::<usize>();
    12
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../input/2023/day12_example.txt");
    const INPUT: &str = include_str!("../input/2023/day12.txt");

    #[test]
    fn test_1() {
        let input = parse_input(EXAMPLE);
        possibilities(&input[5].0, &input[5].1);
    }

    #[test]
    fn example_part1() {
        let input = parse_input(EXAMPLE);
        assert_eq!(part1(&input), 21);
    }

    #[test]
    fn input_part1() {
        let input = parse_input(INPUT);
        assert_eq!(part1(&input), 7753);
    }

    #[test]
    fn example_part2() {
        let input = parse_input(EXAMPLE);
        assert_eq!(part2(&input), 525152);
    }

    //#[test]
    fn input_part2() {
        let input = parse_input(INPUT);
        assert_eq!(part2(&input), 42);
    }
}
