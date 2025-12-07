use crate::common::{Counter, Position};
use failure::{Error, err_msg};
use std::collections::HashMap;

pub struct Grid {
    start: Position,
    splitters: Vec<Vec<bool>>,
}

impl Grid {
    fn total_splits(&self) -> (usize, usize) {
        let mut splits = 0;
        let start_row = self.start.y as usize;
        let mut beams: Counter<usize> = [(self.start.x as usize, 1)].into_iter().collect();

        for row in start_row..self.splitters.len() {
            let (split, not_split): (HashMap<_, _>, HashMap<_, _>) = beams
                .into_iter()
                .partition(|(beam, _)| self.splitters[row][*beam]);
            splits += split.len();
            beams = split
                .into_iter()
                .flat_map(|(beam, count)| [(beam - 1, count), (beam + 1, count)])
                .chain(not_split.into_iter())
                .collect();
        }

        (splits, beams.iter().map(|(_, &count)| count).sum())
    }
}

pub struct Solver {}

impl super::Solver for Solver {
    type Problem = Grid;

    fn parse_input(data: String) -> Result<Self::Problem, Error> {
        let splitters = data
            .lines()
            .map(|row| row.chars().map(|c| c == '^').collect())
            .collect();
        let start = data
            .lines()
            .enumerate()
            .find_map(|(y, row)| {
                row.chars().enumerate().find_map(|(x, c)| {
                    if c == 'S' {
                        Some(Position {
                            x: x as i64,
                            y: y as i64,
                        })
                    } else {
                        None
                    }
                })
            })
            .ok_or(err_msg("Failed to find start position"))?;

        Ok(Grid { start, splitters })
    }

    fn solve(grid: Self::Problem) -> (Option<String>, Option<String>) {
        let (part1, part2) = grid.total_splits();
        (Some(part1.to_string()), Some(part2.to_string()))
    }
}
