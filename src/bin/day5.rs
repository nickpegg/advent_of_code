use log::{debug, error};
use std::collections::{HashMap, HashSet};
use std::num::ParseIntError;

fn main() {
    env_logger::init();

    let (rules, updates) = parse_input(include_str!("../../data/day5.txt"));
    println!("Part 1 solution: {}", part1(&rules, &updates));
}

fn parse_input(s: &str) -> (HashMap<u32, HashSet<u32>>, Vec<Vec<u32>>) {
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

fn part1(rules: &HashMap<u32, HashSet<u32>>, updates: &Vec<Vec<u32>>) -> u32 {
    // Find all correct updates
    let mut correct_updates = Vec::new();
    for update in updates {
        let mut seen: HashSet<u32> = HashSet::new();
        let mut good = true;

        for num in update {
            seen.insert(*num);
            // Get the set of pages that this page must come before. If this page doesn't exist in
            // the rules, then we're good and can continue
            let Some(others) = rules.get(num) else {
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
            correct_updates.push(update);
        } else {
            debug!("Bad update: {:?}", update);
        }
    }

    // Get the sum of the middle number from each update
    correct_updates
        .into_iter()
        .fold(0, |acc, u| acc + u[u.len() / 2])
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
        let result = part1(&rules, &updates);
        assert_eq!(result, 143);
    }
}
