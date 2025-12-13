use std::collections::{BTreeSet, HashMap, HashSet};

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug)]
pub struct ParseResult {
    graph: HashMap<u16, Vec<u16>>,
    lookup: HashMap<String, u16>,
}

#[aoc_generator(day11)]
pub fn parse_input(input: &str) -> ParseResult {
    let mut parsed = ParseResult::new();

    for line in input.trim().lines().map(|line| line.trim()) {
        let (left, right) = line.split_once(": ").unwrap();
        parsed.add(left, right.split(' '));
    }

    parsed
}

impl ParseResult {
    fn new() -> Self {
        ParseResult {
            graph: HashMap::new(),
            lookup: HashMap::new(),
        }
    }

    fn get_id_for_name(&self, name: &str) -> Option<u16> {
        self.lookup.get(name).copied()
    }

    fn add<'a>(&mut self, start: &str, destinations: impl std::iter::Iterator<Item = &'a str>) {
        // Must be a local function to work around partial borrow problem
        // This way the compiler knows only lookup is borrowed not the whole struct
        let mut get_id_for_name = |name: &str| {
            let next_id = self.lookup.len() as u16;
            *self.lookup.entry(name.to_string()).or_insert(next_id)
        };

        let entry = self.graph.entry(get_id_for_name(start)).or_insert(vec![]);

        for dest in destinations {
            entry.push(get_id_for_name(dest));
        }
    }
}

#[aoc(day11, part1)]
pub fn part1(input: &ParseResult) -> usize {
    let you = input.get_id_for_name("you").unwrap();
    let out = input.get_id_for_name("out").unwrap();

    let mut total = 0;
    let mut cache = HashMap::new();
    for path in input.graph[&you].iter().copied() {
        total += count_paths(input, &mut cache, path, out);
    }

    total
}

fn count_paths(
    input: &ParseResult,
    cache: &mut HashMap<u16, usize>,
    start: u16,
    end: u16,
) -> usize {
    if let Some(cached_result) = cache.get(&start) {
        return *cached_result;
    }

    if start == end {
        return 1;
    }

    if !input.graph.contains_key(&start) {
        return 0;
    }

    let mut total = 0;
    for dest in &input.graph[&start] {
        total += count_paths(input, cache, *dest, end)
    }

    cache.insert(start, total);
    total
}

fn count_paths_with_nodes(
    input: &ParseResult,
    cache: &mut HashMap<(u16, BTreeSet<u16>), usize>,
    must_visit: &HashSet<u16>,
    has_visited: &BTreeSet<u16>,
    start: u16,
    end: u16,
) -> usize {
    if let Some(cached_result) = cache.get(&(start, has_visited.clone())) {
        return *cached_result;
    }

    if start == end && has_visited.len() == must_visit.len() {
        return 1;
    }

    if !input.graph.contains_key(&start) {
        return 0;
    }

    let mut total = 0;
    for dest in &input.graph[&start] {
        let new_has_visited = if must_visit.contains(dest) {
            let mut new_visted = has_visited.clone();
            new_visted.insert(*dest);
            Some(new_visted)
        } else {
            None
        };

        total += count_paths_with_nodes(
            input,
            cache,
            must_visit,
            new_has_visited.as_ref().unwrap_or(has_visited),
            *dest,
            end,
        );
    }

    cache.insert((start, has_visited.clone()), total);
    total
}

#[aoc(day11, part2)]
pub fn part2(input: &ParseResult) -> usize {
    let svr = input.get_id_for_name("svr").unwrap();
    let out = input.get_id_for_name("out").unwrap();

    let must_visit = HashSet::from([
        input.get_id_for_name("dac").unwrap(),
        input.get_id_for_name("fft").unwrap(),
    ]);

    let mut total = 0;
    let mut cache = HashMap::new();
    for path in input.graph[&svr].iter().copied() {
        let mut visited = BTreeSet::new();
        if must_visit.contains(&path) {
            visited.insert(path);
        }

        total += count_paths_with_nodes(input, &mut cache, &must_visit, &visited, path, out);
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_PART1: &str = include_str!("../input/2025/day11_example_part1.txt");
    const EXAMPLE_PART2: &str = include_str!("../input/2025/day11_example_part2.txt");
    const INPUT: &str = include_str!("../input/2025/day11.txt");

    #[test]
    fn example_part1() {
        let input = parse_input(EXAMPLE_PART1);
        assert_eq!(part1(&input), 5);
    }

    #[test]
    fn input_part1() {
        let input = parse_input(INPUT);
        assert_eq!(part1(&input), 796);
    }

    #[test]
    fn example_part2() {
        let input = parse_input(EXAMPLE_PART2);
        assert_eq!(part2(&input), 2);
    }

    #[test]
    fn input_part2() {
        let input = parse_input(INPUT);
        assert_eq!(part2(&input), 294053029111296);
    }
}
