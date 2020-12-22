use std::collections::HashSet;

fn main() {
    let player_decks = parse(include_str!("../input/2020/day22.txt"));
    dbg!(part1(&player_decks));
    dbg!(part2(&(player_decks.0.as_slice(), player_decks.1.as_slice())));
}

fn part1(player_decks: &(Vec<u16>, Vec<u16>)) -> usize {
    let mut p1 = player_decks.0.clone();
    let mut p2 = player_decks.1.clone();

    // Run game loop
    while !p1.is_empty() && !&p2.is_empty() {
        if p1[0] > p2[0] {
            p1.push(p1[0]);
            p1.push(p2[0]);
        } else {
            p2.push(p2[0]);
            p2.push(p1[0]);
        }

        p1.remove(0);
        p2.remove(0);
    }

    let winner = if !p1.is_empty() { p1 } else { p2 };
    //dbg!(&winner.iter().rev().enumerate().collect::<Vec<_>>());
    winner
        .iter()
        .rev()
        .enumerate()
        .map(|(idx, card_nb)| *card_nb as usize * (idx + 1))
        .sum()
}

fn part2(player_decks: &(&[u16], &[u16])) -> (bool, usize) {
    let mut p1 = player_decks.0.to_vec();
    let mut p2 = player_decks.1.to_vec();

    let mut loop_prevention = HashSet::new();

    // Run game loop
    while !p1.is_empty() && !&p2.is_empty() {
        let mut p1_winner = false;

        if loop_prevention.contains(&(p1.clone(), p2.clone())) {
            // Loop rule => player 1 win
            p1.push(p1[0]);
            p1.push(p2[0]);
            p1.remove(0);
            p2.remove(0);
            continue;
        } else if (p1[0] as usize) < p1.len() && (p2[0] as usize) < p2.len() {
            let (p1_winner_sub, _) = part2(&(&p1[1..=p1[0] as usize], &p2[1..=p2[0] as usize]));
            p1_winner = p1_winner_sub;
        } else {
            loop_prevention.insert((p1.clone(), p2.clone()));

            if p1[0] > p2[0] {
                p1_winner = true;
            }
        }

        if p1_winner {
            p1.push(p1[0]);
            p1.push(p2[0]);
        } else {
            p2.push(p2[0]);
            p2.push(p1[0]);
        }

        p1.remove(0);
        p2.remove(0);
    }

    let winner = if !&p1.is_empty() { &p1 } else { &p2 };
    //dbg!(&winner.iter().rev().enumerate().collect::<Vec<_>>());
    (
        !p1.is_empty(),
        winner
            .iter()
            .rev()
            .enumerate()
            .map(|(idx, card_nb)| *card_nb as usize * (idx + 1))
            .sum(),
    )
}

fn parse(input: &str) -> (Vec<u16>, Vec<u16>) {
    let parts = input.trim().split(":").collect::<Vec<_>>();

    fn get_cards(lines: &str) -> Vec<u16> {
        lines
            .lines()
            .map(|line| line.trim())
            .filter_map(|line| line.parse::<u16>().ok())
            .collect()
    }

    (get_cards(parts[1]), get_cards(parts[2]))
}
