use aoc::*;
use regex::Regex;
use std::{collections::{HashMap, HashSet}, hash::Hash, path::Path, time::Instant, env};

// Store the description of the cave as a directed graph with flow rates at the nodes
#[derive(Clone, Debug)]
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

// Flow algorithms for a cave
impl Cave {
    // Find the steps for maximizing flow from a single location with a single agent
    fn max_flow(self, start: String, fuel: usize) -> (usize, Vec<usize>) {
        let mut queue = Vec::new();
        queue.push((0, fuel, vec![self.indexes[start.as_str()]]));

        let mut best = (0, vec![0]);
        let mut timer = Instant::now();

        let mut count = 0;

        while !queue.is_empty() {
            let (pressure, fuel, path) = queue.pop().unwrap();
            count += 1;

            if cfg!(debug_assertions) {
                if pressure > best.0 {
                    println!(
                        "new best: pressure={pressure}, path={:?}, fuel={fuel}",
                        path.iter()
                            .map(|i| format!("{}={}", i, self.names[*i].clone()))
                            .collect::<Vec<_>>(),
                    );
                }

                if timer.elapsed().as_secs_f32() > 1.0 {
                    println!("count: {count}, q: {}, current (pressure={pressure}, path={:?}, fuel={fuel}), best: (pressure={}, path={:?})",
                        queue.len(),
                        path.iter().map(|i| format!("{}={}", i, self.names[*i].clone())).collect::<Vec<_>>(),
                        best.0,
                        best.1.iter().map(|i| format!("{}={}", i, self.names[*i].clone())).collect::<Vec<_>>(),
                    );
                    timer = Instant::now();
                }
            }

            if pressure > best.0 {
                best = (pressure, path.clone());
            }

            for i in 0..self.size {
                let d = self.distances[[*path.last().unwrap(), i]];

                if path.contains(&i) || self.flow_rates[i] == 0 || d + 1 > fuel {
                    continue;
                }

                let mut new_path = path.clone();
                new_path.push(i);

                queue.push((
                    pressure + (fuel - d - 1) * self.flow_rates[i],
                    fuel - d - 1,
                    new_path,
                ));
            }
        }

        best
    }

    fn max_flow_multi(self, start: String, fuel: usize, agents: usize) -> (usize, Vec<Vec<usize>>) {
        let mut queue = Vec::new();
        let start_path = vec![self.indexes[start.as_str()]];

        queue.push((0, vec![fuel; agents], vec![start_path.clone(); agents]));

        let start = Instant::now();
        let mut tick = Instant::now();
        let mut count = 0;

        let enable_progress_print = env::var("AOC16_PRINT_PROGRESS").is_ok() || cfg!(debug_assertions);

        let enable_prune_optimization = env::var("AOC16_OPT_PRUNE").is_ok();
        let mut prune_count = 0;

        let enable_seen_optimization = env::var("AOC16_OPT_SEEN").is_ok();
        let mut seen = HashSet::new();
        let mut seen_skip_count = 0;

        let mut best = (0, vec![start_path.clone(); agents]);
        while !queue.is_empty() {
            let (pressure, fuels, paths) = queue.pop().unwrap();
            count += 1;

            if enable_seen_optimization {
                seen.insert((fuels.clone(), paths.clone()));
            }
            
            if enable_progress_print {
                if tick.elapsed().as_secs_f32() > 5.0 {
                    println!(
                        "After {}s, examined {count} states, pruned {prune_count}, seen skipped {seen_skip_count}, {} in queue",
                        start.elapsed().as_secs(),
                        queue.len()
                    );
                    tick = Instant::now();
                }

                if pressure > best.0 {
                    println!(
                        "new best: pressure={pressure}, extra fuel={fuels:?}, paths: [{}]",
                        paths
                            .iter()
                            .map(|path| path
                                .iter()
                                .map(|i| format!("{}={}", i, self.names[*i].clone()))
                                .collect::<Vec<_>>()
                                .join(", "),)
                            .collect::<Vec<_>>()
                            .join("]; [")
                    );
                }
            }

            if pressure > best.0 {
                best = (pressure, paths.clone());
            }

            if enable_prune_optimization {
                // Calculate the best case remaining flow and stop if we can't hit it
                // For each node:
                let remaining_best_case = self
                    .flow_rates
                    .iter()
                    .enumerate()
                    .map(|(i, f)| {
                        // If it's already on, ignore it
                        if paths.iter().any(|path| path.contains(&i)) {
                            0
                        } else {
                            // Otherwise, for each agent, find the agent that would be best
                            // This is defined as the flow rate * the fuel left after moving to that node
                            // Take the best case here
                            // This will over estimate, since it assumes each node can go to all nodes at once
                            paths
                                .iter()
                                .enumerate()
                                .map(|(pi, p)| {
                                    let d = self.distances[[*p.last().unwrap(), i]];
                                    if d + 1 <= fuels[pi] {
                                        f * (fuels[pi] - d - 1)
                                    } else {
                                        0
                                    }
                                })
                                .max()
                                .unwrap()
                        }
                    })
                    .sum::<usize>();

                // If even the best case isn't good enough, don't consider any more cases on this branch
                if pressure + remaining_best_case < best.0 {
                    prune_count += 1;
                    continue;
                }
            }

            // For each path and each next node to visit:
            // - check if the node is worth visiting (no duplicates, has flow, can reach)
            // - if so, add that as a possibility
            for (path_i, path) in paths.iter().enumerate() {
                for next_i in 0..self.size {
                    let d = self.distances[[*path.last().unwrap(), next_i]];

                    if paths.iter().any(|path| path.contains(&next_i))
                        || self.flow_rates[next_i] == 0
                        || d + 1 > fuels[path_i]
                    {
                        continue;
                    }

                    let mut new_paths = paths.clone();
                    new_paths[path_i].push(next_i);

                    let mut new_fuels = fuels.clone();
                    new_fuels[path_i] -= d + 1;

                    if enable_seen_optimization {
                        if seen.contains(&(new_fuels.clone(), new_paths.clone())) {
                            seen_skip_count += 1;
                            continue;
                        }
                    }

                    queue.push((
                        pressure + (fuels[path_i] - d - 1) * self.flow_rates[next_i],
                        new_fuels,
                        new_paths,
                    ));
                }
            }
        }

        if enable_progress_print {
            println!(
                "[Final] After {}s, examined {count} states, pruned {prune_count}, seen skipped {seen_skip_count}, {} in queue\n",
                start.elapsed().as_secs_f32(),
                queue.len()
            );
        }

        best
    }
}

fn part1(filename: &Path) -> String {
    let cave = Cave::from(&mut iter_lines(filename));

    let (max_flow, path) = cave.clone().max_flow(String::from("AA"), 30);
    if cfg!(debug_assertions) {
        for step in path.iter() {
            println!("{step:?} = {}", cave.names[*step]);
        }
    }

    max_flow.to_string()
}

fn part2(filename: &Path) -> String {
    let cave = Cave::from(&mut iter_lines(filename));

    let (max_flow, paths) = cave.clone().max_flow_multi(String::from("AA"), 26, 2);
    if cfg!(debug_assertions) {
        for (path_i, path) in paths.iter().enumerate() {
            println!("=== Agent {path_i} ===");
            for step in path.iter() {
                println!("{step:?} = {}", cave.names[*step]);
            }
            println!();
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
