type SeatMap = Vec<Vec<char>>;

const OCCUPIED: char = '#';
const EMPTY: char = 'L';
const FLOOR: char = '.';

pub fn main() {
    let seat_map = parse(include_str!("../input/2020/day11.txt"));

    let mut now = std::time::Instant::now();
    println!("Part1: {}  [{}]",  run(&seat_map, iteration_part1), humantime::format_duration(now.elapsed()));

    now = std::time::Instant::now();
    println!("Part2: {}  [{}]",  run(&seat_map, iteration_part2), humantime::format_duration(now.elapsed()));
}

pub fn parse(input: &str) -> SeatMap {
    let mut map = input.trim().lines().map(|line| {
        let mut cur = line.trim().chars().collect::<Vec<_>>();
        cur.insert(0, FLOOR);
        cur.push(FLOOR);
        cur
    }).collect::<Vec<Vec<_>>>();

    map.push(vec![FLOOR; map[0].len()]);
    map.insert(0, vec![FLOOR; map[0].len()]);
    map
}

pub fn run(seat_map_orig: &SeatMap, iterate_fn: fn(&mut SeatMap) -> bool) -> u64 {
    let mut seat_map_modified = seat_map_orig.clone();

    while iterate_fn(&mut seat_map_modified) { }
    seat_map_modified.iter().fold(0, |cnt, row| cnt + row.iter().filter(|c| **c == OCCUPIED).count()) as u64
}

pub fn for_all_directions<'a>(mut func: Box<dyn FnMut(i32, i32) + 'a>) {
    for dir in [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)].iter() { // x, y
        func(dir.1, dir.0); // y, x
    }
}

pub fn print_map(seat_map: &SeatMap) {
    for row in seat_map.iter() {
        println!("{}", row.iter().collect::<String>());
    }
    println!();
}

fn are_equal(seat_map1: &SeatMap, seat_map2: &SeatMap) -> bool {
    for row in 0..seat_map1.len() {
        for col in 0..seat_map1[0].len() {
            if seat_map1[row][col] != seat_map2[row][col] {
                return true;
            }
        }
    }
    false
}

pub fn iteration_part1(seat_map: &mut SeatMap) -> bool {
    let orig_seat_map = seat_map.clone();
    let rows = seat_map.len();
    let columms = seat_map[0].len();

    // Prepare
    for row in 1..rows-1 {
        for col in 1..columms-1 {
            if seat_map[row][col] == EMPTY {
                let mut result = true;
                for_all_directions(Box::new(|dy, dx| {
                    result &= seat_map[((row as i32) + dy) as usize][((col as i32) + dx) as usize] != OCCUPIED;
                }));

                if result { seat_map[row][col] = 'P'; }
            }
        }
    }

    update_map(seat_map, 'P', OCCUPIED);

    // Step2
    for row in 1..rows-1 {
        for col in 1..columms-1 {
            if seat_map[row][col] == OCCUPIED {
                let mut cnt = 0;
                for_all_directions(Box::new(|dy, dx| {
                    let v = seat_map[((row as i32) + dy) as usize][((col as i32) + dx) as usize];
                    if v == OCCUPIED || v == 'E' {
                        cnt += 1
                    }
                }));

                if cnt >= 4 { seat_map[row][col] = 'E'; }
            }
        }
    }

    update_map(seat_map, 'E', EMPTY);
    are_equal(seat_map, &orig_seat_map)
}

fn check_direction_occupied(seat_map: &SeatMap, start_x: usize, start_y: usize, dx: i32, dy: i32, check_char1: char, check_char2: char) -> bool {
    let rows = seat_map.len() as i32;
    let columms = seat_map[0].len() as i32;
    let mut x = start_x as i32;
    let mut y = start_y as i32;

    loop {
        x = x + dx;
        y = y + dy;

        if x > 0 && x < rows && y > 0 && y < columms {
            if seat_map[x as usize][y as usize] == '.' {
                continue;
            }

            if seat_map[x as usize][y as usize] == check_char1 || seat_map[x as usize][y as usize] == check_char2 {
                return true;
            }
            else {
                return false;
            }
        }
        else {
            return false;
        }
    }
}

fn update_map(seat_map: &mut SeatMap, from: char, to: char) {
    for row in 1..seat_map.len()-1 {
        for col in 1..seat_map[0].len()-1 {
            if seat_map[row][col] == from {
                seat_map[row][col] = to;
            }
        }
    }
}

fn iteration_part2(seat_map: &mut SeatMap) -> bool {
    let orig_seat_map = seat_map.clone();
    let rows = seat_map.len();
    let columms = seat_map[0].len();

    // Prepare
    for row in 1..rows-1 {
        for col in 1..columms-1 {
            if seat_map[row][col] == EMPTY {
                let mut result = true;
                for_all_directions(Box::new(|dy, dx| {
                    result &= !check_direction_occupied(seat_map, row, col, dx, dy, '#', ' ');
                }));

                if result { seat_map[row][col] = 'P'; }
            }
        }
    }

    update_map(seat_map, 'P', OCCUPIED);

    // Step2
    for row in 1..rows-1 {
        for col in 1..columms-1 {
            if seat_map[row][col] == '#' {
                let mut cnt = 0;
                for_all_directions(Box::new(|dy, dx| {
                    if check_direction_occupied(seat_map, row, col, dx, dy, OCCUPIED, 'E') {
                        cnt += 1
                    }
                }));

                if cnt >= 5 { seat_map[row][col] = 'E'; }
            }
        }
    }

    update_map(seat_map, 'E', EMPTY);
    are_equal(seat_map, &orig_seat_map)
}
