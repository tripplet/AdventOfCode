type NUMBER = i32;
type ParseResult = Vec<NUMBER>;

pub fn parse_input(input: &str) -> ParseResult {
    input.trim().lines().map(|line| {
        line.parse::<NUMBER>().unwrap_or(-1)
    }).collect()
}

pub fn part1(input: &ParseResult) -> NUMBER {
    let mut sum = 0;
    let mut max = 0;

    for x in input {
        if *x != -1 {
            sum += *x;
        }
        else {
            if sum > max {
                max = sum;
            }
            sum = 0;
        }
    }

    if sum > max {
        max = sum;
    }

    max
}

pub fn part2(input: &ParseResult) -> NUMBER {
    let mut sums = Vec::with_capacity(input.len()/2);
    let mut sum = 0;

    for x in input {
        if *x != -1 {
            sum += *x;
        }
        else {
            sums.push(sum);
            sum = 0;
        }
    }

    if sum != 0 {
        sums.push(sum);
    }

    sums.sort_unstable();
    sums.into_iter().rev().take(3).sum()
}
