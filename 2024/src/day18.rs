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

fn dijkstra(input: &ParseResult, mut parse_first: u16, max: I8Vec2) -> Option<u32> {
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
    visited.insert(I8Vec2::ZERO, 0u32);

    while let Some((distance, pos)) = queue.pop() {
        for neighbor in neighbors(pos, max) {
            if corrupted.contains(&neighbor) {
                continue;
            }

            if visited.contains_key(&neighbor) {
                continue;
            }

            if neighbor == max {
                return Some(distance + 1);
            }

            visited.insert(neighbor, distance + 1);
            queue.push((distance + 1, neighbor));
        }

        queue.sort_unstable_by(|a, b| b.0.cmp(&a.0));
    }

    None
}

fn find_first_blocking_pos(input: &ParseResult, start_with: u16, max: I8Vec2) -> I8Vec2 {
    for blocking_byte in start_with..=input.len() as u16 {
        if dijkstra(input, blocking_byte, max).is_none() {
            return input[(blocking_byte -1)  as usize];
        }
    }

    panic!("No blocking position found");
}

#[aoc(day18, part1)]
pub fn part1(input: &ParseResult) -> u32 {
    dijkstra(input, 1024, I8Vec2::new(70, 70)).unwrap()
}

#[aoc(day18, part2, brute_force)]
pub fn part2(input: &ParseResult) -> String {
    let pos = find_first_blocking_pos(&input, 1024, I8Vec2::new(70, 70));
    format!("{},{}", pos.x, pos.y)
}

#[cfg(test)]
mod tests {
    use glam::IVec2;

    use super::*;

    const EXAMPLE: &str = include_str!("../input/2024/day18_example.txt");
    const INPUT: &str = include_str!("../input/2024/day18.txt");

    #[test]
    fn example_part1() {
        let input = parse_input(EXAMPLE);
        assert_eq!(dijkstra(&input, 12, I8Vec2::new(6, 6)), Some(22));
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
