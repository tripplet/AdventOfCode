use itertools::Itertools;

type SeatMap = Vec<Vec<char>>;

pub fn main() {
    let seat_map = parse(include_str!("../input/2020/day11.txt"));

    let mut now = std::time::Instant::now();
    println!("Part1: {}  [{}]",  part1(&seat_map), humantime::format_duration(now.elapsed()));

    now = std::time::Instant::now();
    println!("Part2: {}  [{}]",  part2(&seat_map), humantime::format_duration(now.elapsed()));
}

pub fn parse(input: &str) -> SeatMap {
    let mut map = input.trim().lines().map(|line| {
        let mut cur = line.trim().chars().collect::<Vec<_>>();
        cur.insert(0, '.');
        cur.push('.');
        cur
    }).collect::<Vec<Vec<_>>>();


    map.push(vec!['.'; map[0].len()]);
    map.insert(0, vec!['.'; map[0].len()]);
    map
}


pub fn part1(seat_map: &SeatMap) -> u64 {
    // for row in seat_map.iter() {
    //     println!("{}", row.iter().collect::<String>());
    // }

    // println!();
    let mut seat_map_copy = seat_map.clone();

    loop {
        if !iteration(&mut seat_map_copy) {
            break;
        }
    }

    let mut cnt = 0;
    for row in 1..seat_map_copy.len()-1 {
        for col in 1..seat_map_copy[0].len()-1 {
            if seat_map_copy[row][col] == '#' {
                cnt += 1;
            }
        }
    }

    cnt
}

