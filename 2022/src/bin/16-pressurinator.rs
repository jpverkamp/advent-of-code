use aoc::*;
use im::{HashSet, Vector};
use memoize::memoize;
use regex::Regex;
use std::{collections::HashMap, hash::Hash, path::Path, sync::Arc};

#[derive(Debug, Eq, PartialEq)]
struct Cave {
    flow_rates: HashMap<String, usize>,
    neighbors: HashMap<String, Vec<(usize, String)>>,
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
        let mut names = Vec::new();
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
                    .map(|s| (1, String::from(s)))
                    .collect::<Vec<_>>(),
            );
            names.push(name);
        }

        // Calculate a full distance map
        let mut distances = HashMap::new();
        for (src, neighbors) in neighbors.iter() {
            for (distance, dst) in neighbors {
                distances.insert((src, dst), *distance);
            }
        }

        // Add all possible increments
        for distance in 2..=flow_rates.len() {
            for src in names.iter() {
                for dst in names.iter() {
                    // We already have a (better) distance for this
                    if distances.contains_key(&(src, dst)) {
                        continue;
                    }

                    // Otherwise, see if we have a possible new midpoint
                    for mid in names.iter() {
                        if !distances.contains_key(&(src, mid)) {
                            continue;
                        }

                        if !distances.contains_key(&(mid, dst)) {
                            continue;
                        }

                        // Check if that midpoint has the current distance
                        let d = distances[&(src, mid)] + distances[&(mid, dst)];
                        if d == distance {
                            distances.insert((src, dst), d);
                            break;
                        }
                    }
                }
            }
        }

        // Rebuild the neighbors array using the updated distances
        neighbors = names
            .iter()
            .map(|src| {
                (
                    src.clone(),
                    names
                        .iter()
                        .filter_map(
                            |dst| distances
                                .get(&(src, dst))
                                .and_then(|d| 
                                    if *d >= 1 {
                                        Some((*d, dst.clone()))
                                    } else {
                                        None
                                    }
                                )
                        )
                        .collect::<Vec<_>>()
                )
            })
            .collect::<HashMap<_, _>>();

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
    Move(usize, String),
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
            active: HashSet<String>
        ) -> (Vector<Step>, usize) {
            // println!("max_flow({location}, {fuel}, {})", active.len());

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
            );

            next_path.push_front(Step::DoNothing);
            next_flow += active
                .clone()
                .iter()
                .map(|name| cave.flow_rates[name])
                .sum::<usize>();

            let mut best_path = Some(next_path);
            let mut best_flow = next_flow;

            // If we haven't turned on the current node, try that
            if !active.contains(&location.clone()) {
                let mut next_active = active.clone();
                next_active.insert(location.clone());
                let (mut next_path, mut next_flow) = max_flow(
                    cave.clone(),
                    location.clone(),
                    fuel - 1,
                    next_active.clone(),
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
            for (distance, neighbor) in cave
                .neighbors
                .get(&location.clone())
                .expect("must have neighbors")
            {
                // We've already gone there
                if active.contains(neighbor) {
                    continue;
                }

                // Don't have enough fuel to make it there
                if *distance > fuel {
                    continue;
                }

                let (mut next_path, mut next_flow) = max_flow(
                    cave.clone(),
                    neighbor.clone(),
                    fuel - distance,
                    active.clone(),
                );

                next_path.push_front(Step::Move(*distance, neighbor.clone()));
                next_flow += active
                    .iter()
                    .map(|name| *distance * cave.flow_rates[name])
                    .sum::<usize>();

                if best_path.is_none() || next_flow > best_flow {
                    best_path = Some(next_path);
                    best_flow = next_flow;
                }
            }

            (best_path.expect("must have a best path"), best_flow)
        }

        max_flow(Arc::new(self), location, fuel, active)
    }
}

fn part1(filename: &Path) -> String {
    let cave = Cave::from(&mut iter_lines(filename));

    // println!("{:?}", cave);

    let (steps, max_flow) = cave.max_flow(String::from("AA"), 30, HashSet::new());
    if true || cfg!(debug_assertions) {
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
