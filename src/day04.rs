use crate::common::Position;
use failure::Error;
use std::collections::HashSet;

fn find_reachable_rolls(rolls: &HashSet<Position>) -> HashSet<Position> {
    rolls
        .iter()
        .filter(|pos| pos.surrounding().filter(|adj| rolls.contains(adj)).count() < 4)
        .cloned()
        .collect()
}

fn find_removable_rolls(rolls: &HashSet<Position>) -> HashSet<Position> {
    let mut rolls = rolls.clone();
    let mut removed = HashSet::new();

    loop {
        let reachable = find_reachable_rolls(&rolls);

        if reachable.is_empty() {
            break;
        }

        rolls.retain(|p| !reachable.contains(p));
        removed.extend(reachable);
    }

    removed
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
        let part1 = find_reachable_rolls(&rolls).len();
        let part2 = find_removable_rolls(&rolls).len();
        (Some(part1.to_string()), Some(part2.to_string()))
    }
}
