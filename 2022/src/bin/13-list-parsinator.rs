use aoc::*;
use std::{cell::RefCell, fmt::Display, path::Path};

#[derive(Clone, Debug, Ord, Eq, PartialEq)]
enum Signal {
    Value(u8),
    List(Vec<Signal>),
}

impl From<String> for Signal {
    fn from(line: String) -> Self {
        let stack = RefCell::new(Vec::new());
        let current_number = RefCell::new(String::new());

        // Push an initial 'wrapper' list that we'll remove before returning
        stack.borrow_mut().push(Signal::List(Vec::new()));

        // Helper function that will check if we're currently parsing a number
        // If so, push it onto the current list (malformed input if it's not a list)
        let try_push_number = || {
            if current_number.borrow().is_empty() {
                return;
            }
            let value = current_number
                .borrow()
                .parse::<u8>()
                .expect("number should be a number");

            current_number.borrow_mut().clear();

            match stack
                .borrow_mut()
                .last_mut()
                .expect("must still have one item to push to")
            {
                Signal::List(v) => {
                    v.push(Signal::Value(value));
                }
                _ => panic!("malformed stack, expected list to put thing in"),
            }
        };

        // Process the input one char at a time
        for c in line.chars() {
            match c {
                // Start a new nested list
                '[' => {
                    stack.borrow_mut().push(Signal::List(Vec::new()));
                }
                // Finish the most recent nested list
                // Make sure to finish a number if we were parsing one
                // Then push this list into the one before it as an element
                ']' => {
                    try_push_number();

                    let thing = stack.borrow_mut().pop().expect("mismatched []");
                    match stack
                        .borrow_mut()
                        .last_mut()
                        .expect("must still have one item to push to")
                    {
                        Signal::List(v) => {
                            v.push(thing);
                        }
                        _ => panic!("malformed stack, expected list to put thing in"),
                    }
                }
                // Finish the current number and start a new one
                ',' => {
                    try_push_number();
                }
                // Building up a number one digit at a time
                c if c.is_digit(10) => {
                    current_number.borrow_mut().push(c);
                }
                // Anything else is bad input
                _ => panic!("unexpected char {}", c),
            }
        }

        // Verify that we have exactly one element left
        assert_eq!(stack.borrow().len(), 1);
        let wrapper = stack.borrow_mut().pop().unwrap();

        // Unwrap the initial 'wrapper' list we made at the beginning
        match wrapper {
            Signal::List(v) if v.len() == 1 => v[0].clone(),
            _ => panic!("must end with the final wrapper list with only one element"),
        }
    }
}

impl Display for Signal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // I feel like I should be able to do this directly, but couldn't figure out how
        fn stringify(s: &Signal) -> String {
            let mut result = String::new();

            match s {
                Signal::Value(v) => result.push_str(&v.to_string()),
                Signal::List(v) => {
                    result.push('[');
                    result.push_str(&v.iter().map(stringify).collect::<Vec<String>>().join(", "));
                    result.push(']');
                }
            }

            result
        }

        write!(f, "{}", stringify(self))
    }
}

// Doing PartialOrd instead of Ord to get the 'correct' default behavior for Vecs in List
impl PartialOrd for Signal {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        use Signal::*;

        match (self, other) {
            // Two values or two lists use the built in partial_cmp functions
            (Value(a), Value(b)) => a.partial_cmp(b),
            (List(a), List(b)) => a.partial_cmp(b),
            // One of each turns the value into a singleton list and then compares
            (Value(a), List(..)) => List(vec![Value(*a)]).partial_cmp(other),
            (List(..), Value(b)) => self.partial_cmp(&List(vec![Value(*b)])),
        }
    }
}

fn part1(filename: &Path) -> String {
    let mut lines = iter_lines(filename).peekable();
    let mut sum_of_ordered = 0;

    // Iterate over pairs of input
    for index in 1.. {
        // If is_none, we've reached the end of the file
        if lines.peek().is_none() {
            break;
        }

        // Read the signals, consume the next newline if there is one (otherwise EOF)
        let s1 = Signal::from(lines.next().expect("must have first signal"));
        let s2 = Signal::from(lines.next().expect("must have second signal"));
        lines.next_if_eq(&"");

        if s1 < s2 {
            sum_of_ordered += index;
        }
    }

    sum_of_ordered.to_string()
}

fn part2(filename: &Path) -> String {
    // Remove newlines and parse everything else as signals
    let mut signals = iter_lines(filename)
        .filter(|line| !line.is_empty())
        .map(Signal::from)
        .collect::<Vec<_>>();

    // Define and add our 'divider' signals
    let dividers = vec![
        Signal::from(String::from("[[2]]")),
        Signal::from(String::from("[[6]]")),
    ];

    for divider in dividers.iter() {
        signals.push(divider.clone());
    }

    signals.sort();

    // Extract the indices of any dividers and multiply as requested
    dividers
        .iter()
        .map(|d| 1 + signals.iter().position(|s| s == d).unwrap())
        .product::<usize>()
        .to_string()
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
        aoc_test("13", part1, "5506")
    }

    #[test]
    fn test2() {
        aoc_test("13", part2, "21756")
    }
}
