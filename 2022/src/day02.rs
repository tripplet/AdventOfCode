type ParseResult = Result<Vec<(char, char)>, String>;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Hand {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

#[derive(Debug, PartialEq)]
pub struct Game {
    elf: Hand,
    me: Hand,
}

fn parse_part1(hand: char) -> Hand {
    match hand {
        'A' | 'X' => Hand::Rock,
        'B' | 'Y' => Hand::Paper,
        'C' | 'Z' => Hand::Scissors,
        _ => panic!("Invalid char: {}", hand)
    }
}

impl Game {
    fn from_tuple<F>(chars: &(char, char), parse: F) -> Self
        where F: Fn(char) -> Hand
    {
        Game { elf: parse(chars.0), me: parse(chars.1) }
    }

    fn score(&self) -> u8 {
        let mut score = 0;

        // Add score for own hand
        score += self.me as u8;

        // Add score for win/draw/loss
        score += match self {
            // Draw
            Game { elf, me } if elf == me => 3,

            // Wins
            Game { elf: Hand::Rock, me: Hand::Paper, }
            | Game { elf: Hand::Paper, me: Hand::Scissors, }
            | Game { elf: Hand::Scissors, me: Hand::Rock, } => 6,

            // All other cases are a loss
            _ => 0,
        };

        score
    }
}

pub fn parse_input(input: &str) -> ParseResult {
    input
        .trim()
        .lines()
        .map(|line| {
            let mut parts = line.chars();
            Ok((
                parts.nth(0).ok_or("Missing 1st char")?,
                parts.nth(1).ok_or("Missing 2nd char")?,
            ))
        })
        .collect()
}

pub fn part1(input: &ParseResult) -> usize {
    input.as_ref().unwrap()
        .iter()
        .map(|tuple| Game::from_tuple(tuple, parse_part1).score() as usize)
        .sum()
}

pub fn part2(input: &ParseResult) -> usize {
    let mut total_score = 0;

    for game in input.as_ref().unwrap() {
        let elf_hand = parse_part1(game.0);

        let my_hand = match game.1 {
            // Lose
            'X' =>
                match elf_hand {
                    Hand::Rock => Hand::Scissors,
                    Hand::Paper => Hand::Rock,
                    Hand::Scissors => Hand::Paper,
                },

            // Draw
            'Y' => elf_hand,

            // Win
            'Z' =>
                match elf_hand {
                    Hand::Rock => Hand::Paper,
                    Hand::Paper => Hand::Scissors,
                    Hand::Scissors => Hand::Rock,
                },

            _ => panic!("Invalid char: {}", game.1)
        };

        total_score += Game { elf: elf_hand, me: my_hand }.score() as usize
    }

    total_score
}
