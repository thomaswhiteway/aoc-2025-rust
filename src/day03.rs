
use failure::Error;

fn find_max_joltage_for_bank(bank: &[u8]) -> u64 {
    let mut best = [bank[0], bank[1]];
    for idx in 0..bank.len() {
        if bank[idx] > best[0] && idx + 1 < bank.len() {
            best[0] = bank[idx];
            best[1] = bank[idx+1]
        } else if bank[idx] > best[1] {
            best[1] = bank[idx]
        }
    }

    (best[0] * 10 + best[1]) as u64
}

fn find_max_joltage(banks: &[Box<[u8]>]) -> u64 {
    banks.iter().map(|bank| find_max_joltage_for_bank(&bank)).sum()
}

pub struct Solver {}

impl super::Solver for Solver {
    type Problem = Box<[Box<[u8]>]>;

    fn parse_input(data: String) -> Result<Self::Problem, Error> {
        Ok(data.lines().map(|line| line.chars().map(|c| c.to_digit(10).unwrap() as u8).collect::<Vec<_>>().into_boxed_slice()).collect::<Vec<_>>().into_boxed_slice())
    }

    fn solve(banks: Self::Problem) -> (Option<String>, Option<String>) {
        let part1 = find_max_joltage(&banks);
        (Some(part1.to_string()), None)
    }
}
