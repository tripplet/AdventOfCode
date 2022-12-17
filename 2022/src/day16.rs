use std::collections::HashMap;

use nom::{
    bytes::complete::tag,
    character::complete::{alphanumeric1, multispace0, multispace1, u16},
    combinator::opt,
    multi::separated_list1,
    sequence::{terminated, tuple},
    IResult,
};
use petgraph::Graph;

use crate::utils::ws;

type ParseResult = HashMap<String, Valve>;

#[derive(Debug)]
pub struct Valve {
    name: String,
    flow_rate: u16,
    tunnels_to: Vec<String>,
}

impl Valve {
    fn parse(s: &str) -> IResult<&str, Self> {
        let (s, _) = tuple((tag("Valve"), multispace1))(s)?;
        let (s, name) = terminated(alphanumeric1, multispace1)(s)?;

        let (s, _) = tuple((tag("has flow rate"), multispace0, tag("="), multispace0))(s)?;
        let (s, flow_rate) = terminated(u16, tuple((tag(";"), multispace0)))(s)?;

        let (s, _) = tuple((
            tag("tunnel"),
            opt(tag("s")),
            tag(" lead"),
            opt(tag("s")),
            tag(" to valve"),
            opt(tag("s")),
            multispace1,
        ))(s)?;
        let (s, access_to) = separated_list1(ws(tag(",")), alphanumeric1)(s)?;

        Ok((
            s,
            Valve {
                name: name.to_string(),
                flow_rate,
                tunnels_to: access_to.iter().map(|s| s.to_string()).collect(),
            },
        ))
    }
}

pub fn parse_input(input: &str) -> ParseResult {
    input
        .trim()
        .lines()
        .map(|l| {
            let valve = Valve::parse(l).unwrap().1;
            (valve.name.clone(), valve)
        })
        .collect()
}

pub fn part1(input: &ParseResult) -> isize {
    let mut deps = Graph::<&str, &str>::new();
    for (name, valve) in input {
        let node = deps.add_node(name);
    }

    for (name, valve) in input {
        for tunnel in &valve.tunnels_to {
            deps.update_edge(name, other_node, "tunnel");
        }
    }
    42
}

pub fn part2(input: &ParseResult) -> isize {
    42
}
