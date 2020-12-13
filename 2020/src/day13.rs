fn main() {
    let input = include_str!("../input/2020/day13.txt");
    let data = parse1(input);

    let mut now = std::time::Instant::now();
    println!("Part1: {}  [{}]", part1(&data), humantime::format_duration(now.elapsed()));

    now = std::time::Instant::now();
    let data2 = parse2(input);
    //let data2 = parse2("1\n17,x,13,19");
    //let data2 = parse2("1\n67,7,59,61");

    println!("Part2: {}  [{}]", part2(&data2), humantime::format_duration(now.elapsed()));
}

pub fn parse1(input: &str) -> (u64, Vec<u64>) {
    let mut lines = input.trim().lines();
    let leave_earliest = lines.next().unwrap().trim().parse::<u64>().unwrap();

    (leave_earliest, lines.next().unwrap().trim().split(",").filter_map(|n| n.parse::<u64>().ok()).collect())
}

pub fn parse2(input: &str) -> Vec<(usize, u64)> {
    input.trim()
        .lines()
        .nth(1)
        .unwrap()
        .trim()
        .split(",")
        .enumerate()
        .filter(|n| n.1 != "x")
        .map(|n| (n.0, n.1.parse().unwrap()))
        .collect()
}

pub fn part1(data: &(u64, Vec<u64>)) -> u64 {
    let result = data.1.iter().map(|bus| (bus, bus - (data.0 % bus))).min_by(|a, b| a.1.cmp(&b.1)).unwrap();
    result.0 * result.1
}

pub fn part2(data: &Vec<(usize, u64)>) -> u64 {
    let mut prev = data[0].1;
    let mut result = 0;

    for elem in data.iter().skip(1) {
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