use aoc_runner_derive::{aoc, aoc_generator};

type ParseResult = String;

#[aoc_generator(day15)]
pub fn parse_input(input: &str) -> ParseResult {
    String::from(input.trim())
}

fn calc_hash(input: &str) -> usize {
    input.chars().fold(0, |acc, ch| ((acc + (ch as usize)) * 17) % 256)
}

#[aoc(day15, part1)]
pub fn part1(input: &ParseResult) -> usize {
    input.split(',').map(calc_hash).sum()
}

#[aoc(day15, part2)]
pub fn part2(input: &ParseResult) -> usize {
    let mut boxes: Vec<Vec<(&str, u8)>> = vec![vec![]; 256];

    for command in input.split(',') {
        let (lens_label, part2) = command.split_once(&['=', '-'][..]).expect("AoC input is always valid");
        let target_box = &mut boxes[calc_hash(lens_label)];
        let existing_lens = target_box.iter().position(|(k, _)| *k == lens_label);

        if part2.is_empty() {
            if let Some(found_idx) = existing_lens {
                _ = target_box.remove(found_idx); // Remove the lens
            }
        } else if let Some(found_idx) = existing_lens {
            target_box[found_idx].1 = part2.parse().unwrap(); // Already exists, replace the focal_length
        } else {
            target_box.push((lens_label, part2.parse().unwrap())); // Place new lens in the back
        }
    }

    boxes
        .iter()
        .enumerate()
        .map(|(box_idx, lens_box)| {
            lens_box
                .iter()
                .enumerate()
                .map(|(lens_idx, (_, focal_length))| (box_idx + 1) * (lens_idx + 1) * (*focal_length as usize))
                .sum::<usize>()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../input/2023/day15_example.txt");
    const INPUT: &str = include_str!("../input/2023/day15.txt");

    #[test]
    fn example_part1() {
        let input = parse_input(EXAMPLE);
        assert_eq!(part1(&input), 1320);
    }

    #[test]
    fn input_part1() {
        let input = parse_input(INPUT);
        assert_eq!(part1(&input), 511257);
    }

    #[test]
    fn example_part2() {
        let input = parse_input(EXAMPLE);
        assert_eq!(part2(&input), 145);
    }

    #[test]
    fn input_part2() {
        let input = parse_input(INPUT);
        assert_eq!(part2(&input), 239484);
    }
}
