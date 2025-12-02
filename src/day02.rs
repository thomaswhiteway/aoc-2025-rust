use failure::Error;
use parse::parse_input;

mod parse {
    use failure::{Error, err_msg};
    use nom::bytes::complete::tag;
    use nom::character::complete::newline;
    use nom::combinator::{all_consuming, map};
    use nom::multi::separated_list1;
    use nom::sequence::{separated_pair, terminated};
    use nom::{IResult, Parser};

    use crate::parsers::unsigned;

    fn range(s: &str) -> IResult<&str, (u64, u64)> {
        separated_pair(unsigned, tag("-"), unsigned).parse(s)
    }

    pub(super) fn parse_input(s: &str) -> Result<Box<[(u64, u64)]>, Error> {
        all_consuming(map(
            terminated(separated_list1(tag(","), range), newline),
            Vec::into_boxed_slice,
        ))
        .parse(s)
        .map_err(|err| err_msg(format!("Failed to parse input: {}", err)))
        .map(|(_, rotations)| rotations)
    }
}

fn invalid_ids_in_range(&(start, end): &(u64, u64)) -> impl Iterator<Item = u64> {
    let start_len = start.to_string().len();
    let start_upper = if start_len % 2 == 0 {
        let offset = 10u64.pow(start_len as u32 / 2);
        let upper = start / offset;
        let lower = start % offset;
        if upper >= lower { upper } else { upper + 1 }
    } else {
        10u64.pow(start_len as u32 / 2)
    };

    let end_len = end.to_string().len();
    let end_upper = if end_len % 2 == 0 {
        let offset = 10u64.pow(end_len as u32 / 2);
        let upper = end / offset;
        let lower = end % offset;
        if upper <= lower { upper } else { upper - 1 }
    } else {
        10u64.pow(end_len as u32 / 2) - 1
    };

    (start_upper..=end_upper).map(|upper| {
        let upper_len = upper.to_string().len();
        let offset = 10u64.pow(upper_len as u32);
        upper * offset + upper
    })
}

fn count_invalid_ids(ranges: &[(u64, u64)]) -> u64 {
    ranges
        .iter()
        .flat_map(|range| invalid_ids_in_range(&range))
        .sum()
}

pub struct Solver {}

impl super::Solver for Solver {
    type Problem = Box<[(u64, u64)]>;

    fn parse_input(data: String) -> Result<Self::Problem, Error> {
        parse_input(&data)
    }

    fn solve(ranges: Self::Problem) -> (Option<String>, Option<String>) {
        let part1 = count_invalid_ids(&ranges);
        (Some(part1.to_string()), None)
    }
}
