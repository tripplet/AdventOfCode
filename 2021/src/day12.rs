use std::collections::{HashMap, HashSet};

const INPUT: &str = include_str!("../input/2021/day12.txt");

type Graph<'a> = HashMap<&'a str, HashSet<&'a str>>;

pub fn main() {
    let data = parse_input(INPUT);

    println!("Part1: {}", part1(&data));
    println!("Part2: {}", part2(&data));
}

fn parse_input(input: &str) -> Vec<(&str, &str)> {
    input
        .trim()
        .lines()
        .map(|line| {
            let parts = line.split("-").collect::<Vec<_>>();
            (parts[0], parts[1])
        }).collect()
}

fn buid_graph<'a>(data: &'a [(&str, &str)]) -> Graph<'a> {
    let mut graph: HashMap<&str, HashSet<&str>> = HashMap::new();

    for node in data {
        let x = graph.entry(node.0).or_insert(HashSet::new());
        x.insert(node.1);
        let x = graph.entry(node.1).or_insert(HashSet::new());
        x.insert(node.0);
    }
    graph
}

fn find_paths<'a>(
    graph: &'a Graph,
    cur_node: &'a str,
    visited: &[&'a str],
    double_visit_used: bool,
) -> Vec<Vec<&'a str>> {
    let mut visited = visited.to_vec();
    visited.push(cur_node);

    if cur_node == "end" {
        return vec![visited];
    }

    let mut sub_graphs = vec![];

    for edge in graph.get(cur_node).unwrap() {
        if *edge == cur_node || *edge == "start" {
            continue;
        }

        let mut double_visit_used = double_visit_used;
        let can_visit = if edge.chars().next().unwrap().is_uppercase() || !visited.contains(edge) {
            true
        } else if !double_visit_used {
            double_visit_used = true;
            true
        } else {
            false
        };

        if can_visit {
            sub_graphs.append(&mut find_paths(&graph, edge, &visited, double_visit_used));
        }
    }
    sub_graphs
}

fn part1(data: &[(&str, &str)]) -> usize {
    let graph = buid_graph(data);
    let paths = find_paths(&graph, "start", &mut vec![], true);
    paths.len()
}

fn part2(data: &[(&str, &str)]) -> usize {
    let graph = buid_graph(data);
    let paths = find_paths(&graph, "start", &mut vec![], false);
    paths.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = include_str!("../input/2021/day12_example1.txt");
    const EXAMPLE_2: &str = include_str!("../input/2021/day12_example2.txt");
    const EXAMPLE_3: &str = include_str!("../input/2021/day12_example3.txt");

    #[test]
    fn part1_example() {
        assert_eq!(10, part1(&parse_input(EXAMPLE_1)), "example 1");
        assert_eq!(19, part1(&parse_input(EXAMPLE_2)), "example 2");
        assert_eq!(226, part1(&parse_input(EXAMPLE_3)), "example 3");
    }

    #[test]
    fn part2_example() {
        assert_eq!(36, part2(&parse_input(EXAMPLE_1)), "example 1");
        assert_eq!(103, part2(&parse_input(EXAMPLE_2)), "example 2");
        assert_eq!(3509, part2(&parse_input(EXAMPLE_3)), "example 3");
    }

    #[test]
    fn part1_on_input() {
        assert_eq!(4495, part1(&parse_input(INPUT)));
    }

    #[test]
    fn part2_on_input() {
        assert_eq!(131254, part2(&parse_input(INPUT)));
    }
}
