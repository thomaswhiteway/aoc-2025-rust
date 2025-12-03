use std::collections::HashSet;

use failure::Error;
use parse::parse_input;
use std::ops::RangeInclusive;

mod parse {
    use failure::{Error, err_msg};
    use nom::bytes::complete::tag;
    use nom::character::complete::newline;
    use nom::combinator::{all_consuming, map};
    use nom::multi::separated_list1;
    use nom::sequence::{separated_pair, terminated};
    use nom::{IResult, Parser};
    use std::ops::RangeInclusive;

    use crate::parsers::unsigned;

    fn range(s: &str) -> IResult<&str, RangeInclusive<u64>> {
        map(
            separated_pair(unsigned, tag("-"), unsigned),
            |(start, end)| start..=end,
        )
        .parse(s)
    }

    pub(super) fn parse_input(s: &str) -> Result<Box<[RangeInclusive<u64>]>, Error> {
        all_consuming(map(
            terminated(separated_list1(tag(","), range), newline),
            Vec::into_boxed_slice,
        ))
        .parse(s)
        .map_err(|err| err_msg(format!("Failed to parse input: {}", err)))
        .map(|(_, rotations)| rotations)
    }
}

fn repeat_num(num: u64, repeats: usize) -> u64 {
    let num_len = num.to_string().len();
    let offset = 10u64.pow(num_len as u32);
    (0..repeats)
        .map(|index| num * offset.pow(index as u32))
        .sum()
}

fn invalid_ids_in_range(range: RangeInclusive<u64>, repeats: usize) -> impl Iterator<Item = u64> {
    let start_len = range.start().to_string().len();
    let start_upper = if start_len % repeats == 0 {
        let upper_len = start_len / repeats;
        let offset = 10u64.pow((start_len - upper_len) as u32);
        let upper = range.start() / offset;

        if repeat_num(upper, repeats) >= *range.start() {
            upper
        } else {
            upper + 1
        }
    } else {
        10u64.pow((start_len / repeats) as u32)
    };

    let end_len = range.end().to_string().len();
    let end_upper = if end_len % repeats == 0 {
        let upper_len = end_len / repeats;
        let offset = 10u64.pow((end_len - upper_len) as u32);
        let upper = range.end() / offset;

        if repeat_num(upper, repeats) <= *range.end() {
            upper
        } else {
            upper - 1
        }
    } else {
        10u64.pow((end_len / repeats) as u32) - 1
    };

    (start_upper..=end_upper).map(move |upper| repeat_num(upper, repeats))
}

fn simple_invalid_ids_in_range(range: RangeInclusive<u64>) -> impl Iterator<Item = u64> {
    invalid_ids_in_range(range, 2)
}

fn sum_simple_invalid_ids(ranges: &[RangeInclusive<u64>]) -> u64 {
    ranges
        .iter()
        .flat_map(|range| simple_invalid_ids_in_range(range.clone()))
        .sum()
}

#[allow(unused)]
fn is_invalid_id_complex(id: u64) -> bool {
    let id_str = id.to_string();
    (1..=id_str.len() / 2).any(|len| {
        id_str.len() % len == 0 && {
            let segments = (0..id_str.len() / len)
                .map(|index| &id_str[index * len..(index + 1) * len])
                .collect::<Vec<_>>();
            segments[1..].iter().all(|segment| *segment == segments[0])
        }
    })
}

fn complex_invalid_ids_in_range(range: RangeInclusive<u64>) -> impl Iterator<Item = u64> {
    let end_len = range.end().to_string().len();
    (2..=end_len)
        .flat_map({
            let range = range.clone();
            move |repeats| invalid_ids_in_range(range.clone(), repeats)
        })
        .collect::<HashSet<_>>()
        .into_iter()
}

fn sum_complex_invalid_ids(ranges: &[RangeInclusive<u64>]) -> u64 {
    ranges
        .iter()
        .flat_map(|range| complex_invalid_ids_in_range(range.clone()))
        .sum()
}
pub struct Solver {}

impl super::Solver for Solver {
    type Problem = Box<[RangeInclusive<u64>]>;

    fn parse_input(data: String) -> Result<Self::Problem, Error> {
        parse_input(&data)
    }

    fn solve(ranges: Self::Problem) -> (Option<String>, Option<String>) {
        let part1: u64 = sum_simple_invalid_ids(&ranges);
        let part2: u64 = sum_complex_invalid_ids(&ranges);

        (Some(part1.to_string()), Some(part2.to_string()))
    }
}
