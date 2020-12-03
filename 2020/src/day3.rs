use std::error::Error;
use std::fs;
use std::time::Instant;

pub fn main() -> Result<(), Box<dyn Error>> {
    let mut now = Instant::now();
    let map = parse_file();
    println!("parsing took: {}", humantime::format_duration(now.elapsed()));

    let example_result = get_part1(&parse_input(&fs::read_to_string("./input/2020/day3_example.txt")?), 1, 3);
    println!("Example: {}", example_result);
    assert_eq!(example_result, 7);

    now = Instant::now();
    println!("Part1: {},  [{}]", get_part1(&map, 1, 3), humantime::format_duration(now.elapsed()));

    now = Instant::now();
    let slopes = [(1, 1), (1, 3), (1, 5), (1, 7), (2, 1)];
    let part2_result: usize = slopes.iter().map(|s| get_part1(&map, s.0, s.1)).product();

    println!("Part2: {},  [{}]", part2_result, humantime::format_duration(now.elapsed()));

    Ok(())
}

pub fn get_part1(map: &Vec<Vec<bool>>, delta_y: usize, delta_x: usize) -> usize {
    let mut tree_count = 0;
    let mut y = 0;
    let mut x = 0;

    while y + delta_y <= map.len() {
        if map[y][x] {
            tree_count += 1;
        }

        y = y + delta_y;
        x = (x + delta_x) % map[0].len();
    }

    tree_count
}

pub fn parse_file() -> Vec<Vec<bool>> {
    parse_input(include_str!("../input/2020/day3.txt"))
}

pub fn parse_input(input: &str) -> Vec<Vec<bool>> {
    let lines = input.trim().lines();

    let mut map: Vec<Vec<bool>> = Vec::new();

    for line in lines {
        map.push(line.chars().map(|c| c == '#').collect());
    }

    map
}
