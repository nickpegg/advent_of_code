use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fmt;
use std::ops;

// use indicatif::ProgressIterator;
use itertools::Itertools;
use log::debug;

type Freq = char;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Point(u32, u32);

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Distance(i64, i64);

impl ops::Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self(self.0 + other.0, self.1 + other.1)
    }
}

impl ops::Add<&Distance> for Point {
    type Output = Option<Point>;

    // Add a distance to a point. Returns None if either X or Y of the point is <0
    fn add(self, other: &Distance) -> Self::Output {
        let x = i64::from(self.0).checked_add(other.0)?;
        let y = i64::from(self.1).checked_add(other.1)?;

        Some(Self(x.try_into().ok()?, y.try_into().ok()?))
    }
}

impl ops::Sub<&Distance> for Point {
    type Output = Option<Point>;

    // Subtract a distance from a point. Returns None if either X or Y of the point is <0
    fn sub(self, dist: &Distance) -> Self::Output {
        let x = i64::from(self.0).checked_sub(dist.0)?;
        let y = i64::from(self.1).checked_sub(dist.1)?;

        Some(Self(x.try_into().ok()?, y.try_into().ok()?))
    }
}

impl ops::Sub for Point {
    type Output = Distance;

    fn sub(self, dist: Self) -> Self::Output {
        let x: i64 = i64::from(self.0) - i64::from(dist.0);
        let y: i64 = i64::from(self.1) - i64::from(dist.1);
        Distance(x, y)
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

        Ok(Self { antennas, bounds })
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();
    let s = include_str!("../../data/day8.txt");
    let input = AntennaMap::from_str(s)?;
    println!("Part 1: {}", part1(&input).len());
    println!("Part 2: {}", part2(&input).len());
    Ok(())
}

fn part1(map: &AntennaMap) -> HashSet<Point> {
    let mut antinodes = HashSet::new();
    for (freq, points) in &map.antennas {
        debug!("Finding antinodes for {freq} {points:?}");
        // iterate through all 2-combinations of points
        // Get distance between A and B
        // Antinodes are at (A - dist) and (B + dist) as long as they're within the bounds of
        // the Map
        for (&a, &b) in points.iter().tuple_combinations() {
            debug!("  Comparing {a}, {b}");
            let distance = b - a;
            for point_option in [a - &distance, b + &distance] {
                let Some(point) = point_option else { continue };
                if point.0 <= map.bounds.0 && point.1 <= map.bounds.1 {
                    debug!("    Antinode @ {point}");
                    antinodes.insert(point);
                } else {
                    debug!("    Point {point} outside of range {}", map.bounds);
                }
            }
        }
    }
    antinodes
}

// Like part 1, but antinodes extend forever away from the two antennas
fn part2(map: &AntennaMap) -> HashSet<Point> {
    let mut antinodes: HashSet<Point> = HashSet::new();
    for (freq, points) in &map.antennas {
        debug!("Finding antinodes for {freq} {points:?}");
        for (&a, &b) in points.iter().tuple_combinations() {
            debug!("  Comparing {a}, {b}");
            let distance = b - a;
            debug!("  Distance is {distance:?}");
            // Walk in each direction off of A and B until we end up off the grid marking antinodes
            // along the way.

            let mut a2 = a.clone();
            while let Some(point) = a2 - &distance {
                a2 = point;
                if point.0 <= map.bounds.0 && point.1 <= map.bounds.1 {
                    debug!("    Antinode @ {point}");
                    antinodes.insert(point);
                } else {
                    debug!("    Point {point} outside of range {}", map.bounds);
                    break;
                }
            }
            let mut a2 = a.clone();
            while let Some(point) = a2 + &distance {
                a2 = point;
                if point.0 <= map.bounds.0 && point.1 <= map.bounds.1 {
                    debug!("    Antinode @ {point}");
                    antinodes.insert(point);
                } else {
                    debug!("    Point {point} outside of range {}", map.bounds);
                    break;
                }
            }

            let mut b2 = b.clone();
            while let Some(point) = b2 - &distance {
                b2 = point;
                if point.0 <= map.bounds.0 && point.1 <= map.bounds.1 {
                    debug!("    Antinode @ {point}");
                    antinodes.insert(point);
                } else {
                    debug!("    Point {point} outside of range {}", map.bounds);
                    break;
                }
            }
            let mut b2 = b.clone();
            while let Some(point) = b2 + &distance {
                b2 = point;
                if point.0 <= map.bounds.0 && point.1 <= map.bounds.1 {
                    debug!("    Antinode @ {point}");
                    antinodes.insert(point);
                } else {
                    debug!("    Point {point} outside of range {}", map.bounds);
                    break;
                }
            }
        }
    }
    antinodes
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
    fn test_part1() {
        let input = init();
        let mut antinodes: Vec<Point> = part1(&input).into_iter().collect();
        antinodes.sort();
        let expected = vec![
            Point(0, 7),
            Point(1, 5),
            Point(2, 3),
            Point(3, 1),
            Point(3, 6),
            Point(4, 2),
            Point(6, 0),
            Point(6, 5),
            Point(7, 7),
            Point(9, 4),
            Point(10, 2),
            Point(10, 10),
            Point(10, 11),
            Point(11, 0),
        ];
        assert_eq!(antinodes, expected);
        assert_eq!(antinodes.len(), 14);
    }

    #[test]
    fn test_part2() {
        init();
        let _ = init();
        let input = AntennaMap::from_str(include_str!("../../data/day8_test.txt")).unwrap();
        let antinodes = part2(&input);

        let mut expected: HashSet<Point> = (0..12).map(|i| Point(i, i)).collect();
        expected = expected
            .union(&HashSet::from([
                Point(1, 0),
                Point(1, 10),
                Point(2, 3),
                Point(2, 8),
                Point(3, 1),
                Point(3, 11),
                Point(4, 2),
                Point(6, 5),
                Point(6, 0),
                Point(9, 4),
                Point(1, 5),
                Point(11, 5),
                Point(3, 6),
                Point(0, 7),
                Point(5, 7),
                Point(4, 9),
                Point(10, 2),
                Point(10, 11),
                Point(11, 0),
                Point(8, 1),
                Point(7, 3),
                Point(5, 2),
            ]))
            .map(|p| *p)
            .collect();
        assert_eq!(expected.len(), 34);
        let extra = &antinodes - &expected;
        let missing = &expected - &antinodes;
        assert_eq!(extra, HashSet::new(), "Have extra antinodes");
        assert_eq!(missing, HashSet::new(), "Missing some antinodes");

        assert_eq!(antinodes.len(), 34);
    }
}
