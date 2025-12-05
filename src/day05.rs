use std::{cmp::max, ops::RangeInclusive};

use failure::Error;

mod parse {
    use failure::{Error, err_msg};
    use nom::{
        IResult, Parser,
        bytes::complete::tag,
        character::complete::newline,
        combinator::{all_consuming, map},
        multi::many1,
        sequence::{separated_pair, terminated},
    };
    use std::ops::RangeInclusive;

    use crate::parsers::unsigned;

    fn range(input: &str) -> IResult<&str, RangeInclusive<u64>> {
        map(
            separated_pair(unsigned, tag("-"), unsigned),
            |(start, end)| start..=end,
        )
        .parse(input)
    }

    fn ranges(input: &str) -> IResult<&str, Box<[RangeInclusive<u64>]>> {
        map(many1(terminated(range, newline)), Vec::into_boxed_slice).parse(input)
    }

    fn ids(input: &str) -> IResult<&str, Box<[u64]>> {
        map(many1(terminated(unsigned, newline)), Vec::into_boxed_slice).parse(input)
    }

    pub(super) fn parse_input(
        input: &str,
    ) -> Result<(Box<[RangeInclusive<u64>]>, Box<[u64]>), Error> {
        all_consuming(separated_pair(ranges, newline, ids))
            .parse(input)
            .map(|(_, res)| res)
            .map_err(|err| err_msg(format!("Failed to parse input: {}", err)))
    }
}

struct FreshnessChecker {
    fresh_ranges: Box<[RangeInclusive<u64>]>,
}

impl FreshnessChecker {
    fn new(fresh_ranges: &[RangeInclusive<u64>]) -> Self {
        let mut fresh_ranges = fresh_ranges.to_vec();
        fresh_ranges.sort_by_key(|range| (*range.start(), *range.end()));

        let mut index = 0;

        while index < fresh_ranges.len() - 1 {
            if fresh_ranges[index].contains(fresh_ranges[index + 1].start()) {
                let lower = *fresh_ranges[index].start();
                let upper = max(*fresh_ranges[index].end(), *fresh_ranges[index + 1].end());
                fresh_ranges[index] = lower..=upper;
                fresh_ranges.remove(index + 1);
            } else {
                index += 1;
            }
        }

        fresh_ranges.shrink_to_fit();

        FreshnessChecker {
            fresh_ranges: fresh_ranges.into_boxed_slice(),
        }
    }

    fn is_fresh(&self, ingredient: u64) -> bool {
        let mut lower = 0;
        let mut upper = self.fresh_ranges.len();

        while upper > lower {
            let mid = (lower + upper) / 2;

            let range = &self.fresh_ranges[mid];

            if ingredient < *range.start() {
                upper = mid;
            } else if ingredient > *range.end() {
                lower = mid + 1;
            } else {
                return true;
            }
        }

        false
    }

    fn total_fresh_ingredients(&self) -> u64 {
        self.fresh_ranges
            .iter()
            .map(|range| *range.end() - range.start() + 1)
            .sum()
    }
}

fn count_fresh_ingredients(checker: &FreshnessChecker, ingredients: &[u64]) -> usize {
    ingredients
        .iter()
        .filter(|&&ingredient| checker.is_fresh(ingredient))
        .count()
}

pub struct Solver {}

impl super::Solver for Solver {
    type Problem = (Box<[RangeInclusive<u64>]>, Box<[u64]>);

    fn parse_input(data: String) -> Result<Self::Problem, Error> {
        parse::parse_input(&data)
    }

    fn solve((fresh_ranges, ingredients): Self::Problem) -> (Option<String>, Option<String>) {
        let checker = FreshnessChecker::new(&fresh_ranges);
        let part1 = count_fresh_ingredients(&checker, &ingredients);
        let part2 = checker.total_fresh_ingredients();
        (Some(part1.to_string()), Some(part2.to_string()))
    }
}
