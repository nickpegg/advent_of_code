use std::error::Error;
use std::num::ParseIntError;

fn main() {
    part1().unwrap();
    part2().unwrap();
}

fn part1() -> Result<(), Box<dyn Error>> {
    let safe_count = parse_input()?.into_iter().filter(|r| is_safe(r)).count();
    println!("Part 1 solution: {}", safe_count);
    Ok(())
}

fn part2() -> Result<(), Box<dyn Error>> {
    let mut safe_count = 0;
    for report in parse_input()? {
        if is_safe(&report) {
            safe_count += 1;
        } else {
            // Try removing individual levels to see if the report is safe then
            for idx in 0..report.len() {
                let mut possible = report.clone();
                possible.remove(idx);
                if is_safe(&possible) {
                    safe_count += 1;
                    break;
                }
            }
        }
    }

    println!("Part 2 solution: {}", safe_count);
    Ok(())
}

// Returns whether or not a report is safe
fn is_safe(report: &Vec<i32>) -> bool {
    let mut ascending = false;
    if report[0] < report[1] {
        ascending = true;
    }

    let mut last = report[0];
    for level in &report[1..] {
        let diff = level - last;
        if (diff < 0 && ascending) || (diff > 0 && !ascending) {
            return false;
        } else if diff.abs() < 1 || diff.abs() > 3 {
            return false;
        }
        last = *level;
    }

    true
}

fn parse_input() -> Result<Vec<Vec<i32>>, ParseIntError> {
    let mut reports: Vec<Vec<i32>> = Vec::new();
    let input = include_str!("../../data/day2.txt");
    for line in input.lines() {
        let report: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse())
            .collect::<Result<Vec<i32>, ParseIntError>>()?;
        reports.push(report);
    }
    Ok(reports)
}
