use crate::common::Position;
use failure::Error;
use std::collections::HashSet;

fn find_reachable_rolls(rolls: &HashSet<Position>) -> usize {
    rolls
        .iter()
        .filter(|pos| pos.surrounding().filter(|adj| rolls.contains(adj)).count() < 4)
        .count()
}

pub struct Solver {}

impl super::Solver for Solver {
    type Problem = HashSet<Position>;

    fn parse_input(data: String) -> Result<Self::Problem, Error> {
        Ok(data
            .lines()
            .enumerate()
            .flat_map(|(y, row)| {
                row.chars().enumerate().filter_map(move |(x, c)| {
                    if c == '@' {
                        Some(Position {
                            x: x as i64,
                            y: y as i64,
                        })
                    } else {
                        None
                    }
                })
            })
            .collect())
    }

    fn solve(rolls: Self::Problem) -> (Option<String>, Option<String>) {
        let part1 = find_reachable_rolls(&rolls);
        (Some(part1.to_string()), None)
    }
}
