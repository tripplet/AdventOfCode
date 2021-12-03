pub fn main() {
    let input = include_str!("../input/2021/day3.txt");

    let reports: Vec<_> = input
        .trim()
        .lines()
        .map(|line| line.trim())
        .collect();

    println!("Part1: {}", part1(&reports));
    println!("Part2: {}", part1(&reports));
}

fn part1(reports: &[&str]) -> i32 {
    let len = reports[0].len();
    let mut bits: Vec<i32> = vec![0; len];
    
    for report in reports {
        for (pos, bit) in report.chars().enumerate().skip(0) {
            match bit {
                '0' => bits[pos] -= 1,
                '1' => bits[pos] += 1,
                _ => panic!(),
            }
        }
    }

    // take all positions > 0 and build the number using bit shifts from them
    let gamma_rate = bits.iter().enumerate().filter(|b| *b.1 >= 0).fold(0, |acc, (idx, _)| acc + (1<<len-idx-1));

    // epsilon is just the bit inverse of gamme
    let epsilon_rate = !gamma_rate & ((1<<len)-1);

    gamma_rate * epsilon_rate
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_DATA: &str = "00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010\n";

    #[test]
    fn test_part1() {
        assert_eq!(198, part1(&EXAMPLE_DATA.lines().collect::<Vec<_>>()));
    }

    #[test]
    fn test_part2() {
        assert_eq!(42, part2(&EXAMPLE_DATA.lines().collect::<Vec<_>>()));
    }
}