use aoc::*;
use bitvec::prelude::*;
use im::Vector;
use regex::Regex;
use std::{cell::RefCell, collections::HashMap, hash::Hash, path::Path, rc::Rc};

// Store the description of the cave as a directed graph with flow rates at the nodes
#[derive(Debug)]
struct Cave {
    size: usize,
    names: Vec<String>,
    indexes: HashMap<String, usize>,
    flow_rates: Vec<usize>,
    distances: Matrix<usize>,
}

// A Hack to allow Cave to be memoized, always hash to the same thing
impl Hash for Cave {
    fn hash<H: std::hash::Hasher>(&self, _state: &mut H) {}
}

// Parse a graph from a string iterator
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

        let size = names.len();

        // Calculate a full distance map
        let mut distances = Matrix::<usize>::new(size, size);

        for i in 0..size {
            for j in 0..size {
                distances[[i, j]] = usize::MAX;
            }
        }

        for (src, neighbors) in neighbors.iter() {
            for (distance, dst) in neighbors.iter() {
                distances[[indexes[src], indexes[dst]]] = *distance;
            }
        }

        // Expand to calculate the minimum possible distance between nodes (of any number of steps)
        // For any pair of nodes, if we don't have a distance:
        // - Find a third node between them with a sum of of i->k->l == distance
        // Because distance is increasing from 2 up, this will always fill in minimal values
        loop {
            let mut changed = false;
            for i in 0..size {
                for j in 0..size {
                    for k in 0..size {
                        if i == j || j == k || i == k {
                            continue;
                        }

                        if distances[[i, j]] == usize::MAX || distances[[j, k]] == usize::MAX {
                            continue;
                        }

                        let old_d = distances[[i, k]];
                        let new_d = distances[[i, j]] + distances[[j, k]];
                        if new_d < old_d {
                            changed = true;
                            distances[[i, k]] = new_d;
                        }
                    }
                }
            }

            if !changed {
                break;
            }
        }

        Cave {
            size: names.len(),
            names,
            indexes,
            flow_rates,
            distances,
        }
    }
}

// A single step of the single agent simulation
#[derive(Clone, Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
struct Step(usize, String);

// The state of an agent in the multi agent simulation
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
struct State {
    index: usize,
    ttl: usize,
}

impl State {
    fn new(index: usize) -> State {
        State { index, ttl: 0 }
    }

    fn tick(self, ticks: usize) -> State {
        State {
            index: self.index,
            ttl: self.ttl - ticks,
        }
    }
}

// The more complicated step of agents in the multi agent simulation
#[derive(Clone, Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
struct StepMulti {
    fuel: usize,
    per_tick_flow: usize,
    data: StepMultiData,
}

#[derive(Clone, Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
enum StepMultiData {
    DoNothing,
    AdvanceTime {
        ticks: usize,
        activations: Vec<(usize, String)>,
    },
    Schedule {
        agent: usize,
        distance: usize,
        target: String,
    },
}

