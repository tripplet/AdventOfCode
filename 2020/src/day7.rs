use regex::{Regex};
use std::collections::HashSet;

use petgraph::graphmap::DiGraphMap;
use petgraph::dot::{Dot, Config};
use petgraph::Direction;

pub fn main() {
    let input = include_str!("../input/2020/day7.txt").trim();

    let graph = parse(input);
    drop(input);

    //println!("{:?}", Dot::with_config(&graph, &[Config::EdgeNoLabel])); // Print graph as graphviz

    let mut now = std::time::Instant::now();
    println!("Part1: {}  [{}]", part1("shiny gold", &graph), humantime::format_duration(now.elapsed()));

    now = std::time::Instant::now();
    println!("Part2: {}  [{}]", part2("shiny gold", &graph), humantime::format_duration(now.elapsed()));
}

pub fn parse(input: &str) -> DiGraphMap::<&str, i32> {
    let mut g = DiGraphMap::new();

    let name = Regex::new(r"^(?P<in>\w+ \w+) bags contain ").unwrap();
    let re_inside = Regex::new(r",? ?(?P<count>\d+) (?P<name>\w+ \w+) bags?").unwrap();

    for line in input.trim().lines() {
        if re_inside.find(line).is_none() {
            continue;
        }

        let name = name.captures(line).unwrap().name("in").unwrap().as_str();

        for i in re_inside.captures_iter(line) {
            let contains = i.name("name").unwrap().as_str();
            let count = i.name("count").unwrap().as_str().parse::<i32>().unwrap();

            g.add_edge(name, contains, count);
        }
    }

    g
}

pub fn part1(dest: &str, graph: &DiGraphMap::<&str, i32>) -> i32 {
    let mut parents = HashSet::new();
    let mut still_to_check = vec![dest];

    while still_to_check.len() > 0 {
        for parent in graph.neighbors_directed(still_to_check.remove(0), Direction::Incoming) {
            if parents.insert(parent) {
                still_to_check.push(parent);
            }
        }
    }

    parents.len() as i32
}

pub fn part2(start: &str, graph: &DiGraphMap::<&str, i32>) -> i32 {
    let mut count = 0;

    for edge in graph.edges(start) {
        count += edge.2 + edge.2 * part2(edge.1, graph);
    }

    count
}