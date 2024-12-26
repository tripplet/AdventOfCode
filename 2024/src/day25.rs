use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug)]
pub struct ParseResult {
    keys: Vec<Key>,
    locks: Vec<Lock>,
}

#[derive(Debug, Clone)]
pub struct Key([i8; 5]);

#[derive(Debug, Clone)]
pub struct Lock([i8; 5]);

impl Key {
    fn fits_in(&self, lock: &Lock) -> bool {
        self.0.iter().zip(lock.0.iter()).all(|(&k, &l)| k + l <= 5)
    }
}

#[aoc_generator(day25)]
pub fn parse_input(input: &str) -> ParseResult {
    let mut keys = vec![];
    let mut locks = vec![];

    input.trim().replace('\r', "").split("\n\n").for_each(|s| {
        let mut lines = s.lines();

        if lines.next().unwrap() == "....." {
            // Key
            let mut row_nb = 5;
            let mut key = [0; 5];

            let lines = lines.collect::<Vec<_>>();

            for line in lines {
                for (column, c) in line.chars().enumerate() {
                    if c == '#' && key[column] == 0 {
                        key[column] = row_nb;
                    }
                }
                row_nb -= 1;
            }

            keys.push(Key(key));
        } else {
            // Lock
            let mut lock = [0; 5];

            for line in lines {
                for (column, c) in line.chars().enumerate() {
                    if c == '#' {
                        lock[column] += 1;
                    }
                }
            }

            locks.push(Lock(lock));
        }
    });

    ParseResult { keys, locks }
}

#[aoc(day25, part1)]
pub fn part1(input: &ParseResult) -> usize {
    itertools::iproduct!(input.keys.iter(), input.locks.iter())
        .filter(|(key, lock)| key.fits_in(lock))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../input/2024/day25.txt");

    #[test]
    fn input_part1() {
        let input = parse_input(INPUT);
        assert_eq!(part1(&input), 3317);
    }
}
