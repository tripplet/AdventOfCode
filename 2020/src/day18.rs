fn main() {
    let data = parse(include_str!("../input/2020/day18.txt"));

    let now = std::time::Instant::now();
    let part1 = part1(&data);
    println!("Part1: {}  [{}]", part1, humantime::format_duration(now.elapsed()));
    assert_eq!(part1, 16332191652452);

    assert_eq!(calc2(&parse("2 * 3 + (4 * 5)")[0]).0, 46);
    assert_eq!(calc2(&parse("5 + (8 * 3 + 9 + 3 * 4 * 3)")[0]).0, 1445);
    assert_eq!(calc2(&parse("1 + 2 * 3 + 4 * 5 + 6")[0]).0, 231);
    assert_eq!(calc2(&parse("1 + (2 * 3) + (4 * (5 + 6))")[0]).0, 51);
    assert_eq!(calc2(&parse("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))")[0]).0, 669060);
    assert_eq!(calc2(&parse("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2")[0]).0, 23340);

    let now = std::time::Instant::now();
    let part2 = part2(&data);
    println!("Part2: {}  [{}]", part2, humantime::format_duration(now.elapsed()));
}

fn parse(input: &str) -> Vec<Vec<char>> {
    input.trim().replace(" ", "").lines().map(|l| l.trim().chars().collect::<Vec<_>>()).collect::<Vec<_>>()
}

fn calc2(input: &[char]) -> (u64, usize) {
    let plus_count = input.iter().filter(|v| **v == '+').count();
    let mut patched_equation = input.to_vec();

    // Add braces around all + operations
    for nb in 0..plus_count {
        let pos = patched_equation.iter().enumerate().filter(|(_, c)| **c == '+').collect::<Vec<_>>()[nb].0;

        patch(pos, 1, ')', &mut patched_equation);
        patch(pos, -1, '(', &mut patched_equation);
    }

    // Calc using old part1 func
    calc(&patched_equation)
}

fn patch(pos: usize, direction: isize, symbol: char, equation: &mut Vec<char>) {
    let mut idx: isize = pos as isize;
    let mut level = 0;
    loop {
        let cur = *equation.get(idx as usize).unwrap_or(&' ');
        if cur == '(' {
            level += 1;
        } else if cur == ')' {
            level -= 1;
        }

        if cur != '+' && cur != '*' && level == 0 {
            let mut new_pos = idx;

            if direction > 0 {
                new_pos += direction;
            }

            if new_pos >= equation.len() as isize {
                new_pos = equation.len() as isize;
            } else if new_pos < 0 {
                new_pos = 0;
            }

            equation.insert(new_pos as usize, symbol);
            break;
        }

        idx += direction;
    }
}

fn part1(lines: &Vec<Vec<char>>) -> u64 {
    lines.iter().map(|l| calc(l).0).sum()
}

fn part2(lines: &Vec<Vec<char>>) -> u64 {
    lines.iter().map(|l| calc2(l).0).sum()
}

fn calc(input: &[char]) -> (u64, usize) {
    let mut pos = 0;

    let mut v1: Option<u64> = None;
    let mut op: Option<char> = None;
    let mut v2: Option<u64> = None;

    loop {
        let sym = input[pos];
        let mut cur: Option<u64> = None;

        if sym == '(' {
            let (sub_result, end) = calc(&input[pos + 1..]);
            cur = Some(sub_result);
            pos += end + 1;
        } else if sym == ')' {
            return (v1.unwrap(), pos);
        } else if sym == '+' || sym == '*' {
            op = Some(sym);
        } else {
            cur = Some(sym.to_digit(10).unwrap() as u64);
        }

        if cur.is_some() {
            if v1.is_none() {
                v1 = cur;
            } else if v2.is_none() {
                v2 = cur;
            }
        }

        if let Some(vv1) = v1 {
            if let Some(vv2) = v2 {
                match op.unwrap() {
                    '+' => v1 = Some(vv1 + vv2),
                    '*' => v1 = Some(vv1 * vv2),
                    _ => panic!(),
                }

                v2 = None;
            }
        }

        pos += 1;
        if pos == input.len() {
            break;
        }
    }

    (v1.unwrap(), 0)
}
