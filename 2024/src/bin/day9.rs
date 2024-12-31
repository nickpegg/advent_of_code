use std::num::ParseIntError;

use log::{debug, error};

// A list of block IDs, or None if that block is free space
struct Disk {
    data: Vec<Option<usize>>,
    files: Vec<File>,
}

#[derive(Clone, Debug)]
struct File {
    id: usize,

    // Length of the file in blocks
    len: usize,

    // Index of the start position on disk
    start: usize,

    // Indicies of all the blocks which contain the file
    blocks: Vec<usize>,
}

impl Disk {
    // Expand a string disk map into a list of blocks, which if allocated have Some(id), or are None if
    // they are free space
    fn from_str(s: &str) -> Result<Disk, ParseIntError> {
        let mut data = Vec::new();
        let mut files = Vec::new();

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

            if is_file {
                let mut blocks = Vec::new();
                for _ in 0..len {
                    data.push(Some(cur_file_id));
                    blocks.push(data.len() - 1);
                }
                let f = File {
                    id: cur_file_id,
                    len,
                    start: blocks[0],
                    blocks,
                };
                files.push(f);

                cur_file_id += 1;
            } else {
                for _ in 0..len {
                    data.push(None);
                }
            }

            is_file = !is_file;
        }

        Ok(Disk { data, files })
    }

    // Returns a string based representation of the data layout, which is easier for visualization
    // and testing
    #[cfg(test)]
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
    //
    // NOTE: Because we don't care about maintaining file block order, this will trash the block
    // lists in all the files. I don't care to maintain them because it's not needed for this part
    // of the solution
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
            let fileno = self.data[rear].unwrap();
            let file = &mut self.files[fileno];

            // Move the data block from the back to the front
            self.data[front] = self.data[rear];
            self.data[rear] = None;

            if file.start == rear {
                file.start = front;
                // Trash the file's block list because this defrag screws it up
                file.blocks.clear();
            }

            // Move the pointers to the next spots
            while let Some(_) = self.data[front] {
                front += 1;
            }
            while let None = self.data[rear] {
                rear -= 1;
            }
        }
    }

    // Defragment the data, but keep files contiguous. Again look for free space at the beginning,
    // but look for files starting starting with the highest ID
    fn defrag_files(&mut self) {
        // Go through the files from highest ID to lowest, and through the free spaces from start
        // to end, and move the file to the earliest free space
        self.files.sort_by_key(|f| f.id);
        for mut file in self.files.iter_mut().rev() {
            let mut start = None;
            let mut blocks: Vec<usize> = Vec::new();

            // NOTE: Could optimize by scanning once for free blocks and maintaining a list of free
            // ranges
            for (i, blk) in self.data.iter().enumerate() {
                if *blk == None {
                    if start == None {
                        start = Some(i);
                    }
                    blocks.push(i);

                    // If we found enough space, move the file
                    if blocks.len() == file.len && start.unwrap() < file.start {
                        Disk::move_file(&mut self.data, &mut file, blocks);
                        break;
                    }
                } else {
                    // Found the end of the free space, it didn't fit, so reset and keep looking
                    start = None;
                    blocks = Vec::new();
                }
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

    // Move a file to the specified blocks. The blocks do not need to be contiguous, and do not
    // need to be in order. The first block index will be the start of the file.
    fn move_file(
        disk_blocks: &mut Vec<Option<usize>>,
        file: &mut File,
        block_indicies: Vec<usize>,
    ) {
        debug_assert_eq!(file.len, block_indicies.len());
        debug_assert!(block_indicies.len() > 0);

        debug!(
            "Moving file id:{} from {} to {}",
            file.id, file.start, block_indicies[0]
        );

        file.start = block_indicies[0];
        let old_indicies: Vec<usize> = file.blocks.clone();

        // Move the file's blocks
        debug!("  File's old blocks: {:?}", file.blocks);
        debug!("  File's new blocks: {:?}", block_indicies);
        file.blocks = block_indicies.clone();
        for idx in block_indicies {
            disk_blocks[idx] = Some(file.id);
        }

        // Zero out the old data
        debug!("  Zeroing out {:?}", old_indicies);
        for idx in old_indicies {
            disk_blocks[idx] = None;
        }
    }
}

fn main() {
    env_logger::init();
    let mut disk = Disk::from_str(include_str!("../../data/day9.txt")).unwrap();
    disk.defrag();
    println!("Part 1: {}", disk.checksum());

    disk = Disk::from_str(include_str!("../../data/day9.txt")).unwrap();
    disk.defrag_files();
    println!("Part 2: {}", disk.checksum());
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
    fn test_from_str() {
        init();
        let d = Disk::from_str(TEST_DISK_MAP).unwrap();
        let expected = "00...111...2...333.44.5555.6666.777.888899";
        assert_eq!(d.as_str(), expected);

        // Make sure all the Files are sane
        for file in d.files {
            debug!("Checking file {:#?}", file);

            assert_eq!(file.start, file.blocks[0]);
            assert_eq!(file.blocks.len(), file.len);
            for block in file.blocks {
                assert_eq!(d.data[block], Some(file.id));
            }
        }
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

    #[test]
    fn test_file_defrag() {
        init();
        let mut d = Disk::from_str(TEST_DISK_MAP).unwrap();
        d.defrag_files();
        let expected = "00992111777.44.333....5555.6666.....8888..";
        assert_eq!(d.as_str(), expected);
        assert_eq!(d.checksum(), 2858);
    }
}
