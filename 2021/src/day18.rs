use std::cell::{Cell, RefCell};
use std::error::Error;
use std::rc::Rc;
use std::rc::Weak;
use std::fmt::{self, Debug, Formatter};

enum VV {
    Number(u8),
    Pair(RefCell<Rc<Sfn>>, RefCell<Rc<Sfn>>),
}

struct Sfn {
    value: RefCell<VV>,
    parent: RefCell<Weak<Sfn>>,
}

impl Debug for Sfn {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("{:?}", &self.value.borrow()))
    }
}

impl Debug for VV {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            VV::Number(nb) => f.write_fmt(format_args!("{}", nb)),
            VV::Pair(left, right) => {
                f.write_fmt(format_args!("[{:?},{:?}]", &left.borrow(), &right.borrow()))
            }
        }
    }
}

impl PartialEq for Sfn {
    fn eq(&self, other: &Self) -> bool {
        if let VV::Number(self_value) = *self.value.borrow() {
            if let VV::Number(other_value) = *other.value.borrow() {
                return self_value == other_value;
            }
            return false;
        } else if let VV::Pair(self_left, self_right) = &*self.value.borrow() {
            if let VV::Pair(other_left, other_right) = &*other.value.borrow() {
                return self_left == other_left && self_right == other_right;
            }
            return false;
        }
        return false;
    }
}

const INPUT: &str = include_str!("../input/2021/day18.txt");

pub fn main() {
    let mut now = std::time::Instant::now();
    let data = parse_input(INPUT).unwrap();
    println!("Parsing [{}]\n", humantime::format_duration(now.elapsed()));

    now = std::time::Instant::now();
    println!(
        "Part1: {} [{}]",
        part1(&data),
        humantime::format_duration(now.elapsed())
    );

    now = std::time::Instant::now();
    println!(
        "Part2: {} [{}]",
        part2(&data),
        humantime::format_duration(now.elapsed())
    );
}

fn parse_input(input: &str) -> Result<Vec<Sfn>, Box<dyn Error>> {
    let numbers: Result<Vec<Rc<_>>,_> = input.trim().lines().map(Sfn::from_str).collect();
    let unwraped_numbers: Result<Vec<Sfn>,_> = numbers?.into_iter().map(|sfn| Rc::try_unwrap(sfn)).collect();
    unwraped_numbers.map_err(|_| "".into())
}

fn part1(target: &[Sfn]) -> usize {
    if let Some(xx) = target[0].left().and_then(|v| v.left()).and_then(|v| v.right()) {
        dbg!(&xx);
        xx.explode();
    }

    42
}

fn part2(_target: &[Sfn]) -> usize {
    23
}

impl Sfn {
    #[allow(dead_code)]
    fn pair(left: u8, right: u8) -> Self {
        Sfn {
            value: VV::Pair(
                RefCell::new(Sfn::number(left).into()),
                RefCell::new(Sfn::number(right).into()),
            ).into(),
            parent: RefCell::new(Weak::default())
        }
    }

    fn number(value: u8) -> Self {
        Sfn {
            value: RefCell::new(VV::Number(value)),
            parent: RefCell::new(Weak::default())
        }
    }

    fn from_str(input: &str) -> Result<Rc<Self>, Box<dyn Error>> {
        Sfn::from_chars(&mut input.chars().peekable())
    }

    fn from_chars(chars: &mut std::iter::Peekable<std::str::Chars>) -> Result<Rc<Sfn>, Box<dyn Error>> {
        if let Some(token) = chars.next() {
            if token == '[' {
                let left = Sfn::from_chars(chars)?;
                chars.next(); // skip ','
                let right = Sfn::from_chars(chars)?;
                chars.next(); // skip ']'

                let new = Rc::new(Sfn {
                    value: VV::Pair(RefCell::new(Rc::clone(&left)), RefCell::new(Rc::clone(&right))).into(),
                    parent: RefCell::new(Weak::default()),
                });

                *left.parent.borrow_mut() = Rc::downgrade(&new);
                *right.parent.borrow_mut() = Rc::downgrade(&new);
                return Ok(new);
            } else if token.is_digit(10) {
                if let Some(next_char) = chars.peek() {
                    if next_char.is_digit(10) {
                        let second_digit = chars.next().unwrap();
                        return Ok(Rc::new(Sfn::number(format!("{}{}", token, second_digit).parse()?)));
                    }
                }
                return Ok(Rc::new(Sfn::number(token.to_digit(10).unwrap() as u8)));
            }
        }
        Err("invalid input".into())
    }

    #[allow(dead_code)]
    fn add(left: Rc<Sfn>, right: Rc<Sfn>) -> Rc<Sfn> {
        let new = Rc::new(Sfn {
            value: VV::Pair(RefCell::new(Rc::clone(&left)), RefCell::new(Rc::clone(&left))).into(),
            parent: RefCell::new(Weak::default())
        });

        *left.parent.borrow_mut() = Rc::downgrade(&new);
        *right.parent.borrow_mut() = Rc::downgrade(&new);
        new
    }

