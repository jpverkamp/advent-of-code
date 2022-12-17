use aoc::*;
use im::{HashSet, Vector};
use memoize::memoize;
use regex::Regex;
use std::{collections::HashMap, hash::Hash, path::Path, sync::Arc, fmt::Display, mem, rc::Rc};

#[derive(Debug)]
struct Cave {
    names: Vec<String>,
    indexes: HashMap<String, usize>,
    flow_rates: Vec<usize>,
    distances: Matrix<usize>,
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
        let mut indexes = HashMap::new();
        let mut flow_rates = Vec::new();
        let mut neighbors = HashMap::new();

        let re = Regex::new(
            r"Valve (\w+) has flow rate=(\d+); tunnels? leads? to valves? ((?:\w+)(?:, \w+)*)",
        )
        .expect("regex creation failed");

        for (index, line) in iter.enumerate() {
            let caps = re.captures(&line).expect("regex doesn't match line");
            let name = String::from(&caps[1]);

            neighbors.insert(
                name.clone(),
                caps[3]
                    .split(", ")
                    .map(|s| (1, String::from(s)))
                    .collect::<Vec<_>>(),
            );
            
            indexes.insert(name.clone(), index);
            names.push(name);
            flow_rates.push(caps[2].parse::<usize>().unwrap());
        }

        // Calculate a full distance map
        let mut distances = Matrix::<usize>::new(names.len(), names.len());
        for (src, neighbors) in neighbors.iter() {
            for (distance, dst) in neighbors.iter() {
                distances[[indexes[src], indexes[dst]]] = *distance;
            }
        }

        // Expand to calculate the minimum possible distance between nodes (of any number of steps)
        // For any pair of nodes, if we don't have a distance:
        // - Find a third node between them with a sum of of i->k->l == distance
        // Because distance is increasing from 2 up, this will always fill in minimal values
        for distance in 2..=flow_rates.len() {
            for i in 0..=names.len() {
                for j in 0..=names.len() {
                    if i == j {
                        continue;
                    }

                    if distances[[i, j]] > 0 {
                        continue;
                    }

                    for k in 0..=names.len() {
                        if i == k || j == k {
                            continue;
                        }

                        let d = distances[[i, k]] + distances[[k, j]];
                        if d == distance {
                            distances[[i, k]] = d;
                        }
                    }
                }
            }
        }

        Cave {
            names,
            indexes,
            flow_rates,
            distances,
        }
    }
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
enum Step {
    DoNothing,
    Enable,
    Move(usize, String),
}

#[derive(Clone, Default, Debug)]
struct State {
    steps: Vec<Step>,
    enabled: Vec<bool>,
    flow: usize,
    total_flow: usize,
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let State{steps, enabled, flow, total_flow} = self;
        write!(f, "State(total: {total_flow}, current: {flow}, steps: {steps:?})")
    }
}

impl Cave {
    fn max_flow(self, location: String, fuel: usize) -> (Vec<Step>, usize) {
        let initial_index = self.indexes[&location];
        let size = self.names.len();

        let mut states = vec![None; size];
        states[initial_index] = Some(State{
            steps: Vec::new(),
            enabled: vec![false; size],
            flow: 0,
            total_flow: 0,
        });
        
        for _i in 0..=fuel {
            println!("Tick {_i}");
            for (i, state) in states.iter().enumerate() {
                let name = self.names[i].clone();
                match state {
                    None => println!("\t[state {i}={name}] None"),
                    Some(state) => println!("\t[state {i}={name}] {state}"),
                }
            }
            println!("");

            let mut new_states = vec![None; size];

            for (j, dst) in states.iter().enumerate() {
                let mut possibilites = Vec::new();
                
                // If we can already be in this state, try enabling and try doing nothing
                if let Some(State{steps, enabled, flow, total_flow}) = dst {
                    // Do nothing
                    {
                        let mut steps = steps.clone();
                        steps.push(Step::DoNothing);

                        possibilites.push(Some(State{
                            steps: steps,
                            enabled: enabled.clone(),
                            flow: *flow,
                            total_flow: total_flow + flow,
                        }));
                    }

                    // Try to enable
                    if !enabled[j] {
                        let mut steps = steps.clone();
                        steps.push(Step::Enable);

                        let mut enabled = enabled.clone();
                        enabled[j] = true;

                        possibilites.push(Some(State{
                            steps: steps,
                            enabled: enabled.clone(),
                            flow: flow + self.flow_rates[j],
                            total_flow: total_flow + flow,
                        }));
                    }
                }

                // Otherwise, look at all the states we could move from
                for (i, src) in states.iter().enumerate() {
                    // No moving to myself
                    if i == j {
                        continue;
                    }

                    // Can't move here 
                    if self.distances[[i, j]] == 0 {
                        continue;
                    }

                    // We have to be able to be in that state
                    if let Some(State{steps, enabled, flow, total_flow}) = src {
                        let mut steps = steps.clone();
                        steps.push(Step::Move(1, self.names[j].clone()));

                        possibilites.push(Some(State{
                            steps: steps,
                            enabled: enabled.clone(),
                            flow: *flow,
                            total_flow: total_flow + flow,
                        }))
                    }
                }
            
                // Find the best of these states
                new_states[j] = possibilites
                    .into_iter()
                    .filter_map(|el| el)
                    .max_by(|s1, s2| 
                        (s1.total_flow, s1.flow).cmp(&(s2.total_flow, s2.flow))
                    );
            }
            states = new_states;
        }

        todo!()
    }
}

fn part1(filename: &Path) -> String {
    let cave = Cave::from(&mut iter_lines(filename));

    println!("{:?}", cave);

    let (steps, max_flow) = cave.max_flow(String::from("AA"), 30);
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
