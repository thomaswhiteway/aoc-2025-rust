use crate::common::Position;
use failure::Error;
use std::collections::HashSet;

pub struct Rolls {
    rolls: HashSet<Position>,
}

impl Rolls {
    fn adjacent_rolls(&self, pos: Position) -> impl Iterator<Item = Position> {
        pos.surrounding().filter(|adj| self.rolls.contains(adj))
    }

    fn reachable(
        &self,
        from: Option<&HashSet<Position>>,
    ) -> (HashSet<Position>, HashSet<Position>) {
        let mut reachable = HashSet::new();
        let mut adjacent_to_reachable = HashSet::new();

        let rolls_to_check = from.unwrap_or(&self.rolls);

        for &pos in rolls_to_check.iter() {
            let adjacent: Vec<_> = self.adjacent_rolls(pos).collect();
            if adjacent.len() < 4 {
                reachable.insert(pos);
                adjacent_to_reachable.remove(&pos);
                adjacent_to_reachable
                    .extend(adjacent.into_iter().filter(|adj| !reachable.contains(adj)));
            }
        }

        (reachable, adjacent_to_reachable)
    }

    fn remove_rolls(&mut self) -> impl Iterator<Item = HashSet<Position>> {
        (0..).scan(None, |adjacent, _| {
            let (reachable, new_adjacent) = self.reachable(adjacent.as_ref());

            if reachable.is_empty() {
                return None;
            }

            *adjacent = Some(new_adjacent);

            self.rolls.retain(|pos| !reachable.contains(pos));
            Some(reachable)
        })
    }
}

pub struct Solver {}

impl super::Solver for Solver {
    type Problem = Rolls;

    fn parse_input(data: String) -> Result<Self::Problem, Error> {
        Ok(Rolls {
            rolls: data
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
                .collect(),
        })
    }

    fn solve(mut rolls: Self::Problem) -> (Option<String>, Option<String>) {
        let mut removed_iter = rolls.remove_rolls();

        let mut removed = removed_iter.next().unwrap();
        let part1 = removed.len();

        for new_removed in removed_iter {
            removed.extend(new_removed);
        }

        let part2 = removed.len();
        (Some(part1.to_string()), Some(part2.to_string()))
    }
}
