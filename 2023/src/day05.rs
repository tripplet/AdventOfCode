use std::ops::Range;

use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;
use nom::bytes::complete::tag;
use nom::character::complete::line_ending;
use nom::character::complete::not_line_ending;
use nom::character::complete::space1;
use nom::character::complete::u64;
use nom::multi::count;
use nom::multi::separated_list1;
use nom::sequence::preceded;
use nom::sequence::terminated;
use nom::IResult;

type Number = u64;
type ParseResult = Almanac;

#[derive(Debug)]
pub struct Mapping {
    from: Range<Number>,
    to: Number,
}

#[derive(Debug)]
pub struct Almanac {
    seeds: Vec<Number>,
    instruction: Vec<Vec<Mapping>>,
}

impl Mapping {
    fn parse(input: &str) -> IResult<&str, Self> {
        let (s, values) = separated_list1(space1, u64)(input)?;

        Ok((
            s,
            Self {
                from: values[1]..(values[1] + values[2]),
                to: values[0],
            },
        ))
    }

    //#[inline(always)]
    fn map_number(&self, nb: Number) -> Option<Number> {
        if self.from.contains(&nb) {
            Some(self.to + nb - self.from.start)
        } else {
            None
        }
    }

    fn map_range(&self, range: &Range<Number>) -> Option<Vec<Range<Number>>> {
        if (range.start < self.from.start && range.end <= self.from.start)
            || range.end <= self.from.start
            || range.start >= self.from.end
        {
            // range lies "outside"
            None
        } else if range.start >= self.from.start && range.end < self.from.end {
            // Upper bound of rage resides into mapping array
            Some(vec![
                self.map_number(range.start).unwrap()..self.map_number(range.end).unwrap(),
            ])
        } else if range.start < self.from.start && range.end > self.from.start {
            // Some(vec![
            //     self.map_number(range.start).unwrap()..self.map_number(range.end).unwrap(),
            // ])
            todo!()
        } else if range.start > self.from.start && range.end > self.from.end {
            todo!()
        } else {
            unreachable!("all cases should be covered above")
        }
    }
}

impl Almanac {
    fn parse(input: &str) -> IResult<&str, Self> {
        let (s, seeds) = preceded(tag("seeds: "), separated_list1(space1, u64))(input)?;
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

    for idx in 0..seed_pos.len() {
        for instruction in &input.instruction {
            for mapping in instruction {
                if let Some(res) = mapping.map_number(seed_pos[idx]) {
                    seed_pos[idx] = res;
                    break;
                }
            }
        }
    }

    *seed_pos.iter().min().unwrap()
}

#[aoc(day5, part2)]
pub fn part2(input: &ParseResult) -> u64 {
    let seed_ranges: Vec<_> = input
        .seeds
        .chunks(2)
        .map(|chunk| chunk[0]..chunk[0] + chunk[1])
        .collect();

    let mut existing_min = Number::MAX;
    let mut idx = 0;

    for range in seed_ranges {
        dbg!(idx);
        dbg!(range.end - range.start);
        idx += 1;

        for mut nb in range {
            for instruction in &input.instruction {
                for mapping in instruction {
                    if let Some(res) = mapping.map_number(nb) {
                        nb = res;
                        break;
                    }
                }
            }

            existing_min = nb.min(existing_min);
        }
    }

    existing_min
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../input/2023/day5_example.txt");
    const INPUT: &str = include_str!("../input/2023/day5.txt");

    #[test]
    fn test_mapping() {
        let mapping1 = Mapping::parse("50 98 2").unwrap().1;
        assert_eq!(mapping1.map_number(97), None);
        assert_eq!(mapping1.map_number(98), Some(50));
        assert_eq!(mapping1.map_number(99), Some(51));
        assert_eq!(mapping1.map_number(100), None);

        let mapping2 = Mapping::parse("0 69 1").unwrap().1;

        assert_eq!(mapping2.map_number(68), None);
        assert_eq!(mapping2.map_number(69), Some(0));
        assert_eq!(mapping2.map_number(70), None);
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
