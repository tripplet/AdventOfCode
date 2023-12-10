use std::cmp::Ordering;
use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use nom::bytes::complete::{tag, take};
use nom::character::complete::{line_ending, u16};
use nom::combinator::map;

use nom::multi::{count, separated_list1};
use nom::sequence::separated_pair;
use nom::IResult;

type ParseResult = Vec<Deck>;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[rustfmt::skip]
pub enum Card { _2, _3, _4, _5, _6, _7, _8, _9, T, J, Q, K, A }

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[rustfmt::skip]
pub enum CardJoker { J, _2, _3, _4, _5, _6, _7, _8, _9, T, Q, K, A }

fn map_part1_part2(c: &Card) -> CardJoker {
    match c {
        Card::_2 => CardJoker::_2,
        Card::_3 => CardJoker::_3,
        Card::_4 => CardJoker::_4,
        Card::_5 => CardJoker::_5,
        Card::_6 => CardJoker::_6,
        Card::_7 => CardJoker::_7,
        Card::_8 => CardJoker::_8,
        Card::_9 => CardJoker::_9,
        Card::T => CardJoker::T,
        Card::J => CardJoker::J,
        Card::Q => CardJoker::Q,
        Card::K => CardJoker::K,
        Card::A => CardJoker::A,
    }
}

