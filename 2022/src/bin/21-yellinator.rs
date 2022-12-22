use aoc::*;
use itertools::Itertools;
use std::{
    cell::RefCell,
    collections::{HashMap, VecDeque},
    path::Path,
    rc::Rc,
};

#[derive(Debug)]
enum Monkey {
    Constant {
        value: isize,
    },
    Math {
        op: fn(isize, isize) -> isize,
        left: String,
        right: String,
    },
}

#[derive(Debug)]
struct Troop {
    monkeys: HashMap<String, Monkey>,
}

impl Troop {
    fn new() -> Self {
        Troop {
            monkeys: HashMap::new(),
        }
    }

    fn add(&mut self, line: &String) {
        // Simple/constant value monkey
        if line.chars().filter(|c| c.is_whitespace()).count() == 1 {
            let (mut name, value) = line
                .split_ascii_whitespace()
                .collect_tuple()
                .expect("Constant monkey must be '{name}: {value}'");

            name = name
                .strip_suffix(":")
                .expect("Constant monkey name must have :");

            let value = value
                .parse::<isize>()
                .expect("Constant monkey value must be numeric");

            self.monkeys.insert(
                String::from(name),
                Monkey::Constant { value: value },
            );
        }
        // Mathematical monkey
        else {
            let (mut name, left, op_name, right) = line
                .split_ascii_whitespace()
                .collect_tuple()
                .expect("Math monkey must be '{name}: {name} {op} {name}'");

            let name = String::from(name
                .strip_suffix(":")
                .expect("Constant monkey name must have :"));

            let op = match op_name {
                "+" => |a, b| a + b,
                "-" => |a, b| a - b,
                "*" => |a, b| a * b,
                "/" => |a, b| a / b,
                "=" => |a, b| if a == b { 1 } else { 0 },
                
                _ => panic!("Math monkey unknown op: {op_name}"),
            };

            let left = String::from(left);
            let right = String::from(right);

            self.monkeys.insert(
                String::from(name),
                Monkey::Math { op, left, right },
            );
        }
    }

    fn value(&self, name: &String) -> isize {
        match &self.monkeys[name] {
            Monkey::Constant { value } => *value,
            Monkey::Math { op, left, right } => {
                op(
                    self.value(left),
                    self.value(right),
                )
            }
        }
    }
}

fn part1(filename: &Path) -> String {
    let mut troop = Troop::new();
    for line in iter_lines(filename) {
        troop.add(&line);
    }

    troop.value(&String::from("root")).to_string()
}

fn part2(_filename: &Path) -> String {
    todo!()
}

fn main() {
    aoc_main(part1, part2);
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};
    use aoc::aoc_test;

    #[test]
    fn test1() {
        aoc_test("", part1, "")
    }

    #[test]
    fn test2() {
        aoc_test("", part2, "")
    }
}
