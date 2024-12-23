use log::debug;
use std::collections::HashSet;
use std::fmt;

fn main() {
    env_logger::init();
    let s = include_str!("../../data/day6.txt");
    let w = Walker::from_str(s).unwrap();
    println!("Part 1: {}", part1(w));
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Dir {
    North,
    East,
    South,
    West,
}
const DIRS_CLOCKWISE: [Dir; 4] = [Dir::North, Dir::East, Dir::South, Dir::West];
const DIRS_COUNTER_CLOCKWISE: [Dir; 4] = [Dir::North, Dir::West, Dir::South, Dir::East];

#[derive(Debug)]
enum Turn {
    Clockwise,
    CounterClockwise,
}

#[derive(Debug)]
struct Walker {
    // two-dimensional grid, obstructions are a true value
    grid: Vec<Vec<bool>>,
    pos: (usize, usize),
    dir: Dir,
}

impl Walker {
    // Parse an input string
    fn from_str(s: &str) -> Result<Self, ParseError> {
        let row_count = s.lines().count();
        let col_count = s.lines().next().unwrap().len();

        let mut grid = vec![vec![false; col_count]; row_count];
        let mut pos = (0, 0);

        for (y, line) in s.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                match c {
                    '.' => grid[x][y] = false,
                    '#' => grid[x][y] = true,
                    '^' => {
                        grid[x][y] = false;
                        pos = (x, y);
                    }
                    chr => return Err(ParseError { c: chr }),
                }
            }
        }

        Ok(Walker {
            grid,
            pos,
            dir: Dir::North,
        })
    }

    // Turn 90 degrees in the given direction
    fn turn(&mut self, t: Turn) {
        let dirs: [Dir; 4];
        match t {
            Turn::Clockwise => dirs = DIRS_CLOCKWISE,
            Turn::CounterClockwise => dirs = DIRS_COUNTER_CLOCKWISE,
        }
        debug!("Turning {:?}", t);

        self.dir = dirs
            .iter()
            .cycle()
            .skip_while(|d| **d != self.dir)
            .skip(1)
            .next()
            .unwrap()
            .clone();
        debug!("Walker turned {:?}", self.dir);
    }

    // Take a step in the direction we're facing, and return the new position
    fn step(&mut self) -> StepResult {
        let (offset_x, offset_y): (i8, i8) = match self.dir {
            Dir::North => (0, -1),
            Dir::East => (1, 0),
            Dir::South => (0, 1),
            Dir::West => (-1, 0),
        };
        let Some(x) = self.pos.0.checked_add_signed(offset_x.into()) else {
            return StepResult::OffGrid;
        };
        let Some(y) = self.pos.1.checked_add_signed(offset_y.into()) else {
            return StepResult::OffGrid;
        };

        if y >= self.grid.len() || x >= self.grid[y].len() {
            return StepResult::OffGrid;
        }

        if self.grid[x][y] == true {
            return StepResult::Obstructed;
        }

        self.pos.0 = x;
        self.pos.1 = y;

        StepResult::Stepped
    }
}

impl fmt::Display for Walker {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (x, y) = self.pos;
        write!(f, "Walker @ {x},{y}")?;
        Ok(())
    }
}

// The result of a step
enum StepResult {
    // Success, walker's new position
    Stepped,
    Obstructed,
    OffGrid,
}

#[derive(Debug)]
struct ParseError {
    c: char,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Unable to parse input, invalid character: {}", self.c)?;
        Ok(())
    }
}

// Count up how many distinct positions the walker visits on their walk
fn part1(mut walker: Walker) -> usize {
    let mut steps: HashSet<(usize, usize)> = HashSet::new();

    loop {
        match walker.step() {
            StepResult::Stepped => {
                debug!("{walker}");
                steps.insert(walker.pos);
            }
            StepResult::Obstructed => walker.turn(Turn::Clockwise),
            StepResult::OffGrid => break,
        }
    }

    steps.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    mod walker {
        use super::*;

        #[test]
        fn parse() {
            init();
            let walker = Walker::from_str(include_str!("../../data/day6_test.txt")).unwrap();
            assert_eq!(walker.pos, (4, 6));
            // Check for a couple of obstructions
            assert_eq!(walker.grid[0][0], false);
            assert_eq!(walker.grid[3][0], false);
            assert_eq!(walker.grid[4][0], true);
            assert_eq!(walker.grid[2][3], true);
        }

        #[test]
        fn turn() {
            init();
            let mut walker = Walker::from_str(include_str!("../../data/day6_test.txt")).unwrap();
            // Should start facing north
            assert_eq!(walker.dir, Dir::North);
            // Turn clockwise should be facing East
            walker.turn(Turn::Clockwise);
            assert_eq!(walker.dir, Dir::East);
            walker.turn(Turn::Clockwise);
            assert_eq!(walker.dir, Dir::South);
            // Try the other way
            walker.turn(Turn::CounterClockwise);
            assert_eq!(walker.dir, Dir::East);
        }

        #[test]
        fn step() {
            init();
            let mut walker = Walker::from_str(include_str!("../../data/day6_test.txt")).unwrap();
            assert_eq!(walker.pos, (4, 6));
            assert_eq!(walker.dir, Dir::North);

            match walker.step() {
                StepResult::Stepped => assert_eq!(walker.pos, (4, 5)),
                _ => panic!("DOH"),
            }

            // Try walking until we hit an obstruction
            while let StepResult::Stepped = walker.step() {}
            assert_eq!(walker.pos, (4, 1));

            walker.turn(Turn::Clockwise);
            while let StepResult::Stepped = walker.step() {}
            assert_eq!(walker.pos, (8, 1));
        }
    }

    #[test]
    fn test_part1() {
        init();
        let walker = Walker::from_str(include_str!("../../data/day6_test.txt")).unwrap();
        let result = part1(walker);
        assert_eq!(result, 41);
    }

    // #[test]
    // fn test_part2() {
    //     init();
    //     let (rules, updates) = parse_input(include_str!("../../data/day5_test.txt"));
    //     assert!(rules.len() > 0);
    //     assert!(updates.len() > 0);
    //     let result = part2(&rules, updates);
    //     assert_eq!(result, 123);
    // }
}