impl std::fmt::Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Card::_2 => write!(f, "2"),
            Card::_3 => write!(f, "3"),
            Card::_4 => write!(f, "4"),
            Card::_5 => write!(f, "5"),
            Card::_6 => write!(f, "6"),
            Card::_7 => write!(f, "7"),
            Card::_8 => write!(f, "8"),
            Card::_9 => write!(f, "9"),
            Card::T => write!(f, "T"),
            Card::J => write!(f, "J"),
            Card::Q => write!(f, "Q"),
            Card::K => write!(f, "K"),
            Card::A => write!(f, "A"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[rustfmt::skip]
pub enum DeckScore { HighCard, OnePair, TwoPair, ThreeOfAKind, FullHouse, FourOfAKind, FiveOfAKind }

#[derive(Debug, Clone, Eq)]
pub struct Deck {
    cards: [Card; 5],
    bid: u16,
    counted: Option<HashMap<usize, Vec<Card>>>,
    score: Option<DeckScore>,
    part2: bool,
}

impl Card {
    #[rustfmt::skip]
    fn from(c: char) -> Self {
        match c {
            'A' => Card::A, 'K' => Card::K, 'Q' => Card::Q, 'J' => Card::J, 'T' => Card::T,
            '9' => Card::_9, '8' => Card::_8, '7' => Card::_7, '6' => Card::_6, '5' => Card::_5,
            '4' => Card::_4, '3' => Card::_3, '2' => Card::_2,
            _ => panic!("not allowed"),
        }
    }
}

impl DeckScore {
    fn from_deck(deck: &Deck) -> Self {
        let counted = deck.counted.as_ref().unwrap();

        if deck.cards.iter().all_equal() {
            DeckScore::FiveOfAKind
        } else if counted.get(&4).is_some() {
            DeckScore::FourOfAKind
        } else if counted.get(&3).is_some() {
            if counted.get(&2).is_some() {
                DeckScore::FullHouse
            } else {
                DeckScore::ThreeOfAKind
            }
        } else if let Some(pairs) = counted.get(&2) {
            if pairs.len() == 2 {
                DeckScore::TwoPair
            } else {
                DeckScore::OnePair
            }
        } else {
            DeckScore::HighCard
        }
    }

    fn from_deck_part2(deck: &Deck) -> Self {
        let part1 = Self::from_deck(deck);
        let counted = deck.counted.as_ref().unwrap();

        match part1 {
            DeckScore::FourOfAKind => {
                if counted.get(&1).unwrap()[0] == Card::J || counted.get(&4).unwrap()[0] == Card::J {
                    return DeckScore::FiveOfAKind;
                }
            }
            DeckScore::FullHouse => {
                if counted.get(&3).unwrap()[0] == Card::J || counted.get(&2).unwrap()[0] == Card::J {
                    return DeckScore::FiveOfAKind;
                }
            }
            DeckScore::ThreeOfAKind => {
                if deck.cards.iter().any(|c| *c == Card::J) {
                    return DeckScore::FourOfAKind;
                }
            }
            DeckScore::TwoPair => {
                if counted.get(&1).unwrap()[0] == Card::J {
                    return DeckScore::FullHouse;
                } else if counted.get(&2).unwrap().iter().any(|c| *c == Card::J) {
                    return DeckScore::FourOfAKind;
                }
            }
            DeckScore::OnePair => {
                if deck.cards.iter().any(|c| *c == Card::J) {
                    return DeckScore::ThreeOfAKind;
                }
            }
            DeckScore::HighCard => {
                if deck.cards.iter().any(|c| *c == Card::J) {
                    return DeckScore::OnePair;
                }
            }
            _ => {}
        }

        part1
    }
}

impl Deck {
    fn count(&mut self) {
        let mut counted: HashMap<usize, Vec<Card>> = HashMap::with_capacity(5);

        for count in self.cards.iter().counts() {
            if let Some(existing) = counted.get_mut(&count.1) {
                existing.push(*count.0);
            } else {
                counted.insert(count.1, vec![*count.0]);
            }
        }

        self.counted = Some(counted);
    }

    fn score(&mut self) {
        self.score = Some(DeckScore::from_deck(self));
    }
    fn score_part2(&mut self) {
        self.score = Some(DeckScore::from_deck_part2(self));
    }
}

impl PartialEq for Deck {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}

impl PartialOrd for Deck {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Deck {
    fn cmp(&self, other: &Self) -> Ordering {
        let mut res = self.score.cmp(&other.score);
        if res == Ordering::Equal {
            if self.part2 {
                res = compare_cards_part2(&self.cards, &other.cards)
            } else {
                res = compare_cards(&self.cards, &other.cards)
            }
        }

        res
    }
}

#[inline]
fn compare_cards(a: &[Card], b: &[Card]) -> Ordering {
    if a.len() != b.len() {
        panic!("card deck length must be the same")
    }

    let mut result = Ordering::Equal;
    for idx in 0..a.len() {
        result = a[idx].cmp(&b[idx]);

        if result != Ordering::Equal {
            break;
        }
    }
    result
}

#[inline]
fn compare_cards_part2(a: &[Card], b: &[Card]) -> Ordering {
    if a.len() != b.len() {
        panic!("card deck length must be the same")
    }

    let a = a.iter().map(map_part1_part2).collect_vec();
    let b = b.iter().map(map_part1_part2).collect_vec();

    let mut result = Ordering::Equal;
    for idx in 0..a.len() {
        result = a[idx].cmp(&b[idx]);

        if result != Ordering::Equal {
            break;
        }
    }
    result
}

#[aoc_generator(day7)]
pub fn parse_input(input: &str) -> ParseResult {
    let decks: IResult<&str, _> = separated_list1(
        line_ending,
        map(
            separated_pair(
                count(map(take(1usize), |c: &str| Card::from(c.chars().next().unwrap())), 5),
                tag(" "),
                u16,
            ),
            |(cards, bid)| Deck {
                bid,
                cards: std::array::from_fn(|c| cards[c]),
                counted: None,
                score: None,
                part2: false,
            },
        ),
    )(input.trim());

    decks.unwrap().1
}

#[aoc(day7, part1)]
pub fn part1(input: &ParseResult) -> usize {
    let mut decks = input.clone();
    decks.iter_mut().for_each(Deck::count);
    decks.iter_mut().for_each(Deck::score);
    decks.sort();

    // for deck in &decks {
    //     println!("{}, {:?}", deck.cards.iter().join(""), deck.score.as_ref().unwrap());
    // }

    decks
        .iter()
        .enumerate()
        .map(|deck| (deck.0 + 1) * deck.1.bid as usize)
        .sum()
}

#[aoc(day7, part2)]
pub fn part2(input: &ParseResult) -> usize {
    let mut decks = input.clone();

    decks.iter_mut().for_each(|d| d.part2 = true);
    decks.iter_mut().for_each(Deck::count);
    decks.iter_mut().for_each(Deck::score_part2);
    decks.sort();

    for deck in &decks {
        println!("{}, {:?}", deck.cards.iter().join(""), deck.score.as_ref().unwrap());
    }

    decks
        .iter()
        .enumerate()
        .map(|deck| (deck.0 + 1) * deck.1.bid as usize)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../input/2023/day7_example.txt");
    const INPUT: &str = include_str!("../input/2023/day7.txt");

    #[test]
    fn example_part1() {
        let input = parse_input(EXAMPLE);
        assert_eq!(part1(&input), 6440);
    }

    #[test]
    fn input_part1() {
        let input = parse_input(INPUT);
        assert_eq!(part1(&input), 253866470);
    }

    #[test]
    fn example_part2() {
        let input = parse_input(EXAMPLE);
        assert_eq!(part2(&input), 5905);
    }

    #[test]
    fn input_part2() {
        let input = parse_input(INPUT);
        assert_eq!(part2(&input), 42);
    }
}
