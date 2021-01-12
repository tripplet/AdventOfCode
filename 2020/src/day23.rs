fn main() {
    let cubs = include_str!("../input/2020/day23.txt").trim().chars().map(|c| c.to_digit(10).unwrap() as u32).collect::<Vec<_>>();
    let cubs_example = "389125467".chars().map(|c| c.to_digit(10).unwrap() as u32).collect::<Vec<_>>();

    assert_eq!(part1(&cubs_example, 100), "67384529");

    let now = std::time::Instant::now();
    let result1 = part1(&cubs, 100);
    println!("Part1: {:?}  [{}]", result1, humantime::format_duration(now.elapsed()));
    assert_eq!(result1, "65432978");

    assert_eq!(part2(&cubs_example, 10_000_000), 149245887792);

    let now = std::time::Instant::now();
    let result2 = part2(&cubs, 10_000_000);
    println!("Part1: {:?}  [{}]", result2, humantime::format_duration(now.elapsed()));
    assert_eq!(result2, 287230227046);
}

fn part2(cubs_orig: &[u32], rounds: usize) -> u64 {
    let mut cubs: Vec<u32> = Vec::with_capacity(1_000_000);
    cubs.splice(0..0, cubs_orig.into_iter().copied());
    cubs.splice(cubs_orig.len()..cubs_orig.len(), (10..=1_000_000).into_iter());

    let linked_list = solve(&cubs, rounds);

    let first = linked_list[1];
    let second = linked_list[first as usize];

    first as u64 * second as u64
}

fn part1(cubs_orig: &[u32], rounds: usize) -> String {
    let linked_list = solve(cubs_orig, rounds);

    let mut result = Vec::with_capacity(linked_list.len());
    let mut next = 1 as u32;

    for _ in 0..linked_list.len() - 2 {
        next = linked_list[next as usize] as u32;
        result.push(next);
    }

    result.iter().map(|nb| nb.to_string()).collect::<Vec<_>>().join("")
}

fn solve(cubs: &[u32], rounds: usize) -> Vec<u32> {
    let len = cubs.len();
    let min = *cubs.iter().min().unwrap();
    let max = *cubs.iter().max().unwrap();

    let mut indexed_linked_list = vec![0; len + 1];

    for idx in 0..len - 1 {
        indexed_linked_list[cubs[idx] as usize] = cubs[idx + 1];
    }

    indexed_linked_list[cubs[len - 1] as usize] = cubs[0];

    let mut cur_cub_value = cubs[0];

    for _ in 1..=rounds {
        let remove_1 = indexed_linked_list[cur_cub_value as usize];
        let remove_2 = indexed_linked_list[remove_1 as usize];
        let remove_3 = indexed_linked_list[remove_2 as usize];

        let mut destination_value = cur_cub_value - 1;
        while destination_value < min
            || destination_value == remove_1
            || destination_value == remove_2
            || destination_value == remove_3 {
            if destination_value < min {
                destination_value = max;
            }
            else {
                destination_value -= 1;
            }
        }

        let mut prev_pointing_to = indexed_linked_list[destination_value as usize];

        if prev_pointing_to == remove_1 {
            prev_pointing_to = indexed_linked_list[remove_3 as usize];
        }

        indexed_linked_list[destination_value as usize] = remove_1;
        indexed_linked_list[cur_cub_value as usize] = indexed_linked_list[remove_3 as usize];
        indexed_linked_list[remove_3 as usize] = prev_pointing_to;

        cur_cub_value = indexed_linked_list[cur_cub_value as usize];
    }

    indexed_linked_list
}
