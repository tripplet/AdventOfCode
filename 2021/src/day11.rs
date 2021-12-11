use itertools::iproduct;
use ndarray::{Array, Array2, Ix2};

const INPUT: &str = include_str!("../input/2021/day11.txt");

type Data = Array2<u8>;

pub fn main() {
    let data = parse_input(INPUT);

    println!("Part1: {}", part1(&data));
    println!("Part2: {}", part2(&data));
}

fn parse_input(input: &str) -> Data {
    let input_data: Vec<Vec<u8>> = input
        .trim()
        .lines()
        .map(|line| {
            line.chars()
                .filter_map(|c| c.to_digit(10).and_then(|d| Some(d as u8)))
                .collect()
        })
        .collect();

    Array2::from_shape_vec((input_data.len(), input_data[0].len()), input_data.iter().flatten().cloned().collect()).unwrap()
}

fn flash(y: usize, x: usize, octo: &mut Data, flashed: &mut Array2<bool>) {
    flashed[[y, x]] = true;

    for [dy, dx] in [[-1, -1], [0, -1], [1, -1], [1, 0], [1, 1], [0, 1], [-1, 1], [-1, 0]] {
        let y = y as isize + dy;
        let x = x as isize + dx;

        if y < 0 || x < 0 {
            continue;
        }

        if let Some(value) = octo.get_mut([y as usize, x as usize]) {
            if !flashed[[y as usize, x as usize]] {
                *value += 1;
            }
        }
    }
}

fn part1(input_data: &Data) -> usize {
    solve(input_data, false)
}

fn part2(input_data: &Data) -> usize {
    solve(input_data, true)
}

fn solve(input_data: &Data, part2: bool) -> usize {
    let mut octo = input_data.clone();
    let mut flashed = Array::from_elem(octo.shape(), false).into_dimensionality::<Ix2>().unwrap();
    let mut flash_count = 0;

    let it: Box<dyn Iterator<Item = usize>> = if part2 {
        Box::new((0..).into_iter())
    } else {
        Box::new((0..100).into_iter())
    };

    for idx in it {
        flashed.fill(false);
        octo += 1;

        loop {
            let mut any_flash = false;

            for (y, x) in iproduct!(0..octo.shape()[0], 0..octo.shape()[1]) {
                if octo[[y, x]] > 9 && !flashed[[y, x]] {
                    flash(y, x, &mut octo, &mut flashed);
                    any_flash = true;
                    flash_count += 1;
                }
            }

            if !any_flash {
                break;
            }
        }

        octo.iter_mut().filter(|o| **o > 9).for_each(|o| *o = 0);

        if part2 && octo.iter().all(|&o| o == 0) {
            return idx + 1;
        }
    }
    flash_count
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../input/2021/day11_example.txt");

    #[test]
    fn part1_example() { assert_eq!(1656, part1(&parse_input(EXAMPLE))); }

    #[test]
    fn part2_example() { assert_eq!(195, part2(&parse_input(EXAMPLE))); }

    #[test]
    fn part1_on_input() { assert_eq!(1694, part1(&parse_input(INPUT))); }

    #[test]
    fn part2_on_input() { assert_eq!(346, part2(&parse_input(INPUT))); }
}
