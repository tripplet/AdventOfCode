use std::{str::FromStr};

type ParseResult = Supplies;

macro_rules! regex {
    ($re:literal $(,)?) => {{
        static RE: once_cell::sync::OnceCell<regex::Regex> = once_cell::sync::OnceCell::new();
        RE.get_or_init(|| regex::Regex::new($re).unwrap())
    }};
}

#[derive(Debug, Clone)]
pub struct Supplies {
    stacks: Vec<Vec<char>>,
    moves: Vec<Move>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Move {
    elements: u8,
    from: u8,
    to: u8,
}

impl FromStr for Move {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cap = regex!(r"move (?P<nb>\d+) from (?P<from>\d+) to (?P<to>\d+)")
            .captures(s)
            .ok_or("regex does not match")?;

        Ok(Move {
            elements: cap.name("nb").unwrap().as_str().parse()?,
            from: cap.name("from").unwrap().as_str().parse::<u8>()? - 1,
            to: cap.name("to").unwrap().as_str().parse::<u8>()? - 1,
        })
    }
}

impl FromStr for Supplies {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let trimmed = s.trim().replace("\r\n", "\n");
        let parts = trimmed.split_once("\n\n").ok_or("Invalid input")?;

        let mut stack_lines: Vec<_> = parts.0.lines().map(|line| line.chars().collect::<Vec<_>>()).collect();
        let nb_stacks = stack_lines.last().unwrap().len();

        // Remove the last line containing the stack numbers
        stack_lines.pop();

        let mut stacks = vec![];
        let mut stack_idx = 1;

        while stack_idx < nb_stacks {
            let mut stack = Vec::with_capacity(stack_lines.len() - 1);
            for line in &stack_lines {
                let container = line[stack_idx];

                if container != ' ' {
                    stack.insert(0, line[stack_idx]);
                }
            }

            stacks.push(stack);
            stack_idx += 4; // 4 spaces between stack columns
        }

        Ok(Supplies {
            stacks,
            moves: parts
                .1
                .lines()
                .map(|line| line.parse())
                .collect::<Result<Vec<_>, Self::Err>>()?,
        })
    }
}

impl Supplies {
    fn execute(&mut self, mv: &Move) {
        let from = &mut self.stacks[mv.from as usize];
        let to_move = from
            .drain(from.len() - (mv.elements as usize)..)
            .rev()
            .collect::<Vec<_>>();
        self.stacks[mv.to as usize].extend(to_move);
    }

    fn execute_9001(&mut self, mv: &Move) {
        let from = &mut self.stacks[mv.from as usize];
        let to_move = from.drain(from.len() - (mv.elements as usize)..).collect::<Vec<_>>();
        self.stacks[mv.to as usize].extend(to_move);
    }

    #[allow(dead_code)]
    fn print_stack(&self) {
        for stack in &self.stacks {
            println!("{}", stack.iter().collect::<String>());
        }
    }
}

pub fn parse_input(input: &str) -> ParseResult {
    input.parse().unwrap()
}

// Generic function to execute a list of moves using a given function
fn execute(input: &ParseResult, exec: &mut dyn FnMut(&mut Supplies, &Move)) -> String
{
    let mut supplies = input.clone();

    for mv in input.moves.iter() {
        exec(&mut supplies, mv);
    }

    supplies
        .stacks
        .iter()
        .map(|stack| stack.last().map(|c| c.to_string()).unwrap_or_else(|| "".into()))
        .collect()
}

pub fn part1(input: &ParseResult) -> String { execute(input, &mut Supplies::execute) }

pub fn part2(input: &ParseResult) -> String { execute(input, &mut Supplies::execute_9001) }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_move() {
        let mv = "move 10 from 4 to 9".parse::<Move>();
        assert_eq!(
            mv.unwrap(),
            Move {
                elements: 10,
                from: 3,
                to: 8
            }
        );
    }
}
