#![allow(clippy::must_use_candidate)]
#![allow(clippy::missing_panics_doc)]

// Allow some clippy warnings regarding casting
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_sign_loss)]
#![allow(clippy::cast_possible_wrap)]
#![allow(clippy::cast_precision_loss)]

use aoc_runner_derive::aoc_lib;

pub mod utils;

pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;
pub mod day09;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;
// pub mod day15;
pub mod day16;
// pub mod day17;
pub mod day18;
pub mod day19;
pub mod day20;
//pub mod day21;
pub mod day22;
pub mod day23;
pub mod day24;
pub mod day25;

// mod utils;

aoc_lib! { year = 2024 }
