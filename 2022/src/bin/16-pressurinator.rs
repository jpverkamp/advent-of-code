use aoc::*;
use im::{Vector};
use regex::Regex;
use std::{cell::RefCell, hash::Hash, path::Path, rc::Rc, collections::HashMap};

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

#[derive(Clone, Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
struct Step(usize, String);

impl Cave {
    fn max_flow(self, location: String, fuel: usize) -> (usize, Vector<Step>) {
        type CacheKey = (usize, usize, Vector<bool>);
        type CacheValue = (usize, Vector<Step>);

        fn recur(
            cave: Rc<Cave>,
            cache: Rc<RefCell<HashMap<CacheKey, CacheValue>>>,
            index: usize,
            fuel: usize,
            enabled: Vector<bool>,
        ) -> CacheValue {
            let cache_key = (index, fuel, enabled.clone());
            if cache.borrow().contains_key(&cache_key) {
                return cache.borrow_mut()[&cache_key].clone();
            }

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
            // Don't bother moving to something that's already on
            // Don't bother moving to nodes with 0 flow
            for next_index in 0..cave.clone().size {
                if index == next_index || enabled[next_index] || cave.clone().flow_rates[next_index] == 0 {
                    continue;
                }

                let d = cave.clone().distances[[index, next_index]];
                if d + 1 > fuel {
                    continue;
                }

                let mut next_enabled = enabled.clone();
                next_enabled[next_index] = true;

                let mut sub_result = recur(
                    cave.clone(),
                    cache.clone(),
                    next_index,
                    fuel - d - 1,
                    next_enabled,
                );
                sub_result.0 += (d + 1) * per_tick_flow;
                sub_result.1.push_front(Step(d, cave.clone().names[next_index].clone()));
                result = result.max(sub_result);
            }

            cache.borrow_mut().insert(cache_key, result.clone());
            result
        }

        let cave = Rc::new(self);
        recur(
            cave.clone(),
            Rc::new(RefCell::new(HashMap::new())),
            cave.clone().indexes[&location],
            fuel,
            Vector::from(vec![false; cave.clone().size]),
        )
    }
}

fn part1(filename: &Path) -> String {
    let cave = Cave::from(&mut iter_lines(filename));

    // println!("{:?}", cave);

    let (max_flow, steps) = cave.max_flow(String::from("AA"), 30);
    if true || cfg!(debug_assertions) {
        for (_i, step) in steps.iter().enumerate() {
            println!("{step:?}");
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
