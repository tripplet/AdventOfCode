use std::cmp;
use std::error::Error;

pub fn main() -> Result<(), Box<dyn Error>> {
    let input: Vec<&str> = include_str!("../input/2020/day5.txt").trim().lines().collect();

    assert_eq!(parse_seatpass("BFFFBBFRRR"), (70, 7, 567));
    assert_eq!(parse_seatpass("FFFBBBFRRR"), (14, 7, 119));
    assert_eq!(parse_seatpass("BBFFBBFRLL"), (102, 4, 820));

    println!("Part1: {}", part1(&input));
    println!("Part2: {:?}", part2(&input));

    Ok(())
}

fn part1(passes: &Vec<&str>) -> usize {
    passes
        .iter()
        .map(|s| parse_seatpass(s).2)
        .fold(0, |a, b| cmp::max(a, b))
}

fn part2(passes: &Vec<&str>) -> Option<usize> {
    let mut parsed: Vec<(usize, usize, usize)> = passes.iter().map(|s| parse_seatpass(s)).collect();
    parsed.sort_by(|a, b| a.2.cmp(&b.2));

    for idx in 0..parsed.len() - 1 {
        if parsed[idx + 1].2 != parsed[idx].2 + 1 {
            return Some(parsed[idx].2 + 1);
        }
    }

    None
}

fn parse_seatpass(pass: &str) -> (usize, usize, usize) {
    let mut row = 0;
    let mut row_bit = 6;

    let mut col = 0;
    let mut col_bit = 2;

    for c in pass.chars() {
        match c {
            'R' => {
                col += 1 << col_bit;
                col_bit -= 1
            }
            'L' => col_bit -= 1,

            'B' => {
                row += 1 << row_bit;
                row_bit -= 1
            }
            'F' => row_bit -= 1,

            _ => panic!(),
        }
    }

    (row, col, row * 8 + col)
}
