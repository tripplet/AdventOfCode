pub fn main() {
    let input = include_str!("../input/2021/day1.txt");

    let deeps: Vec<_> = input
        .trim()
        .lines()
        .map(|line| line.trim().parse::<usize>().unwrap())
        .collect();

    println!("Part1: {}", part1(&deeps));
    println!("Part2: {} ", part2(&deeps));
}

fn part1(deeps: &[usize]) -> usize {
    deeps.windows(2).map(|x| x[1] > x[0]).filter(|&x| x).count()
}

fn part2(deeps: &[usize]) -> usize {
    part1(&deeps.windows(3).map(|x| x.iter().sum()).collect::<Vec<_>>())
}