// Flow algorithms for a cave
impl Cave {
    // Find the steps for maximizing flow from a single location with a single agent
    fn max_flow(self, location: String, fuel: usize) -> (usize, Vector<Step>) {
        type CacheKey = (usize, usize, Vector<bool>);
        type CacheValue = (usize, Vector<Step>);

        // The memoized recursive function that actually does the work
        // cave and cache don't change
        // index is where the agent currently is
        // fuel is how much fuel is left in the simulation (stop at 0)
        // enabled is a list of which cave pumps are currently enabled
        fn recur(
            cave: Rc<Cave>,
            cache: Rc<RefCell<HashMap<CacheKey, CacheValue>>>,
            index: usize,
            fuel: usize,
            enabled: Vector<bool>,
        ) -> CacheValue {
            // If we have already calculated a result at this index/fuel/enabled, return it
            let cache_key = (index, fuel, enabled.clone());
            if cache.borrow().contains_key(&cache_key) {
                return cache.borrow_mut()[&cache_key].clone();
            }

            // Calculate the current flow based on the enabled gates
            let per_tick_flow = cave
                .clone()
                .flow_rates
                .iter()
                .zip(enabled.clone().iter())
                .filter_map(|(f, c)| if *c { Some(*f) } else { None })
                .sum::<usize>();

            // Base case: try doing nothing for the rest of the simulation
            let mut result = (fuel * per_tick_flow, Vector::new());

            // Try each possible move
            // A move is move to a node (inc multiple hops) + enable that node
            for next_index in 0..cave.clone().size {
                // Don't bother moving to something that's already on
                // Don't bother moving to nodes with 0 flow
                if index == next_index
                    || enabled[next_index]
                    || cave.clone().flow_rates[next_index] == 0
                {
                    continue;
                }

                // Calculate the distance to this new node
                // If we don't have enough fuel to make that trip, this isn't valid
                let d = cave.clone().distances[[index, next_index]];
                if d + 1 > fuel {
                    continue;
                }

                // Calculate which nodes will be enabled after this step
                let mut next_enabled = enabled.clone();
                next_enabled[next_index] = true;

                // Recursively calculate the result from taking this step
                let mut sub_result = recur(
                    cave.clone(),
                    cache.clone(),
                    next_index,
                    fuel - d - 1,
                    next_enabled,
                );

                // Update that result with the total flow from moving
                // And the instruction for output
                sub_result.0 += (d + 1) * per_tick_flow;
                sub_result
                    .1
                    .push_front(Step(d, cave.clone().names[next_index].clone()));

                // If that result is better than what we have so far, update our best result
                result = result.max(sub_result);
            }

            // Store the result in the cache and return it
            cache.borrow_mut().insert(cache_key, result.clone());
            result
        }

        // Fire off the recursive function
        let cave = Rc::new(self);
        recur(
            cave.clone(),
            Rc::new(RefCell::new(HashMap::new())),
            cave.clone().indexes[&location],
            fuel,
            Vector::from(vec![false; cave.clone().size]),
        )
    }

