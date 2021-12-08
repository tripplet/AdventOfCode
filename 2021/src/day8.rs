const INPUT: &str = include_str!("../input/2021/day8.txt");

// Ugly mega Vector
type Data = Vec<Vec<Vec<Vec<char>>>>;

pub fn main() {
    let data = parse_input(INPUT);

    println!("Part1: {}", part1(&data));
    //println!("Part2: {}", part2(&data));
}

fn parse_input(input: &str) -> Data {
    input
        .trim()
        .lines()
        .map(|line| {
            line.split("|")
                .map(|digits| {
                    digits
                        .trim()
                        .split_whitespace()
                        .map(|digits| digits.chars().collect::<Vec<_>>())
                        .collect::<Vec<_>>()
                }).collect::<Vec<_>>()
        }).collect::<Data>()
}

fn part1(data: &Data) -> usize {
    data.iter().fold(0, |sum, line| {
        sum + line[1]
            .iter().map(|digits| {
                let len = digits.len();
                if len == 2 || len == 4 || len == 3 || len == 7 { 1 } else { 0 }
            })
            .sum::<usize>()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../input/2021/day8_example.txt");

    #[test]
    fn part1_example() { assert_eq!(26, part1(&parse_input(EXAMPLE))); }

    //#[test]
    fn part2_example() {
        //assert_eq!(, part2(&parse_input(EXAMPLE)));
    }

    //#[test]
    fn part1_on_input() {
        //assert_eq!(, part1(&parse_input(INPUT)));
    }

    //#[test]
    fn part2_on_input() {
        //assert_eq!(, part2(&parse_input(INPUT)));
    }
}
