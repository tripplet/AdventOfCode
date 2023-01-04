use std::{
    borrow::Cow,
    collections::{HashMap, VecDeque},
    fmt::{Debug, Formatter},
};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, line_ending, one_of},
    combinator::map,
    multi::separated_list1,
    sequence::{separated_pair, tuple},
    IResult,
};

use crate::utils::ws;

type Number = u64;
type ParseResult<'a> = HashMap<&'a str, MonkeyEquation<'a>>;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Op {
    Add,
    Sub,
    Mult,
    Div,
    Eq,
}

#[derive(Clone)]
pub enum Variable<'a> {
    Number(Number),
    MonkeyRef(&'a str),
}

#[derive(Clone)]
pub enum MonkeyEquation<'a> {
    Number(Number),
    Operation {
        op: Op,
        left: Variable<'a>,
        right: Variable<'a>,
    },
}

impl Debug for Variable<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Variable::Number(nb) => write!(f, "{}", nb),
            Variable::MonkeyRef(s) => write!(f, "{}", s),
        }
    }
}

impl Debug for MonkeyEquation<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Number(arg0) => write!(f, "{}", arg0),
            Self::Operation { op, left, right } => write!(f, "{:?} {:?} {:?}", left, op, right),
        }
    }
}

impl Debug for Op {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Op::Add => write!(f, "+"),
            Op::Sub => write!(f, "-"),
            Op::Mult => write!(f, "*"),
            Op::Div => write!(f, "/"),
            Op::Eq => write!(f, "="),
        }
    }
}

impl Variable<'_> {
    fn resolve_if_direct_ref(&self, map: &HashMap<&str, MonkeyEquation>) -> Cow<Variable> {
        match self {
            Variable::Number(_) => Cow::Borrowed(self),
            Variable::MonkeyRef(monkey) => map
                .get(monkey)
                .and_then(|eq| eq.eval())
                .map(|nb| Cow::Owned(Variable::Number(nb)))
                .unwrap_or(Cow::Borrowed(self)),
        }
    }
}

impl<'a> MonkeyEquation<'a> {
    fn parse(input: &'a str) -> IResult<&str, Self> {
        alt((
            map(nom::character::complete::u64, MonkeyEquation::Number),
            map(
                tuple((
                    map(alpha1, Variable::MonkeyRef),
                    map(ws(one_of("+-*/")), |symbol| match symbol {
                        '+' => Op::Add,
                        '-' => Op::Sub,
                        '*' => Op::Mult,
                        '/' => Op::Div,
                        '=' => Op::Eq,
                        _ => unreachable!(),
                    }),
                    map(alpha1, Variable::MonkeyRef),
                )),
                |res| MonkeyEquation::Operation {
                    left: res.0,
                    op: res.1,
                    right: res.2,
                },
            ),
        ))(input)
    }

    fn eval(&self) -> Option<Number> {
        match self {
            MonkeyEquation::Number(nb) => Some(*nb),
            MonkeyEquation::Operation { op, left, right } => match (left, op, right) {
                (Variable::Number(nb1), Op::Add, Variable::Number(nb2)) => Some(nb1 + nb2),
                (Variable::Number(nb1), Op::Sub, Variable::Number(nb2)) => Some(nb1 - nb2),
                (Variable::Number(nb1), Op::Mult, Variable::Number(nb2)) => Some(nb1 * nb2),
                (Variable::Number(nb1), Op::Div, Variable::Number(nb2)) => Some(nb1 / nb2),
                _ => None,
            },
        }
    }

    fn rearrange(&self, nb: Number, map: &HashMap<&str, MonkeyEquation>) -> Option<(String, Number)> {
        match self {
            MonkeyEquation::Number(_) => None,
            MonkeyEquation::Operation { op, left, right } => {
                let left_cow = left.resolve_if_direct_ref(map);
                let left = left_cow.as_ref();

                let right_cow = right.resolve_if_direct_ref(map);
                let right = right_cow.as_ref();

                match (left, op, right) {
                    (Variable::Number(nb1), Op::Add, Variable::MonkeyRef(monkey)) => {
                        Some((monkey.to_string(), nb - nb1))
                    }
                    (Variable::MonkeyRef(monkey), Op::Add, Variable::Number(nb1)) => {
                        Some((monkey.to_string(), nb - nb1))
                    }

                    (Variable::Number(nb1), Op::Sub, Variable::MonkeyRef(monkey)) => {
                        Some((monkey.to_string(), nb1 - nb))
                    }
                    (Variable::MonkeyRef(monkey), Op::Sub, Variable::Number(nb1)) => {
                        Some((monkey.to_string(), nb + nb1))
                    }

                    (Variable::Number(nb1), Op::Mult, Variable::MonkeyRef(monkey)) => {
                        Some((monkey.to_string(), nb / nb1))
                    }
                    (Variable::MonkeyRef(monkey), Op::Mult, Variable::Number(nb1)) => {
                        Some((monkey.to_string(), nb / nb1))
                    }

                    (Variable::Number(nb1), Op::Div, Variable::MonkeyRef(monkey)) => {
                        Some((monkey.to_string(), nb1 / nb))
                    }
                    (Variable::MonkeyRef(monkey), Op::Div, Variable::Number(nb1)) => {
                        Some((monkey.to_string(), nb * nb1))
                    }

                    _ => None,
                }
            }
        }
    }
}

