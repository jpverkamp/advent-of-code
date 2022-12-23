use aoc::*;
use itertools::Itertools;
use std::{collections::HashMap, path::Path, rc::Rc, cell::RefCell};

#[derive(Debug)]
struct Op {
    f: fn(isize, isize) -> isize,
    name: String,
}

impl From<&str> for Op {
    fn from(text: &str) -> Self {
        Op {
            name: String::from(text),
            f: match text {
                "+" => |a, b| a + b, 
                "-" => |a, b| a - b, 
                "*" => |a, b| a * b, 
                "/" => |a, b| a / b, 
                "=" => |a, b| if a == b { 1 } else { 0 }, 

                _ => panic!("unknown op: {text}"),
            },
        }
    }
}

impl Op {
    fn from_inverse(name: &str) -> Self {
        match name {
            "+" => Op::from("-"),
            "-" => Op::from("+"),
            "*" => Op::from("/"),
            "/" => Op::from("*"),

            _ => panic!("don't know the inverse of: {}", name)
        }
    }
}

#[allow(dead_code)]
#[derive(Debug)]
enum Monkey {
    Constant {
        value: isize,
    },
    Math {
        op: Op,
        left: String,
        right: String,
    },
}

impl Monkey {
    fn try_op_name(&self) -> Option<String> {
        match self {
            Monkey::Math { op: Op { name, ..}, ..} => Some(name.clone()),
            _ => None,
        }
    }

    fn try_constant_value(&self) -> Option<isize> {
        match self {
            Monkey::Constant { value } => Some(*value),
            _ => None,
        }
    }

    fn try_math_left(&self) -> Option<String> {
        match self {
            Monkey::Math { left, .. } => Some(left.clone()),
            _ => None,
        }
    }

    fn try_math_right(&self) -> Option<String> {
        match self {
            Monkey::Math { right, .. } => Some(right.clone()),
            _ => None,
        }
    }
}

#[derive(Debug)]
struct Troop {
    monkeys: HashMap<String, Rc<Monkey>>,
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
                .insert(name, Rc::new(Monkey::Constant { value: value }));
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

            let op = Op::from(op_name);

            let left = String::from(left);
            let right = String::from(right);

