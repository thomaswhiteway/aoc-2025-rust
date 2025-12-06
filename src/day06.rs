use failure::Error;

#[derive(Clone, Copy, Debug)]
pub enum Operation {
    Add,
    Multiply,
}

impl Operation {
    fn apply(self, nums: impl Iterator<Item = u64>) -> u64 {
        match self {
            Operation::Add => nums.fold(0, |x, y| x + y),
            Operation::Multiply => nums.fold(1, |x, y| x * y),
        }
    }
}

trait NumberParser {
    fn parse_numbers(digits: &Vec<Vec<Option<u8>>>) -> impl Iterator<Item = u64>;
}

struct BasicNumberParser {}

impl NumberParser for BasicNumberParser {
    fn parse_numbers(digits: &Vec<Vec<Option<u8>>>) -> impl Iterator<Item = u64> {
        digits.iter().map(|row| {
            row.iter()
                .rev()
                .filter_map(|d| *d)
                .zip(0..)
                .map(|(d, pos)| d as u64 * 10u64.pow(pos))
                .sum()
        })
    }
}

struct CorrectNumberParser {}

impl NumberParser for CorrectNumberParser {
    fn parse_numbers(digits: &Vec<Vec<Option<u8>>>) -> impl Iterator<Item = u64> {
        (0..digits[0].len()).rev().map(|index| {
            digits
                .iter()
                .rev()
                .map(|row| row[index])
                .filter_map(|d| d)
                .zip(0..)
                .map(|(d, pos)| d as u64 * 10u64.pow(pos))
                .sum()
        })
    }
}

pub struct Problem {
    operation: Operation,
    digits: Vec<Vec<Option<u8>>>,
}

impl Problem {
    fn solve<N: NumberParser>(&self) -> u64 {
        self.operation.apply(N::parse_numbers(&self.digits))
    }
}

fn calculate_grand_total<N: NumberParser>(problems: &[Problem]) -> u64 {
    problems.iter().map(|problem| problem.solve::<N>()).sum()
}

pub struct Solver {}

impl super::Solver for Solver {
    type Problem = Vec<Problem>;

    fn parse_input(data: String) -> Result<Self::Problem, Error> {
        let mut lines: Vec<_> = data
            .lines()
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect();
        let mut operations: Vec<_> = lines
            .pop()
            .unwrap()
            .into_iter()
            .rev()
            .filter_map(|c| match c {
                '+' => Some(Operation::Add),
                '*' => Some(Operation::Multiply),
                _ => None,
            })
            .collect();

        let mut problems = vec![];
        let mut digits = (0..lines.len()).map(|_| vec![]).collect();

        for index in 0..lines[0].len() {
            if lines.iter().map(|line| line[index]).all(|c| c == ' ') {
                problems.push(Problem {
                    operation: operations.pop().unwrap(),
                    digits: digits,
                });
                digits = (0..lines.len()).map(|_| vec![]).collect()
            } else {
                for row in 0..digits.len() {
                    digits[row].push(lines[row][index].to_digit(10).map(|d| d as u8));
                }
            }
        }

        problems.push(Problem {
            operation: operations.pop().unwrap(),
            digits: digits,
        });

        Ok(problems)
    }

    fn solve(problems: Self::Problem) -> (Option<String>, Option<String>) {
        let part1 = calculate_grand_total::<BasicNumberParser>(&problems);
        let part2 = calculate_grand_total::<CorrectNumberParser>(&problems);
        (Some(part1.to_string()), Some(part2.to_string()))
    }
}
