mod utils;

mod day22;
use day22::*;

const INPUT: &str = include_str!("../input/2022/day22_example.txt");
//const INPUT_EXAMPLE: &str = include_str!("../input/2022/day1.txt");

fn main() {
    let mut now = std::time::Instant::now();

    let data = parse_input(INPUT);
    println!("Parsing [{}]\n", humantime::format_duration(now.elapsed()));

    now = std::time::Instant::now();
    let results_part1 = part1(&data);
    println!("Part1: {} [{}]", results_part1, humantime::format_duration(now.elapsed()));

    now = std::time::Instant::now();
    let results_part1 = part2(&data);
    println!("Part2: {} [{}]", results_part1, humantime::format_duration(now.elapsed()));
}