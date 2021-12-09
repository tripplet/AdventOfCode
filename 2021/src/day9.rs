use itertools::Itertools;

const INPUT: &str = include_str!("../input/2021/day9.txt");
const EXAMPLE: &str = include_str!("../input/2021/day9_example.txt");

type Data = Vec<Vec<u8>>;

struct Map {
    data: Vec<Vec<u8>>
}

pub fn main() {
    let data = parse_input(INPUT);

    println!("Part1: {}", part1(&data));
    println!("Part2: {}", part2(&data));
}

fn parse_input(input: &str) -> Data {
    input
        .trim()
        .lines()
        .map(|line| line.chars().map(|d| d.to_digit(10).unwrap() as u8).collect())
        .collect()
}

fn part1(data: &Data) -> usize {
    find_low_points(data).iter().fold(0, |sum, p| sum + 1 + data[p.0][p.1] as usize)
}

fn find_low_points(data: &Data) -> Vec<(usize, usize)> {
    let leny = data.len() as isize;
    let lenx = data[0].len() as isize;

    let mut low_points = vec![];

    for y in 0..leny as usize {
        for x in 0..lenx as usize {
            let adjescent = [
                if y as isize - 1 >= 0 {Some(data[y-1][x])} else { None },
                if y as isize + 1 < leny {Some(data[y+1][x])} else { None },
                if x as isize - 1 >= 0 {Some(data[y][x-1])} else { None },
                if x as isize + 1 < lenx {Some(data[y][x+1])} else { None },
            ];

            if adjescent.iter().filter_map(|x| *x).all(|adj| adj > data[y][x]) {
                low_points.push((y, x));
            }
        }
    }
    low_points
}

fn part2(data: &Data) -> usize {
    let leny = data.len() as isize;
    let lenx = data[0].len() as isize;
    let low_points = find_low_points(data);

    let mut bassin_sizes = vec![];

    for p in low_points {
        let mut bassin = vec![p];

        let mut new_points = vec![p];
        loop {
            if new_points.len() == 0 {
                break
            }

            let cur = new_points.pop().unwrap();

            let adjescent = [
                if cur.0 as isize - 1 >= 0 {Some((cur.0-1, cur.1))} else { None },
                if cur.0 as isize + 1 < leny {Some((cur.0+1, cur.1))} else { None },
                if cur.1 as isize - 1 >= 0 {Some((cur.0, cur.1-1))} else { None },
                if cur.1 as isize + 1 < lenx {Some((cur.0, cur.1+1))} else { None },
            ];

            let mut newnew: Vec<_> = adjescent.iter().filter_map(|p| *p).filter(|p| !new_points.contains(p) && !bassin.contains(p) && data[p.0][p.1] != 9).collect();

            for elem in newnew.iter() {
                if !bassin.contains(elem) {
                    bassin.push(*elem);
                }
            }

            new_points.append(&mut newnew);
        }

        bassin_sizes.push(bassin.len());
    }

    bassin_sizes.iter().sorted().rev().take(3).product()
}
// fn part2(data: ) -> usize {

// }

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "";

    // #[test]
    // fn part1_example() { assert_eq!(, part1(&parse_input(EXAMPLE).unwrap())); }

    // #[test]
    // fn part2_example() { assert_eq!(, part2(&parse_input(EXAMPLE).unwrap())); }

    // #[test]
    // fn part1_on_input() { assert_eq!(, part1(&parse_input(INPUT).unwrap())); }

    // #[test]
    // fn part2_on_input() { assert_eq!(, part2(&parse_input(INPUT).unwrap())); }
}
