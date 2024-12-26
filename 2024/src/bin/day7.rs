use std::error::Error;
use std::fmt;

use itertools::Itertools;
use log::debug;

fn main() {
    env_logger::init();
    let s = include_str!("../../data/day7.txt");
    let input = parse_input(s).unwrap();
    // println!("Part 2: {}", part2());
    println!("Part 1: {:?}", part1(&input));
}

type ProblemInput = Vec<(u64, Vec<u64>)>;

#[derive(Debug)]
struct ParseError {
    line: String,
    lineno: usize,
}
impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "Invalid line ({}): {}", self.lineno, self.line)?;
        Ok(())
    }
}
impl Error for ParseError {}

#[derive(Clone, Debug)]
enum Oper {
    Add,
    Mul,
}

impl fmt::Display for Oper {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            Oper::Add => write!(f, "+")?,
            Oper::Mul => write!(f, "*")?,
        };
        Ok(())
    }
}

fn parse_input(s: &str) -> Result<ProblemInput, Box<dyn Error>> {
    let mut input: ProblemInput = Vec::new();
    for (lineno, line) in s.lines().enumerate() {
        let parts = line.split(":").collect::<Vec<&str>>();
        if parts.len() != 2 {
            return Err(Box::new(ParseError {
                line: line.to_string(),
                lineno,
            }));
        }

        let value: u64 = match parts[0].parse() {
            Ok(v) => v,
            Err(_) => {
                return Err(Box::new(ParseError {
                    line: line.to_string(),
                    lineno,
                }))
            }
        };
        let mut numbers: Vec<u64> = Vec::new();

        for part in parts[1].split_whitespace() {
            numbers.push(part.parse()?);
        }

        input.push((value, numbers));
    }
    Ok(input)
}

fn perform(oper: &Oper, a: &u64, b: &u64) -> u64 {
    match oper {
        Oper::Add => a + b,
        Oper::Mul => a * b,
    }
}

fn part1(input: &ProblemInput) -> u64 {
    let mut sum = 0;
    for (value, orig_numbers) in input {
        // Generate the list of all possible operations, which should be 1 shorter than our number
        // list
        let possible_operations: Vec<Vec<Oper>> = (0..(orig_numbers.len() - 1))
            .map(|_| [Oper::Add, Oper::Mul])
            .multi_cartesian_product()
            .collect();
        debug!(
            "Possible operations for {:?}: {:?}",
            orig_numbers, possible_operations
        );

        for orig_operations in possible_operations {
            // Make a new reversed set of numbers, so that we can pop() from the end but still read the
            // numbers from left-to-right
            let mut numbers: Vec<u64> = Vec::new();
            let mut n = orig_numbers.clone();
            while let Some(n) = n.pop() {
                numbers.push(n);
            }

            let mut operations = orig_operations.clone();
            // We need the operations to be in the same order as the numbers
            operations.reverse();

            while numbers.len() > 0 && operations.len() > 0 {
                let a = numbers.pop().unwrap();
                let b = numbers.pop().unwrap();
                let oper = operations.pop().unwrap();
                let r = perform(&oper, &a, &b);
                debug!("{a} {oper} {b} = {r}");
                numbers.push(r);
                debug!("Numbers: {numbers:?}");
            }

            if &numbers[0] == value {
                debug!(
                    "This works: {value} = {:?}x{:?}",
                    orig_numbers, orig_operations
                );
                sum += numbers[0];
                // We only need to know if one of the equations works, so stop looking
                break;
            }
        }
    }
    sum
}

#[cfg(test)]
mod day7_tests {
    use super::*;

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn test_part1() {
        init();
        let input = parse_input(include_str!("../../data/day7_test.txt")).unwrap();
        assert_eq!(part1(&input), 3749);
    }
}
