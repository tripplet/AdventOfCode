use std::fmt::Display;

use aoc_runner_derive::{aoc, aoc_generator};
use nom::bytes::complete::tag;
use nom::character::complete::{i64, line_ending, not_line_ending, space1};
use nom::multi::{count, separated_list1};
use nom::sequence::{preceded, terminated};
use nom::IResult;

type Number = i64;
type ParseResult = Almanac;

#[derive(Debug)]
pub struct Mapping {
    /// Start of mapping range (inclusive)
    from_start: Number,

    /// End of mapping range (inclusive)
    from_end: Number,

    /// Start of the range where number is mapped to
    delta: Number,
}

#[derive(Debug)]
pub struct Almanac {
    seeds: Vec<Number>,
    instruction: Vec<Vec<Mapping>>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct SeedRange<T> {
    /// Start of range (inclusive)
    start: T,

    /// End of range (inclusive)
    end: T,
}

impl<T> std::fmt::Display for SeedRange<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}, {}]", self.start, self.end)
    }
}

macro_rules! sr {
    ($start:expr, $end:expr) => {
        SeedRange {
            start: $start,
            end: $end,
        }
    };
}

impl Mapping {
    fn parse(input: &str) -> IResult<&str, Self> {
        let (s, values) = separated_list1(space1, i64)(input)?;

        Ok((
            s,
            Self {
                from_start: values[1],
                from_end: values[1] + values[2] - 1,
                delta: values[0] - values[1],
            },
        ))
    }

    #[inline(always)]
    fn map_number(&self, nb: Number) -> (Number, bool) {
        if nb >= self.from_start && nb <= self.from_end {
            (nb + self.delta, true)
        } else {
            (nb, false)
        }
    }

    fn map_range(&self, range: &SeedRange<Number>) -> Vec<(SeedRange<Number>, bool)> {
        if range.end < self.from_start || range.start > self.from_end {
            // range lies "outside"
            vec![(*range, false)]
        } else if range.start >= self.from_start && range.end <= self.from_end {
            // Upper bound of rage resides into mapping array
            vec![(sr!(self.map_number(range.start).0, self.map_number(range.end).0), true)]
        } else if range.start < self.from_start && range.end >= self.from_start && range.end <= self.from_end {
            vec![
                (sr!(range.start, self.from_start - 1), false),
                (sr!(self.map_number(self.from_start).0, self.map_number(range.end).0), true),
            ]
        } else if range.start >= self.from_start && range.end > self.from_end {
            vec![
                (sr!(self.map_number(range.start).0, self.map_number(self.from_end).0), true),
                (sr!((self.from_end + 1), range.end), false),
            ]
        } else {
            vec![
                (sr!(range.start, self.from_start - 1), false),
                (sr!(self.map_number(self.from_start).0, self.map_number(self.from_end).0), true),
                (sr!((self.from_end + 1), range.end), false),
            ]
        }
    }
}

impl Almanac {
    fn parse(input: &str) -> IResult<&str, Self> {
        let (s, seeds) = preceded(tag("seeds: "), separated_list1(space1, i64))(input)?;
        let (s, _) = count(line_ending, 2)(s)?;

        let (s, instruction) = separated_list1(
            count(line_ending, 2),
            preceded(
                terminated(not_line_ending, line_ending),
                separated_list1(line_ending, Mapping::parse),
            ),
        )(s)?;

        Ok((s, Self { seeds, instruction }))
    }
}

#[aoc_generator(day5)]
pub fn parse_input(input: &str) -> ParseResult {
    Almanac::parse(input).expect("only valid input in aoc").1
}

#[aoc(day5, part1)]
pub fn part1(input: &ParseResult) -> Number {
    let mut seed_pos = input.seeds.clone();

    for instruction in &input.instruction {
        for seed in seed_pos.iter_mut() {
            for mapping in instruction {
                let (new_seed, mapped) = mapping.map_number(*seed);
                *seed = new_seed;
                if mapped {
                    break;
                }
            }
        }
    }

    *seed_pos.iter().min().unwrap()
}

