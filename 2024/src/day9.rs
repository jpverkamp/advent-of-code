use std::collections::BTreeMap;

use aoc_runner_derive::aoc;

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

// #[aoc_generator(day9)]
// fn parse(input: &str) -> Disk {
//     Disk::from(input)
// }

#[aoc(day9, part1, v1)]
fn part1_v1(input: &str) -> usize {
    let mut disk = Disk::from(input);
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
fn part2_v1(input: &str) -> usize {
    let mut disk = Disk::from(input);

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

#[derive(Debug, Clone, Copy)]
enum BTreeBlock {
    Empty { size: usize },
    File { id: usize, size: usize },
}

#[derive(Debug, Clone)]
struct BTreeDisk {
    blocks: BTreeMap<usize, BTreeBlock>,
}

impl From<&str> for BTreeDisk {
    fn from(input: &str) -> Self {
        let mut data = BTreeMap::new();

        let mut next_index = 0;
        let mut next_is_file = true;
        let mut next_file_id = 0;

        for c in input.chars() {
            if c.is_ascii_digit() {
                let size = c.to_digit(10).unwrap() as usize;
                if next_is_file {
                    data.insert(
                        next_index,
                        BTreeBlock::File {
                            id: next_file_id,
                            size,
                        },
                    );
                    next_file_id += 1;
                } else {
                    data.insert(next_index, BTreeBlock::Empty { size });
                }

                next_index += size;
                next_is_file = !next_is_file;
            }
        }

        BTreeDisk { blocks: data }
    }
}

impl std::fmt::Display for BTreeDisk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output = self.blocks.iter().flat_map(|(_, &block)| match block {
            BTreeBlock::Empty { size } => std::iter::repeat('.').take(size),
            BTreeBlock::File { id, size } => {
                if id < 10 {
                    std::iter::repeat(std::char::from_digit(id as u32, 10).unwrap()).take(size)
                } else if id < 36 {
                    std::iter::repeat(std::char::from_digit(id as u32 - 10, 36).unwrap()).take(size)
                } else {
                    std::iter::repeat('#').take(size)
                }
            }
        });

        write!(f, "{}", output.collect::<String>())
    }
}

impl BTreeDisk {
    fn checksum(&self) -> usize {
        self.blocks
            .iter()
            .map(|(&index, &b)| match b {
                BTreeBlock::Empty { .. } => 0,
                BTreeBlock::File { id, size } => {
                    // TODO: We should be able to calculate this directly
                    // id * (2 * index + size) * (size - 1) / 2
                    ((index)..(index + size)).map(|i| i * id).sum::<usize>()
                }
            })
            .sum()
    }
}

#[aoc(day9, part2, btree)]
fn part2_btree(input: &str) -> usize {
    let mut disk = BTreeDisk::from(input);

    // Collect the starting start index of each file
    let files = disk
        .blocks
        .iter()
        .filter_map(|(i, block)| match block {
            BTreeBlock::File { size, .. } => Some((*i, *size)),
            _ => None,
        })
        .collect::<Vec<_>>();

    // Try to move each file exactly once, from right to left
    for (_, &(file_start, file_size)) in files.iter().enumerate().rev() {
        // Find the first empty space we can that will fit it
        let empty_index = disk.blocks.iter().find(|(_, block)| match block {
            BTreeBlock::Empty { size } => size >= &file_size,
            _ => false,
        });

        // No blocks that fit it
        if empty_index.is_none() {
            continue;
        }
        let (&empty_index, _) = empty_index.unwrap();

        // Only move left
        if empty_index >= file_start {
            continue;
        }

        let removed_empty_node = disk.blocks.remove(&empty_index).unwrap();
        let removed_file_node = disk.blocks.remove(&file_start).unwrap();

        disk.blocks.insert(empty_index, removed_file_node);
        disk.blocks
            .insert(file_start, BTreeBlock::Empty { size: file_size });

        // If we have extra empty space, insert a new empty node
        match (removed_empty_node, removed_file_node) {
            (
                BTreeBlock::Empty { size: empty_size },
                BTreeBlock::File {
                    id: _,
                    size: file_size,
                },
            ) if empty_size > file_size => {
                disk.blocks.insert(
                    empty_index + file_size,
                    BTreeBlock::Empty {
                        size: empty_size - file_size,
                    },
                );
            }
            _ => {}
        }

        // While we have two neighboring empty nodes, combine them
        loop {
            // There are empty consecutive blocks at a and b
            let maybe_empties = disk.blocks.iter().zip(disk.blocks.iter().skip(1)).find_map(
                |((index, block), (next_index, next_block))| match (block, next_block) {
                    (BTreeBlock::Empty { size: size1 }, BTreeBlock::Empty { size: size2 }) => {
                        Some((*index, *next_index, size1 + size2))
                    }
                    _ => None,
                },
            );

            if let Some((a, b, size)) = maybe_empties {
                disk.blocks.remove(&a);
                disk.blocks.remove(&b);
                disk.blocks.insert(a, BTreeBlock::Empty { size });
            } else {
                // No more consecutive empty blocks
                break;
            }
        }
    }

    disk.checksum()
}

// This compresses the disk from left to right, filling empty space
// But this isn't what the problem actually asks for...
// #[aoc(day9, part2, wrong)]
#[allow(dead_code)]
fn part2_wrong(input: &str) -> usize {
    let mut disk = Disk::from(input);
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
    make_test!([part2_v1, part2_btree] => "day9.txt", 2858, "6221662795602");
}

// For codspeed
fn parse(input: &str) -> &str {
    input
}

pub fn part1(input: &str) -> String {
    part1_v1(parse(input)).to_string()
}

pub fn part2(input: &str) -> String {
    part2_v1(parse(input)).to_string()
}
