use aoc::*;
use std::path::Path;

type MonOp = dyn Fn(isize) -> isize;
type BinOp = fn(isize, isize) -> isize;

/* ----- A single monkey with a brain that can hold, throw, and catch items ----- */
#[derive(Debug)]
struct Monkey {
    held: Vec<isize>,
    operation: (BinOp, Option<isize>),
    test_divisor: isize,
    true_friend: usize,
    false_friend: usize,
    inspection_count: usize,
}

impl<I> From<&mut I> for Monkey
where
    I: Iterator<Item = String>,
{
    fn from(iter: &mut I) -> Self {
        iter.next()
            .expect("skip name line, assume monkeys are ordered");

        // Read starting items
        let mut line = iter.next().expect("must have starting items");
        let mut parts = line.split_ascii_whitespace().skip(2);
        let mut held = Vec::new();
        for value in parts {
            held.push(
                value
                    .strip_suffix(",")
                    .or(Some(value))
                    .expect("strip/or success")
                    .parse::<isize>()
                    .expect("held items must be numbers"),
            );
        }

        // Read operator
        line = iter.next().expect("must have an operation");
        parts = line.split_ascii_whitespace().skip(4);

        let op: fn(isize, isize) -> isize = match parts.next().expect("must have an operator") {
            "*" => |a, b| a * b,
            "+" => |a, b| a + b,
            _ => panic!("unknown operator in {:?}", line),
        };

        let value = parts
            .next()
            .expect("operation must have a value")
            .parse::<isize>()
            .ok();

        let operation = (op, value);

        // Read test
        let test_divisor = iter
            .next()
            .expect("must have a test line")
            .split_ascii_whitespace()
            .last()
            .expect("test must have at least one elmeent")
            .parse::<isize>()
            .expect("divisor must be a number");

        // Read friends
        let true_friend = iter
            .next()
            .expect("must have true friend")
            .split_ascii_whitespace()
            .last()
            .expect("true friend must have at least one element")
            .parse::<usize>()
            .expect("true friend must be an index");

        let false_friend = iter
            .next()
            .expect("must have false friend")
            .split_ascii_whitespace()
            .last()
            .expect("false friend must have at least one element")
            .parse::<usize>()
            .expect("false friend must be an index");

        Monkey {
            held,
            operation,
            test_divisor,
            true_friend,
            false_friend,
            inspection_count: 0,
        }
    }
}

impl Monkey {
    fn toss(&mut self, worry_fix: &MonOp) -> Vec<(usize, isize)> {
        let mut items_in_the_air = Vec::new();

        // For each currently held item
        for old_item in self.held.iter() {
            // Count inspected items
            self.inspection_count += 1;

            // Calculate the new value, None means op is op(old, old)
            let (op, value) = self.operation;
            let mut new_item = op(*old_item, value.or(Some(*old_item)).unwrap());

            // Divide worry by three after each toss
            new_item = worry_fix(new_item);

            // Use the test_divisor to determine which friend we're passing to
            let target = if new_item % self.test_divisor == 0 {
                self.true_friend
            } else {
                self.false_friend
            };

            // Put that in the correct 'in the air' bucket
            items_in_the_air.push((target, new_item));
        }

        // Clear my held items since we just threw them all
        self.held.clear();

        items_in_the_air
    }

    fn catch(&mut self, item: isize) {
        self.held.push(item);
    }
}

/* ----- A collection of monkeys passing things around ----- */
#[derive(Debug)]
struct MonkeyPile {
    monkeys: Vec<Monkey>,
}

impl<I> From<&mut I> for MonkeyPile
where
    I: Iterator<Item = String>,
{
    fn from(iter: &mut I) -> Self {
        let mut iter = iter.peekable();
        let mut monkeys = Vec::new();

        'monkeys: loop {
            monkeys.push(Monkey::from(&mut iter));

            'whitespace: loop {
                match iter.peek() {
                    Some(line) if line.is_empty() => {
                        iter.next();
                    }
                    None => {
                        break 'monkeys;
                    }
                    _ => {
                        break 'whitespace;
                    }
                }
            }
        }

        MonkeyPile { monkeys: monkeys }
    }
}

impl MonkeyPile {
    fn step(&mut self, worry_fix: &MonOp) {
        // Toss all of the items into the air
        for index in 0..self.monkeys.len() {
            let items_in_the_air = self.monkeys.get_mut(index).unwrap().toss(worry_fix);

            for (index, item) in items_in_the_air.into_iter() {
                self.monkeys
                    .get_mut(index)
                    .expect("monkey at index must exist to catch items")
                    .catch(item);
            }
        }
    }
}

fn part1(filename: &Path) -> String {
    let mut iter = iter_lines(filename);
    let mut monkey_pile = MonkeyPile::from(&mut iter);

    let worry_fix = |v| v / 3;

    for _i in 0..20 {
        monkey_pile.step(&worry_fix);
    }

    let mut sorted_monkeys = monkey_pile.monkeys.iter().collect::<Vec<_>>();
    sorted_monkeys.sort_by(|m1, m2| m1.inspection_count.cmp(&m2.inspection_count));
    sorted_monkeys.reverse();

    let monkey_business_score =
        sorted_monkeys[0].inspection_count * sorted_monkeys[1].inspection_count;
    monkey_business_score.to_string()
}

fn part2(filename: &Path) -> String {
    let mut iter = iter_lines(filename);
    let mut monkey_pile = MonkeyPile::from(&mut iter);

    // All tests are divisibility tests, so we only need to word mod(lcm(...))
    // I don't see an LCM function in the stdlib, so this will work (just less efficient)
    let mut worry_value = 1;
    for monkey in monkey_pile.monkeys.iter() {
        worry_value *= monkey.test_divisor;
    }
    let worry_fix = move |v| v % worry_value;

    for _i in 0..10000 {
        monkey_pile.step(&worry_fix);
    }

    let mut sorted_monkeys = monkey_pile.monkeys.iter().collect::<Vec<_>>();
    sorted_monkeys.sort_by(|m1, m2| m1.inspection_count.cmp(&m2.inspection_count));
    sorted_monkeys.reverse();

    let monkey_business_score =
        sorted_monkeys[0].inspection_count * sorted_monkeys[1].inspection_count;
    monkey_business_score.to_string()
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
        aoc_test("11", part1, "99840")
    }

    #[test]
    fn test2() {
        aoc_test("11", part2, "20683044837")
    }
}
