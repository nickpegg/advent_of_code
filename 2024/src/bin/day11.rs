use indicatif::{ProgressBar, ProgressStyle};
use std::collections::HashMap;
use std::fmt;
use std::num::ParseIntError;

fn main() {
    env_logger::init();
    let stones = Stones::try_from(include_str!("../../data/day11.txt")).unwrap();
    println!("Part 1: {}", part1(stones.clone()));
    println!("Part 2: {}", part2(stones));
}

type Stone = u64;

#[derive(Clone)]
struct Stones {
    stones: Vec<Stone>,
}

impl TryFrom<&str> for Stones {
    type Error = ParseIntError;

    fn try_from(source: &str) -> Result<Self, Self::Error> {
        let mut stones = Vec::new();
        for stone in source.split_whitespace() {
            stones.push(stone.parse()?);
        }
        Ok(Stones { stones })
    }
}

impl fmt::Display for Stones {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            self.stones
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
                .join(" ")
        )
    }
}

impl Iterator for Stones {
    type Item = Vec<Stone>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut new_stones = Vec::new();
        for &stone in self.stones.iter() {
            let stone_digits = stone.to_string();
            if stone == 0 {
                new_stones.push(1);
            } else if stone_digits.len() % 2 == 0 {
                // Replace with two stones
                let (l, r) = stone_digits.split_at(stone_digits.len() / 2);
                new_stones.push(l.parse().unwrap());
                new_stones.push(r.parse().unwrap());
            } else {
                new_stones.push(stone * 2024);
            }
        }
        self.stones = new_stones.clone();
        Some(new_stones)
    }
}

// How many stones do we have after 25 iterations?
fn part1(stones: Stones) -> usize {
    let mut count: usize = 0;
    let bar = ProgressBar::new_spinner().with_style(
        ProgressStyle::with_template("{spinner} {human_pos} [{elapsed_precise}] {per_sec}")
            .unwrap(),
    );
    for stone in stones.stones {
        count += process(stone, 25, &mut HashMap::new(), &bar);
    }
    count
}

// How many stones do we have after 75 iterations?
//
// I originally kept the whole stones list in memory, which is untenable once we get into the
// hundreds of millions of stones to process - it's many gigabytes, even with memory optimization
// (u64 -> u32, holding fewer copies in memory, etc.)
//
// The stones are indepedent from each other, so we can do all 75 iterations on one at a time. We
// can do this recursively one stone at a time. The call stack should only get as deep as our
// number of iterations.
fn part2(stones: Stones) -> usize {
    let mut count: usize = 0;
    let bar = ProgressBar::new_spinner().with_style(
        ProgressStyle::with_template("{spinner} {human_pos} [{elapsed_precise}] {per_sec}")
            .unwrap(),
    );
    for stone in stones.stones {
        count += process(stone, 75, &mut HashMap::new(), &bar);
    }
    count
}

// Recursively process a stone
fn process(
    stone: Stone,
    iterations: u8,
    cache: &mut HashMap<(Stone, u8), usize>,
    bar: &ProgressBar,
) -> usize {
    match cache.get(&(stone, iterations)) {
        Some(&count) => {
            bar.inc(count.try_into().unwrap());
            return count;
        }
        None => {}
    }

    if iterations == 0 {
        bar.inc(1);
        return 1;
    }

    let mut count = 0;
    if stone == 0 {
        count += process(1, iterations - 1, cache, bar);
    } else if count_digits(&stone) % 2 == 0 {
        let (l, r) = split_number(stone);
        count += process(l, iterations - 1, cache, bar);
        count += process(r, iterations - 1, cache, bar);
    } else {
        count += process(stone * 2024, iterations - 1, cache, bar);
    }

    cache.insert((stone, iterations), count);

    count
}

fn count_digits(s: &Stone) -> u32 {
    let mut num_digits = 1;
    let mut n = s / 10;
    while n > 0 {
        num_digits += 1;
        n /= 10;
    }

    num_digits
}

// Split a number by half down the middle of its digits
// e.g. 123456 -> 123, 456
// Will panic if given a number with an odd number of digits
fn split_number(s: Stone) -> (Stone, Stone) {
    let num_digits = count_digits(&s);
    assert!(num_digits % 2 == 0);

    // l is the left side, r is right side
    // We'll basically "pop" off digits from the end by taking l % 10 to get the digit and l / 10
    // to pop it off.
    //
    // We "push left" the popped digits in r by multiplying them by x, which gets multiplied by 10
    // each time.
    let mut l = s.clone();
    let mut r = 0;
    let mut x = 1;
    for _ in 0..(num_digits / 2) {
        r += (l % 10) * x;
        l /= 10;
        x *= 10;
    }

    (l, r)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_STONES: &str = "125 17";

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn test_parse() {
        init();
        let stones = Stones::try_from(TEST_STONES).unwrap();
        assert_eq!(stones.stones, vec![125, 17]);
    }

    #[test]
    fn test_step() {
        init();
        let mut stones = Stones::try_from(TEST_STONES).unwrap();
        assert_eq!(stones.stones, vec![125, 17]);
        assert_eq!(stones.next(), Some(vec![253000, 1, 7]));
        stones.next();
        assert_eq!(stones.to_string(), "253 0 2024 14168");
        stones.next();
        assert_eq!(stones.to_string(), "512072 1 20 24 28676032");
    }

    #[test]
    fn test_num_digits() {
        init();
        assert_eq!(count_digits(&12345), 5);
        assert_eq!(count_digits(&12340), 5);
        assert_eq!(count_digits(&1234), 4);
    }

    #[test]
    fn test_split_number() {
        init();
        assert_eq!(split_number(12), (1, 2));
        assert_eq!(split_number(1234), (12, 34));
        assert_eq!(split_number(123456), (123, 456));
    }

    #[test]
    fn test_recursive_method_real() {
        init();
        let stones = Stones::try_from(include_str!("../../data/day11.txt")).unwrap();
        let mut count = 0;
        for stone in stones.stones {
            count += process(stone, 25, &mut HashMap::new(), &ProgressBar::hidden());
        }
        assert_eq!(count, 191690);
    }

    #[test]
    fn test_process_with_cache() {
        init();
        let stones = Stones::try_from(TEST_STONES).unwrap();
        let mut count = 0;
        for stone in stones.stones {
            count += process(stone, 25, &mut HashMap::new(), &ProgressBar::hidden());
        }
        assert_eq!(count, 55312);
    }
}
