fn main() {
    let cubs = include_str!("../input/2020/day23.txt").trim().chars().map(|c| c.to_digit(10).unwrap() as u32).collect::<Vec<_>>();
    let cubs_example = "389125467".chars().map(|c| c.to_digit(10).unwrap() as u32).collect::<Vec<_>>();

    assert_eq!(part1(&cubs_example, 100), [6,7,3,8,4,5,2,9]);

    let now = std::time::Instant::now();
    let part1_result = part1(&cubs, 100).iter().map(|nb| nb.to_string()).collect::<Vec<_>>().join("");
    println!("Part1: {:?}  [{}]", part1_result, humantime::format_duration(now.elapsed()));
    assert_eq!(part1_result, "65432978");


    part2(&cubs_example)
    //assert_eq!(, [6,7,3,8,4,5,2,9]);
}

fn part2(cubs_orig: &[u32]) {

    let mut cubs: Vec<u32> = Vec::with_capacity(1_000_000);
    cubs.splice(0..0, cubs_orig.into_iter().copied());
    cubs.splice(9..9, (10..=1_000_000).into_iter());
    // = .collect();

    //println!("{:?}", cubs);
    //

    let result = part1(&cubs, 10_000_000);
    println!("{}, {}", result[0], result[1]);
}

fn part1(cubs_orig: &[u32], rounds: usize) -> Vec<u32> {
    let mut cubs: Vec<u32> = cubs_orig.into_iter().copied().collect();

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

fn print(prefix: &str, slice: &[u32], highlight: Option<usize>) {
    print!("{}: ", prefix);
    for idx in 0..slice.len() {
        match highlight {
            Some(pos) if pos == idx => print!("({}) ", slice[idx]),
            _ => print!("{} ", slice[idx])
        }
    }
    println!();
}