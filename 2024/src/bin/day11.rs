use std::fmt;
use std::num::ParseIntError;

fn main() {
    env_logger::init();
    let stones = Stones::try_from(include_str!("../../data/day11.txt")).unwrap();
    println!("Part 1: {}", part1(stones));
}

struct Stones {
    stones: Vec<u64>,
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
    type Item = Vec<u64>;

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
fn part1(mut stones: Stones) -> usize {
    for _ in 0..25 {
        stones.next();
    }
    stones.stones.len()
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
    fn test_part1() {
        init();
        let stones = Stones::try_from(TEST_STONES).unwrap();
        assert_eq!(part1(stones), 55312);
    }
}
