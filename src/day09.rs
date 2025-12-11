use crate::common::{Direction, Position};
use failure::{Error, err_msg};
use itertools::Itertools;
use std::{
    cmp::{max, min},
    ops::RangeInclusive,
};

fn get_sector(pos_a: Position, pos_b: Position, pos_c: Position, clockwise: bool) -> Sector {
    let d1 = pos_b.direction_to(pos_c);
    let d2 = pos_b.direction_to(pos_a);

    if clockwise {
        Sector::new(d1, d2)
    } else {
        Sector::new(d2, d1)
    }
}

#[derive(Debug, Clone, Copy)]
struct Sector {
    start: Direction,
    end: Direction,
}

impl Sector {
    fn new(start: Direction, end: Direction) -> Self {
        Sector { start, end }
    }

    fn contains(self, dir: Direction) -> bool {
        self.start.angle_to(dir) < self.start.angle_to(self.end)
    }
}

struct InternalChecker {
    tiles: Vec<Position>,
    vertical_lines: Vec<(i64, RangeInclusive<i64>)>,
    horizontal_lines: Vec<(i64, RangeInclusive<i64>)>,
    sectors: Vec<Sector>,
}

impl InternalChecker {
    fn new(tiles: &[Position]) -> Self {
        let clockwise = [&tiles[tiles.len() - 1]]
            .into_iter()
            .chain(tiles.iter())
            .chain([&tiles[0]])
            .tuple_windows()
            .map(|(&pos_a, &pos_b, &pos_c)| {
                let v1 = pos_b - pos_a;
                let v2 = pos_c - pos_b;
                let prod = v1.x * v2.y - v2.x * v1.y;
                assert_ne!(prod, 0);
                prod / prod.abs()
            })
            .sum::<i64>()
            < 0;

        let vertical_lines = tiles
            .iter()
            .chain([&tiles[0]])
            .tuple_windows()
            .filter_map(|(pos_a, pos_b)| {
                if pos_a.x == pos_b.x {
                    Some((pos_a.x, (min(pos_a.y, pos_b.y)..=max(pos_a.y, pos_b.y))))
                } else {
                    None
                }
            })
            .sorted_by_key(|(x, _)| *x)
            .collect();

        let horizontal_lines = tiles
            .iter()
            .chain([&tiles[0]])
            .tuple_windows()
            .filter_map(|(pos_a, pos_b)| {
                if pos_a.y == pos_b.y {
                    Some((pos_a.y, (min(pos_a.x, pos_b.x)..=max(pos_a.x, pos_b.x))))
                } else {
                    None
                }
            })
            .sorted_by_key(|(y, _)| *y)
            .collect();

        let sectors = [&tiles[tiles.len() - 1]]
            .into_iter()
            .chain(tiles.iter())
            .chain([&tiles[0]])
            .tuple_windows()
            .map(|(&pos_a, &pos_b, &pos_c)| get_sector(pos_a, pos_b, pos_c, clockwise))
            .collect();

        InternalChecker {
            tiles: tiles.to_vec(),
            vertical_lines,
            horizontal_lines,
            sectors,
        }
    }

    fn is_internal(&self, tile_a_idx: usize, tile_b_idx: usize) -> bool {
        let tile_a = self.tiles[tile_a_idx];
        let tile_b = self.tiles[tile_b_idx];

        let direction = tile_a.direction_to(tile_b);
        if !self.sectors[tile_a_idx].contains(direction) {
            return false;
        }

        if !self.sectors[tile_b_idx].contains(direction.reverse()) {
            return false;
        }

        let min_x = min(tile_a.x, tile_b.x);
        let max_x = max(tile_a.x, tile_b.x);
        let min_y = min(tile_a.y, tile_b.y);
        let max_y = max(tile_a.y, tile_b.y);

        if self
            .horizontal_lines
            .iter()
            .skip_while(|(y, _)| *y <= min_y)
            .take_while(|(y, _)| *y < max_y)
            .any(|(_, xs)| *xs.start() < max_x && *xs.end() > min_x)
        {
            return false;
        }

        if self
            .vertical_lines
            .iter()
            .skip_while(|(x, _)| *x <= min_x)
            .take_while(|(x, _)| *x < max_x)
            .any(|(_, ys)| *ys.start() < max_y && *ys.end() > min_y)
        {
            return false;
        }

        true
    }
}

fn find_max_area<F>(tiles: &[Position], is_valid: F) -> u64
where
    F: Fn(usize, usize) -> bool,
{
    let tiles_by_x: Vec<_> = tiles
        .iter()
        .cloned()
        .enumerate()
        .sorted_by_key(|(_, pos)| (pos.x, pos.y))
        .map(|(index, _)| index)
        .collect();

    let (min_y, max_y) = tiles
        .iter()
        .map(|pos| pos.y)
        .minmax()
        .into_option()
        .unwrap();

    let mut best = 0;

    for &tile_a_idx in tiles_by_x.iter() {
        let tile_a = tiles[tile_a_idx];
        let max_height = max(tile_a.y.abs_diff(min_y), tile_a.y.abs_diff(max_y)) + 1;
        for &tile_b_idx in tiles_by_x.iter().rev() {
            let tile_b = tiles[tile_b_idx];
            if tile_b.x < tile_a.x {
                break;
            }
            let area = (tile_a.x.abs_diff(tile_b.x) + 1) * (tile_a.y.abs_diff(tile_b.y) + 1);
            if area > best && is_valid(tile_a_idx, tile_b_idx) {
                best = area;
            }

            if (tile_a.x.abs_diff(tile_b.x) + 1) * max_height < best {
                break;
            }
        }
    }

    best
}

pub struct Solver {}

impl super::Solver for Solver {
    type Problem = Box<[Position]>;

    fn parse_input(data: String) -> Result<Self::Problem, Error> {
        data.lines()
            .map(|line| {
                let (x_str, y_str) = line
                    .split_once(",")
                    .ok_or_else(|| err_msg(format!("Invalid position: {}", line)))?;
                let x = x_str.parse()?;
                let y = y_str.parse()?;
                Ok(Position { x, y })
            })
            .collect()
    }

    fn solve(tiles: Self::Problem) -> (Option<String>, Option<String>) {
        let part1 = find_max_area(&tiles, |_, _| true);

        let internal_checker = InternalChecker::new(&tiles);
        let part2 = find_max_area(&tiles, |pos_a, pos_b| {
            internal_checker.is_internal(pos_a, pos_b)
        });

        (Some(part1.to_string()), Some(part2.to_string()))
    }
}
