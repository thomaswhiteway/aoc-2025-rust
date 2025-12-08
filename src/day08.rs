use failure::{Error, err_msg};
use itertools::Itertools;
use std::cmp::Reverse;

fn make_connections(boxes: &[(i64, i64, i64)]) -> impl Iterator<Item = (usize, usize)> {
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
}

struct Circuits {
    circuits: Vec<Vec<usize>>,
    box_circuits: Vec<Option<usize>>,
}

impl Circuits {
    fn new(num_boxes: usize) -> Self {
        Circuits {
            circuits: vec![],
            box_circuits: (0..num_boxes).map(|_| None).collect(),
        }
    }

    fn connect(&mut self, box_a: usize, box_b: usize) {
        match (self.box_circuits[box_a], self.box_circuits[box_b]) {
            (Some(circ_a), Some(circ_b)) => {
                if circ_a != circ_b {
                    let (merge_to, merge_from) = if circ_a > circ_b {
                        (circ_b, circ_a)
                    } else {
                        (circ_a, circ_b)
                    };

                    let merging = self
                        .circuits
                        .splice(merge_from..=merge_from, [vec![]])
                        .next()
                        .unwrap();
                    for &box_ix in merging.iter() {
                        self.box_circuits[box_ix] = Some(merge_to)
                    }
                    self.circuits[merge_to].extend(merging);
                }
            }
            (Some(circ_a), None) => {
                self.circuits[circ_a].push(box_b);
                self.box_circuits[box_b] = Some(circ_a);
            }
            (None, Some(circ_b)) => {
                self.circuits[circ_b].push(box_a);
                self.box_circuits[box_a] = Some(circ_b);
            }
            (None, None) => {
                let circ = self.circuits.len();
                self.circuits.push(vec![box_a, box_b]);
                self.box_circuits[box_a] = Some(circ);
                self.box_circuits[box_b] = Some(circ);
            }
        }
    }

    fn circuit_sizes(&self) -> Vec<usize> {
        self.circuits
            .iter()
            .map(|circuit| circuit.len())
            .filter(|&size| size > 0)
            .sorted_by_key(|&size| Reverse(size))
            .collect()
    }

    fn is_fully_merged(&self) -> bool {
        self.circuits
            .first()
            .map(|circuit| circuit.len())
            .unwrap_or_default()
            == self.box_circuits.len()
    }
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
        let mut circuits = Circuits::new(boxes.len());
        let mut connections = make_connections(&boxes);

        for _ in 0..1000 {
            let (box_a, box_b) = connections.next().unwrap();
            circuits.connect(box_a, box_b);
        }

        let circuit_sizes = circuits.circuit_sizes();
        let part1: usize = (0..3).map(|idx| circuit_sizes[idx]).product();

        let mut part2 = None;
        while let Some((box_a, box_b)) = connections.next() {
            circuits.connect(box_a, box_b);
            if circuits.is_fully_merged() {
                part2 = Some((boxes[box_a].0 * boxes[box_b].0).to_string());
                break;
            }
        }

        (Some(part1.to_string()), part2)
    }
}
