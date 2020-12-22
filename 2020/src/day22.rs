fn main() {
    let player_decks = parse(include_str!("../input/2020/day22.txt"));
    dbg!(part1(&player_decks));
}

fn part1(player_decks: &(Vec<u8>, Vec<u8>)) -> usize {
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

fn parse(input: &str) -> (Vec<u8>, Vec<u8>) {
    let parts = input.trim().split(":").collect::<Vec<_>>();

    fn get_cards(lines: &str) -> Vec<u8> {
        lines
            .lines()
            .map(|line| line.trim())
            .filter_map(|line| line.parse::<u8>().ok())
            .collect()
    }

    (get_cards(parts[1]), get_cards(parts[2]))
}
