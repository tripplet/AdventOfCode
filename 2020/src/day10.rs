use num::bigint::BigInt;
use num::bigint::ToBigInt;
use num::traits::One;

fn binom(n: u64, k: u64) -> BigInt {
    let mut res = BigInt::one();
    for i in 0..k {
        res = (res * (n - i).to_bigint().unwrap()) / (i + 1).to_bigint().unwrap();
    }
    res
}

pub fn main() {
    let input = include_str!("../input/2020/day10.txt").trim();

    let numbers = parse(input).unwrap();
    drop(input);

    let mut now = std::time::Instant::now();
    let part1_solution = part1(&numbers);
    println!(
        "Part1: {:?}  [{}]",
        part1_solution.0 * part1_solution.1,
        humantime::format_duration(now.elapsed())
    );

    //assert_eq!(part1_solution, 57195069);

    now = std::time::Instant::now();
    println!(
        "Part2: {}  [{}]",
        part2(&numbers),
        humantime::format_duration(now.elapsed())
    );
}

pub fn parse(input: &str) -> Result<Vec<i64>, std::num::ParseIntError> {
    input
        .trim()
        .lines()
        .map(|l| l.trim().parse::<i64>())
        .collect()
}

pub fn part1(numbers: &[i64]) -> (i64, i64) {
    let mut sorted = numbers.clone().to_vec();
    sorted.sort();

    let mut deltas: Vec<_> = sorted[..sorted.len() - 1]
        .iter()
        .zip(sorted[1..].iter())
        .map(|(a, b)| b - a)
        .collect();

    deltas.insert(0, sorted[0]);

    let mut ones = 0;
    let mut threes = 0;

    for elem in &deltas {
        if *elem == 1 {
            ones += 1;
        }
        if *elem == 3 {
            threes += 1;
        }
    }

    println!("{:?}", &sorted);
    println!("{:?}", &deltas);

    (ones, threes + 1)
}

pub fn part2(numbers: &[i64]) -> BigInt {
    let mut sorted = numbers.clone().to_vec();
    sorted.sort();
    sorted.push(sorted.last().unwrap() + 3);

    let mut deltas: Vec<_> = sorted[..sorted.len() - 1]
        .iter()
        .zip(sorted[1..].iter())
        .map(|(a, b)| b - a)
        .collect();

    deltas.insert(0, sorted[0]);

    let mut ones: Vec<u64> = Vec::new();

    let mut cnt = 0;
    for elem in &deltas {
        if *elem == 1 {
            cnt += 1;
        } else if cnt > 1 {
            ones.push(cnt);
            cnt = 0;
        } else {
            cnt = 0;
        }
    }

    println!("delta: {:?}", &deltas);
    println!("ones: {:?}", ones);

    let x: Vec<_> = ones.iter().map(|o| calc(*o - 1)).collect();

    println!("comb: {:?}", x);

    x.iter().fold(BigInt::one(), |a, b| a * b)
}

fn calc(c: u64) -> BigInt {
    binom(c, 2) + binom(c, 1) + 1
}
