fn main() {

    assert_eq!(brute_force_loop_size(5764801, 7), 8);
    assert_eq!(transform(8, 17807724), 14897079);

    let pub_keys = include_str!("../input/2020/day25.txt").trim().lines().map(|line| line.parse::<u64>().unwrap()).collect::<Vec<_>>();

    let loop_size_1 = brute_force_loop_size(pub_keys[0], 7);
    let loop_size_2 = brute_force_loop_size(pub_keys[1], 7);

    dbg!(transform(loop_size_1, pub_keys[1]));
    dbg!(transform(loop_size_2, pub_keys[0]));
}

fn transform(loop_size: u64, subject_num: u64) -> u64 {
    let mut value = 1;
    for _ in 0..loop_size {
        value = (value * subject_num) % 20201227;
    }
    value
}

fn brute_force_loop_size(result: u64, subject_num: u64) -> u64 {
    let mut value = 1;
    for idx in 1.. {
        value = (value * subject_num) % 20201227;

        if value == result {
            return idx;
        }
    }

    return 0;
}