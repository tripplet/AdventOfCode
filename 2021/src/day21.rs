#[derive(Debug)]
struct Die {
    last_number: u8,
    rolled: usize,
}

#[derive(Debug, Clone)]
struct Player {
    position: u8,
    score: u16,
}

impl Die {
    fn new() -> Self {
        Die { last_number: 99, rolled: 0 }
    }

    fn roll(&mut self) -> u8 {
        self.last_number = (self.last_number + 1) % 100;
        self.rolled += 1;
        self.last_number + 1
    }
}

impl Player {
    fn move_pawn(&mut self, move_by: u16) {
        self.position = ((((self.position - 1) as u16 + (move_by % 10)) % 10) + 1) as u8;
        self.score += self.position as u16;
    }
}

fn input() -> Vec<Player> {
    vec![Player { position: 6, score: 0 }, Player { position: 3, score: 0 }]
}

fn example() -> Vec<Player> {
    vec![Player { position: 4, score: 0 }, Player { position: 8, score: 0 }]
}

pub fn main() {
    let data = input();

    let now = std::time::Instant::now();
    println!(
        "Part1: {} [{}]",
        part1(&data),
        humantime::format_duration(now.elapsed())
    );

    // now = std::time::Instant::now();
    // println!(
    //     "Part2: {} [{}]",
    //     part2(&data),
    //     humantime::format_duration(now.elapsed())
    // );
}

fn part1(players: &[Player]) -> usize {
    let mut die = Die::new();
    let mut players = players.to_vec();

    'game: for _ in 1.. {
        for player in players.iter_mut() {
            player.move_pawn(die.roll() as u16 + die.roll() as u16 + die.roll() as u16);
            if player.score >= 1000 {
                break 'game;
            }
        }
        //println!("{} - p1: {:?}, p2: {:?}", round, players[0], players[1]);
    }

    die.rolled * players.iter().find(|p| p.score < 1000).unwrap().score as usize
}
