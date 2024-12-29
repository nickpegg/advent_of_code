use log::debug;
use std::collections::HashSet;
use std::error::Error;
use std::fmt;

const MAX_HEIGHT: u8 = 9;

const DIRECTIONS: [(i8, i8); 4] = [(1, 0), (0, -1), (-1, 0), (0, 1)];

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
struct Position(usize, usize);

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{},{}", self.0, self.1)
    }
}

struct Map {
    grid: Vec<Vec<u8>>,
    starts: Vec<Position>,
}

impl Map {
    fn from_str(s: &str) -> Result<Self, Box<dyn Error>> {
        let rows = s.lines().count();
        let cols = s.lines().next().unwrap().len();

        let mut grid: Vec<Vec<u8>> = vec![vec![0; cols]; rows];
        let mut starts: Vec<Position> = Vec::new();

        for (y, line) in s.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let val = c.to_string().parse()?;
                grid[x][y] = val;
                if val == 0 {
                    starts.push(Position(x.try_into()?, y.try_into()?));
                }
            }
        }

        Ok(Map { grid, starts })
    }
}

fn main() {
    env_logger::init();
    let map = Map::from_str(include_str!("../../data/day10.txt")).unwrap();
    println!("Part 1: {}", part1(&map));
}

fn part1(map: &Map) -> usize {
    let mut score = 0;
    for start in map.starts.iter() {
        debug!("Starting walk at {start}");
        let mut peaks = HashSet::new();
        walk(map, *start, 0, &mut peaks);

        let walk_score = peaks.len();
        debug!("Got walk score of {walk_score} from {start}");
        score += walk_score;
    }
    score
}

// Try walking from a position in the various directions trying to reach the max height
fn walk(map: &Map, pos: Position, expected: u8, peaks: &mut HashSet<Position>) {
    debug!("Trying {pos}");
    let val = map.grid[pos.0][pos.1];
    if val != expected {
        return; // Not moving upward
    } else if val == MAX_HEIGHT {
        debug!("  Found a peak @ {pos}");
        peaks.insert(pos);
        return; // found a peak
    }
    debug!("  Found {val}");

    for direction in DIRECTIONS {
        let Some(new_x) = pos.0.checked_add_signed(direction.0.into()) else {
            continue;
        };
        let Some(new_y) = pos.1.checked_add_signed(direction.1.into()) else {
            continue;
        };
        if new_x >= map.grid.len() || new_y >= map.grid[0].len() {
            continue;
        }
        let new_pos = Position(new_x, new_y);

        walk(map, new_pos, expected + 1, peaks);
    }
}

#[cfg(test)]
mod day10_tests {
    use super::*;

    const TEST_MAP: &str = include_str!("../../data/day10_test.txt");

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn test_from_str() {
        init();
        let m = Map::from_str(TEST_MAP).unwrap();
        assert_eq!(m.grid[0][0], 8);
        assert_eq!(m.grid[7][7], 2);
    }

    #[test]
    fn test_part1() {
        init();
        let m = Map::from_str(TEST_MAP).unwrap();
        assert_eq!(part1(&m), 36);
    }
}
