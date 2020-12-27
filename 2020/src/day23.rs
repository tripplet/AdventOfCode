fn main() {
    let cubs = include_str!("../input/2020/day23.txt").trim().chars().map(|c| c.to_digit(10).unwrap() as u8).collect::<Vec<_>>();

    let now = std::time::Instant::now();
    let part1_result = part1(&cubs);
    println!("Part1: {}  [{}]", part1_result, humantime::format_duration(now.elapsed()));
    assert_eq!(part1_result, 33098);
}

fn part1(cubs_orig: &Vec<u8>) -> usize {
    let mut cubs = cubs_orig.clone();

    let len = cubs.len();
    let mut cur_cub = 0;
    let mut cur_cub_value = 0;
    let mut removed_cub_range = (100, 100);
    let mut previous_picked_cub_value = -1;

    for _ in 0..100 {
        cur_cub_value = cubs[cur_cub];
        removed_cub_range = ((cur_cub + 1) % len, (cur_cub + 3) % len);

        for delta in 1..len-3 {
            cur_cub + delta + 1 %
        }
    }



    0
}