use aoc::*;
use itertools::Itertools;
use std::{collections::HashMap, path::Path};

#[allow(dead_code)]
#[derive(Debug)]
enum Monkey {
    Constant {
        value: isize,
    },
    Math {
        op: fn(isize, isize) -> isize,
        op_name: String,
        left: String,
        right: String,
    },
}

impl Monkey {
    fn try_constant_value(&self) -> Option<isize> {
        match self {
            Monkey::Constant { value } => Some(*value),
            _ => None,
        }
    }
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
            let (name, value) = line
                .split_ascii_whitespace()
                .collect_tuple()
                .expect("Constant monkey must be '{name}: {value}'");

            let name = String::from(
                name
                .strip_suffix(":")
                .expect("Constant monkey name must have :")
            );

            let value = value
                .parse::<isize>()
                .expect("Constant monkey value must be numeric");

            self.monkeys
                .insert(name, Monkey::Constant { value: value });
        }
        // Mathematical monkey
        else {
            let (name, left, op_name, right) = line
                .split_ascii_whitespace()
                .collect_tuple()
                .expect("Math monkey must be '{name}: {name} {op} {name}'");

            let name = String::from(
                name.strip_suffix(":")
                    .expect("Constant monkey name must have :"),
            );

            let op = match op_name {
                "+" => |a, b| a + b,
                "-" => |a, b| a - b,
                "*" => |a, b| a * b,
                "/" => |a, b| a / b,
                "=" => |a, b| if a == b { 1 } else { 0 },

                _ => panic!("Math monkey unknown op: {op_name}"),
            };
            let op_name = String::from(op_name);

            let left = String::from(left);
            let right = String::from(right);

            self.monkeys.insert(
                String::from(name),
                Monkey::Math {
                    op,
                    op_name,
                    left,
                    right,
                },
            );
        }
    }

    fn value(&self, name: &String) -> isize {
        match &self.monkeys[name] {
            Monkey::Constant { value } => *value,
            Monkey::Math {
                op, left, right, ..
            } => op(self.value(left), self.value(right)),
        }
    }

    fn simplify(&mut self) {
        // Simplify all non-human and non-human-dependant lifeforms
        loop {
            let mut target = None;

            'found_one: for (name, monkey) in self.monkeys.iter() {
                match monkey {
                    Monkey::Constant { .. } => continue,
                    Monkey::Math {
                        op, left, right, ..
                    } => {
                        if left.as_str() == "humn" || right.as_str() == "humn" {
                            continue;
                        }

                        if let Some(response) = self
                            .monkeys
                            .get(left)
                            .and_then(|m| m.try_constant_value())
                            .and_then(|left_value| {
                                self.monkeys
                                    .get(right)
                                    .and_then(|m| m.try_constant_value())
                                    .and_then(|right_value| {
                                        Some((name.clone(), op(left_value, right_value)))
                                    })
                            })
                        {
                            target = Some(response);
                            break 'found_one;
                        }
                    }
                }
            }

            if let Some((name, value)) = target {
                self.monkeys.insert(name, Monkey::Constant { value });
                continue;
            }

            break;
        }

        // Remove all unneeded monkeys
        let to_remove = self
            .monkeys
            .iter()
            .filter(|(potential, _)| {
                !self.monkeys.iter().any(|(_, monkey)| {
                    potential.as_str() == "root"
                        || match monkey {
                            Monkey::Constant { .. } => false,
                            Monkey::Math { left, right, .. } => {
                                left == *potential || right == *potential
                            }
                        }
                })
            })
            .map(|(name, _)| name.clone())
            .collect::<Vec<_>>();

        for name in to_remove.iter() {
            self.monkeys.remove(&name.clone());
        }
    }

    #[allow(dead_code)]
    fn dot(&self) -> String {
        let mut result = String::from("graph G {\n");
        for (name, monkey) in self.monkeys.iter() {
            match monkey {
                Monkey::Constant { value } => {
                    result.push_str(format!("\t{name} [label=\"{name}\\n{value}\"];\n").as_str());
                }
                Monkey::Math {
                    op_name,
                    left,
                    right,
                    ..
                } => {
                    result.push_str(format!("\t{name} [label=\"{name}\\n{op_name}\"];").as_str());
                    result.push_str(format!(" {name} -- {left}, {right};\n").as_str());
                }
            }
        }

        result.push('}');

        result
    }
}

fn part1(filename: &Path) -> String {
    let mut troop = Troop::new();
    for line in iter_lines(filename) {
        troop.add(&line);
    }

    troop.value(&String::from("root")).to_string()
}

fn part2(filename: &Path) -> String {
    let mut troop = Troop::new();
    for mut line in iter_lines(filename) {
        // Hacky, :shrug:
        if line.starts_with("root:") {
            line = line
                .replace("+", "=")
                .replace("-", "=")
                .replace("*", "=")
                .replace("/", "=");
        }

        troop.add(&line);
    }
    troop.simplify();

    let mut human_value = 0; // nihilism, yo
    loop {
        {
            let me = troop.monkeys.get_mut(&String::from("humn")).unwrap();
            match me {
                Monkey::Constant { value } => {
                    *value = human_value;
                }
                _ => panic!("haven't deal with non-constant me yet"),
            }
        }

        let result = troop.value(&String::from("root"));
        if result == 1 {
            break;
        }

        human_value += 1; // better?
    }

    human_value.to_string()
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
