use nom::{
    bytes::complete::tag,
    character::complete::{multispace1, space1, u8, line_ending, space0},
    combinator::opt,
    multi::{separated_list1, many1},
    sequence::{delimited, preceded, terminated, tuple},
    IResult,
};

use crate::utils::ws;

type Number = u8;
type ParseResult = Vec<Blueprint>;

#[derive(Debug)]
struct Cost {
    ore: Number,
    clay: Number,
    obsidian: Number,
}

#[derive(Debug)]
pub struct Blueprint {
    id: u8,
    ore_robot_cost: Cost,
    clay_robot_cost: Cost,
    obsidian_robot_cost: Cost,
    geode_robot_cost: Cost,
}

impl Cost {
    fn parse(input: &str) -> IResult<&str, Self> {
        let (input, ore) = opt(terminated(u8, tuple((space1, tag("ore")))))(input)?;
        let (input, clay) = opt(preceded(ws(tag("and")), terminated(u8, tuple((space1, tag("clay"))))))(input)?;
        let (input, obsidian) = opt(preceded(ws(tag("and")), terminated(u8, tuple((space1, tag("obsidian"))))))(input)?;

        Ok((
            input,
            Cost {
                ore: ore.unwrap_or(0_u8),
                clay: clay.unwrap_or(0_u8),
                obsidian: obsidian.unwrap_or(0_u8),
            },
        ))
    }
}

impl Blueprint {
    fn parse(input: &str) -> IResult<&str, Self> {
        let (input, id) = terminated(preceded(tuple((tag("Blueprint"), multispace1)), u8), tag(":"))(input)?;

        let (input, ore_robot_cost) = preceded(
            multispace1,
            delimited(tuple((tag("Each ore robot costs"), multispace1)), Cost::parse, tag(".")),
        )(input)?;

        let (input, clay_robot_cost) = preceded(
            multispace1,
            delimited(tuple((tag("Each clay robot costs"), space1)), Cost::parse, tag(".")),
        )(input)?;

        let (input, obsidian_robot_cost) = preceded(
            multispace1,
            delimited(tuple((tag("Each obsidian robot costs"), space1)), Cost::parse, tag(".")),
        )(input)?;

        let (input, geode_robot_cost) = preceded(
            multispace1,
            delimited(tuple((tag("Each geode robot costs"), space1)), Cost::parse, tag(".")),
        )(input)?;

        Ok((
            input,
            Blueprint {
                id,
                ore_robot_cost,
                clay_robot_cost,
                obsidian_robot_cost,
                geode_robot_cost,
            },
        ))
    }
}

pub fn parse_input(input: &str) -> ParseResult {
    separated_list1(many1(tuple((line_ending, space0))), Blueprint::parse)(input)
        .map(|(_, result)| result)
        .unwrap()
}

pub fn part1(input: &ParseResult) -> isize {
    dbg!(input);
    12
}

pub fn part2(input: &ParseResult) -> isize {
    42
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../input/2022/day19_example.txt");
    const INPUT: &str = include_str!("../input/2022/day19.txt");

    #[test]
    fn example_part1() {
        let input = parse_input(EXAMPLE);
        assert_eq!(part1(&input), 42);
    }

    #[test]
    fn example_part2() {
        let input = parse_input(EXAMPLE);
        assert_eq!(part2(&input), 42);
    }

    #[test]
    fn input_part1() {
        let input = parse_input(INPUT);
        assert_eq!(part1(&input), 42);
    }

    #[test]
    fn input_part2() {
        let input = parse_input(INPUT);
        assert_eq!(part2(&input), 42);
    }
}
