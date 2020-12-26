use std::collections::HashSet;
use std::str::FromStr;
use strum::IntoEnumIterator;
use strum_macros::{EnumIter, EnumString};

#[derive(Debug, Clone, Copy, PartialEq, EnumString, EnumIter)]
enum Direction {
    #[strum(serialize = "nw")]
    NorthWest,
    #[strum(serialize = "ne")]
    NorthEast,
    #[strum(serialize = "e")]
    East,
    #[strum(serialize = "se")]
    SouthEast,
    #[strum(serialize = "sw")]
    SouthWest,
    #[strum(serialize = "w")]
    West,
}

use Direction::*;
type InputData = Vec<Vec<Direction>>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos {
    x: i32,
    y: i32,
}

fn main() {
    let example_data = parse(include_str!("../input/2020/day24_example.txt"));
    assert_eq!(part1(&example_data), 10);

    let data = parse(include_str!("../input/2020/day24.txt"));

    let now = std::time::Instant::now();
    let part1_result = part1(&data);
    println!("Part1: {}  [{}]", part1_result, humantime::format_duration(now.elapsed()));
    assert_eq!(part1_result, 317);

    assert_eq!(part2(&example_data), 2208);

    let now = std::time::Instant::now();
    let part2_result = part2(&data);
    println!("Part2: {}  [{}]", part2_result, humantime::format_duration(now.elapsed()));
    assert_eq!(part2_result, 3804);
}

fn parse(input: &str) -> InputData {
    input.trim().lines().map(|line| {
        let mut directions = vec![];
        let mut chars = line.chars();

        loop {
            let next_char = chars.next();
            if let Some(next_char) = next_char {
                directions.push(match next_char {
                    'e' => East,
                    'w' => West,
                    _ => Direction::from_str(
                        [next_char, chars.next().unwrap()]
                            .iter()
                            .collect::<String>()
                            .as_str(),
                    )
                    .unwrap(),
                });
            } else {
                break;
            }
        }
        directions
    })
    .collect()
}

fn part1(directions: &InputData) -> usize {
    let mut black = HashSet::new();

    directions.iter().map(|dir| get_end_coor(dir)).for_each(|end| {
        if !black.insert(end) {
            black.remove(&end);
        }
    });
    black.len()
}

fn part2(directions: &InputData) -> u32 {
    let mut blacks = HashSet::new();

    let ends: Vec<_> = directions.iter().map(|dir| get_end_coor(dir)).collect();

    // Get initial black coordinates
    for end in ends {
        if !blacks.insert(end) {
            blacks.remove(&end);
        }
    }

    let all_directions = Direction::iter().collect::<Vec<_>>();

    for _ in 0..100 {
        let mut blacks_copy: HashSet<_> = blacks.clone();

        // black -> white
        for cur in &blacks {
            let mut adjacent = 0;
            for dir in &all_directions {
                if blacks.contains(&get_in_direction(*cur, *dir)) {
                    adjacent += 1;
                }
            }

            if adjacent == 0 || adjacent > 2 {
                blacks_copy.remove(&cur);
            }
        }

        // white -> black
        for cur_black_tile in &blacks {
            for dir in &all_directions {
                let cur_white_tile = get_in_direction(*cur_black_tile, *dir);
                if blacks.contains(&cur_white_tile) {
                    continue;
                }

                let mut adjacent = 0;
                for dir2 in &all_directions {
                    if blacks.contains(&get_in_direction(cur_white_tile, *dir2)) {
                        adjacent += 1;
                    }
                }

                if adjacent == 2 {
                    blacks_copy.insert(cur_white_tile);
                }
            }
        }

        blacks = blacks_copy;
    }

    blacks.len() as u32
}

fn get_in_direction(pos: Pos, dir: Direction) -> Pos {
    match dir {
        East => Pos {x: pos.x + 1, y: pos.y},
        West => Pos {x: pos.x - 1, y: pos.y},
        NorthEast => Pos {x: pos.x + num::abs(pos.y) % 2, y: pos.y + 1},
        SouthEast => Pos {x: pos.x + num::abs(pos.y) % 2, y: pos.y - 1},
        NorthWest => Pos {x: pos.x + (num::abs(pos.y) % 2) - 1, y: pos.y + 1},
        SouthWest => Pos {x: pos.x + (num::abs(pos.y) % 2) - 1, y: pos.y - 1},
    }
}

fn get_end_coor(directions: &Vec<Direction>) -> Pos {
    let mut pos = Pos { x: 0, y: 0 };

    for dir in directions {
        pos = get_in_direction(pos, *dir);
    }
    pos
}
