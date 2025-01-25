use std::collections::{BinaryHeap, HashMap, HashSet};

use aoc_runner_derive::{aoc, aoc_generator};
use glam::{i16vec2, I16Vec2};
use tinyvec::tiny_vec;

type Number = u32;

const NEIGHBORS: [I16Vec2; 4] = [I16Vec2::X, I16Vec2::Y, I16Vec2::NEG_X, I16Vec2::NEG_Y];

#[derive(Debug)]
pub struct ParseResult {
    maze: HashSet<I16Vec2>,
    start: I16Vec2,
    finish: I16Vec2,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
struct Position {
    pos: I16Vec2,
    direction: I16Vec2,
    score: Number,
}

impl Ord for Position {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.score.cmp(&self.score)
    }
}

impl PartialOrd for Position {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[aoc_generator(day16)]
pub fn parse_input(input: &str) -> ParseResult {
    let mut start = I16Vec2::ZERO;
    let mut finish = I16Vec2::ZERO;
    let mut maze = HashSet::new();

    for (y, line) in input.trim().lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                'S' => {
                    start = i16vec2(x as i16, y as i16);
                }
                'E' => {
                    finish = i16vec2(x as i16, y as i16);
                }
                _ => {}
            }

            if c == '.' || c == 'S' || c == 'E' {
                maze.insert(i16vec2(x as i16, y as i16));
            }
        }
    }

    ParseResult { maze, start, finish }
}

#[aoc(day16, part1)]
pub fn part1(input: &ParseResult) -> Number {
    dijkstra(input)
}

#[aoc(day16, part2)]
pub fn part2(input: &ParseResult) -> Number {
    dijkstra_all_paths(input)
}

fn dijkstra(input: &ParseResult) -> Number {
    let mut queue = BinaryHeap::new();
    let mut visited = HashMap::new();

    // Initial position at 'start' with direction east
    let start = Position {
        pos: input.start,
        direction: I16Vec2::X,
        score: 0,
    };
    visited.insert(start.pos, 0);
    queue.push(start);

    while let Some(pos) = queue.pop() {
        for neighbor in input.neighbors(&pos) {
            if neighbor.pos == input.finish {
                return neighbor.score;
            }

            if let Some(&old_score) = visited.get(&neighbor.pos) {
                if neighbor.score < old_score {
                    visited.insert(neighbor.pos, neighbor.score);
                    queue.push(neighbor);
                }
            } else {
                visited.insert(neighbor.pos, neighbor.score);
                queue.push(neighbor);
            }
        }
    }

    unreachable!()
}

fn dijkstra_all_paths(input: &ParseResult) -> Number {
    let mut queue = BinaryHeap::new();
    let mut finish = Position::default();
    let mut visited = HashMap::new();

    // Initial position at 'start' with direction east
    let start = Position { pos: input.start, direction: I16Vec2::X, score: 0 };
    visited.insert(start.clone(), (0, tiny_vec!([(Number, Position); 2])));
    queue.push(start);

    while let Some(current_pos) = queue.pop() {
        dbg!(&current_pos);
        for neighbor in input.neighbors(&current_pos.clone()) {
            if let Some((old_score, previous)) = visited.get_mut(&neighbor) {
                if *old_score > neighbor.score {
                    *old_score = neighbor.score;
                    queue.push(neighbor.clone());
                }
                previous.push((neighbor.score, current_pos.clone()));
            } else {
                visited.insert(neighbor.clone(), (neighbor.score, tiny_vec!([(Number, Position); 2] => (neighbor.score, current_pos.clone()))));
                queue.push(neighbor.clone());
            }

            if neighbor.pos == input.finish {
                finish = neighbor;
            }
        }
    }

    // Run the path backwards
    let mut current = finish;
    let mut steps = 0;

    while current.pos != input.start {
        let (_, previous) = &visited[&current];
        println!("{current:?}");
        current = previous[0].1.clone();

        if previous.len() != 1 {
            println!("Multiple paths: {previous:?} @ {current:?}");
        }
        steps += 1;
    }

    println!("{current:?}");

    // for y in 0..15 {
    //     for x in 0..15 {
    //         let pos = i16vec2(x, y);
    //         if let Some((dist, _)) = visited.get(&pos) {
    //             print!("{dist:04} ");
    //         } else if pos == input.start {
    //             print!("S");
    //         } else if pos == input.finish {
    //             print!("E");
    //         } else {
    //             print!("---- ");
    //         }
    //     }
    //     println!();
    // }

    steps
}

fn count_backwards_path_length(
    mut current: I16Vec2,
    visited: &HashMap<I16Vec2, (Number, Vec<I16Vec2>)>,
    dest: I16Vec2,
) -> Number {
    // Run the path backwards
    let mut steps = 0;

    while current != dest {
        let (_, previous) = &visited[&current];
        if previous.len() == 1 {}

        current = previous[0];
        steps += 1;
    }

    steps
}

impl ParseResult {
    fn neighbors<'s, 'p>(&'s self, pos: &'p Position) -> impl Iterator<Item = Position> + 'p
    where
        's: 'p,
    {
        NEIGHBORS.iter().filter_map(move |new_dir| {
            let new_pos = pos.pos + *new_dir;

            if self.maze.contains(&new_pos) {
                Some(Position {
                    pos: new_pos,
                    direction: *new_dir,
                    score: pos.score + rotation_score(pos.direction, *new_dir) + 1,
                })
            } else {
                None
            }
        })
    }
}

fn rotation_score(a: I16Vec2, b: I16Vec2) -> Number {
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
    fn examples_part1() {
        assert_eq!(part1(&parse_input(EXAMPLE_1)), 7036);
        assert_eq!(part1(&parse_input(EXAMPLE_2)), 11048);
    }

    #[test]
    fn input_part1() {
        let input = parse_input(INPUT);
        assert_eq!(part1(&input), 91464);
    }

    #[test]
    fn examples_part2() {
        assert_eq!(part2(&parse_input(EXAMPLE_1)), 45);
        //assert_eq!(part2(&parse_input(EXAMPLE_2)), 64);
    }

    #[test]
    fn input_part2() {
        let input = parse_input(INPUT);
        assert_eq!(part2(&input), 1);
    }
}
