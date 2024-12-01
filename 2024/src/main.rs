//mod utils;

//const INPUT: &str = include_str!("../input/2023/day1_example.txt");
//const INPUT: &str = include_str!("../input/2024/day1.txt");

use aoc_runner_derive::aoc_main;

aoc_main! { lib = advent_of_code_2024 }

// fn main() {
//     let mut now = std::time::Instant::now();

//     let data = parse_input(INPUT);
//     println!("Parsing [{}]\n", humantime::format_duration(now.elapsed()));

//     now = std::time::Instant::now();
//     let results_part1 = part1(&data);
//     println!(
//         "Part1: {} [{}]",
//         results_part1,
//         humantime::format_duration(now.elapsed())
//     );

//     now = std::time::Instant::now();
//     let results_part1 = part2(&data);
//     println!(
//         "Part2: {} [{}]",
//         results_part1,
//         humantime::format_duration(now.elapsed())
//     );
// }
