fn main() {
    let data = parse(include_str!("../input/2020/day13.txt")).unwrap();

    let mut now = std::time::Instant::now();
    let part1 = part1(&data);
    println!("Part1: {}  [{}]", part1, humantime::format_duration(now.elapsed()));
    assert_eq!(part1, 4722);

    now = std::time::Instant::now();
    let part2 = part2(&data);
    println!("Part2: {}  [{}]", part2, humantime::format_duration(now.elapsed()));
    assert_eq!(part2, 825305207525452);
}

pub fn parse(input: &str) -> Option<(u64, Vec<(usize, u64)>)> {
    let mut lines = input.trim().lines();
    let leave_earliest = lines.next()?.trim().parse::<u64>().ok();

    let values = lines.next()?
        .trim()
        .split(",")
        .enumerate()
        .filter(|n| n.1 != "x")
        .map(|n| (n.0, n.1.parse().unwrap()))
        .collect();

    Some((leave_earliest?, values))
}

pub fn part1(data: &(u64, Vec<(usize, u64)>)) -> u64 {
    let result = data.1.iter().map(|bus| (bus.1, bus.1 - (data.0 % bus.1))).min_by(|a, b| a.1.cmp(&b.1)).unwrap();
    result.0 * result.1
}

pub fn part2(data: &(u64, Vec<(usize, u64)>)) -> u64 {
    let mut prev = data.1[0].1;
    let mut result = 0;

    for elem in data.1.iter().skip(1) {
        let mut mult = 0;
        for x in 1.. {
            if ((result + x*prev) + elem.0 as u64) % elem.1 == 0 as u64 {
                mult = x;
                break;
            }
        }

        result += mult*prev;
        prev = num::integer::lcm(prev, elem.1);
    }

    result
}