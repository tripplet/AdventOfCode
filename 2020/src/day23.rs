fn main() {
    let cubs = include_str!("../input/2020/day23.txt").trim().chars().map(|c| c.to_digit(10).unwrap() as u32).collect::<Vec<_>>();
    let cubs_example = "389125467".chars().map(|c| c.to_digit(10).unwrap() as u32).collect::<Vec<_>>();

    assert_eq!(part1(&cubs_example, 100), [6,7,3,8,4,5,2,9]);

    let now = std::time::Instant::now();
    let part1_result = part1(&cubs, 100).iter().map(|nb| nb.to_string()).collect::<Vec<_>>().join("");
    println!("Part1: {:?}  [{}]", part1_result, humantime::format_duration(now.elapsed()));
    assert_eq!(part1_result, "65432978");


    let a = part2(&cubs_example, 10_000_000);
    assert_eq!(a[0], 934001);
    assert_eq!(a[1], 159792);

    let now = std::time::Instant::now();
    let part2_result = part2(&cubs, 10_000_000);
    println!("Part1: {:?}  [{}]", part2_result[0] as u64 * part2_result[1] as u64, humantime::format_duration(now.elapsed()));
    assert_eq!(part1_result, "65432978");
}

fn part2(cubs_orig: &[u32], rounds: usize) -> Vec<u32> {
    let mut cubs: Vec<u32> = Vec::with_capacity(1_000_000);
    cubs.splice(0..0, cubs_orig.into_iter().copied());
    cubs.splice(cubs_orig.len()..cubs_orig.len(), (10..=1_000_000).into_iter());

    part1(&cubs, rounds)
}

fn part1(cubs_orig: &[u32], rounds: usize) -> Vec<u32> {
    let mut cubs: Vec<u32> = Vec::with_capacity(1_000_000);
    cubs.splice(0..0, cubs_orig.into_iter().copied());
    //cubs.splice(cubs_orig.len()..cubs_orig.len(), (10..=1_000_000).into_iter());

    let len = cubs.len();
    let min = *cubs.iter().min().unwrap();
    let max = *cubs.iter().max().unwrap();

    let mut indexed_linked_list = vec![0; len + 1];

    for idx in 0..len - 1 {
        indexed_linked_list[cubs[idx] as usize] = cubs[idx + 1];
    }

    indexed_linked_list[cubs[len - 1] as usize] = cubs[0];

    let mut cur_cub_value = cubs[0];


    // println!("Done");

    // print("orig         ", &cubs_orig, Some(0));
    // print("Linked list", &indexed_linked_list, None);
    // println!("cur: {}", cur_cub_value);

    for round_idx in 1..=rounds {
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

        // println!("{}", round_idx);
        // print("L", &indexed_linked_list, None);
        // println!("remove: {} {} {}", remove_1, remove_2, remove_3);
        // println!("dest:   {}", destination_value);
        // println!();

        let mut prev_pointing_to = indexed_linked_list[destination_value as usize];

        if prev_pointing_to == remove_1 {
            prev_pointing_to = indexed_linked_list[remove_3 as usize];
        }

        indexed_linked_list[destination_value as usize] = remove_1;

        //let a = indexed_linked_list[remove_3 as usize];
        indexed_linked_list[cur_cub_value as usize] = indexed_linked_list[remove_3 as usize];
        indexed_linked_list[remove_3 as usize] = prev_pointing_to;


        // cur (3), dest 2
        // rem: 8 9 1
        //   1 2  3  4 5 6 7 8 9
        // 0 2 5 (8) 6 4 7 3 9 1
        //   5 8  2
        //

        cur_cub_value = indexed_linked_list[cur_cub_value as usize];
    }

    //print("test1", &indexed_linked_list, None);

    let mut result = Vec::with_capacity(cubs_orig.len());
    let mut next = 1 as u32;
    for _ in 0..len - 1 {
        next = indexed_linked_list[next as usize] as u32;
        result.push(next);
    }

    result
}

fn part1_old(cubs_orig: &[u8], rounds: usize) -> Vec<u8> {
    let mut cubs: Vec<u8> = cubs_orig.into_iter().copied().collect();

    let min = *cubs_orig.iter().min().unwrap();
    let max = *cubs_orig.iter().max().unwrap();
    let len = cubs_orig.len();
    let mut cur_cub = 0;
    let mut cur_cub_value;

    for round_idx in 1..=rounds {
        cur_cub_value = cubs[cur_cub];
        if round_idx % 1000 == 0 {
            println!("{}", round_idx);
        }
        //print("cubs", &cubs, Some(cur_cub));

        if cur_cub + 3 > len {
            cubs.rotate_left(3);
            cur_cub -= 3;
        }

        let mut removed_cubs = vec![];
        for _ in 0..3 {
            removed_cubs.push(cubs.remove((cur_cub + 1) % cubs.len()));
        }

        //print("removed cubs", &removed_cubs, None);

        let mut destination = cur_cub_value - 1;
        while destination < min
            || removed_cubs.iter().any(|c| *c == destination) {
            if destination < min {
                destination = max;
            }
            else {
                destination -= 1;
            }
        }

        //println!("destination: {}", destination);
        //println!();

        let destination_pos = cubs.iter().position(|c| *c == destination).unwrap() + 1;

        removed_cubs.reverse();
        for idx in 0..3 {
            cubs.insert(destination_pos, removed_cubs[idx]);
        }
        cur_cub = (cubs.iter().position(|c| *c == cur_cub_value).unwrap() + 1) % len;
    }

    let pos_1 = cubs.iter().position(|c| *c == 1).unwrap();
    cubs.rotate_left(pos_1 + 1);
    cubs.remove(len - 1);
    cubs
}

fn print<T>(prefix: &str, slice: &[T], highlight: Option<usize>) where T: std::fmt::Display {
    print!("{}: ", prefix);
    for idx in 0..slice.len() {
        match highlight {
            Some(pos) if pos == idx => print!("({}) ", slice[idx]),
            _ => print!("{} ", slice[idx])
        }
    }
    println!();
}