pub fn iteration(seat_map: &mut SeatMap) -> bool {
    let seat_map_backup = seat_map.clone();

    let rows = seat_map.len();
    let columms = seat_map[0].len();

    // for row in seat_map.iter() {
    //     println!("{}", row.iter().collect::<String>());
    // }

    // println!();

    // Prepare
    for row in 1..rows-1 {
        for col in 1..columms-1 {
            if seat_map[row][col] == 'L'
                && seat_map[row-1][col] != '#'
                && seat_map[row-1][col+1] != '#'
                && seat_map[row][col+1] != '#'
                && seat_map[row+1][col+1] != '#'
                && seat_map[row+1][col] != '#'
                && seat_map[row+1][col-1] != '#'
                && seat_map[row][col-1] != '#'
                && seat_map[row-1][col-1] != '#' {
                    seat_map[row][col] = 'P';
                }
        }
    }

    for row in 1..rows-1 {
        for col in 1..columms-1 {
            if seat_map[row][col] == 'P' {
                seat_map[row][col] = '#';
            }
        }
    }

    // for row in seat_map.iter() {
    //     println!("{}", row.iter().collect::<String>());
    // }

    // Step2
    for row in 1..rows-1 {
        for col in 1..columms-1 {
            if seat_map[row][col] == '#' {

                let mut cnt = 0;

                cnt += if seat_map[row-1][col-1] == '#' || seat_map[row-1][col-1] == 'E' { 1 } else { 0 };
                cnt += if seat_map[row-1][col] == '#' || seat_map[row-1][col] == 'E' { 1 } else { 0 };
                cnt += if seat_map[row-1][col+1] == '#' || seat_map[row-1][col+1] == 'E' { 1 } else { 0 };
                cnt += if seat_map[row][col+1] == '#' || seat_map[row][col+1] == 'E' { 1 } else { 0 };
                cnt += if seat_map[row][col-1] == '#' || seat_map[row][col-1] == 'E' { 1 } else { 0 };
                cnt += if seat_map[row+1][col+1] == '#' || seat_map[row+1][col+1] == 'E' { 1 } else { 0 };
                cnt += if seat_map[row+1][col] == '#' || seat_map[row+1][col] == 'E' { 1 } else { 0 };
                cnt += if seat_map[row+1][col-1] == '#' || seat_map[row+1][col-1] == 'E' { 1 } else { 0 };

                if cnt >= 4 {
                    seat_map[row][col] = 'E';
                }
            }
        }
    }

    // Empty
    for row in 0..rows {
        for col in 0..columms {
            if seat_map[row][col] == 'E' {
                seat_map[row][col] = 'L';
            }
        }
    }

    // for row in seat_map.iter() {
    //     println!("{}", row.iter().collect::<String>());
    // }

    for row in 0..rows {
        for col in 0..columms {
            if seat_map[row][col] != seat_map_backup[row][col] {
                return true;
            }
        }
    }

    false
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

pub fn part2(seat_map: &SeatMap) -> u64 {
    // for row in seat_map.iter() {
    //     println!("{}", row.iter().collect::<String>());
    // }

    // println!();
    let mut seat_map_copy = seat_map.clone();

    loop {
        if !iteration2(&mut seat_map_copy) {
            break;
        }
    }

    let mut cnt = 0;
    for row in 1..seat_map_copy.len()-1 {
        for col in 1..seat_map_copy[0].len()-1 {
            if seat_map_copy[row][col] == '#' {
                cnt += 1;
            }
        }
    }

    cnt
}

fn iteration2(seat_map: &mut SeatMap) -> bool {
    let seat_map_backup = seat_map.clone();

    let rows = seat_map.len();
    let columms = seat_map[0].len();

    // for row in seat_map.iter() {
    //     println!("{}", row.iter().collect::<String>());
    // }

    // println!();

    // Prepare
    for row in 1..rows-1 {
        for col in 1..columms-1 {
            if seat_map[row][col] == 'L'
                && !check_direction_occupied(seat_map, row, col, -1, 0, '#', ' ')
                && !check_direction_occupied(seat_map, row, col, -1, 1, '#', ' ')
                && !check_direction_occupied(seat_map, row, col, 0, 1, '#', ' ')
                && !check_direction_occupied(seat_map, row, col, 1, 1, '#', ' ')
                && !check_direction_occupied(seat_map, row, col, 1, 0, '#', ' ')
                && !check_direction_occupied(seat_map, row, col, 1, -1, '#', ' ')
                && !check_direction_occupied(seat_map, row, col, 0, -1, '#', ' ')
                && !check_direction_occupied(seat_map, row, col, -1, -1, '#', ' ') {
                    seat_map[row][col] = 'P';
                }
        }
    }

    for row in 1..rows-1 {
        for col in 1..columms-1 {
            if seat_map[row][col] == 'P' {
                seat_map[row][col] = '#';
            }
        }
    }

    // for row in seat_map.iter() {
    //     println!("{}", row.iter().collect::<String>());
    // }
    // println!();

    // Step2
    for row in 1..rows-1 {
        for col in 1..columms-1 {
            if seat_map[row][col] == '#' {

                let mut cnt = 0;

                cnt += if check_direction_occupied(seat_map, row, col, -1, -1, '#', 'E') { 1 } else { 0 };
                cnt += if check_direction_occupied(seat_map, row, col, -1, 0, '#', 'E') { 1 } else { 0 };
                cnt += if check_direction_occupied(seat_map, row, col, -1, 1, '#', 'E') { 1 } else { 0 };
                cnt += if check_direction_occupied(seat_map, row, col, 0,  1, '#', 'E') { 1 } else { 0 };
                cnt += if check_direction_occupied(seat_map, row, col, 0,  -1, '#', 'E') { 1 } else { 0 };
                cnt += if check_direction_occupied(seat_map, row, col, 1, 1, '#', 'E') { 1 } else { 0 };
                cnt += if check_direction_occupied(seat_map, row, col, 1, 0, '#', 'E') { 1 } else { 0 };
                cnt += if check_direction_occupied(seat_map, row, col, 1, -1, '#', 'E') { 1 } else { 0 };

                if cnt >= 5 {
                    seat_map[row][col] = 'E';
                }
            }
        }
    }

    // Empty
    for row in 0..rows {
        for col in 0..columms {
            if seat_map[row][col] == 'E' {
                seat_map[row][col] = 'L';
            }
        }
    }

    // for row in seat_map.iter() {
    //     println!("{}", row.iter().collect::<String>());
    // }
    // println!();

    for row in 0..rows {
        for col in 0..columms {
            if seat_map[row][col] != seat_map_backup[row][col] {
                return true;
            }
        }
    }

    false
}
