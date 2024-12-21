use std::marker::Copy;

const WORD: [char; 4] = ['X', 'M', 'A', 'S'];

type Direction = (i8, i8);
struct Directions;
impl Directions {
    pub const NORTH: Direction = (0, -1);
    pub const SOUTH: Direction = (0, 1);
    pub const EAST: Direction = (1, 0);
    pub const WEST: Direction = (-1, 0);

    pub const NORTHEAST: Direction = (1, -1);
    pub const SOUTHEAST: Direction = (1, 1);
    pub const SOUTHWEST: Direction = (-1, 1);
    pub const NORTHWEST: Direction = (-1, -1);

    // All directions, useful for iterating over
    pub const ALL: [Direction; 8] = [
        Directions::NORTH,
        Directions::NORTHEAST,
        Directions::EAST,
        Directions::SOUTHEAST,
        Directions::SOUTH,
        Directions::SOUTHWEST,
        Directions::WEST,
        Directions::NORTHWEST,
    ];
}

#[derive(Debug)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct OffGridError;

#[derive(Debug)]
struct GridWalker<T: Copy> {
    grid: Vec<Vec<T>>,
    pos: Position,
}

impl<T: Copy> GridWalker<T> {
    fn new(grid: Vec<Vec<T>>) -> Self {
        Self { grid: grid, pos: Position {x: 0, y: 0} }
    }

    // Move the position to the x/y coordinate
    fn move_pos(&mut self, x: usize, y: usize) {
        self.pos = Position {x, y};
    }

    // Returns the value at the current position
    fn peek(&self) -> T {
        self.grid[self.pos.y][self.pos.x]
    }

    // Look at what's in that direction on the grid, but don't walk there. Returns None if that
    // direction would take us off the grid
    fn peek_direction(&self, dir: &Direction) -> Option<T> {
        let x = self.pos.x.checked_add_signed(dir.0.into())?;
        let y = self.pos.y.checked_add_signed(dir.1.into())?;

        if y >= self.grid.len() || x >= self.grid[y].len() {
            return None;
        }

        Some(self.grid[y][x])
    }

    // Take a step in that direction, and return what's there. Returns an error without taking a
    // step if we would have walked off the grid
    fn step(&mut self, dir: &Direction) -> Result<T, OffGridError> {
        let x = self.pos.x.checked_add_signed(dir.0.into()).ok_or(OffGridError)?;
        let y = self.pos.y.checked_add_signed(dir.1.into()).ok_or(OffGridError)?;

        if y > self.grid.len() || x > self.grid[y].len() {
            return Err(OffGridError);
        }

        self.pos = Position {x, y};
        Ok(self.grid[y][x])
    }
}

fn main() {
    let input = parse_input(include_str!("../../data/day4.txt"));
    part1(input);
}

fn part1(input: Vec<Vec<char>>) -> i32 {
    // 714 - too low
    // 1479 - too low
    let mut walker = GridWalker::new(input);
    let mut count = 0;

    for y in 0..walker.grid.len() {
        for x in 0..walker.grid[y].len() {
            walker.move_pos(x, y);
            println!("Walker at {:?}", walker.pos);

            // TODO remove debug
            println!("{}", walker.peek());
            if walker.peek() == WORD[0] {
                // Found the start of our word
                println!("Found an X");
                for direction in Directions::ALL {
                    println!("Walking {direction:?}");
                    let mut word_pos = 0;

                    while let Some(letter) = walker.peek_direction(&direction) {
                        println!("{}", letter);
                        if letter == WORD[word_pos + 1] {
                            println!("yes..");
                            walker.step(&direction).unwrap();
                            word_pos += 1;
                        } else {
                            println!("Nope");
                            break;
                        }

                        if word_pos == WORD.len() - 1 {
                            println!("YES! Counting it");
                            // Got to the end of the word, add it to our count
                            count += 1;
                            break;
                        }
                    }

                    // Make sure to put the walker back after a walk
                    walker.move_pos(x, y);
                    println!("Walker at {:?}", walker.pos);

                }
            }
        }
    }

    println!("Part 1: {count}");
    count
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|l| l.chars().collect())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    // Make sure part 1 works with the test input
    #[test]
    fn test_part1() {
        let input = parse_input(include_str!("../../data/day4_test.txt"));
        let result = part1(input);
        assert_eq!(result, 18);
    }
}
