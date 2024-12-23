use std::collections::HashMap;
use std::num::ParseIntError;

fn main() {
    part1().unwrap();
    part2().unwrap();
}

fn part1() -> Result<(), ParseIntError> {
    let (left, right) = get_lists()?;

    let mut acc = 0;
    for (l, r) in left.iter().zip(right.iter()) {
        acc += l.abs_diff(*r);
    }

    println!("Part 1 solution: {}", acc);

    Ok(())
}

fn part2() -> Result<(), ParseIntError> {
    let (left, right) = get_lists()?;

    // Count how many times each item occurs in the right list
    let mut right_counts: HashMap<i32, i32> = HashMap::new();
    for r in right {
        right_counts.entry(r).and_modify(|i| *i += 1).or_insert(1);
    }

    let mut score = 0;
    for l in left {
        score += l * *(right_counts.entry(l).or_default());
    }
    println!("Part 2 solution: {}", score);

    Ok(())
}

fn get_lists() -> Result<(Vec<i32>, Vec<i32>), ParseIntError> {
    let input = include_str!("../../data/day1.txt");
    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();

    for line in input.lines() {
        let parts: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse())
            .collect::<Result<Vec<i32>, ParseIntError>>()?;

        if parts.len() != 2 {
            println!("Invalid line: {}", line);
            break;
        }

        left.push(parts[0]);
        right.push(parts[1]);
    }
    left.sort();
    right.sort();

    Ok((left, right))
}