pub fn parse_input(input: &str) -> ParseResult {
    separated_list1(line_ending, separated_pair(alpha1, ws(tag(":")), MonkeyEquation::parse))(input)
        .unwrap()
        .1
        .into_iter()
        .collect()
}

pub fn part1(input: &ParseResult) -> Number {
    let mut equations = input.clone();

    let mut work_queue: VecDeque<&str> = VecDeque::new();
    work_queue.push_back("root");

    while let Some(monkey_to_solve) = work_queue.pop_back() {
        let Some((op, nb1, nb2)) = (match &equations[monkey_to_solve] {
            MonkeyEquation::Number(_) => { None },
            MonkeyEquation::Operation{op, left, right } => {
                let nb_left = match left {
                    Variable::Number(_) => left.clone(),
                    Variable::MonkeyRef(monkey) => {
                        if let MonkeyEquation::Number(nb) = equations[monkey] {
                            Variable::Number(nb)
                        } else {
                            work_queue.push_back(monkey_to_solve);
                            work_queue.push_back(monkey);
                            Variable::MonkeyRef(monkey)
                        }
                    }
                };

                let nb_right = match right {
                    Variable::Number(_) => right.clone(),
                    Variable::MonkeyRef(monkey) => {
                        if let MonkeyEquation::Number(nb) = equations[monkey] {
                            Variable::Number(nb)
                        } else {
                            work_queue.push_back(monkey_to_solve);
                            work_queue.push_back(monkey);
                            Variable::MonkeyRef(monkey)
                        }
                    }
                };

                Some((op, nb_left, nb_right))
            },
        })
        else {
            continue;
        };

        match (&nb1, &nb2) {
            (Variable::Number(nb1), Variable::Number(nb2)) => {
                equations.insert(
                    monkey_to_solve,
                    MonkeyEquation::Number(match op {
                        Op::Add => nb1 + nb2,
                        Op::Sub => nb1 - nb2,
                        Op::Mult => nb1 * nb2,
                        Op::Div => nb1 / nb2,
                        Op::Eq => unreachable!(),
                    }),
                );
            }
            _ => {
                equations.insert(
                    monkey_to_solve,
                    MonkeyEquation::Operation {
                        op: *op,
                        left: nb1.clone(),
                        right: nb2.clone(),
                    },
                );
            }
        }
    }

    let MonkeyEquation::Number(result) = equations["root"] else {
        panic!("Algorithm not working");
    };
    result
}