    #[allow(dead_code)]
    fn reduce(&self) {
    }

    #[allow(dead_code)]
    fn split(&self) -> bool {
        if let VV::Pair(ref left, ref right) = *self.value.borrow() {
            let nb = if let VV::Number(number) = *left.borrow().value.borrow() {
                Some(number)
            } else {
                None
            };

            if let Some(number) = nb {
                if number > 9 {
                    Sfn::replace(number, left);
                    return true;
                }
            }

            let nb = if let VV::Number(number) = *right.borrow().value.borrow() {
                Some(number)
            } else {
                None
            };

            if let Some(number) = nb {
                if number > 9 {
                    Sfn::replace(number, right);
                    return true;
                }
            }
        }
        false
    }


    fn replace(number: u8, nb: &RefCell<Rc<Sfn>>) {
        nb.swap(&RefCell::new(Rc::new(Sfn::pair(
            (number as f64 / 2.0).floor() as u8,
            (number as f64 / 2.0).ceil() as u8,
        ))));
    }

    fn left(&self) -> Option<Rc<Sfn>> {
        match *self.value.borrow() {
            VV::Pair(ref left, ..) => {
                Some(left.borrow().clone())
            },
            _ => None
        }
    }

    fn right(&self) -> Option<Rc<Sfn>> {
        match *self.value.borrow() {
            VV::Pair(.., ref right) => {
                Some(right.borrow().clone())
            },
            _ => None
        }
    }

    fn explode(&self) -> bool {
        let nb_left: u8;
        let nb_right: u8;

        if let VV::Pair(ref left, ref right) = *self.value.borrow() {
            nb_left = if let VV::Number(number) = *left.borrow().value.borrow() {
                number
            } else {
                return false;
            };

            nb_right = if let VV::Number(number) = *right.borrow().value.borrow() {
                number
            } else {
                return false;
            };
        }

        let pp = &self.parent.borrow().upgrade();

        loop {
            if let Some(cur) = pp {

            }
        }

        self.value.swap(&RefCell::new(VV::Number(0)));
        false
    }
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn parse() {
    //     let expected = Sfn::pair(1, 9);
    //     assert_eq!(expected, "[1,9]".parse::<Sfn>().unwrap());

    //     let expected = Sfn{
    //         left: Sfn::Pair {
    //             left: Sfn::Pair {
    //                 left: Sfn::pair(7,3).into(),
    //                 right: Sfn::number(1).into(),
    //                 parent: Weak::default(),
    //             }
    //             .into(),
    //             right: Sfn::Pair {
    //                 left: Sfn::pair(4, 6).into(),
    //                 right: Sfn::pair(5, 1).into(),
    //                 parent: Weak::default(),
    //             }
    //             .into(),
    //             parent: Weak::default(),
    //         }
    //         .into(),
    //         right: Sfn::Pair {
    //             left: Sfn::Pair {
    //                 left: Sfn::pair(4, 7).into(),
    //                 right: Sfn::number(4).into(),
    //                 parent: Weak::default(),
    //             }
    //             .into(),
    //             right: Sfn::Pair {
    //                 left: Sfn::pair(5, 2).into(),
    //                 right: Sfn::pair(3, 7).into(),
    //                 parent: Weak::default(),
    //             }
    //             .into(),
    //             parent: Weak::default(),
    //         }
    //         .into(),
    //         parent: Weak::default(),
    //     };

    //     assert_eq!(
    //         expected,
    //         "[[[[7,3],1],[[4,6],[5,1]]],[[[4,7],4],[[5,2],[3,7]]]]"
    //             .parse::<Sfn>()
    //             .unwrap()
    //     );
    // }

    #[test]
    fn add() {
        let expected = Sfn::from_str("[[[1,2],4],[7,3]]").unwrap();

        dbg!(&expected);

        assert_eq!(
            expected,
            Sfn::add(Sfn::from_str("[[1,2],4]").unwrap(), Sfn::from_str("[7,3]").unwrap())
        );
    }

    #[test]
    fn check_input() {
        for (idx, line) in INPUT.trim().lines().enumerate() {
            assert!(Sfn::from_str(line).is_ok(), "Error parsing line {}", idx + 1);
        }
    }

    #[test]
    fn split() {
        let check = Sfn::from_str("[13,2]").unwrap();
        check.split();
        assert_eq!(Sfn::from_str("[[6,7],2]").unwrap(), check);

        let check = Sfn::from_str("[4,17]").unwrap();
        check.split();
        assert_eq!(Sfn::from_str("[4,[8,9]]").unwrap(), check);

        let check = Sfn::from_str("[4,18]").unwrap();
        check.split();
        assert_eq!(Sfn::from_str("[4,[9,9]]").unwrap(), check);

        let check = Sfn::from_str("[13,18]").unwrap();
        check.split();
        assert_eq!(Sfn::from_str("[[6,7|,18]").unwrap(), check);
    }
}
