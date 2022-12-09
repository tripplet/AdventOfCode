use core::panic;
use std::collections::HashSet;

type NUMBER = i32;
type ParseResult = Vec<(NUMBER, NUMBER)>;

pub fn parse_input(input: &str) -> ParseResult {
    input
        .trim()
        .lines()
        .map(|line| {
            let parts = line.trim().split_at(2);

            match parts.0 {
                "U " => (-parts.1.parse::<NUMBER>().unwrap(), 0 as NUMBER),
                "D " => (parts.1.parse::<NUMBER>().unwrap(), 0 as NUMBER),
                "L " => (0 as NUMBER, -parts.1.parse::<NUMBER>().unwrap()),
                "R " => (0 as NUMBER, parts.1.parse::<NUMBER>().unwrap()),
                _ => panic!("Invalid direction"),
            }
        })
        .collect()
}

pub fn part1(input: &ParseResult) -> usize {
    let mut tail_map: HashSet<(NUMBER, NUMBER)> = HashSet::new();

    let mut head_pos = (0, 0); // (y, x)
    let mut tail_pos = (0, 0); // (y, x)

    tail_map.insert(tail_pos);

    for (dy, dx) in input.into_iter() {
        // Move the head
        for _ in 1..=(dx + dy).abs() {
            // Move the head
            head_pos = (head_pos.0 + dy.signum(), head_pos.1 + dx.signum());

            // Check tail needs to be moved
            if (head_pos.0 - tail_pos.0).abs() > 1 || (head_pos.1 - tail_pos.1).abs() > 1 {
                // Move the tail
                // Brutefore the new tail_pos until it's only one step away from the head
                tail_pos = [(-1, 0), (-1, 1), (0, 1), (1, 1), (1, 0), (1, -1), (0, -1), (-1, -1)].iter().map(
                    |(tdy, tdx)| {
                        let new_tail_pos = (tail_pos.0 + tdy, tail_pos.1 + tdx);
                        let delta = (new_tail_pos.0 - head_pos.0).abs() + (new_tail_pos.1 - head_pos.1).abs();
                        (new_tail_pos, delta)

                    }
                ).min_by_key(|(_, delta)| *delta).unwrap().0;
                tail_map.insert(tail_pos);
            }
        }
    }

    //draw_tail_map(&tail_map);

    tail_map.len()
}

#[allow(dead_code)]
fn draw_tail_map(tail_map: &HashSet<(i32, i32)>) {
    let mut min_x = 0;
    let mut max_x = 0;
    let mut min_y = 0;
    let mut max_y = 0;

    for (y, x) in tail_map {
        if *x < min_x {
            min_x = *x;
        }
        if *x > max_x {
            max_x = *x;
        }
        if *y < min_y {
            min_y = *y;
        }
        if *y > max_y {
            max_y = *y;
        }
    }

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            if tail_map.contains(&(y, x)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

pub fn part2(input: &ParseResult) -> usize {
    let mut tail_map: HashSet<(NUMBER, NUMBER)> = HashSet::new();
    tail_map.insert((0,0));

    let mut snake = vec![(0, 0); 10]; // (y, x)

    for (dy, dx) in input.into_iter() {
        // Move the head
        for _ in 1..=(dx + dy).abs() {
            // Move the head
            snake[0] = (snake[0].0 + dy.signum(), snake[0].1 + dx.signum());


            for i in 1..snake.len() {
                if (snake[i].0 - snake[i-1].0).abs() > 1 || (snake[i].1 - snake[i-1].1).abs() > 1 {
                    // Move the reast of the snake
                    // Brutefore the new tail_pos until it's only one step away from the head
                    snake[i] = [(-1, 0), (-1, 1), (0, 1), (1, 1), (1, 0), (1, -1), (0, -1), (-1, -1)].iter().map(
                        |(tdy, tdx)| {
                            let new_pos = (snake[i].0 + tdy, snake[i].1 + tdx);
                            let delta = (new_pos.0 - snake[i-1].0).abs() + (new_pos.1 - snake[i-1].1).abs();
                            (new_pos, delta)
                        }
                    ).min_by_key(|(_, delta)| *delta).unwrap().0;

                    if i == snake.len() - 1 {
                        tail_map.insert(snake[i]);
                    }
                }
            }
        }
    }

    //draw_tail_map(&tail_map);

    tail_map.len()
}
