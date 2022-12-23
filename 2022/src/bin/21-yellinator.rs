use aoc::*;
use itertools::Itertools;
use std::{collections::HashMap, path::Path, rc::Rc, cell::RefCell, fs::File, io::Write};

type INumber = isize;

#[derive(Debug)]
struct Op {
    f: fn(INumber, INumber) -> INumber,
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
                "=" => |a, b| if a == b { 1 as INumber } else { 0 as INumber }, 

                _ => panic!("unknown op: {text}"),
            },
        }
    }
}

#[allow(dead_code)]
#[derive(Debug)]
enum Monkey {
    Human,
    Constant {
        value: INumber,
    },
    Math {
        op: Op,
        left: String,
        right: String,
    },
}

impl Monkey {
    fn is_human(&self) -> bool {
        match self {
            Monkey::Human => true,
            _ => false,
        }
    }

    fn try_op_name(&self) -> Option<String> {
        match self {
            Monkey::Math { op: Op { name, ..}, ..} => Some(name.clone()),
            _ => None,
        }
    }

    fn try_constant_value(&self) -> Option<INumber> {
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
                .parse::<INumber>()
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

    fn value(&self, name: &String) -> INumber {
        match &self.monkeys[name].as_ref() {
            Monkey::Constant { value } => *value,
            Monkey::Math {
                op: Op {f, ..}, left, right, ..
            } => f(self.value(left), self.value(right)),
            _ => panic!("humans have no value"),
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
                            Monkey::Math { left, right, .. } => {
                                left == *potential || right == *potential
                            },
                            _ => false,
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
                    },
                    _ => continue,
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

    // Simplify cases where one first level and one second level child are constant
    fn simplify_equality(&mut self) {
        let rc_self = Rc::new(RefCell::new(self));
        let mut name_count = 0;

        'found_human: for _i in 1.. { 
            let root = rc_self.clone().borrow().monkeys[&String::from("root")].clone();
            if root.try_op_name().is_none() || root.try_op_name().unwrap() != "=" {
                panic!("root must be = to use this method");
            }

            let left_name = root.try_math_left().unwrap().clone();
            let left = rc_self.clone().borrow().monkeys[&left_name].clone();

            let right_name = root.try_math_right().unwrap().clone();
            let right = rc_self.clone().borrow().monkeys[&right_name].clone();
            
            #[allow(unused_assignments)]
            let mut v_level_1 = None; // First level value
            #[allow(unused_assignments)]
            let mut v_level_1_is_left = false;

            #[allow(unused_assignments)]
            let mut v_level_2 = None; // Second level value
            #[allow(unused_assignments)]
            let mut v_level_2_is_left = false;

            #[allow(unused_assignments)]
            let mut op_name = None;
            #[allow(unused_assignments)]
            let mut t_level_2 = None;

            let mut to_remove = vec![
                left_name.clone(),
                right_name.clone(),
            ];

            if left.is_human() || right.is_human() {
                break 'found_human;
            }

            // Left is the constant side
            if let Some(lv) = left.try_constant_value() {
                v_level_1 = Some(lv);
                v_level_1_is_left = true;
                op_name = Some(right.try_op_name().unwrap().clone());

                // Right left is the other constant
                if let Some(rlv) = rc_self.clone().borrow().monkeys[&right.try_math_left().unwrap()].try_constant_value() {
                    v_level_2 = Some(rlv);
                    v_level_2_is_left = true;
                    t_level_2 = Some(right.try_math_right().unwrap());
                    to_remove.push(right.try_math_left().unwrap().clone());
                }

                // Right right is the other constant
                else if let Some(rrv) = rc_self.clone().borrow().monkeys[&right.try_math_right().unwrap()].try_constant_value() {
                    v_level_2 = Some(rrv);
                    v_level_2_is_left = false;
                    t_level_2 = Some(right.try_math_left().unwrap());
                    to_remove.push(right.try_math_right().unwrap().clone());
                }

                // Something went wrong
                else {
                    panic!("neither child of right ({right:?}) is constant");
                }
            }

            // Right is the constant side
            else if let Some(rv) = right.try_constant_value() {
                v_level_1 = Some(rv);
                v_level_1_is_left = false;
                op_name = Some(left.try_op_name().unwrap().clone());

                // Left left is the other constant
                if let Some(llv) = rc_self.clone().borrow().monkeys[&left.try_math_left().unwrap()].try_constant_value() {
                    v_level_2 = Some(llv);
                    v_level_2_is_left = true;
                    t_level_2 = Some(left.try_math_right().unwrap());
                    to_remove.push(left.try_math_left().unwrap().clone());
                }

                // Left right is the other constant
                else if let Some(lrv) = rc_self.clone().borrow().monkeys[&left.try_math_right().unwrap()].try_constant_value() {
                    v_level_2 = Some(lrv);
                    v_level_2_is_left = false;
                    t_level_2 = Some(left.try_math_left().unwrap());
                    to_remove.push(left.try_math_right().unwrap().clone());
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
                
            // Build and attach the new root and frankenmonkey

            // Calculate the various possible inverse functions
            // The annoying one was [[v2 - SUB] = v1] since you need to subtract v2 and then negate
            // I haven't handled all of the cases, just the ones that actually show up in the problem
            let op_name = op_name.unwrap();
            let f_inverse = match (op_name.as_str(), v_level_2_is_left) {
                ("+", _)        => |v1, v2| v1 - v2,
                ("-", true)     => |v1, v2| -1 * (v1 - v2),
                ("-", false)    => |v1, v2| v1 + v2,
                ("*", _)        => |v1, v2| v1 / v2,
                ("/", false)    => |v1, v2| v1 * v2,

                _ => panic!("unknown pattern ({op_name}, {v_level_2_is_left})"),
            };

            // Generate new, unique names for each monkey
            let new_monkey_name = format!("C_{name_count}");
            name_count += 1;

            // Build and insert the new monkey with a constant value based on f_inverse above
            let new_monkey = Monkey::Constant { value: f_inverse(v_level_1.unwrap(), v_level_2.unwrap()) };
            rc_self.clone().borrow_mut().monkeys.insert(new_monkey_name.clone(), Rc::new(new_monkey));

            // Figure out which side we should re-insert the new and old SUB monkes
            let t_level_2_name = t_level_2.unwrap();
            let left_name = if v_level_1_is_left { new_monkey_name.clone() } else { t_level_2_name.clone() };
            let right_name = if v_level_1_is_left { t_level_2_name.clone() } else { new_monkey_name.clone() };

            // Replace the new root node with one a single level down
            rc_self.clone().borrow_mut().monkeys.insert(
                String::from("root"),
                Rc::new(Monkey::Math { 
                    op: Op::from("="),
                    left: left_name,
                    right: right_name,
                }),
            );
            
            // Remove the nodes that we no longer have in our tree (the constant values + what became the root)
            for name in to_remove.into_iter() {
                rc_self.borrow_mut().monkeys.remove(&name);
            }

            if cfg!(debug_assertions) {
                let mut f = File::create(format!("aoc16_{_i:04}.dot")).unwrap();
                writeln!(&mut f, "{}\n", rc_self.borrow().dot(format!("g{_i:04}").as_str())).unwrap();
            }
            
        }
    }

    #[allow(dead_code)]
    fn dot(&self, graph_name: &str) -> String {
        let mut result = format!("graph {graph_name} {{\n");
        for (name, monkey) in self.monkeys.iter() {
            match monkey.as_ref() {
                Monkey::Constant { value } => {
                    result.push_str(format!("\t\"{graph_name}.{name}\" [label=\"{name}\\n{value}\"];\n").as_str());
                }
                Monkey::Math {
                    op: Op {name: op_name, ..},
                    left,
                    right,
                    ..
                } => {
                    result.push_str(format!("\t\"{graph_name}.{name}\" [label=\"{name}\\n{op_name}\", ordering=\"out\"];\n").as_str());
                    result.push_str(format!("\t\t\"{graph_name}.{name}\" -- \"{graph_name}.{left}\" [label=L];\n").as_str());
                    result.push_str(format!("\t\t\"{graph_name}.{name}\" -- \"{graph_name}.{right}\" [label=R];\n").as_str());
                },
                Monkey::Human => {
                    result.push_str(format!("\t\"{graph_name}.humn\" [label=\"HELPME!\"];\n").as_str());
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

    troop.monkeys.insert(String::from("humn"), Rc::new(Monkey::Human));
    
    if cfg!(debug_assertions) {
        let mut f = File::create(format!("aoc16_0a_initial.dot")).unwrap();
        writeln!(&mut f, "{}\n", troop.dot(format!("0a_initial").as_str())).unwrap();
    }

    troop.simplify_constants();
    
    if cfg!(debug_assertions) {
        let mut f = File::create(format!("aoc16_0b_constants.dot")).unwrap();
        writeln!(&mut f, "{}\n", troop.dot(format!("0b_constants").as_str())).unwrap();
    }

    troop.simplify_equality();

    let root = &troop.monkeys[&String::from("root")];
    let left = &troop.monkeys[&root.try_math_left().unwrap()];
    let right = &troop.monkeys[&root.try_math_right().unwrap()];

    if left.is_human() {
        right.try_constant_value().unwrap().to_string()
    } else if right.is_human() {
        left.try_constant_value().unwrap().to_string()
    } else {
        panic!("absolutely murdered the human")
    }
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
        aoc_test("21", part1, "31017034894002")
    }

    #[test]
    fn test2() {
        aoc_test("21", part2, "3555057453229")
    }
}
