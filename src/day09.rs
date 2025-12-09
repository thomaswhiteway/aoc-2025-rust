use failure::{Error, err_msg};
use itertools::Itertools;
use crate::common::Position;
use std::cmp::max;

fn find_max_area(tiles: &[Position]) -> u64 {
    let tiles_by_x: Vec<_> = tiles.iter().cloned().sorted_by_key(|pos| (pos.x, pos.y)).collect();
    let (min_y, max_y) = tiles.iter().map(|pos| pos.y).minmax().into_option().unwrap();

    let mut best = 0;

    for tile_a in tiles_by_x.iter() {
        let max_height = max(tile_a.y.abs_diff(min_y), tile_a.y.abs_diff(max_y)) + 1;
        for tile_b in tiles_by_x.iter().rev() {
            if tile_b.x < tile_a.x {
                break
            }
            let area = (tile_a.x.abs_diff(tile_b.x) + 1) *(tile_a.y.abs_diff(tile_b.y) + 1);
            if area > best {
                best = area;
            } else if (tile_a.x.abs_diff(tile_b.x) + 1) * max_height < best {
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
        data.lines().map(|line| {
            let (x_str, y_str) = line.split_once(",").ok_or_else(|| err_msg(format!("Invalid position: {}", line)))?;
            let x = x_str.parse()?;
            let y = y_str.parse()?;
            Ok(Position { x, y })
        }).collect()
    }

    fn solve(tiles: Self::Problem) -> (Option<String>, Option<String>) {
        let part1 = find_max_area(&tiles);
        (Some(part1.to_string()), None)
    }
}