            self.monkeys.insert(
                String::from(name),
                Rc::new(Monkey::Math {
                    op,
                    left,
                    right,
                }),
            );
        }
    }

    fn value(&self, name: &String) -> isize {
        match &self.monkeys[name].as_ref() {
            Monkey::Constant { value } => *value,
            Monkey::Math {
                op: Op {f, ..}, left, right, ..
            } => f(self.value(left), self.value(right)),
        }
    }

    // Remove all monkeys that aren't root and aren't referenced by any others
    fn remove_dead_nodes(&mut self) {
        let to_remove = self
            .monkeys
            .iter()
            .filter(|(potential, _)| {
                !self.monkeys.iter().any(|(_, monkey)| {
                    potential.as_str() == "root"
                        || match monkey.as_ref() {
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

    // Simplify all non-human and non-human-dependant lifeforms
    // If both children of a node are constant, apply the expression
    // Do not simplify the 'humn' node
    fn simplify_constants(&mut self) {
        loop {
            let mut target = None;

            'found_one: for (name, monkey) in self.monkeys.iter() {
                match monkey.as_ref() {
                    Monkey::Constant { .. } => continue,
                    Monkey::Math {
                        op: Op {f, ..}, left, right, ..
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
                                        Some((name.clone(), f(left_value, right_value)))
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
                self.monkeys.insert(name, Rc::new(Monkey::Constant { value }));
                continue;
            }

            break;
        }

        self.remove_dead_nodes();
    }

    // Simplify 
    fn simplify_equality(&mut self) {
        let rc_self = Rc::new(RefCell::new(self));

        let root_name = String::from("root");
        let humn_name = String::from("humn");

        'found_human: loop { 
            let root = rc_self.clone().borrow().monkeys[&root_name].clone();
            if root.try_op_name().is_none() || root.try_op_name().unwrap() != "=" {
                panic!("root must be = to use this method");
            }

            let left = rc_self.clone().borrow().monkeys[&root.try_math_left().unwrap()].clone();
            let right = rc_self.clone().borrow().monkeys[&root.try_math_right().unwrap()].clone();

            // Left is the constant side
            if let Some(lv) = left.try_constant_value() {
                // Right left is the other constant
                if let Some(rlv) = rc_self.clone().borrow().monkeys[&right.try_math_left().unwrap()].try_constant_value() {
                    if right.try_math_left().unwrap() == humn_name {
                        break 'found_human;
                    }

                    // lv = R
                    // lv = Rop(rlv, RR)
                    // Rop'(rlv, lv) = RR
                    let op_name = left.try_op_name().unwrap();
                    let new_monkey_name = format!("{}{:?}{}", rlv, op_name, lv);
                    let new_monkey = Monkey::Constant { value: (Op::from_inverse(op_name.as_str()).f)(rlv, lv) };
                    rc_self.clone().borrow_mut().monkeys.insert(new_monkey_name.clone(), Rc::new(new_monkey));

                    rc_self.clone().borrow_mut().monkeys.insert(
                        root_name,
                        Rc::new(Monkey::Math { 
                            op: Op::from("="),
                            left: new_monkey_name,
                            right: right.try_math_right().unwrap(),
                        }),
                    );
                }

                // Right right is the other constant
                else if let Some(rrv) = rc_self.clone().borrow().monkeys[&right.try_math_right().unwrap()].try_constant_value() {
                    if right.try_math_right().unwrap() == humn_name {
                        break 'found_human;
                    }

                    // lv = R
                    // lv = Rop(RL, rrv)
                    // Rop'(lv, rrv) = RR
                    let op_name = left.try_op_name().unwrap();
                    let new_monkey_name = format!("{}{:?}{}", lv, op_name, rrv);
                    let new_monkey = Monkey::Constant { value: (Op::from_inverse(op_name.as_str()).f)(lv, rrv) };
                    rc_self.clone().borrow_mut().monkeys.insert(new_monkey_name.clone(), Rc::new(new_monkey));

                    rc_self.clone().borrow_mut().monkeys.insert(
                        root_name,
                        Rc::new(Monkey::Math { 
                            op: Op::from("="),
                            left: new_monkey_name,
                            right: right.try_math_right().unwrap(),
                        }),
                    );
                }

                // Something went wrong
                else {
                    panic!("neither child of right ({right:?}) is constant");
                }
            }

            // Right is the constant side
            else if let Some(rv) = right.try_constant_value() {
                // Left left is the other constant
                if let Some(llv) = rc_self.clone().borrow().monkeys[&left.try_math_left().unwrap()].try_constant_value() {
                    if left.try_math_left().unwrap() == humn_name {
                        break 'found_human;
                    }

                    // L = rv
                    // Lop(llv, LR) = rv
                    // LR = Lop'(llv, rv)
                    let op_name = left.try_op_name().unwrap();
                    let new_monkey_name = format!("{}{:?}{}", llv, op_name, rv);
                    let new_monkey = Monkey::Constant { value: (Op::from_inverse(op_name.as_str()).f)(llv, rv) };
                    rc_self.clone().borrow_mut().monkeys.insert(new_monkey_name.clone(), Rc::new(new_monkey));

                    rc_self.clone().borrow_mut().monkeys.insert(
                        root_name,
                        Rc::new(Monkey::Math { 
                            op: Op::from("="),
                            left: right.try_math_left().unwrap(),
                            right: new_monkey_name,
                        }),
                    );
                }

                // Left right is the other constant
                else if let Some(lrv) = rc_self.clone().borrow().monkeys[&left.try_math_right().unwrap()].try_constant_value() {
                    if left.try_math_right().unwrap() == humn_name {
                        break 'found_human;
                    }

                    // L = rv
                // Lop(LL, lrv) = rv
                // LR = Lop'(rv, lrv)
                    let op_name = left.try_op_name().unwrap();
                    let new_monkey_name = format!("{}{:?}{}", rv, op_name, lrv);
                    let new_monkey = Monkey::Constant { value: (Op::from_inverse(op_name.as_str()).f)(rv, lrv) };

                    rc_self.clone().borrow_mut().monkeys.insert(new_monkey_name.clone(), Rc::new(new_monkey));

                    rc_self.clone().borrow_mut().monkeys.insert(
                        root_name,
                        Rc::new(Monkey::Math { 
                            op: Op::from("="),
                            left: right.try_math_left().unwrap(),
                            right: new_monkey_name,
                        }),
                    );
                }

                // Something went wrong
                else {
                    panic!("neither child of left ({left:?}) is constant");
                }
            }

            // Something went wrong
            else {
                panic!("neither left nor root of root is constant, left: {left:?}, right: {right:?}");
            }
            

            // DEBUG
            println!("{root:?}, {left:?}, {right:?}");
            break; 

            

        }
    }

    #[allow(dead_code)]
    fn dot(&self) -> String {
        let mut result = String::from("graph G {\n");
        for (name, monkey) in self.monkeys.iter() {
            match monkey.as_ref() {
                Monkey::Constant { value } => {
                    result.push_str(format!("\t{name} [label=\"{name}\\n{value}\"];\n").as_str());
                }
                Monkey::Math {
                    op: Op {name: op_name, ..},
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

    println!("before: {}", troop.monkeys.len());
    troop.simplify_constants();
    println!("{}", troop.dot());
    println!("after simplify constant: {}", troop.monkeys.len());
    troop.simplify_equality();
    println!("after simplify equality: {}", troop.monkeys.len());
    todo!();

    let mut human_value = 0; // nihilism, yo
    loop {
        {
            let me = troop.monkeys.get_mut(&String::from("humn")).unwrap();
            match me.as_ref() {
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
