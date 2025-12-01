use std::fmt::Display;

use failure::Error;
use parse::parse_input;

mod parse {
    use crate::parsers::unsigned;
    use failure::{Error, err_msg};
    use nom::branch::alt;
    use nom::bytes::complete::tag;
    use nom::character::complete::newline;
    use nom::combinator::{all_consuming, map, value};
    use nom::multi::many1;
    use nom::sequence::terminated;
    use nom::{IResult, Parser};

    use super::{Direction, Rotation};

    fn direction(s: &str) -> IResult<&str, Direction> {
        alt((
            value(Direction::Left, tag("L")),
            value(Direction::Right, tag("R")),
        ))
        .parse(s)
    }

    fn rotation(s: &str) -> IResult<&str, Rotation> {
        map((direction, unsigned), |(direction, clicks)| Rotation {
            direction,
            clicks,
        })
        .parse(s)
    }

    pub(super) fn parse_input(s: &str) -> Result<Box<[Rotation]>, Error> {
        all_consuming(map(
            many1(terminated(rotation, newline)),
            Vec::into_boxed_slice,
        ))
        .parse(s)
        .map_err(|err| err_msg(format!("Failed to parse input: {}", err)))
        .map(|(_, rotations)| rotations)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Left,
    Right,
}

#[derive(Debug, Clone, Copy)]
pub struct Rotation {
    direction: Direction,
    clicks: u64,
}

impl Rotation {
    fn apply(&self, mut pos: u64) -> (u64, usize) {
        let mut zeroes = 0;
        match self.direction {
            Direction::Left => {
                while pos < self.clicks {
                    if pos != 0 {
                        zeroes += 1;
                    }
                    pos += 100;
                }

                pos -= self.clicks;

                if pos == 0 {
                    zeroes += 1
                }
            }
            Direction::Right => {
                pos += self.clicks;
                while pos >= 100 {
                    pos -= 100;
                    zeroes += 1;
                }
            }
        }

        (pos, zeroes)
    }
}

impl Display for Rotation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.direction {
            Direction::Left => write!(f, "L{}", self.clicks),
            Direction::Right => write!(f, "R{}", self.clicks),
        }
    }
}

fn count_zeroes(rotations: &[Rotation]) -> usize {
    rotations
        .iter()
        .scan(50, |current, rotation| {
            *current = rotation.apply(*current).0;
            Some(*current)
        })
        .filter(|pos| *pos == 0)
        .map(|_| 1)
        .sum()
}

fn count_total_zeroes(rotations: &[Rotation]) -> usize {
    rotations
        .iter()
        .scan(50, |current, rotation| {
            let (new, zeroes) = rotation.apply(*current);
            *current = new;
            Some(zeroes)
        })
        .sum()
}

pub struct Solver {}

impl super::Solver for Solver {
    type Problem = Box<[Rotation]>;

    fn parse_input(data: String) -> Result<Self::Problem, Error> {
        parse_input(&data)
    }

    fn solve(rotations: Self::Problem) -> (Option<String>, Option<String>) {
        let part1 = count_zeroes(&rotations);
        let part2 = count_total_zeroes(&rotations);

        (Some(part1.to_string()), Some(part2.to_string()))
    }
}