pub fn part2(input: &ParseResult) -> Number {
    let mut equations = input.clone();
    let mut already_solved = HashMap::new();

    if let MonkeyEquation::Operation { op: _, left, right } = equations.remove("root").unwrap() {
        equations.insert(
            "root",
            MonkeyEquation::Operation {
                op: Op::Eq,
                left,
                right,
            },
        );
    }

    let mut work_queue: VecDeque<&str> = VecDeque::new();
    work_queue.push_back("root");

    equations.insert(
        "humn",
        MonkeyEquation::Operation {
            op: Op::Add,
            left: Variable::Number(0),
            right: Variable::MonkeyRef("finished"),
        },
    );

    while let Some(monkey_to_solve) = work_queue.pop_back() {
        if let Some(nb) = already_solved.get_mut(monkey_to_solve) {
            if *nb == 3 {
                continue;
            }

            *nb += 1;
        } else {
            already_solved.insert(monkey_to_solve, 1);
        }

        let Some(solved_eq) = (match &equations[monkey_to_solve] {
            MonkeyEquation::Number(_) => { None },
            MonkeyEquation::Operation{op, left, right } => {
                let nb_left = match left {
                    Variable::Number(_) => left.clone(),
                    Variable::MonkeyRef(monkey) => {
                        if *monkey == "humn" {
                            Variable::MonkeyRef(monkey)
                        }
                        else if let MonkeyEquation::Number(nb) = equations[monkey] {
                            Variable::Number(nb)
                        } else {
                            work_queue.push_back(monkey_to_solve);
                            work_queue.push_back(monkey);
                            Variable::MonkeyRef(monkey)
                        }
                    }
                };

                let nb_right = match right {
                    Variable::Number(_) => right.clone(),
                    Variable::MonkeyRef(monkey) => {
                        if *monkey == "humn" {
                            Variable::MonkeyRef(monkey)
                        }
                        else if let MonkeyEquation::Number(nb) = equations[monkey] {
                            Variable::Number(nb)
                        } else {
                            work_queue.push_back(monkey_to_solve);
                            work_queue.push_back(monkey);
                            Variable::MonkeyRef(monkey)
                        }
                    }
                };

                Some(MonkeyEquation::Operation {
                    op: *op,
                    left: nb_left,
                    right: nb_right,
                })
            },
        })
        else {
            continue;
        };

        if let Some(result) = solved_eq.eval() {
            equations.insert(monkey_to_solve, MonkeyEquation::Number(result));
        } else {
            equations.insert(monkey_to_solve, solved_eq);
        }
    }

    //dbg!(&equations);

    // reverse the root equation
    let (rev_root, mut value) = if let MonkeyEquation::Operation { op: _, left, right } = equations.get("root").unwrap()
    {
        match (left, right) {
            (Variable::MonkeyRef(monkey), Variable::Number(nb)) => (*monkey, *nb),
            (Variable::Number(nb), Variable::MonkeyRef(monkey)) => (*monkey, *nb),
            _ => panic!(),
        }
    } else {
        panic!()
    };

    let mut rev_root = rev_root.to_string();

    loop {
        let Some((rev_root_new, value_new)) = equations.get(rev_root.as_str()).unwrap().rearrange(value, &equations) else {
            dbg!(equations.get(rev_root.as_str()));
            panic!("Algorithm not working");
        };

        if rev_root_new == "humn" {
            return value;
        }

        rev_root = rev_root_new.to_string();
        value = value_new;
        //dbg!(&rev_root, &value);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../input/2022/day21_example.txt");
    const INPUT: &str = include_str!("../input/2022/day21.txt");

    #[test]
    fn example_part1() {
        let input = parse_input(EXAMPLE);
        assert_eq!(part1(&input), 152);
    }

    #[test]
    fn example_part2() {
        let input = parse_input(EXAMPLE);
        assert_eq!(part2(&input), 301);
    }

    #[test]
    fn input_part1() {
        let input = parse_input(INPUT);
        assert_eq!(part1(&input), 155708040358220);
    }

    #[test]
    fn input_part2() {
        let input = parse_input(INPUT);
        assert_eq!(part2(&input), 3342154812537);
    }

    #[test]
    fn test_rearrange() {
        let eq = MonkeyEquation::Operation {
            left: Variable::MonkeyRef("x"),
            op: Op::Add,
            right: Variable::Number(4),
        };
        assert_eq!(eq.rearrange(20, &HashMap::new()), Some(("x".to_string(), 16)));

        let eq = MonkeyEquation::Operation {
            left: Variable::Number(4),
            op: Op::Add,
            right: Variable::MonkeyRef("x"),
        };
        assert_eq!(eq.rearrange(20, &HashMap::new()), Some(("x".to_string(), 16)));

        let eq = MonkeyEquation::Operation {
            left: Variable::MonkeyRef("x"),
            op: Op::Sub,
            right: Variable::Number(4),
        };
        assert_eq!(eq.rearrange(20, &HashMap::new()), Some(("x".to_string(), 24)));

        let eq = MonkeyEquation::Operation {
            left: Variable::Number(30),
            op: Op::Sub,
            right: Variable::MonkeyRef("x"),
        };
        assert_eq!(eq.rearrange(20, &HashMap::new()), Some(("x".to_string(), 10)));

        let eq = MonkeyEquation::Operation {
            left: Variable::MonkeyRef("x"),
            op: Op::Mult,
            right: Variable::Number(4),
        };
        assert_eq!(eq.rearrange(20, &HashMap::new()), Some(("x".to_string(), 5)));

        let eq = MonkeyEquation::Operation {
            left: Variable::Number(4),
            op: Op::Mult,
            right: Variable::MonkeyRef("x"),
        };
        assert_eq!(eq.rearrange(20, &HashMap::new()), Some(("x".to_string(), 5)));

        let eq = MonkeyEquation::Operation {
            left: Variable::MonkeyRef("x"),
            op: Op::Div,
            right: Variable::Number(4),
        };
        assert_eq!(eq.rearrange(20, &HashMap::new()), Some(("x".to_string(), 80)));

        let eq = MonkeyEquation::Operation {
            left: Variable::Number(100),
            op: Op::Div,
            right: Variable::MonkeyRef("x"),
        };
        assert_eq!(eq.rearrange(20, &HashMap::new()), Some(("x".to_string(), 5)));
    }
}
