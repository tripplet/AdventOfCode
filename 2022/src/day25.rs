use nom::{
    branch::alt,
    character::complete::{char, line_ending},
    combinator::{iterator, map},
    multi::separated_list1,
    IResult,
};
use num::integer::div_mod_floor;

type Number = isize;
type ParseResult = Vec<Number>;

fn parse_snafu_number(input: &str) -> IResult<&str, Number> {
    let mut it = iterator(
        input,
        alt((
            map(char('0'), |_| 0_i8),
            map(char('1'), |_| 1_i8),
            map(char('2'), |_| 2_i8),
            map(char('-'), |_| -1_i8),
            map(char('='), |_| -2_i8),
        )),
    );

    let digits = it.collect::<Vec<_>>(); // rev() directly on the iterator doesn't work
    let number = digits
        .iter()
        .rev()
        .enumerate()
        .fold(0, |nb, (place, &value)| nb + (num::pow(5, place) * value as Number));

    Ok((it.finish()?.0, number))
}

fn number_to_snafu(number: isize) -> String {
    if number == 0 {
        return "".to_string();
    }

    let (div, reminder) = div_mod_floor(number + 2, 5);

    number_to_snafu(div) + match reminder {
        0 => "=",
        1 => "-",
        2 => "0",
        3 => "1",
        4 => "2",
        _ => unimplemented!(),
    }
}

pub fn parse_input(input: &str) -> ParseResult {
    separated_list1(line_ending, parse_snafu_number)(input).unwrap().1
}

pub fn part1(numbers: &ParseResult) -> String {
    let sum: isize = numbers.iter().sum();
    number_to_snafu(sum)
}

pub fn part2(_: &ParseResult) -> &str {
    "There is not part 2 for this day"
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../input/2022/day25_example.txt");
    const INPUT: &str = include_str!("../input/2022/day25.txt");

    #[test]
    fn feature() {
        println!("{}", parse_snafu_number("1==").unwrap().1);
        println!("{}", parse_snafu_number("1==").unwrap().1);
    }

    #[test]
    fn example_part1() {
        let input = parse_input(EXAMPLE);
        assert_eq!(part1(&input), "2=-1=0");
    }

    #[test]
    fn input_part1() {
        let input = parse_input(INPUT);
        assert_eq!(part1(&input), "todo");
    }
}
