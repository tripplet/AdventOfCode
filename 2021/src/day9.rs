use itertools::Itertools;

const INPUT: &str = include_str!("../input/2021/day9.txt");

type Data = Vec<Vec<u8>>;

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

fn get_adjescent(leny: usize, lenx: usize, point: (usize, usize)) -> Vec<(usize, usize)> {
    let adjescent = [
        if point.0 as isize - 1 >= 0 {Some((point.0-1, point.1))} else { None },
        if point.0 as isize + 1 < leny as isize {Some((point.0+1, point.1))} else { None },
        if point.1 as isize - 1 >= 0 {Some((point.0, point.1-1))} else { None },
        if point.1 as isize + 1 < lenx as isize {Some((point.0, point.1+1))} else { None },
    ];

    adjescent.iter().filter_map(|x| *x).collect()
}

fn find_low_points(data: &Data) -> Vec<(usize, usize)> {
    let leny = data.len();
    let lenx = data[0].len();

    let mut low_points = vec![];

    for y in 0..leny as usize {
        for x in 0..lenx as usize {
            if get_adjescent(leny, lenx, (y, x)).iter().all(|adj| data[adj.0][adj.1] > data[y][x]) {
                low_points.push((y, x));
            }
        }
    }
    low_points
}

fn part2(data: &Data) -> usize {
    let leny = data.len();
    let lenx = data[0].len();
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
            let adjescent = get_adjescent(leny, lenx, cur);

            let mut new_found: Vec<_> = adjescent.into_iter().filter(|p| !new_points.contains(p) && !bassin.contains(p) && data[p.0][p.1] != 9).collect();

            for elem in new_found.iter() {
                if !bassin.contains(elem) {
                    bassin.push(*elem);
                }
            }

            new_points.append(&mut new_found);
        }

        bassin_sizes.push(bassin.len());
    }
    bassin_sizes.iter().sorted().rev().take(3).product()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../input/2021/day9_example.txt");

    #[test]
    fn part1_example() { assert_eq!(15, part1(&parse_input(EXAMPLE))); }

    #[test]
    fn part2_example() { assert_eq!(1134, part2(&parse_input(EXAMPLE))); }

    #[test]
    fn part1_on_input() { assert_eq!(494, part1(&parse_input(INPUT))); }

    #[test]
    fn part2_on_input() { assert_eq!(1048128, part2(&parse_input(INPUT))); }
}
