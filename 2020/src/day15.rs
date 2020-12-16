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
    let part2 = calc(&data, 30_000_000);
    println!("Part2: {}  [{}]", part2, humantime::format_duration(now.elapsed()));
    assert_eq!(part2, 1407);
}

fn parse(input: &str) -> Vec<usize> {
    input.trim().split(",").map(|number| number.parse().unwrap()).collect()
}

fn calc(start_data: &[usize], until_round: usize) -> u32 {
    let mut numbers = vec![0; until_round];

    start_data.iter().enumerate().for_each(|(turn, nb)| {
        numbers[*nb] = (turn + 1) as u32;
    });

    let mut last_number_spoken = *start_data.last().unwrap();

    for turn in start_data.len()..until_round {
        let pre_turn = numbers[last_number_spoken];
        numbers[last_number_spoken] = turn as u32;

        if pre_turn == 0 {
            last_number_spoken = 0;
        } else {
            last_number_spoken = turn - pre_turn as usize;
        }
    }
    last_number_spoken as u32
}
