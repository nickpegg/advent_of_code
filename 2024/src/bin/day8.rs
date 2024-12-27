use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fmt;
use std::ops;

// use indicatif::ProgressIterator;
use itertools::Itertools;
use log::debug;

type Freq = char;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Point(i64, i64);

impl ops::Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self(self.0 + other.0, self.1 + other.1)
    }
}

impl ops::Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self(self.0 - other.0, self.1 - other.1)
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{},{}", self.0, self.1)?;
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq)]
struct AntennaMap {
    antennas: HashMap<Freq, HashSet<Point>>,
    antinodes: HashSet<Point>,
    bounds: Point,
}
use std::num::TryFromIntError;
impl AntennaMap {
    fn from_str(s: &str) -> Result<Self, TryFromIntError> {
        let mut antennas: HashMap<Freq, HashSet<Point>> = HashMap::new();
        let mut bounds = Point(0, 0);
        for (y, line) in s.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                bounds = Point(x.try_into()?, y.try_into()?);
                match c {
                    '.' => continue,
                    c => antennas
                        .entry(c)
                        .or_insert(HashSet::new())
                        .insert(Point(x.try_into()?, y.try_into()?)),
                };
            }
        }

        Ok(Self {
            antennas,
            antinodes: HashSet::new(),
            bounds,
        })
    }

    // Find all the antinodes from all the antennas and update our antinode set
    fn update_antinodes(&mut self) {
        for (freq, points) in &self.antennas {
            debug!("Finding antinodes for {freq} {points:?}");
            // TODO iterate through all 2-combinations of points
            // Get distance between A and B
            // Antinodes are at (A - dist) and (B + dist) as long as they're within the bounds of
            // the Map
            for (&a, &b) in points.iter().tuple_combinations() {
                debug!("  Comparing {a}, {b}");
                let distance = b - a;
                for point in [a - distance, b + distance] {
                    if point.0 >= 0
                        && point.1 >= 0
                        && point.0 <= self.bounds.0
                        && point.1 <= self.bounds.1
                    {
                        debug!("    Antinode @ {point}");
                        self.antinodes.insert(point);
                    } else {
                        debug!("    Point {point} outside of range {}", self.bounds);
                    }
                }
            }
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();
    let s = include_str!("../../data/day8.txt");
    let input = AntennaMap::from_str(s)?;
    println!("Part 1: {}", part1(input.clone()));
    println!("Part 2: {}", part2(&input));
    Ok(())
}

fn part1(mut map: AntennaMap) -> usize {
    map.update_antinodes();
    map.antinodes.len()
}

fn part2(map: &AntennaMap) -> u32 {
    // TODO
    0
}

#[cfg(test)]
mod day8_tests {
    use super::*;

    fn init() -> AntennaMap {
        let _ = env_logger::builder().is_test(true).try_init();
        AntennaMap::from_str(include_str!("../../data/day8_test.txt")).unwrap()
    }

    #[test]
    fn test_map_from_str() {
        let input = init();
        assert_eq!(
            input.antennas.get(&'0').unwrap(),
            &HashSet::from([Point(8, 1), Point(5, 2), Point(7, 3), Point(4, 4)])
        );
        assert_eq!(
            input.antennas.get(&'A').unwrap(),
            &HashSet::from([Point(8, 8), Point(6, 5), Point(9, 9)])
        );
    }

    #[test]
    fn test_map_antinodes() {
        let mut input = init();
        input.update_antinodes();
        assert!(input.antinodes.len() > 0);
        let mut antinodes: Vec<&Point> = input.antinodes.iter().collect();
        let expected = vec![
            &Point(0, 7),
            &Point(1, 5),
            &Point(2, 3),
            &Point(3, 1),
            &Point(3, 6),
            &Point(4, 2),
            &Point(6, 0),
            &Point(6, 5),
            &Point(7, 7),
            &Point(9, 4),
            &Point(10, 2),
            &Point(10, 10),
            &Point(10, 11),
            &Point(11, 0),
        ];
        antinodes.sort();
        assert_eq!(antinodes, expected);
    }

    #[test]
    fn test_part1() {
        let input = init();
        assert_eq!(part1(input), 14);
    }

    //#[test]
    //fn test_part2() {
    //    init();
    //    let input = parse_input(include_str!("../../data/day8_test.txt")).unwrap();
    //    assert_eq!(part2(&input), 11387);
    //}
}
