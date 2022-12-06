use std::collections::HashSet;

pub fn parse_input(input: &str) -> Vec<char> { input.chars().collect() }

fn search_for_marker<const LENGTH: usize>(msg: &[char]) -> Option<usize> {
    // Create a single HashSet to be reused for performance
    let mut marker_set: HashSet<char> = HashSet::with_capacity(LENGTH);

    for idx in 0..msg.len() - LENGTH {
        marker_set.extend(&msg[idx..idx + LENGTH]);

        if marker_set.len() == LENGTH {
            return Some(idx + LENGTH);
        }

        marker_set.clear();
    }

    None
}

pub fn part1(input: &[char]) -> usize { search_for_marker::<4>(input).unwrap() }
pub fn part2(input: &[char]) -> usize { search_for_marker::<14>(input).unwrap() }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exmaples_part1() {
        assert_eq!(part1(&parse_input("mjqjpqmgbljsphdztnvjfqwrcgsmlb")), 7);
        assert_eq!(part1(&parse_input("bvwbjplbgvbhsrlpgdmjqwftvncz")), 5);
        assert_eq!(part1(&parse_input("nppdvjthqldpwncqszvftbrmjlhg")), 6);
        assert_eq!(part1(&parse_input("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg")), 10);
        assert_eq!(part1(&parse_input("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw")), 11);
    }

    #[test]
    fn test_exmaples_part2() {
        assert_eq!(search_for_marker::<14>(&parse_input("zcfzfwzzqfrljwzlrf")), None);

        assert_eq!(part2(&parse_input("mjqjpqmgbljsphdztnvjfqwrcgsmlb")), 19);
        assert_eq!(part2(&parse_input("bvwbjplbgvbhsrlpgdmjqwftvncz")), 23);
        assert_eq!(part2(&parse_input("nppdvjthqldpwncqszvftbrmjlhg")), 23);
        assert_eq!(part2(&parse_input("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg")), 29);
        assert_eq!(part2(&parse_input("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw")), 26);
    }
}
