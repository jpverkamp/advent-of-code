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
        left: Rc<Monkey>,
        right: Rc<Monkey>,
    },
}

impl Monkey {
    fn value(&self) -> isize {
        match self {
            Monkey::Constant { value } => *value,
            Monkey::Math { op, left, right } => op(left.value(), right.value()),
        }
    }
}

#[derive(Debug)]
struct Troop {
    monkeys: Rc<RefCell<HashMap<String, Rc<Monkey>>>>,
}

impl Troop {
    fn new() -> Self {
        Troop {
            monkeys: Rc::new(RefCell::new(HashMap::new())),
        }
    }

    fn try_add(&mut self, line: String) -> bool {
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

            self.monkeys.borrow_mut().insert(
                String::from(name),
                Rc::new(Monkey::Constant { value: value }),
            );

            true
        }
        // Mathematical monkey
        else {
            let (mut name, left_name, op_name, right_name) = line
                .split_ascii_whitespace()
                .collect_tuple()
                .expect("Math monkey must be '{name}: {name} {op} {name}'");

            name = name
                .strip_suffix(":")
                .expect("Constant monkey name must have :");

            let op = match op_name {
                "+" => |a, b| a + b,
                "-" => |a, b| a - b,
                "*" => |a, b| a * b,
                "/" => |a, b| a / b,
                "=" => |a, b| if a == b { 1 } else { 0 },
                
                _ => panic!("Math monkey unknown op: {op_name}"),
            };

            let left_name = String::from(left_name);
            let right_name = String::from(right_name);

            if !(self.monkeys.borrow().contains_key(&left_name)
                && self.monkeys.borrow().contains_key(&right_name))
            {
                false
            } else {
                let left = self.monkeys.borrow().get(&left_name).unwrap().clone();
                let right = self.monkeys.borrow().get(&right_name).unwrap().clone();

                self.monkeys.borrow_mut().insert(
                    String::from(name),
                    Rc::new(Monkey::Math { op, left, right }),
                );

                true
            }
        }
    }
}

fn part1(filename: &Path) -> String {
    let mut lines = VecDeque::from(read_lines(filename));
    let mut troop = Troop::new();

    while !lines.is_empty() {
        let line = lines.pop_front().unwrap();

        // Couldn't add the monkey yet
        if !troop.try_add(line.clone()) {
            lines.push_back(line);
        }
    }

    troop.monkeys.clone().borrow()[&String::from("root")]
        .value()
        .to_string()
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
