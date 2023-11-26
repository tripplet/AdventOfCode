use std::collections::{HashMap, HashSet};

use nom::{
    bytes::complete::tag,
    character::complete::{alphanumeric1, multispace0, multispace1, u16},
    combinator::opt,
    multi::separated_list1,
    sequence::{terminated, tuple},
    IResult,
};

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

fn dfs(graph: &ParseResult, minute: i8, current: &Valve, open_valves: &HashSet<String>) -> u32 {
    if minute <= 0 || open_valves.is_empty() {
        return 0;
    }

    let mut max_score = 0;

    for tunnel in &current.tunnels_to {
        let next = &graph[tunnel];

        let mut with_current_valve_open = [0, 0];
        if open_valves.contains(&current.name) {
            //dbg!(minute, &current.name, &next.name, closed_valves, &scores);

            // Current valve is still closed
            let open_valves = &mut open_valves.clone();
            open_valves.remove(&current.name);
            with_current_valve_open[0] =
                (current.flow_rate * (minute - 1) as u16) as u32 + dfs(graph, minute - 2_i8, next, &open_valves);
        }
        else {
            with_current_valve_open[1] = dfs(graph, minute - 1, next, open_valves);
        }

        max_score = max_score.max(with_current_valve_open[0].max(with_current_valve_open[1]));
    }

    max_score
}

pub fn part1(input: &ParseResult) -> u32 {
    let a = input
        .iter()
        .filter(|valve| valve.1.flow_rate != 0)
        .map(|valve| valve.0)
        .cloned();

    dfs(input, 19, &input["AA"], &HashSet::from_iter(a))
}

pub fn part2(input: &ParseResult) -> isize {
    _= input;
    42
}
