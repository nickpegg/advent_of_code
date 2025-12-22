use log::debug;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};

fn main() {
    env_logger::init();
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd)]
struct Point(usize, usize);

impl Point {
    /// Returns true if `other` Point is adjacent to this one. Only checks the 4 cardinal
    /// directions, not diagonals.
    fn is_adjacent(&self, other: &Point) -> bool {
        let x: usize;
        let y: usize;
        if self.0 > other.0 {
            x = self.0 - other.0;
        } else {
            x = other.0 - self.0;
        }
        if self.1 > other.1 {
            y = self.1 - other.1;
        } else {
            y = other.1 - self.1;
        }

        x == 1 && y == 0 || x == 0 && y == 1
    }
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.0 == other.0 && self.1 == other.1 {
            Ordering::Equal
        } else if self.0 < other.0 || self.1 < other.1 {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    }
}

struct Garden {
    grid: Vec<Vec<char>>,
    regions: Vec<Region>,
}

impl From<&str> for Garden {
    fn from(s: &str) -> Self {
        let row_count = s.lines().count();
        let col_count = s.lines().next().unwrap().len();

        let mut grid = vec![vec![' '; col_count]; row_count];
        let mut letter_points: HashMap<char, Vec<Point>> = HashMap::new();

        for (y, line) in s.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                grid[x][y] = c;

                if x == 8 {
                    debug!("Point: {x},{y}, char: {c}");
                }
                // Collect all points for all the letters. We'll figure out which ones belong to
                // which region later
                letter_points
                    .entry(c)
                    .and_modify(|v| v.push(Point(x, y)))
                    .or_insert(vec![Point(x, y)]);
            }
        }

        // Now that we know all the points for all the regions, construct Region structs
        let mut regions = Vec::new();
        for (c, pts) in region_points.iter() {
            let mut points = HashSet::new();
            for point in pts {
                points.insert(point.clone());
            }
            regions.push(Region::new(*c, points, grid.len()));
        }

        Self { grid, regions }
    }
}

#[derive(Debug)]
struct Region {
    character: char,
    points: HashSet<Point>,
    area: usize,
    perimeter: u32,
}

impl Region {
    fn new(character: char, points: HashSet<Point>, grid_size: usize) -> Self {
        let directions: [(i8, i8); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        let mut perimeter = 0;
        for point in &points {
            for direction in directions {
                let Some(x) = point.0.checked_add_signed(direction.0.into()) else {
                    perimeter += 1;
                    continue;
                };
                let Some(y) = point.1.checked_add_signed(direction.1.into()) else {
                    perimeter += 1;
                    continue;
                };

                if !points.contains(&Point(x, y)) {
                    perimeter += 1;
                }
                if x >= grid_size || y >= grid_size {
                    perimeter += 1
                }
            }
        }
        Region {
            character,
            area: points.len(),
            perimeter,
            points,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    mod point {
        use super::*;
        #[test]
        fn test_is_adjacent() {
            init();
            assert!(Point(1, 1).is_adjacent(&Point(1, 2)));
            assert!(!Point(1, 1).is_adjacent(&Point(2, 2)));
            assert!(Point(9, 9).is_adjacent(&Point(8, 9)))
        }
    }
    #[test]
    fn test_from_str() {
        init();
        let g = Garden::from(include_str!("../../data/day12_test.txt"));
        for reg in g.regions {
            if reg.character == 'R' {
                let mut p: Vec<&Point> = reg.points.iter().collect();
                p.sort();
                debug!("R points: {p:?}");
                assert_eq!(reg.area, 12);
                assert_eq!(reg.perimeter, 18);
            }
            if reg.character == 'E' {
                let mut p: Vec<&Point> = reg.points.iter().collect();
                p.sort();
                debug!("E points: {p:?}");
                assert_eq!(reg.area, 13);
                assert_eq!(reg.perimeter, 18);
            }
        }
    }
}
