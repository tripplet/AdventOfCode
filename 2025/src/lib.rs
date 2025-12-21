#![allow(clippy::must_use_candidate)]
#![allow(clippy::missing_panics_doc)]
// Allow some clippy warnings regarding casting
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_sign_loss)]
#![allow(clippy::cast_possible_wrap)]
#![allow(clippy::cast_precision_loss)]

use aoc_runner_derive::aoc_lib;

#[macro_use]
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

// mod utils;

aoc_lib! { year = 2025 }
