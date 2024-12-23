use log::{debug, error};
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::num::ParseIntError;

fn main() {
    env_logger::init();

    let (rules, updates) = parse_input(include_str!("../../data/day5.txt"));
    println!("Part 1 solution: {}", part1(&rules, updates.clone()));
    println!("Part 2 solution: {}", part2(&rules, updates));
}

// Map of page ordering rules. The keys are page numbers which must come before the numbers
// contained in the values (set of numbers)
type Rules = HashMap<u32, HashSet<u32>>;

type Updates = Vec<Vec<u32>>;

fn parse_input(s: &str) -> (Rules, Updates) {
    let mut rules = HashMap::new();
    let mut updates = Vec::new();

    for line in s.lines() {
        if line.contains("|") {
            // It's a page ordering rule
            let parts = line
                .split("|")
                .map(|n| n.parse())
                .collect::<Result<Vec<u32>, ParseIntError>>()
                .unwrap();

            if parts.len() == 2 {
                let pages = rules.entry(parts[0]).or_insert(HashSet::new());
                pages.insert(parts[1]);
            } else {
                error!("Invalid rule: {line}");
            }
        } else if line.contains(",") {
            // It's an update, which is a comma-separated list of ints
            updates.push(
                line.split(",")
                    .map(|n| n.parse())
                    .collect::<Result<Vec<u32>, ParseIntError>>()
                    .unwrap(),
            );
        }
    }

    (rules, updates)
}

fn part1(rules: &Rules, updates: Updates) -> u32 {
    // Find all correct updates
    let (correct_updates, _) = bucket_updates(&rules, updates);

    // Get the sum of the middle number from each update
    correct_updates
        .into_iter()
        .fold(0, |acc, u| acc + u[u.len() / 2])
}

fn part2(rules: &Rules, updates: Updates) -> u32 {
    let (_, mut incorrect) = bucket_updates(&rules, updates);

    for update in &mut incorrect {
        // Fix the incorrect updates by sorting them according to the rules.
        // Rules in text are like "a|b" meaning A must come before B, and we store these rules in a
        // Map of Sets, where the values of the sets are all the pages the key must come before.
        update.sort_by(|a, b| match rules.get(&a) {
            Some(r) => match r.contains(&b) {
                true => Ordering::Less,
                false => Ordering::Equal,
            },
            None => Ordering::Equal,
        });
    }

    incorrect.into_iter().fold(0, |acc, u| acc + u[u.len() / 2])
}

// Bucket updates into two lists: correct ones an incorrect ones
fn bucket_updates(rules: &Rules, updates: Updates) -> (Updates, Updates) {
    let mut correct: Updates = Vec::new();
    let mut incorrect: Updates = Vec::new();

    for update in updates {
        let mut seen: HashSet<u32> = HashSet::new();
        let mut good = true;

        for num in &update {
            seen.insert(num.clone());
            // Get the set of pages that this page must come before. If this page doesn't exist in
            // the rules, then we're good and can continue
            let Some(others) = rules.get(&num) else {
                continue;
            };

            // If the sets seen and others have any pages in common, then we've seen some page that
            // we shouldn't have so far, so this update is invalid
            if !seen.is_disjoint(others) {
                good = false;
                break;
            }
        }
        if good {
            debug!("Good update: {:?}", update);
            correct.push(update);
        } else {
            debug!("Bad update: {:?}", update);
            incorrect.push(update);
        }
    }

    (correct, incorrect)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn test_part1() {
        init();
        let (rules, updates) = parse_input(include_str!("../../data/day5_test.txt"));
        assert!(rules.len() > 0);
        assert!(updates.len() > 0);
        let result = part1(&rules, updates);
        assert_eq!(result, 143);
    }

    #[test]
    fn test_part2() {
        init();
        let (rules, updates) = parse_input(include_str!("../../data/day5_test.txt"));
        assert!(rules.len() > 0);
        assert!(updates.len() > 0);
        let result = part2(&rules, updates);
        assert_eq!(result, 123);
    }
}
