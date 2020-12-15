use std::collections::HashMap;

fn main() {
    let data = parse(include_str!("../input/2020/day15.txt"));

    assert_eq!(calc(&parse("0,3,6"), 2020), 436);
    assert_eq!(calc(&parse("1,3,2"), 2020), 1);
    assert_eq!(calc(&parse("2,1,3"), 2020), 10);
    assert_eq!(calc(&parse("1,2,3"), 2020), 27);
    assert_eq!(calc(&parse("2,3,1"), 2020), 78);
    assert_eq!(calc(&parse("3,2,1"), 2020), 438);
    assert_eq!(calc(&parse("3,1,2"), 2020), 1836);

    let now = std::time::Instant::now();
    let part1 = calc(&data, 2020);
    println!("Part1: {}  [{}]", part1, humantime::format_duration(now.elapsed()));
    assert_eq!(part1, 610);

    let now = std::time::Instant::now();
    let part2 = calc(&data, 30000000);
    println!("Part1: {}  [{}]", part2, humantime::format_duration(now.elapsed()));
    assert_eq!(part2, 1407);
}

fn parse(input: &str) -> Vec<u64> {
    input.trim().split(",").map(|number| number.parse().unwrap()).collect()
}

fn calc(data: &[u64], until_round: u64) -> u64 {
    let mut numbers: HashMap<u64, (u64, u64)> = HashMap::new();

    for turn in 0..data.len() {
        let nb = data.get(turn).unwrap();
        numbers.insert(*nb, ((turn + 1) as u64, 0));
    }

    let mut last_number_spoken = *data.last().unwrap();

    for turn in data.len()+1..=(until_round as usize) {
        if let Some(pre_turn) = numbers.get(&last_number_spoken).cloned() {
            if pre_turn.1 == 0 {
                last_number_spoken = 0;
            }
            else {
                last_number_spoken = pre_turn.0 - pre_turn.1;
            }
        }
        else {
            last_number_spoken = 0;
        }

        // "Speak" number
        if let Some(last_time) = numbers.get(&last_number_spoken).cloned() {
            numbers.insert(last_number_spoken, (turn as u64, last_time.0));
        }
        else {
            numbers.insert(last_number_spoken, (turn as u64, 0));
        }
    }
    last_number_spoken
}
