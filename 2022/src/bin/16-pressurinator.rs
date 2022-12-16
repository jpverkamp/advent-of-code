use aoc::*;
use im::{HashSet, Vector};
use memoize::memoize;
use regex::Regex;
use std::{collections::HashMap, hash::Hash, path::Path, sync::Arc};

#[derive(Debug, Eq, PartialEq)]
struct Cave {
    flow_rates: HashMap<String, usize>,
    neighbors: HashMap<String, Vec<String>>,
}

// A Hack to allow Cave to be memoized, always hash to the same thing
impl Hash for Cave {
    fn hash<H: std::hash::Hasher>(&self, _state: &mut H) {}
}

impl<I> From<&mut I> for Cave
where
    I: Iterator<Item = String>,
{
    fn from(iter: &mut I) -> Self {
        let mut flow_rates = HashMap::new();
        let mut neighbors = HashMap::new();

        let re = Regex::new(
            r"Valve (\w+) has flow rate=(\d+); tunnels? leads? to valves? ((?:\w+)(?:, \w+)*)",
        )
        .expect("regex creation failed");

        for line in iter {
            let caps = re.captures(&line).expect("regex doesn't match line");
            let name = String::from(&caps[1]);

            flow_rates.insert(name.clone(), caps[2].parse::<usize>().unwrap());
            neighbors.insert(
                name.clone(),
                caps[3]
                    .split(", ")
                    .map(|s| String::from(s))
                    .collect::<Vec<_>>(),
            );
        }

        Cave {
            flow_rates,
            neighbors,
        }
    }
}

#[derive(Clone, Debug, Hash)]
enum Step {
    DoNothing,
    Enable,
    Move(String),
}

impl Cave {
    fn max_flow(
        self,
        location: String,
        fuel: usize,
        active: HashSet<String>,
    ) -> (Vector<Step>, usize) {
        #[memoize]
        fn max_flow(
            cave: Arc<Cave>,
            location: String,
            fuel: usize,
            active: HashSet<String>,
            max_active: usize,
        ) -> (Vector<Step>, usize) {
            // Base case: out of fuel
            if fuel == 0 {
                return (Vector::new(), 0);
            }

            // Do nothing (base case, only replace if we find something *better*)
            let (mut next_path, mut next_flow) = max_flow(
                cave.clone(),
                location.clone(),
                fuel - 1,
                active.clone(),
                max_active,
            );

            next_path.push_front(Step::DoNothing);
            next_flow += active
                .clone()
                .iter()
                .map(|name| cave.flow_rates[name])
                .sum::<usize>();

            let mut best_path = Some(next_path);
            let mut best_flow = next_flow;

            // Optimization, once we've turned on all nodes, always do nothing
            if active.len() == max_active {
                return (best_path.expect("must have a best path"), best_flow);
            }

            // If we haven't turned on the current node, try that
            if !active.contains(&location.clone()) {
                let mut next_active = active.clone();
                next_active.insert(location.clone());
                let (mut next_path, mut next_flow) = max_flow(
                    cave.clone(),
                    location.clone(),
                    fuel - 1,
                    next_active.clone(),
                    max_active,
                );

                next_path.push_front(Step::Enable);
                next_flow += active
                    .clone()
                    .iter()
                    .map(|name| cave.flow_rates[name])
                    .sum::<usize>();

                if best_path.is_none() || next_flow > best_flow {
                    best_path = Some(next_path);
                    best_flow = next_flow;
                }
            }

            // Try moving to each neighbor node
            for neighbor in cave
                .neighbors
                .get(&location.clone())
                .expect("must have neighbors")
            {
                let (mut next_path, mut next_flow) = max_flow(
                    cave.clone(),
                    neighbor.clone(),
                    fuel - 1,
                    active.clone(),
                    max_active,
                );

                next_path.push_front(Step::Move(neighbor.clone()));
                next_flow += active
                    .iter()
                    .map(|name| cave.flow_rates[name])
                    .sum::<usize>();

                if best_path.is_none() || next_flow > best_flow {
                    best_path = Some(next_path);
                    best_flow = next_flow;
                }
            }

            (best_path.expect("must have a best path"), best_flow)
        }

        let max_active = self.flow_rates.iter().filter(|(_, flow)| **flow > 0).count();
        max_flow(Arc::new(self), location, fuel, active, max_active)
    }
}

fn part1(filename: &Path) -> String {
    let cave = Cave::from(&mut iter_lines(filename));

    let (steps, max_flow) = cave.max_flow(String::from("AA"), 30, HashSet::new());
    if cfg!(debug_assertions) {
        for (i, step) in steps.iter().enumerate() {
            println!("[{:2}] {:?}", i + 1, step);
        }
    }

    max_flow.to_string()
}

fn part2(filename: &Path) -> String {
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
