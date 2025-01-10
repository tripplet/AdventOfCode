use std::collections::{HashMap, HashSet};

use aoc_runner_derive::{aoc, aoc_generator};
use glam::{ivec2, IVec2};

type Number = u32;

const NEIGHBORS: [IVec2; 4] = [IVec2::X, IVec2::Y, IVec2::NEG_X, IVec2::NEG_Y];

#[derive(Debug)]
pub struct ParseResult {
    maze: HashSet<IVec2>,
    start: IVec2,
    finish: IVec2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    pos: IVec2,
    direction: IVec2,
}

#[aoc_generator(day16)]
pub fn parse_input(input: &str) -> ParseResult {
    let mut start = IVec2::ZERO;
    let mut finish = IVec2::ZERO;
    let mut maze = HashSet::new();

    for (y, line) in input.trim().lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                'S' => {
                    start = ivec2(x as i32, y as i32);
                }
                'E' => {
                    finish = ivec2(x as i32, y as i32);
                }
                _ => {}
            }

            if c == '.' || c == 'S' || c == 'E' {
                maze.insert(ivec2(x as i32, y as i32));
            }
        }
    }

    ParseResult { maze, start, finish }
}

#[aoc(day16, part1)]
pub fn part1(input: &ParseResult) -> usize {
    dijkstra(input)
}

#[aoc(day16, part2)]
pub fn part2(input: &ParseResult) -> isize {
    todo!()
}



fn dijkstra(input: &ParseResult) -> usize {
    let mut queue = vec![];
    let mut visited = HashMap::new();

    // Initial position at 'start' with direction east
    queue.push((0u32, Position { pos: input.start, direction: IVec2::X }));
    visited.insert(queue[0].1.pos, 0);

    while let Some((distance, pos)) = queue.pop() {
        for (score, neighbor) in input.neighbors(&pos) {
            let new_distance = distance + score;

            if neighbor.pos == input.finish {
                return new_distance as usize;
            }

            if let Some(&old_distance) = visited.get(&neighbor.pos) {
                if new_distance < old_distance {
                    visited.insert(neighbor.pos, new_distance);
                    queue.push((new_distance, neighbor));
                }
            } else {
                visited.insert(neighbor.pos, new_distance);
                queue.push((new_distance, neighbor));
            }

            queue.sort_unstable_by(|a, b| b.0.cmp(&a.0));
        }
    }

    unreachable!()

}

impl ParseResult {
    fn neighbors<'s, 'p>(&'s self, pos: &'p Position) -> impl Iterator<Item = (Number, Position)> + 'p
    where
        's: 'p,
    {
        NEIGHBORS.iter().filter_map(move |new_dir| {
            let new_pos = pos.pos + *new_dir;

            if self.maze.contains(&new_pos) {
                Some((rotation_score(pos.direction, *new_dir) + 1, Position { pos: new_pos, direction: *new_dir }))
            } else {
                None
            }
        })
    }
}

fn rotation_score(a: IVec2, b: IVec2) -> Number {
    if a == b {
        0
    } else if a == -b {
        2000
    } else {
        1000
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = include_str!("../input/2024/day16_example1.txt");
    const EXAMPLE_2: &str = include_str!("../input/2024/day16_example2.txt");
    const INPUT: &str = include_str!("../input/2024/day16.txt");

    #[test]
    fn example_1_part1() {
        let input = parse_input(EXAMPLE_1);
        assert_eq!(part1(&input), 7036);
    }

    #[test]
    fn example_2_part1() {
        let input = parse_input(EXAMPLE_2);
        assert_eq!(part1(&input), 11048);
    }

    #[test]
    fn input_part1() {
        let input = parse_input(INPUT);
        assert_eq!(part1(&input), 91464);
    }

    //#[test]
    fn example_part2() {
        let input = parse_input(EXAMPLE_1);
        assert_eq!(part2(&input), todo!());
    }

    //#[test]
    fn input_part2() {
        let input = parse_input(INPUT);
        assert_eq!(part2(&input), todo!());
    }
}