#[aoc(day5, part2)]
pub fn part2(input: &ParseResult) -> Number {
    let mut seed_ranges: Vec<_> = input
        .seeds
        .chunks(2)
        .map(|chunk| sr!(chunk[0], chunk[0] + chunk[1] - 1))
        .collect();

    // Go through all instructions
    for instruction in &input.instruction {
        let mut new_seed_ranges = vec![];

        for seed_range in &seed_ranges {
            new_seed_ranges.extend(map_range_with_mappings(seed_range, instruction));
        }

        seed_ranges = new_seed_ranges;
    }

    seed_ranges.iter().map(|r| r.start).min().unwrap()
}

fn map_range_with_mappings(range: &SeedRange<Number>, mappings: &[Mapping]) -> Vec<SeedRange<Number>> {
    let mut ranges = vec![(*range, false)];

    for mapping in mappings {
        let mut new_ranges = ranges.iter().filter(|r| r.1).cloned().collect::<Vec<_>>();
        for range in ranges.iter().filter(|r| !r.1) {
            new_ranges.extend(mapping.map_range(&range.0));
        }

        ranges = new_ranges;
    }

    ranges.into_iter().map(|r| r.0).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../input/2023/day5_example.txt");
    const INPUT: &str = include_str!("../input/2023/day5.txt");

    #[test]
    fn test_map_number() {
        let mapping1 = Mapping::parse("50 98 2").unwrap().1;

        assert_eq!(mapping1.from_start, 98);
        assert_eq!(mapping1.from_end, 99);
        assert_eq!(mapping1.delta, -48);
        assert_eq!(mapping1.map_number(97), (97, false));
        assert_eq!(mapping1.map_number(98), (50, true));
        assert_eq!(mapping1.map_number(99), (51, true));
        assert_eq!(mapping1.map_number(100), (100, false));

        let mapping2 = Mapping::parse("0 69 1").unwrap().1;

        assert_eq!(mapping2.map_number(68), (68, false));
        assert_eq!(mapping2.map_number(69), (0, true));
        assert_eq!(mapping2.map_number(70), (70, false));
    }

    #[test]
    fn test_map_range() {
        let mapping1 = Mapping::parse("50 98 2").unwrap().1;
        assert_eq!(mapping1.map_range(&sr!(0, 97)), vec![(sr!(0, 97), false)]);
        assert_eq!(mapping1.map_range(&sr!(0, 98)), vec![(sr!(0, 97), false), (sr!(50, 50), true)]);
        assert_eq!(mapping1.map_range(&sr!(90, 98)), vec![(sr!(90, 97), false), (sr!(50, 50), true)]);
        assert_eq!(mapping1.map_range(&sr!(90, 99)), vec![(sr!(90, 97), false), (sr!(50, 51), true)]);
        assert_eq!(mapping1.map_range(&sr!(98, 104)), vec![(sr!(50, 51), true), (sr!(100, 104), false)]);
        assert_eq!(
            mapping1.map_range(&sr!(90, 104)),
            vec![(sr!(90, 97), false), (sr!(50, 51), true), (sr!(100, 104), false)]
        );
        assert_eq!(mapping1.map_range(&sr!(97, 97)), vec![(sr!(97, 97), false)]);
        assert_eq!(mapping1.map_range(&sr!(98, 98)), vec![(sr!(50, 50), true)]);
        assert_eq!(mapping1.map_range(&sr!(99, 99)), vec![(sr!(51, 51), true)]);
        assert_eq!(mapping1.map_range(&sr!(100, 100)), vec![(sr!(100, 100), false)]);
    }

    #[test]
    fn test_mapping() {
        let mapping1 = Mapping::parse("50 98 2").unwrap().1;
        let mapping2 = Mapping::parse("22 105 5").unwrap().1;

        assert_eq!(
            map_range_with_mappings(&sr!(40, 200), &vec![mapping1, mapping2]),
            vec![sr!(50, 51), sr!(40, 97), sr!(100, 104), sr!(22, 26), sr!(110, 200)]
        );
    }

    #[test]
    fn example_part1() {
        let input = parse_input(EXAMPLE);
        assert_eq!(part1(&input), 35);
    }

    #[test]
    fn example_part2() {
        let input = parse_input(EXAMPLE);
        assert_eq!(part2(&input), 46);
    }

    #[test]
    fn input_part1() {
        let input = parse_input(INPUT);
        assert_eq!(part1(&input), 218513636);
    }

    #[test]
    fn input_part2() {
        let input = parse_input(INPUT);
        assert_eq!(part2(&input), 81956384);
    }
}
