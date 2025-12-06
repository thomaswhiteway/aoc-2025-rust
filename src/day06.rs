use failure::Error;

mod parse {
    use failure::Error;
    use failure::err_msg;
    use nom::character::complete::space0;
    use nom::{
        IResult, Parser,
        branch::alt,
        bytes::complete::tag,
        character::complete::{newline, space1},
        combinator::{all_consuming, map, value},
        multi::{many1, separated_list1},
        sequence::terminated,
    };

    use crate::{day06::Operation, parsers::unsigned};

    fn operation(input: &str) -> IResult<&str, Operation> {
        alt((
            value(Operation::Add, tag("+")),
            value(Operation::Multiply, tag("*")),
        ))
        .parse(input)
    }

    pub(super) fn parse_input(input: &str) -> Result<(Box<[Box<[u64]>]>, Box<[Operation]>), Error> {
        let num_row = map(separated_list1(space1, unsigned), Vec::into_boxed_slice);
        let op_row = map(separated_list1(space1, operation), Vec::into_boxed_slice);
        all_consuming((
            map(
                many1(terminated(num_row, (space0, newline))),
                Vec::into_boxed_slice,
            ),
            terminated(op_row, (space0, newline)),
        ))
        .parse(input)
        .map(|(_, out)| out)
        .map_err(|err| err_msg(format!("Failed to parse input: {}", err)))
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Operation {
    Add,
    Multiply,
}

impl Operation {
    fn apply(self, nums: impl Iterator<Item = u64>) -> u64 {
        match self {
            Operation::Add => nums.fold(0, |x, y| x + y),
            Operation::Multiply => nums.fold(1, |x, y| x * y),
        }
    }
}

fn calculate_grand_total(numbers: &[Box<[u64]>], operations: &[Operation]) -> u64 {
    (0..operations.len())
        .map(|index| operations[index].apply(numbers.iter().map(|row| row[index])))
        .sum()
}

pub struct Solver {}

impl super::Solver for Solver {
    type Problem = (Box<[Box<[u64]>]>, Box<[Operation]>);

    fn parse_input(data: String) -> Result<Self::Problem, Error> {
        parse::parse_input(&data)
    }

    fn solve((numbers, operations): Self::Problem) -> (Option<String>, Option<String>) {
        let part1 = calculate_grand_total(&numbers, &operations);
        (Some(part1.to_string()), None)
    }
}
