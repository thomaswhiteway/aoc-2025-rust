use failure::Error;

fn find_max_joltage_for_bank(bank: &[u8], num_batteries: usize) -> u64 {
    let mut best_idxs: Vec<_> = (0..num_batteries).into_iter().collect();
    for idx in 1..bank.len() {
        for best_idx in 0..num_batteries {
            if best_idxs[best_idx] == idx {
                break;
            }

            let remaining_best = num_batteries - best_idx;
            if bank[idx] > bank[best_idxs[best_idx]] && idx + remaining_best - 1 < bank.len() {
                for offset in 0..num_batteries - best_idx {
                    best_idxs[best_idx + offset] = idx + offset;
                }
                break;
            }
        }
    }

    best_idxs
        .iter()
        .rev()
        .cloned()
        .zip(0..)
        .map(|(idx, pow)| bank[idx] as u64 * 10u64.pow(pow))
        .sum()
}

fn find_max_joltage(banks: &[Box<[u8]>], num_batteries: usize) -> u64 {
    banks
        .iter()
        .map(|bank| find_max_joltage_for_bank(&bank, num_batteries))
        .sum()
}

pub struct Solver {}

impl super::Solver for Solver {
    type Problem = Box<[Box<[u8]>]>;

    fn parse_input(data: String) -> Result<Self::Problem, Error> {
        Ok(data
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).unwrap() as u8)
                    .collect::<Vec<_>>()
                    .into_boxed_slice()
            })
            .collect::<Vec<_>>()
            .into_boxed_slice())
    }

    fn solve(banks: Self::Problem) -> (Option<String>, Option<String>) {
        let part1 = find_max_joltage(&banks, 2);
        let part2 = find_max_joltage(&banks, 12);
        (Some(part1.to_string()), Some(part2.to_string()))
    }
}
