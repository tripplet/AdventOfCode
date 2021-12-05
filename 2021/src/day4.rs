use std::str::FromStr;

pub fn main() {
    let input = include_str!("../input/2021/day4.txt");
    let game: BingoGame = input.parse().unwrap();

    println!("Part1: {}", part1(&game));
    println!("Part2: {}", part2(&game));
}

fn part1(game: &BingoGame) -> usize {
    let mut my_game = game.clone();
    let (winner_board, last_number) = my_game.run().unwrap();
    winner_board.get_score() * last_number as usize
}

fn part2(game: &BingoGame) -> usize {
    let mut my_game = game.clone();
    let (last_winner_board, last_number) = my_game.find_last_win_board().unwrap();
    last_winner_board.get_score() * last_number as usize
}

#[derive(Debug, Clone)]
struct BingoBoard {
    size: u8,
    fields: Vec<u8>,
    already_won: bool,
    checked: Vec<bool>,
}

#[derive(Debug, Clone)]
struct BingoGame {
    drawn_numbers: Vec<u8>,
    boards: Vec<BingoBoard>,
}

impl BingoBoard {
    fn try_check_number(&mut self, number: u8) {
        for pos in self
            .fields
            .iter()
            .enumerate()
            .filter(|nb| *nb.1 == number)
            .map(|v| v.0)
            .collect::<Vec<_>>()
        {
            self.checked[pos] = true;
        }
    }

    fn update_win_status(&mut self) -> bool {
        if self.already_won {
            return true;
        }

        // Check all horizontal
        for y in 0..self.size as usize {
            let start = y * self.size as usize;
            if self.checked[start..start + self.size as usize]
                .iter()
                .all(|&v| v)
            {
                self.already_won = true;
                return true;
            }
        }

        // Check all vertical
        for x in 0..self.size {
            if self
                .checked
                .iter()
                .skip(x as usize)
                .step_by(self.size as usize)
                .all(|&v| v)
            {
                self.already_won = true;
                return true;
            }
        }
        false
    }

    fn get_score(&self) -> usize {
        self.fields
            .iter()
            .enumerate()
            .filter(|f| !self.checked[f.0])
            .map(|(_, value)| *value as usize)
            .sum()
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
            already_won: false,
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

impl BingoGame {
    fn run(&mut self) -> Option<(&BingoBoard, u8)> {
        for &nb in self.drawn_numbers.iter() {
            for idx in 0..self.boards.len() {
                let board = &mut self.boards[idx];

                board.try_check_number(nb);

                if board.update_win_status() {
                    return Some((&self.boards[idx], nb));
                }
            }
        }
        None
    }

    fn find_last_win_board(&mut self) -> Option<(&BingoBoard, u8)> {
        let board_count = self.boards.len();
        for &nb in self.drawn_numbers.iter() {
            for idx in 0..board_count {
                let board = &mut self.boards[idx]; {
                    board.try_check_number(nb);
                    board.update_win_status();
                }

                if self.boards.iter().filter(|b| b.already_won).count() == board_count {
                    return Some((&self.boards[idx], nb));
                }
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    impl BingoBoard {
        fn check(&mut self, x: u8, y: u8) {
            self.checked[(y * self.size + x) as usize] = true;
        }
    }

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
    fn part1_example() {
        assert_eq!(
            4512,
            part1(
                &include_str!("../input/2021/day4_example.txt")
                    .parse()
                    .unwrap()
            )
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            1924,
            part2(
                &include_str!("../input/2021/day4_example.txt")
                    .parse()
                    .unwrap()
            )
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

    #[test]
    fn score() {
        let mut board = BingoBoard {
            size: 3,
            already_won: false,
            checked: vec![true; 9],
            fields: (1..=9).collect(),
        };

        board.checked[4] = false;
        board.checked[2] = false;

        assert_eq!(8, board.get_score());
    }

    #[test]
    fn game() {
        let game_str =
            "1,2,3,11,4\n\n22 13 17 11  0\n8  2 23  4 24\n1  9 14 16  7\n6 10  3 18  5\n1 12 20 15 19\n";

        let game: BingoGame = game_str.parse().unwrap();
        assert_eq!(vec![1, 2, 3, 11, 4], game.drawn_numbers);
        assert_eq!(1, game.boards.len());
        assert_eq!(5, game.boards[0].size);
    }

    #[test]
    fn won() {
        let mut board = BingoBoard {
            size: 3,
            already_won: false,
            checked: vec![false; 9],
            fields: vec![0; 9],
        };

        assert_eq!(false, board.update_win_status());
        board.check(0, 0);
        board.check(1, 0);
        board.check(2, 0);
        assert_eq!(true, board.update_win_status());

        let mut board = BingoBoard {
            size: 3,
            already_won: false,
            checked: vec![false; 9],
            fields: vec![0; 9],
        };
        assert_eq!(false, board.update_win_status());
        board.check(0, 2);
        board.check(1, 2);
        board.check(2, 2);
        assert_eq!(true, board.update_win_status());

        board = BingoBoard {
            size: 3,
            already_won: false,
            checked: vec![false; 9],
            fields: vec![0; 9],
        };
        board.check(1, 0);
        board.check(1, 1);
        assert_eq!(false, board.update_win_status());
        board.check(1, 2);
        assert_eq!(true, board.update_win_status());

        board = BingoBoard {
            size: 3,
            already_won: false,
            checked: vec![false; 9],
            fields: vec![0; 9],
        };
        board.check(0, 0);
        board.check(0, 1);
        assert_eq!(false, board.update_win_status());
        board.check(0, 2);
        assert_eq!(true, board.update_win_status());
    }
}
