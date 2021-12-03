use std::str::FromStr;

pub fn main() {
    let input = include_str!("../input/2021/day4_example.txt");

    let game: BingoGame = input.parse().unwrap();

    println!("Part1: {}", part1(&game));
    //println!("Part2: {}", part2(&numbers, number_of_bits));
}

fn part1(game: &BingoGame) -> usize {
    42
}

#[derive(Debug)]
struct BingoBoard {
    size: u8,
    fields: Vec<u8>,
    checked: Vec<bool>,
}

#[derive(Debug)]
struct BingoGame {
    drawn_numbers: Vec<u8>,
    boards: Vec<BingoBoard>,
}

impl BingoBoard {
    fn array_pos(&self, x: u8, y: u8) -> usize {
        (y * &self.size + x) as usize
    }

    fn get_number(&self, x: u8, y: u8) -> u8 {
        *&self.fields[self.array_pos(x, y)]
    }

    fn is_checked(&self, x: u8, y: u8) -> bool {
        *&self.checked[self.array_pos(x, y)]
    }
}

impl FromStr for BingoBoard {
    type Err = std::num::ParseIntError;

    fn from_str(input: &str) -> Result<BingoBoard, Self::Err> {
        let mut fields = vec![];
        for line in input.trim().lines() {
            let numbers: Result<_, _> = line.split_whitespace().map(|v| v.trim().parse()).collect();
            fields.append(&mut numbers?);
        }

        let size = (fields.len() as f64).sqrt() as u8;
        Ok(BingoBoard {
            fields: fields,
            size: size,
            checked: vec![false; (size * size) as usize],
        })
    }
}

impl FromStr for BingoGame {
    type Err = String;

    fn from_str(input: &str) -> Result<BingoGame, Self::Err> {
        let trimmed_str = input.trim().replace("\r", "");
        let mut sections = trimmed_str.split("\n\n");

        let drawn_numbers: Result<Vec<_>, Self::Err> = sections
            .next()
            .ok_or("no drawn numbers")?
            .split(",")
            .map(|v| v.trim().parse::<u8>().map_err(|e| format!("{}", e)))
            .collect();

        let mut boards = vec![];
        for board in sections {
            boards.push(
                board
                    .parse::<BingoBoard>()
                    .map_err(|e| format!("Error parsing board {}", e))?,
            );
        }

        Ok(BingoGame {
            drawn_numbers: drawn_numbers?,
            boards: boards,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_board() {
        let board_str =
            "22 13 17 11  0\n8  2 23  4 24\n1  9 14 16  7\n6 10  3 18  5\n1 12 20 15 19\n";

        let board: BingoBoard = board_str.parse().unwrap();
        assert_eq!(5, board.size);
        assert_eq!(
            vec![
                22, 13, 17, 11, 0, 8, 2, 23, 4, 24, 1, 9, 14, 16, 7, 6, 10, 3, 18, 5, 1, 12, 20,
                15, 19
            ],
            board.fields
        );
    }

    #[test]
    fn parse_game() {
        let game_str =
            "1,2,3,11,4\n\n22 13 17 11  0\n8  2 23  4 24\n1  9 14 16  7\n6 10  3 18  5\n1 12 20 15 19\n";

        let game: BingoGame = game_str.parse().unwrap();
        assert_eq!(vec![1, 2, 3, 11, 4], game.drawn_numbers);
        assert_eq!(1, game.boards.len());
        assert_eq!(5, game.boards[0].size);
    }
}
