use failure::{Error, err_msg};
use itertools::Itertools;
use std::cmp::Reverse;

fn make_connections(boxes: &[(i64, i64, i64)]) -> Vec<(usize, usize)> {
    boxes
        .iter()
        .enumerate()
        .flat_map(|(box_a, pos_a)| {
            (box_a + 1..)
                .zip(boxes[box_a + 1..].iter())
                .map(move |(box_b, pos_b)| ((box_a, pos_a), (box_b, pos_b)))
        })
        .sorted_by_cached_key(|((_, (x_a, y_a, z_a)), (_, (x_b, y_b, z_b)))| {
            (x_a - x_b).pow(2) + (y_a - y_b).pow(2) + (z_a - z_b).pow(2)
        })
        .map(|((box_a, _), (box_b, _))| (box_a, box_b))
        .take(1000)
        .collect()
}

fn find_circuits(connections: &[(usize, usize)]) -> Vec<Vec<usize>> {
    let max_box = connections
        .iter()
        .flat_map(|(box_a, box_b)| [*box_a, *box_b])
        .max()
        .unwrap_or_default();
    let mut circuits: Vec<Vec<usize>> = vec![];
    let mut box_circuits: Vec<Option<usize>> = (0..=max_box).map(|_| None).collect();

    for &(box_a, box_b) in connections {
        match (box_circuits[box_a], box_circuits[box_b]) {
            (Some(circ_a), Some(circ_b)) => {
                if circ_a != circ_b {
                    let circuit_b = circuits.splice(circ_b..=circ_b, [vec![]]).next().unwrap();
                    for &box_ix in circuit_b.iter() {
                        box_circuits[box_ix] = Some(circ_a)
                    }
                    circuits[circ_a].extend(circuit_b);
                }
            }
            (Some(circ_a), None) => {
                circuits[circ_a].push(box_b);
                box_circuits[box_b] = Some(circ_a);
            }
            (None, Some(circ_b)) => {
                circuits[circ_b].push(box_a);
                box_circuits[box_a] = Some(circ_b);
            }
            (None, None) => {
                let circ = circuits.len();
                circuits.push(vec![box_a, box_b]);
                box_circuits[box_a] = Some(circ);
                box_circuits[box_b] = Some(circ);
            }
        }
    }

    circuits.retain(|circ| !circ.is_empty());
    circuits
}

fn solve_part_1(boxes: &[(i64, i64, i64)]) -> usize {
    let connections = make_connections(boxes);
    let mut circuits = find_circuits(&connections);
    circuits.sort_by_key(|circuit| Reverse(circuit.len()));
    (0..3)
        .map(|idx| circuits.get(idx).map(|circ| circ.len()).unwrap_or(1))
        .product()
}

pub struct Solver {}

impl super::Solver for Solver {
    type Problem = Box<[(i64, i64, i64)]>;

    fn parse_input(data: String) -> Result<Self::Problem, Error> {
        data.lines()
            .map(|line| {
                let mut parts = line.split(",");
                let mut get_coordinate = |coord: &str| -> Result<i64, Error> {
                    parts
                        .next()
                        .ok_or_else(|| err_msg(format!("No {} coordinate", coord)))?
                        .parse()
                        .map_err(|err| err_msg(format!("Invalid {} coordinate: {}", coord, err)))
                };
                let x = get_coordinate("x")?;
                let y = get_coordinate("y")?;
                let z = get_coordinate("z")?;

                Ok((x, y, z))
            })
            .collect::<Result<Vec<_>, _>>()
            .map(Vec::into_boxed_slice)
    }

    fn solve(boxes: Self::Problem) -> (Option<String>, Option<String>) {
        let part1 = solve_part_1(&boxes);
        (Some(part1.to_string()), None)
    }
}
