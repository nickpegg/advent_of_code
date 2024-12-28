use std::num::ParseIntError;

use log::{debug, error};

// A list of block IDs, or None if that block is free space
struct Disk {
    data: Vec<Option<usize>>,
}

impl Disk {
    // Expand a string disk map into a list of blocks, which if allocated have Some(id), or are None if
    // they are free space
    fn from_str(s: &str) -> Result<Disk, ParseIntError> {
        let mut data = Vec::new();

        let mut cur_file_id = 0;
        let mut is_file = true;
        for (idx, c) in s.chars().enumerate() {
            if c.is_whitespace() {
                continue;
            }
            let len = match c.to_string().parse() {
                Ok(l) => l,
                Err(e) => {
                    error!("Unable to parse char into number at index {idx}: '{c}'");
                    return Err(e);
                }
            };
            debug!("len: {len}");
            if is_file {
                for _ in 0..len {
                    data.push(Some(cur_file_id));
                }
                cur_file_id += 1;
            } else {
                for _ in 0..len {
                    data.push(None);
                }
            }

            is_file = !is_file;
        }

        Ok(Disk { data })
    }

    // Returns a string based representation of the data layout, which is easier for visualization
    // and testing
    fn as_str(&self) -> String {
        let mut s = String::new();

        for block in self.data.iter() {
            match block {
                Some(id) => s.push(id.to_string().chars().next().unwrap()),
                None => s.push('.'),
            }
        }

        s
    }

    // Defragment the data by moving all blocks from the end to the first available free space
    fn defrag(&mut self) {
        // Two pointers:
        // - One walks from the front and sits at free space ready to accept a block
        // - One walks from the rear and pulls data blocks
        let mut front = 0;
        let mut rear = self.data.len() - 1;

        // Move the front pointer to the next available free space
        while let Some(_) = self.data[front] {
            front += 1;
        }
        // Move the rear pointer to the next data block
        while let None = self.data[rear] {
            rear -= 1;
        }
        while front < rear {
            debug!("f:{front} r:{rear}");
            // Move the data block from the back to the front
            self.data[front] = self.data[rear];
            self.data[rear] = None;

            // Move the pointers to the next spots
            while let Some(_) = self.data[front] {
                front += 1;
            }
            while let None = self.data[rear] {
                rear -= 1;
            }
        }
    }

    // Calculate the checksum, which is the sum of each block position multiplied by its ID
    fn checksum(&self) -> usize {
        let mut sum = 0;
        for (idx, block) in self.data.iter().enumerate() {
            match block {
                Some(id) => sum += idx * id,
                None => {}
            }
        }

        sum
    }
}

fn main() {
    env_logger::init();
    let mut disk = Disk::from_str(include_str!("../../data/day9.txt")).unwrap();
    disk.defrag();
    println!("Part 1: {}", disk.checksum());
}

#[cfg(test)]
mod day9_tests {
    use super::*;

    const TEST_DISK_MAP: &str = "2333133121414131402";

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    // Test expanding a disk map into a block layout
    #[test]
    fn test_expand() {
        init();
        let d = Disk::from_str(TEST_DISK_MAP).unwrap();
        let expected = "00...111...2...333.44.5555.6666.777.888899";
        assert_eq!(d.as_str(), expected);
    }

    #[test]
    fn test_defrag() {
        init();
        let mut d = Disk::from_str(TEST_DISK_MAP).unwrap();
        let expected = "00...111...2...333.44.5555.6666.777.888899";
        assert_eq!(d.as_str(), expected);

        d.defrag();
        let expected = "0099811188827773336446555566..............";
        assert_eq!(d.as_str(), expected);
    }

    #[test]
    fn test_checksum() {
        init();
        let mut d = Disk::from_str(TEST_DISK_MAP).unwrap();
        d.defrag();
        assert_eq!(d.checksum(), 1928);
    }
}
