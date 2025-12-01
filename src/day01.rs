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
    fn apply(&self, pos: u64) -> u64 {
        match self.direction {
            Direction::Left => {
                let mut pos = pos;
                while pos < self.clicks {
                    pos += 100
                }
                pos - self.clicks
            }
            Direction::Right => (pos + self.clicks) % 100,
        }
    }
}

fn count_zeroes(rotations: &[Rotation]) -> usize {
    rotations
        .iter()
        .scan(50, |current, rotation| {
            *current = rotation.apply(*current);
            Some(*current)
        })
        .filter(|pos| *pos == 0)
        .map(|_| 1)
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

        (Some(part1.to_string()), None)
    }
}
