use std::ops::{Add, AddAssign, Sub};

use nom::{
    bytes::complete::tag,
    character::complete::{line_ending, multispace1, space0, space1, u8},
    combinator::opt,
    multi::{many1, separated_list1},
    sequence::{delimited, preceded, terminated, tuple},
    IResult,
};
use rayon::prelude::IntoParallelRefIterator;
use rayon::{iter::ParallelIterator, prelude::IndexedParallelIterator};

use crate::utils::ws;

type Number = u8;
type ParseResult = Vec<Blueprint>;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Values {
    ore: Number,
    clay: Number,
    obsidian: Number,
    geode: Number,
}

#[derive(Copy, Clone, PartialEq, Eq)]
struct Step {
    resources: Values,
    robots: Values,
}

#[derive(Debug)]
pub struct Blueprint {
    id: u8,
    ore_robot_cost: Values,
    clay_robot_cost: Values,
    obsidian_robot_cost: Values,
    geode_robot_cost: Values,
}

const ZERO: Values = Values {
    ore: 0,
    clay: 0,
    obsidian: 0,
    geode: 0,
};
const ONE_ORE: Values = Values { ore: 1, ..ZERO };

impl std::fmt::Debug for Step {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "Robots=> Ore: {:2}, Clay: {:2}, Obsidian: {:2}, Geode: {:2}",
            self.robots.ore, self.robots.clay, self.robots.obsidian, self.robots.geode
        ))
        .unwrap();

        f.write_str("  |  ").unwrap();
        f.write_fmt(format_args!(
            "Res=> Ore: {:2}, Clay: {:2}, Obsidian: {:2}, Geode: {:2}",
            self.resources.ore, self.resources.clay, self.resources.obsidian, self.resources.geode
        ))
    }
}

impl Values {
    fn parse(input: &str) -> IResult<&str, Self> {
        let (input, ore) = opt(terminated(u8, tuple((space1, tag("ore")))))(input)?;
        let (input, clay) = opt(preceded(ws(tag("and")), terminated(u8, tuple((space1, tag("clay"))))))(input)?;
        let (input, obsidian) = opt(preceded(
            ws(tag("and")),
            terminated(u8, tuple((space1, tag("obsidian")))),
        ))(input)?;

        Ok((
            input,
            Values {
                ore: ore.unwrap_or(0_u8),
                clay: clay.unwrap_or(0_u8),
                obsidian: obsidian.unwrap_or(0_u8),
                geode: 0,
            },
        ))
    }

    fn try_build(&self, resources: &Values) -> Option<Values> {
        if self.ore <= resources.ore && self.clay <= resources.clay && self.obsidian <= resources.obsidian {
            Some(*resources - *self)
        } else {
            None
        }
    }
}

impl Sub for Values {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            ore: self.ore - rhs.ore,
            clay: self.clay - rhs.clay,
            obsidian: self.obsidian - rhs.obsidian,
            geode: self.geode - rhs.geode,
        }
    }
}

impl Add for Values {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            ore: self.ore + rhs.ore,
            clay: self.clay + rhs.clay,
            obsidian: self.obsidian + rhs.obsidian,
            geode: self.geode + rhs.geode,
        }
    }
}

impl AddAssign for Values {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Blueprint {
    fn parse(input: &str) -> IResult<&str, Self> {
        let (input, id) = terminated(preceded(tuple((tag("Blueprint"), multispace1)), u8), tag(":"))(input)?;

        let (input, ore_robot_cost) = preceded(
            multispace1,
            delimited(
                tuple((tag("Each ore robot costs"), multispace1)),
                Values::parse,
                tag("."),
            ),
        )(input)?;

        let (input, clay_robot_cost) = preceded(
            multispace1,
            delimited(tuple((tag("Each clay robot costs"), space1)), Values::parse, tag(".")),
        )(input)?;

        let (input, obsidian_robot_cost) = preceded(
            multispace1,
            delimited(
                tuple((tag("Each obsidian robot costs"), space1)),
                Values::parse,
                tag("."),
            ),
        )(input)?;

        let (input, geode_robot_cost) = preceded(
            multispace1,
            delimited(tuple((tag("Each geode robot costs"), space1)), Values::parse, tag(".")),
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

pub fn part1(input: &ParseResult) -> usize {
    input
        .par_iter()
        .map(find_max_number_of_geodes)
        .zip(input)
        .max_by_key(|(res, _)| *res)
        .map(|(max, blueprint)| blueprint.id as usize * max)
        .unwrap()
}

fn find_max_number_of_geodes(blueprint: &Blueprint) -> usize {
    let mut paths = vec![Step {
        resources: ZERO,
        robots: ONE_ORE,
    }];

    for minute in 1..=24 {
        // Determine which robot to build
        let mut next_step = vec![];

        while let Some(path) = paths.pop() {
            // Try build a geode robot first if it is possible
            if let Some(resources) = blueprint.geode_robot_cost.try_build(&path.resources) {
                next_step.push(Step {
                    resources: resources + path.robots,
                    robots: Values {
                        geode: path.robots.geode + 1,
                        ..path.robots
                    },
                });
                continue;
            }

            if let Some(resources) = blueprint.obsidian_robot_cost.try_build(&path.resources) {
                next_step.push(Step {
                    resources: resources + path.robots,
                    robots: Values {
                        obsidian: path.robots.obsidian + 1,
                        ..path.robots
                    },
                });
            }

            if let Some(resources) = blueprint.clay_robot_cost.try_build(&path.resources) {
                next_step.push(Step {
                    resources: resources + path.robots,
                    robots: Values {
                        clay: path.robots.clay + 1,
                        ..path.robots
                    },
                });
            }

            if let Some(resources) = blueprint.ore_robot_cost.try_build(&path.resources) {
                next_step.push(Step {
                    resources: resources + path.robots,
                    robots: Values {
                        ore: path.robots.ore + 1,
                        ..path.robots
                    },
                });
            }

            // Robots working only producing more resources
            next_step.push(Step {
                resources: path.resources + path.robots,
                robots: path.robots,
            });
        }

        paths = next_step;

        // Do agressive pruning of path which are not worth following
        let mut pruned: Vec<Step> = Vec::with_capacity(paths.len());

        paths.sort_unstable_by(|a, b| {
            a.robots
                .geode
                .cmp(&b.robots.geode)
                .then(a.robots.clay.cmp(&b.robots.clay))
                .then(a.robots.obsidian.cmp(&b.robots.obsidian))
                .then(a.robots.geode.cmp(&b.robots.geode))
        });

        while let Some(path) = paths.pop() {
            if pruned.iter().any(|s| s.robots.geode > path.robots.geode) {
                // Skip
                continue;
            }

            if pruned.iter().any(|s| s.robots.obsidian > path.robots.obsidian) {
                // Skip
                //continue;
            }

            if pruned.iter().any(|s| s.robots.clay > path.robots.clay) {
                // Skip
                //continue;
            }

            if pruned.iter().any(|s| s.robots.ore > path.robots.ore) {
                // Skip
                //continue;
            }

            if pruned
                .iter()
                .any(|s| s.robots == path.robots && s.resources == path.resources)
            {
                // Skip
                continue;
            }

            pruned.push(path);
        }

        paths = pruned;

        dbg!(minute);
        dbg!(paths.len());
        //dbg!(&paths);
        println!();
    }

    //dbg!(&paths);

    paths.iter().map(|s| s.resources.geode).max().unwrap() as usize
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
