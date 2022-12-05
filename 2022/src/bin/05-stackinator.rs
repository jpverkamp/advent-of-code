use std::{path::Path, collections::LinkedList};
use aoc::*;

#[derive(Debug)]
struct Stack {
    data: Vec<char>,
}

#[derive(Debug)]
struct Warehouse {
    stacks: Vec<Stack>,
}

impl Warehouse {
    fn from(lines: &Vec<String>) -> Warehouse {
        let mut data = Vec::new();

        for line in lines {
            for i in 0..=(line.len() / 3) {
                let center = 1 + i * 4;
                let c = line.chars().nth(center);

                match c {
                    Some(c) if c != ' ' => {
                        while data.len() <= i {
                            data.push( Vec::new() );
                        }

                        data[i].push(c);
                    }
                    _ => {} // No box in this stack, do nothing
                }
            }
        }

        let mut stacks = Vec::new();
        for ls in data {
            stacks.push(Stack { data: ls.into_iter().rev().collect::<Vec<char>>() });
        }

        Warehouse { stacks }
    }

    fn apply(&mut self, instruction: &Instruction) {
        for _ in 0..instruction.qty {
            let value = self.stacks[instruction.src - 1].data.pop().expect("tried to pop from empty stack");
            self.stacks[instruction.dst - 1].data.push(value);
        }
    }

    fn apply_9001(&mut self, instruction: &Instruction) {
        let mut values = LinkedList::new();

        for _ in 0..instruction.qty {
            values.push_back(self.stacks[instruction.src - 1].data.pop().expect("tried to pop from empty stack"));
        }

        for _ in 0..instruction.qty {
            self.stacks[instruction.dst - 1].data.push(values.pop_back().expect("must pop as many as we pushed"));
        }
    }

    fn tops(&self) -> String {
        let mut result = String::new();

        for stack in self.stacks.iter() {
            let c = stack.data.last().expect("each stack should have at least one item");
            result.push(*c);
        }

        result
    }
}

#[derive(Debug)]
struct Instruction {
    qty: usize,
    src: usize,
    dst: usize,
}

impl Instruction {
    fn list_from(lines: &Vec<String>) -> Vec<Instruction> {
        let mut result = Vec::new();

        for line in lines {
            let mut parts = line.split_ascii_whitespace();
            
            // Note: nth consumes previous values
            let qty = parts.nth(1).expect("part 2 is qty").parse::<usize>().expect("part 2 must be a uint");
            let src = parts.nth(1).expect("part 4 is src").parse::<usize>().expect("part 4 must be a uint");
            let dst = parts.nth(1).expect("part 6 is dst").parse::<usize>().expect("part 6 must be a uint");

            result.push(Instruction{ qty, src, dst });
        }

        result
    }
}

fn parse(filename: &Path) -> (Warehouse, Vec<Instruction>) {
    let mut lines = read_lines(filename);
    let split_index = lines.iter().position(|line| line.len() == 0).expect("should have empty line");
    let instruction_lines = lines.split_off(split_index + 1);
    
    // Ignore the indexes and empty line
    lines.pop();
    lines.pop();

    let warehouse = Warehouse::from(&lines);
    let instructions = Instruction::list_from(&instruction_lines);
    
    (warehouse, instructions)
}


fn part1(filename: &Path) -> String {
    let (mut warehouse, instructions) = parse(filename);

    for instruction in instructions {
        warehouse.apply(&instruction);
    }

    warehouse.tops()
}

fn part2(filename: &Path) -> String {
    let (mut warehouse, instructions) = parse(filename);

    for instruction in instructions {
        warehouse.apply_9001(&instruction);
    }

    warehouse.tops()
}

fn main() {
    aoc_main(part1, part2);
}

#[cfg(test)]
mod tests {
    use aoc::aoc_test;
    use crate::{part1, part2};

    #[test]   
    fn test1() { aoc_test("05", part1, "TLFGBZHCN") }

    #[test]
    fn test2() { aoc_test("05", part2, "QRQFHFWCL") }
}
