use std::fmt::Display;

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

#[derive(Debug, Clone)]
struct Sector {
    len: usize,
    value: i16,
    move_tried: bool,
}

impl Sector {
    fn is_free(&self) -> bool {
        self.value == -1
    }
}

impl Display for Sector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_free() {
            for _ in 0..self.len {
                write!(f, "{}", ".")?;
            }
        } else {
            for _ in 0..self.len {
                write!(f, "{}", self.value)?;
            }
        }

        Ok(())
    }
}

#[aoc(day9, part2)]
pub fn part2(input: &ParseResult) -> usize {
    let mut disk = Vec::with_capacity(input.len());
    for (idx, &nb) in input.iter().enumerate() {
        let value = if idx % 2 == 0 { (idx / 2) as i16 } else { -1i16 };

        if nb == 0 {
            continue;
        }

        disk.push(Sector {
            len: nb as usize,
            value,
            move_tried: false,
        });
    }

    let mut skip_from_end = 0;
    let mut skip_from_start = 0;

    loop {
        let Some(next_data) = disk
            .iter()
            .rev()
            .skip(skip_from_end)
            .position(|sec| !sec.is_free() && !sec.move_tried)
        else {
            break;
        };

        let next_data = disk.len() - next_data - 1 - skip_from_end;
        disk[next_data].move_tried = true;

        let mut skip_from_start_next = 0;
        let mut continuous_non_free_sectors = true;

        // Find a free sector to move the data to
        let free_sector = disk
            .iter()
            .skip(skip_from_start)
            .enumerate()
            .position(|(sector_idx, sector)| {
                if !sector.is_free() {
                    if continuous_non_free_sectors {
                        skip_from_start_next = sector_idx;
                    }
                    return false;
                } else {
                    continuous_non_free_sectors = false;
                }

                sector.len >= disk[next_data].len && sector_idx + skip_from_start < next_data
            });

        if let Some(free_sector) = free_sector {
            let free_sector = free_sector + skip_from_start;
            let remaining_free_len = disk[free_sector].len - disk[next_data].len;

            if remaining_free_len == 0 {
                // just move the sector
                disk[free_sector].value = disk[next_data].value;
                disk[next_data].value = -1;
            } else {
                // split the sector
                disk[free_sector].len = remaining_free_len;
                disk.insert(
                    free_sector,
                    Sector {
                        len: disk[next_data].len,
                        value: disk[next_data].value,
                        move_tried: true,
                    },
                );

                disk[next_data + 1].value = -1;
                skip_from_end += 1;
            }
        }

        skip_from_end += 1;
        skip_from_start = skip_from_start_next;
    }

    // calculate the checksum
    let mut checksum = 0;
    let mut delta = 0;

    for sector in disk.iter() {
        if !sector.is_free() {
            for sector_idx in 0..sector.len {
                checksum += (sector_idx + delta) * sector.value as usize;
            }
        }
        delta += sector.len;
    }

    checksum
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

    #[test]
    fn input_part2() {
        let input = parse_input(INPUT);
        assert_eq!(part2(&input), 6239783302560);
    }
}
