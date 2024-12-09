use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Clone, Copy, Default)]
enum Block {
    #[default]
    Empty,
    File(usize),
}

#[derive(Debug, Clone, Copy, Default)]
struct File {
    start: usize,
    size: usize,
}

#[derive(Debug, Clone)]
struct Disk {
    blocks: Vec<Block>,
    files: Vec<File>,
}

impl From<&str> for Disk {
    fn from(input: &str) -> Self {
        let mut blocks = vec![];
        let mut files = vec![];

        let mut next_index = 0;
        let mut next_is_file = true;

        for c in input.chars() {
            if !c.is_ascii_digit() {
                continue;
            }

            let v = c.to_digit(10).unwrap() as usize;
            if next_is_file {
                files.push(File {
                    start: blocks.len(),
                    size: v,
                });
                for _ in 0..v {
                    blocks.push(Block::File(next_index));
                }
                next_index += 1;
            } else {
                for _ in 0..v {
                    blocks.push(Block::Empty);
                }
            }
            next_is_file = !next_is_file;
        }

        Disk { blocks, files }
    }
}

impl std::fmt::Display for Disk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output = self.blocks.iter().map(|b| match b {
            Block::Empty => '.',
            Block::File(id) => {
                if *id < 10 {
                    std::char::from_digit(*id as u32, 10).unwrap()
                } else if *id < 36 {
                    std::char::from_digit(*id as u32 - 10, 36).unwrap()
                } else {
                    '#'
                }
            }
        });

        write!(f, "{}", output.collect::<String>())
    }
}

impl Disk {
    fn checksum(&self) -> usize {
        self.blocks
            .iter()
            .enumerate()
            .map(|(i, b)| match b {
                Block::Empty => 0,
                Block::File(id) => i * id,
            })
            .sum()
    }
}

#[aoc_generator(day9)]
fn parse(input: &str) -> Disk {
    Disk::from(input)
}

#[aoc(day9, part1, v1)]
fn part1_v1(input: &Disk) -> usize {
    let mut disk = input.clone();
    let mut left_index = 0;
    let mut right_index = disk.blocks.len() - 1;

    while left_index < right_index {
        // Right index should always point at a file node
        match disk.blocks[right_index] {
            Block::Empty => {
                right_index -= 1;
                continue;
            }
            Block::File { .. } => {}
        }

        // If left index is empty, swap the right index into it
        // Otherwise, advance
        match disk.blocks[left_index] {
            Block::Empty => {
                disk.blocks.swap(left_index, right_index);
                left_index += 1;
                right_index -= 1;
            }
            Block::File(id) => {
                left_index += disk.files[id].size;
            }
        }
    }

    disk.checksum()
}

#[aoc(day9, part2, v1)]
fn part2_v1(input: &Disk) -> usize {
    let mut disk = input.clone();

    // We're going to try to move each file from right to left exactly once
    'each_file: for moving_id in (0..disk.files.len()).rev() {
        // TODO: We can probably cache the leftmost empty block to start at
        let mut left_index = 0;
        let mut empty_starts_at = None;

        while left_index < disk.files[moving_id].start {
            match disk.blocks[left_index] {
                Block::File(_) => {
                    left_index += 1;
                    empty_starts_at = None;
                }
                Block::Empty => {
                    if empty_starts_at.is_none() {
                        empty_starts_at = Some(left_index);
                    }

                    // Found a large enough space
                    if empty_starts_at.is_some_and(|empty_starts_at| {
                        left_index - empty_starts_at + 1 >= disk.files[moving_id].size
                    }) {
                        for i in 0..disk.files[moving_id].size {
                            disk.blocks.swap(
                                disk.files[moving_id].start + i,
                                empty_starts_at.unwrap() + i,
                            );
                        }
                        disk.files[moving_id].start = empty_starts_at.unwrap();
                        continue 'each_file;
                    } else {
                        left_index += 1;
                    }
                }
            }
        }
    }

    disk.checksum()
}

// This compresses the disk from left to right, filling empty space
// But this isn't what the problem actually asks for...
// #[aoc(day9, part2, wrong)]
#[allow(dead_code)]
fn part2_wrong(input: &Disk) -> usize {
    let mut disk = input.clone();
    let mut left_index = 0;

    'main_loop: while left_index < disk.blocks.len() {
        match disk.blocks[left_index] {
            // Already have a file, do nothing
            Block::File(id) => {
                left_index += disk.files[id].size;
            }
            // Empty space, try to move the rightmost fitting file
            Block::Empty => {
                // Calculate how much space we have
                let mut new_index = left_index + 1;
                while new_index < disk.blocks.len() {
                    if let Block::File(_) = disk.blocks[new_index] {
                        break;
                    }
                    new_index += 1;
                }
                let space_available = new_index - left_index;

                // Now iterate from the right until we find a file node that can fit there
                // TODO: We should be able to cache the rightmost empty block
                let mut right_index = disk.blocks.len() - 1;
                while right_index > new_index {
                    match disk.blocks[right_index] {
                        Block::Empty => {
                            right_index -= 1;
                        }
                        Block::File(id) => {
                            let size = disk.files[id].size;
                            if size <= space_available {
                                for i in 0..size {
                                    disk.blocks.swap(right_index - i, left_index + i);
                                }
                                left_index += size;
                                continue 'main_loop;
                            } else {
                                right_index -= size;
                            }
                        }
                    }
                }

                // If we made it here, we couldn't move a file
                break;
            }
        }
    }

    disk.checksum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::make_test;

    const EXAMPLE: &str = "2333133121414131402";

    make_test!([part1_v1] => "day9.txt", 1928, "6201130364722");
    make_test!([part2_v1] => "day9.txt", 2858, "6221662795602");
}

// For codspeed
pub fn part1(input: &str) -> String {
    part1_v1(&parse(input)).to_string()
}

pub fn part2(input: &str) -> String {
    part2_v1(&parse(input)).to_string()
}