    // The same simulation but with multiple agents
    fn max_flow_multi(
        self,
        location: String,
        fuel: usize,
        agents: usize,
    ) -> (usize, Vec<StepMulti>) {
        type CacheKey = (Vec<State>, usize, BitVec);
        type CacheValue = (usize, Vec<StepMulti>);

        // Main recursive function with multiple agents
        // cave and cache still don't change (other than to cache values)
        // agents is an im::Vector of agent states, can be any number (even 1)
        // - this contains the next index
        // - plus a new value ttl which is how long it will take the agent to get to the index
        // fuel is how long the simulation can still run
        // enabled is the map of which flows are enabled
        fn recur(
            cave: Rc<Cave>,
            cache: Rc<RefCell<HashMap<CacheKey, CacheValue>>>,
            agents: Vec<State>,
            fuel: usize,
            enabled: BitVec,
        ) -> CacheValue {
            // Cache based on the state of all agents/fuel/enabled
            let cache_key = (agents.clone(), fuel, enabled.clone());
            if cache.borrow().contains_key(&cache_key) {
                return cache.borrow_mut()[&cache_key].clone();
            }

            // Calculate flow per tick (even if we won't actually tick)
            let per_tick_flow = cave
                .clone()
                .flow_rates
                .iter()
                .zip(enabled.clone().iter())
                .filter_map(|(f, c)| if *c { Some(*f) } else { None })
                .sum::<usize>();

            // Base case: try doing nothing for the rest of the simulation
            let mut result = (
                fuel * per_tick_flow,
                vec![StepMulti {
                    fuel,
                    per_tick_flow,
                    data: StepMultiData::DoNothing,
                }],
            );

            // Once all useful flows are active, allow moving to anywhere
            // This fixes a previous bug where the first free agent would claim the last valve even it was further away
            let potential_enabled = cave
                .clone()
                .flow_rates
                .iter()
                .zip(enabled.clone())
                .filter_map(|(f, e)| if *f > 0 && !e { Some(true) } else { None })
                .count();

            // If the TTL of any agent is 0, schedule it's next move
            // This doesn't advance time
            if let Some((i, agent)) = agents.clone().iter().enumerate().find(|(_, a)| a.ttl == 0) {
                for next_index in 0..cave.clone().size {
                    // Not allowed to move to the same target as any other agent
                    // Can only move to an already enabled valve if we're in the end state
                    if agents.clone().iter().any(|a| next_index == a.index)
                        || (potential_enabled >= agents.len() && enabled[next_index])
                        || cave.clone().flow_rates[next_index] == 0
                    {
                        continue;
                    }

                    // Check that we have enough fuel to move there
                    let d = cave.clone().distances[[agent.index, next_index]];
                    if d + 1 > fuel {
                        continue;
                    }

                    // Update the agent with where it's going + how long to get there and enable
                    let mut new_agents = agents.clone();
                    new_agents[i] = State {
                        index: next_index,
                        ttl: d + 1,
                    };

                    // Make the recursive call and record that we did
                    let mut sub_result = recur(
                        cave.clone(),
                        cache.clone(),
                        new_agents,
                        fuel,
                        enabled.clone(),
                    );
                    sub_result.1.push(StepMulti {
                        fuel,
                        per_tick_flow,
                        data: StepMultiData::Schedule {
                            agent: i,
                            distance: d,
                            target: cave.clone().names[next_index].clone(),
                        },
                    });

                    // If making this call was better than the current result (of do nothing)
                    // Use it instead
                    result = result.max(sub_result);
                }
            }
            // Otherwise, advance by the ttl of the lowest agent
            else {
                let mut activations = Vec::new();

                // Find time until the agent(s) that will finish moving soonest
                let ticks = agents
                    .clone()
                    .iter()
                    .min_by(|a, b| a.ttl.cmp(&b.ttl))
                    .expect("must have at least one agent")
                    .ttl;

                // Enable any flows for agents with TTL=0 at the end of this move
                let mut next_enabled = enabled.clone();
                for (i, agent) in agents.clone().iter().enumerate() {
                    if agent.ttl == ticks {
                        next_enabled.set(agent.index, true);
                        activations.push((i, cave.clone().names[agent.index].clone()));
                    }
                }

                // Update all agents (including those that will go to 0)
                let mut next_agents = agents.clone();
                for (i, agent) in agents.clone().iter().enumerate() {
                    next_agents[i] = agent.tick(ticks);
                }

                // Make the recursive call
                let mut sub_result = recur(
                    cave.clone(),
                    cache.clone(),
                    next_agents,
                    fuel - ticks,
                    next_enabled,
                );

                // Update flow by that many ticks + record what step we took
                // As always, if this result is better than nothing, record it
                sub_result.0 += ticks * per_tick_flow;
                sub_result.1.push(StepMulti {
                    fuel,
                    per_tick_flow,
                    data: StepMultiData::AdvanceTime { ticks, activations },
                });
                result = result.max(sub_result);
            }

            // Memoize the result and finally return
            cache.borrow_mut().insert(cache_key, result.clone());
            result
        }

        // Init the agents and kick the recursive function off
        let cave = Rc::new(self);
        let (total_flow, steps) = recur(
            cave.clone(),
            Rc::new(RefCell::new(HashMap::new())),
            vec![State::new(cave.clone().indexes[&location]); agents],
            fuel,
            BitVec::from_vec(vec![0; cave.clone().size]),
        );

        // Because we're using Vec, the steps end up in reverse order
        (total_flow, steps.into_iter().rev().collect::<Vec<_>>())
    }
}

fn part1(filename: &Path) -> String {
    let cave = Cave::from(&mut iter_lines(filename));

    let (max_flow, steps) = cave.max_flow(String::from("AA"), 30);
    if true || cfg!(debug_assertions) {
        for (_i, step) in steps.iter().enumerate() {
            println!("{step:?}");
        }
    }

    max_flow.to_string()
}

fn part2(filename: &Path) -> String {
    let cave = Cave::from(&mut iter_lines(filename));

    let (max_flow, steps) = cave.max_flow_multi(String::from("AA"), 26, 2);
    if true || cfg!(debug_assertions) {
        for (_i, step) in steps.iter().enumerate() {
            println!("{step:?}");
        }
    }

    max_flow.to_string()
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
        aoc_test("16", part1, "1720")
    }

    #[ignore]
    #[test]
    fn test2() {
        aoc_test("16", part2, "")
    }
}
