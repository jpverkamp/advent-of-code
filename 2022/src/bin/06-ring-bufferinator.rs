use std::{path::Path, collections::HashSet};
use aoc::*;

#[derive(Debug)]
struct CharRingBuffer {
    size: usize,
    head: usize,
    count: usize,
    data: Vec<char>,
}

impl CharRingBuffer {
    pub fn new(size: usize) -> Self {
        let mut data = Vec::new();
        for _ in 0..size {
            data.push('\0');
        }

        CharRingBuffer{ size, head: 0, count: 0, data }
    }

    pub fn push(&mut self, c: char) {
        self.data[self.head] = c;
        self.head = (self.head + 1) % self.size;

        if self.count < self.size {
            self.count += 1
        }
    }

    pub fn len(&self) -> usize {
        self.count
    }
}

impl<'a> CharRingBuffer {
    pub fn iter(&'a self) -> impl Iterator<Item = &'a char> {
        self.data.iter()
            .chain(self.data.iter())
            .skip(self.head)
            .take(self.count)
    }
}

fn first_duplicate_at(line: &String, size: usize) -> Option<usize> {
    let mut crb = CharRingBuffer::new(size);
    
    for (i, c) in line.chars().enumerate() {
        crb.push(c);
        if crb.len() < size {
            continue;
        }

        let mut s = HashSet::new();
        for c in crb.iter() {
            s.insert(c);
        }

        if s.len() == size {
            return Some(i + 1);
        }
    }

    None
}

fn part1(filename: &Path) -> String {
    let mut result = String::new();

    for line in read_lines(filename) {
        let index = first_duplicate_at(&line, 4).expect("must have a duplicate").to_string();
        result.push_str(&index);
        result.push('\n');
    }
    
    String::from(result.strip_suffix('\n').expect("must return at least one value"))
}

fn part2(filename: &Path) -> String {
    let mut result = String::new();

    for line in read_lines(filename) {
        let index = first_duplicate_at(&line, 14).expect("must have a duplicate").to_string();
        result.push_str(&index);
        result.push('\n');
    }
    
    String::from(result.strip_suffix('\n').expect("must return at least one value"))
}

fn main() {
    aoc_main(part1, part2);
}

#[cfg(test)]
mod tests {
    use aoc::aoc_test;
    use crate::{part1, part2};

    #[test]   
    fn test1() { aoc_test("06", part1, "1760") }

    #[test]
    fn test2() { aoc_test("06", part2, "2974") }
}
