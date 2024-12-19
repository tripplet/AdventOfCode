use std::collections::{HashMap, HashSet, VecDeque};

use aoc_runner_derive::{aoc, aoc_generator};
use glam::I8Vec2;

type ParseResult = Vec<I8Vec2>;

const NEIGHBORS: [I8Vec2; 4] = [I8Vec2::X, I8Vec2::Y, I8Vec2::NEG_X, I8Vec2::NEG_Y];

#[aoc_generator(day18)]
pub fn parse_input(input: &str) -> ParseResult {
    input
        .trim()
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(',').unwrap();
            I8Vec2::new(x.parse().unwrap(), y.parse().unwrap())
        })
        .collect()
}

fn neighbors(pos: I8Vec2, max: I8Vec2) -> impl Iterator<Item = I8Vec2> {
    NEIGHBORS.iter().filter_map(move |&dxy| {
        let new_pos = pos + dxy;
        if new_pos.x < 0 || new_pos.y < 0 || new_pos.x > max.x || new_pos.y > max.y {
            return None;
        }

        Some(new_pos)
    })
}

fn dijkstra(input: &ParseResult, mut parse_first: u16, max: I8Vec2) -> Option<(u32, HashSet<I8Vec2>)> {
    let mut corrupted = HashSet::new();

    for pos in input {
        corrupted.insert(*pos);
        parse_first -= 1;
        if parse_first == 0 {
            break;
        }
    }

    // Find the shortest path from (0, 0) to (max_x, max_y) (aka Dijkstra)
    let mut queue = vec![];
    queue.push((0u32, I8Vec2::ZERO));
    let mut visited = HashMap::new();
    visited.insert(I8Vec2::ZERO, (0u32, I8Vec2::ZERO));

    while let Some((distance, pos)) = queue.pop() {
        for neighbor in neighbors(pos, max) {
            if corrupted.contains(&neighbor) {
                continue;
            }

            if visited.contains_key(&neighbor) {
                continue;
            }

            visited.insert(neighbor, (distance + 1, pos));

            if neighbor == max {
                return Some((distance + 1, collect_path(&visited, max)));
            }

            queue.push((distance + 1, neighbor));
        }

        queue.sort_unstable_by(|a, b| b.0.cmp(&a.0));
    }

    None
}

fn collect_path(visited: &HashMap<I8Vec2, (u32, I8Vec2)>, end: I8Vec2) -> HashSet<I8Vec2> {
    let mut path = HashSet::new();
    let mut current = end;

    while current != I8Vec2::ZERO {
        path.insert(current);
        (_, current) = *visited.get(&current).unwrap();
    }

    path
}

fn find_first_blocking_pos(input: &ParseResult, start_with: u16, max: I8Vec2) -> I8Vec2 {
    let (_, mut current_path) = dijkstra(input, start_with, max).unwrap();

    for new_blocking_byte in (start_with + 1)..input.len() as u16 {
        if !current_path.contains(&input[new_blocking_byte as usize]) {
            // The new blocking byte does not block the current path, so we can skip the dijkstra
            continue;
        }

        let Some((_, new_path)) = dijkstra(input, new_blocking_byte, max) else {
            return input[(new_blocking_byte) as usize];
        };

        current_path = new_path;
    }

    panic!("No blocking position found");
}

#[aoc(day18, part1)]
pub fn part1(input: &ParseResult) -> u32 {
    dijkstra(input, 1024, I8Vec2::new(70, 70)).unwrap().0
}

#[aoc(day18, part2, brute_force)]
pub fn part2(input: &ParseResult) -> String {
    let pos = find_first_blocking_pos(&input, 1024, I8Vec2::new(70, 70));
    format!("{},{}", pos.x, pos.y)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../input/2024/day18_example.txt");
    const INPUT: &str = include_str!("../input/2024/day18.txt");

    #[test]
    fn example_part1() {
        let input = parse_input(EXAMPLE);
        assert_eq!(dijkstra(&input, 12, I8Vec2::new(6, 6)).unwrap().0, 22);
    }

    #[test]
    fn input_part1() {
        let input = parse_input(INPUT);
        assert_eq!(part1(&input), 284);
    }

    #[test]
    fn example_part2() {
        let input = parse_input(EXAMPLE);
        assert_eq!(find_first_blocking_pos(&input, 12, I8Vec2::new(6, 6)), I8Vec2::new(6, 1));
    }

    #[test]
    fn input_part2() {
        let input = parse_input(INPUT);
        assert_eq!(part2(&input), "51,50");
    }
}
