use aoc_runner_derive::{aoc, aoc_generator};

type Number = u8;
type ParseResult = Vec<Number>;

#[aoc_generator(day9)]
pub fn parse_input(input: &str) -> ParseResult {
    input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as Number)
        .collect()
}

fn create_disk_image(input: &ParseResult) -> Vec<i16> {
    let disk_size = input.iter().map(|&x| x as usize).sum::<usize>();

    let mut disk = vec![-1i16; disk_size];
    let mut pos = 0usize;

    for (idx, &nb) in input.iter().enumerate() {
        let value = if idx % 2 == 0 { (idx / 2) as i16 } else { -1i16 };

        disk[pos..pos + (nb as usize)].as_mut().fill(value);
        pos += nb as usize;
    }

    disk
}

#[aoc(day9, part1)]
pub fn part1(input: &ParseResult) -> usize {
    // create the disk image
    let mut disk = create_disk_image(input);
    let disk_size = disk.len();

    // compress the disk
    let mut pos_free = 0;
    let mut pos_last_data = disk_size;

    loop {
        let next_free = disk.iter().skip(pos_free).position(|x| *x == -1).unwrap();
        pos_free = pos_free + next_free;

        let next_data = disk
            .iter()
            .enumerate()
            .rev()
            .skip(disk_size - pos_last_data)
            .position(|(_, x)| *x != -1)
            .unwrap();
        pos_last_data -= next_data + 1;

        if pos_free > pos_last_data {
            break;
        }

        disk[pos_free] = disk[pos_last_data];
        disk[pos_last_data] = -1;
    }

    // calculate the checksum
    let mut checksum = 0;
    for (idx, &nb) in disk.iter().enumerate() {
        if nb == -1 {
            break;
        }
        checksum += idx * nb as usize;
    }

    checksum
}

#[aoc(day9, part2)]
pub fn part2(input: &ParseResult) -> isize {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../input/2024/day9_example.txt");
    const INPUT: &str = include_str!("../input/2024/day9.txt");

    #[test]
    fn example_part1() {
        let input = parse_input(EXAMPLE);
        assert_eq!(part1(&input), 1928);
    }

    #[test]
    fn input_part1() {
        let input = parse_input(INPUT);
        assert_eq!(part1(&input), 6211348208140);
    }

    #[test]
    fn example_part2() {
        let input = parse_input(EXAMPLE);
        assert_eq!(part2(&input), 2858);
    }

    //#[test]
    fn input_part2() {
        let input = parse_input(INPUT);
        assert_eq!(part2(&input), todo!());
    }
}
