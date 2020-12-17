use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::str::FromStr;

#[macro_use]
extern crate lazy_static;

#[derive(Debug)]
struct Range {
    name: String,
    min1: u32,
    max1: u32,
    min2: u32,
    max2: u32,
}

#[derive(Debug)]
struct Data {
    ranges: Vec<Range>,
    your_ticket: Vec<u32>,
    nearby_tickets: Vec<Vec<u32>>,
}

lazy_static! {
    static ref REGEX_RANGE: Regex = Regex::new(
        r"(?m)^(?P<name>[^:]+): (?P<min1>\d+)-(?P<max1>\d+) or (?P<min2>\d+)-(?P<max2>\d+)"
    )
    .unwrap();
}

impl FromStr for Range {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let range = REGEX_RANGE.captures(s);

        match range {
            Some(r) => Ok(Range {
                name: r.name("name").ok_or("invalid")?.as_str().into(),
                min1: r.name("min1").ok_or("invalid")?.as_str().parse()?,
                max1: r.name("max1").ok_or("invalid")?.as_str().parse()?,
                min2: r.name("min2").ok_or("invalid")?.as_str().parse()?,
                max2: r.name("max2").ok_or("invalid")?.as_str().parse()?,
            }),
            None => Err("Malformed".into()),
        }
    }
}

fn main() {
    let data = parse(include_str!("../input/2020/day16.txt")).unwrap();

    let now = std::time::Instant::now();
    let part1 = part1(&data);
    println!(
        "Part1: {}  [{}]",
        part1,
        humantime::format_duration(now.elapsed())
    );
    assert_eq!(part1, 26026);

    let now = std::time::Instant::now();
    let part2 = part2(&data);
    println!(
        "Part2: {}  [{}]",
        part1,
        humantime::format_duration(now.elapsed())
    );
    assert_eq!(part2, 1305243193339);
}

fn parse(input: &str) -> Result<Data, Box<dyn Error>> {
    let mut section = 0;
    let mut ranges = Vec::new();
    let mut your_ticket: Option<Vec<u32>> = None;
    let mut nearby_tickets = Vec::new();

    for line in input.trim().lines() {
        if line.trim().is_empty() {
            section += 1;
            continue;
        }

        if section == 0 {
            ranges.push(Range::from_str(line)?);
        } else if section == 1 {
            if line.trim() == "your ticket:" {
                continue;
            }

            your_ticket = Some(
                line.trim()
                    .split(",")
                    .map(|nb| nb.parse().unwrap())
                    .collect(),
            );
        } else if section == 2 {
            if line.trim() == "nearby tickets:" {
                continue;
            }

            nearby_tickets.push(
                line.trim()
                    .split(",")
                    .map(|nb| nb.parse::<u32>().unwrap())
                    .collect::<Vec<u32>>(),
            );
        }
    }

    Ok(Data {
        ranges: ranges,
        your_ticket: your_ticket.unwrap(),
        nearby_tickets: nearby_tickets,
    })
}

fn part1(data: &Data) -> u32 {
    data.nearby_tickets
        .iter()
        .map(|ticket_numbers| {
            ticket_numbers
                .iter()
                .filter(|nb| {
                    data.ranges.iter().all(|rule| {
                        !((rule.min1 <= **nb && **nb <= rule.max1)
                            || (rule.min2 <= **nb && **nb <= rule.max2))
                    })
                })
                .sum::<u32>()
        })
        .sum()
}

fn part2(data: &Data) -> u64 {
    let valid_tickets = data
        .nearby_tickets
        .iter()
        .filter(|ticket_numbers| {
            ticket_numbers.iter().all(|nb| {
                data.ranges.iter().any(|rule| {
                    (rule.min1 <= *nb && *nb <= rule.max1) || (rule.min2 <= *nb && *nb <= rule.max2)
                })
            })
        })
        .collect::<Vec<_>>();

    let mut rule_associactions: HashMap<usize, HashSet<usize>> = HashMap::new();

    for idx in 0..data.nearby_tickets[0].len() {
        let x = valid_tickets
            .iter()
            .map(|ticket_numbers| ticket_numbers[idx])
            .map(|nb| {
                data.ranges
                    .iter()
                    .enumerate()
                    .filter(move |(_, rule)| {
                        (rule.min1 <= nb && nb <= rule.max1)
                            || (rule.min2 <= nb && nb <= rule.max2)
                    })
                    .map(|(idx, _)| idx)
                    .collect::<HashSet<_>>()
            })
            .collect::<Vec<_>>();

        let z = x.iter().skip(1).fold(x[0].clone(), |acc, hs| {
            acc.intersection(hs).cloned().collect()
        });

        rule_associactions.insert(idx, z);
    }

    loop {
        let f = rule_associactions.values().filter(|s| s.len() == 1).flatten().cloned().collect::<HashSet<_>>();

        for elem in rule_associactions.clone().iter().filter(|(_,v)| v.len() > 1) {
            rule_associactions.insert(*elem.0, rule_associactions[elem.0].difference(&f).cloned().collect());
        }

        if rule_associactions.values().all(|s| s.len() == 1) {
            break;
        }
    }

    let result_rules = rule_associactions.iter().filter(|(_,v)| data.ranges[*v.iter().next().unwrap()].name.starts_with("departure"));
    result_rules.map(|(k,_)| data.your_ticket[*k]).fold(1 as u64, |a,b| a as u64 * b as u64)
}
