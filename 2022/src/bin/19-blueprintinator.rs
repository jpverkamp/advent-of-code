use aoc::*;
use itertools::Itertools;
use regex::Regex;
use std::{path::Path, time::Instant};

type ID = u16;
type Qty = u16;
type Qtys = [Qty; Material::COUNT];

fn make_qtys() -> Qtys {
    [0; Material::COUNT]
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Material {
    Ore = 0,
    Clay = 1,
    Obsidian = 2,
    Geode = 3,
}

impl Material {
    const COUNT: usize = 4;
}

impl From<String> for Material {
    fn from(s: String) -> Self {
        match s.to_lowercase().as_str() {
            "ore" => Material::Ore,
            "clay" => Material::Clay,
            "obsidian" => Material::Obsidian,
            "geode" => Material::Geode,
            _ => panic!("unknown material {s}"),
        }
    }
}

#[derive(Clone, Debug)]
struct Robot {
    inputs: Qtys,
}

#[derive(Debug)]
struct Blueprint {
    id: ID,
    robots: Vec<Robot>,
}

impl From<String> for Blueprint {
    fn from(line: String) -> Self {
        let id = line
            .split_ascii_whitespace()
            .nth(1)
            .expect("must have id")
            .strip_suffix(":")
            .expect("ID ends with :")
            .parse::<ID>()
            .expect("ID must be numeric");

        let re = Regex::new(r"Each (\w+) robot costs (.*?)\.").expect("regex creation failed");

        let robots = re
            .captures_iter(&line)
            .map(|definition| {
                let mut inputs = make_qtys();

                definition[2].split(" and ").for_each(|each| {
                    let (qty, mat) = each
                        .split_ascii_whitespace()
                        .collect_tuple()
                        .expect("must have qty and material");

                    let mat = Material::from(String::from(mat));
                    let qty = qty.parse::<Qty>().expect("qty must be numeric");

                    inputs[mat as usize] += qty;
                });

                Robot { inputs }
            })
            .collect::<Vec<_>>();

        Blueprint { id, robots }
    }
}

impl Blueprint {
    fn solve(&self, max_time: usize) -> (u16, Vec<Option<u16>>) {
        #[derive(Clone, Debug)]
        struct State {
            time: u16,
            inventory: Qtys,
            population: Qtys,
            builds: Vec<Option<ID>>,
        }

        let mut queue = Vec::new();

        // Generate the initial state, no inventory but one of each material
        let inventory = make_qtys();
        let mut population = make_qtys();
        population[0] = 1;
        queue.push(State {
            time: max_time as Qty,
            inventory,
            population,
            builds: Vec::new(),
        });

        // Best case is # of geodes + the build order to get there
        let mut best = (0 as ID, Vec::new());

        // Analytics data
        let mut count = 0;
        let mut skip_count = 0;

        let start = Instant::now();
        let mut tick = start;

        while !queue.is_empty() {
            let State {
                time,
                inventory,
                population,
                builds,
            } = queue.pop().unwrap();
            count += 1;

            if builds.len() > max_time {
                panic!();
            }

            if cfg!(debug_assertions) {
                if tick.elapsed().as_secs_f32() > 1.0 {
                    println!(
                        "[{}s] (q: {}, count: {count}, skip={skip_count}): time={time}, inventory={inventory:?}, population={population:?}, builds={}",
                        start.elapsed().as_secs(),
                        queue.len(),
                        builds.iter().map(|el| if let Some(v) = el { v.to_string() } else { String::from("_") }).join(",")
                    );
                    tick = Instant::now();
                }
            }

            let geode_qty = inventory[Material::Geode as usize];
            if geode_qty > best.0 {
                if cfg!(debug_assertions) {
                    println!(
                        "[{}s] [NEW BEST={geode_qty}] (q={}, count={count}, skip={skip_count}): time={time}, inventory={inventory:?}, population={population:?}, builds={}",
                        start.elapsed().as_secs(),
                        queue.len(),
                        builds.iter().map(|el| if let Some(v) = el { v.to_string() } else { String::from("_") }).join(",")
                    );
                }
                best = (geode_qty, builds.clone());
            }

            if time == 0 {
                continue;
            }

            // Best case: build a new geode robot each frame (ignore inputs)
            let best_case_geodes =
                geode_qty + population[Material::Geode as usize] * time + time * (time + 1) / 2;
            if best_case_geodes < best.0 {
                skip_count += 1;
                continue;
            }

            // For each kind of robot, try to build it next
            for (id, robot) in self.robots.iter().enumerate() {
                // It's impossible to build, we don't make the right resources
                if robot
                    .inputs
                    .iter()
                    .enumerate()
                    .any(|(input_id, input_qty)| *input_qty > 0 && population[input_id] == 0)
                {
                    continue;
                }

                // When is the next time we'll have enough inputs to build it?
                let ticks = robot
                    .inputs
                    .iter()
                    .enumerate()
                    .map(|(input_id, input_qty)| {
                        if inventory[input_id] >= *input_qty {
                            0
                        } else {
                            ((*input_qty - inventory[input_id]) as f32
                                / population[input_id] as f32)
                                .ceil() as Qty
                        }
                    })
                    .max()
                    .unwrap()
                    + 1;

                // If it won't be done in time, don't try to
                if ticks > time {
                    continue;
                }

                // Update inventory for those ticks - this build
                let mut new_inventory = inventory.clone();

                population
                    .iter()
                    .enumerate()
                    .for_each(|(id, qty)| new_inventory[id] += *qty * ticks);

                self.robots[id]
                    .inputs
                    .iter()
                    .enumerate()
                    .for_each(|(id, qty)| new_inventory[id] -= *qty);

                // Update the population with the new robot
                let mut new_population = population.clone();
                new_population[id] += 1;

                // Update the steps with the number of skips + the build
                let mut new_builds = builds.clone();
                for _ in 0..(ticks - 1) {
                    new_builds.push(None);
                }
                new_builds.push(Some(id as ID));

                // Add to queue
                queue.push(State {
                    time: time - ticks,
                    inventory: new_inventory,
                    population: new_population,
                    builds: new_builds,
                });
            }
        }

        best
    }
}

fn part1(filename: &Path) -> String {
    let blueprints = iter_lines(filename)
        .map(Blueprint::from)
        .collect::<Vec<_>>();

    let mut total_quality = 0;

    for blueprint in blueprints.into_iter() {
        let (geode_count, steps) = blueprint.solve(24);

        if cfg!(debug_assertions) {
            println!(
                "===== Blueprint {} done, {geode_count} geodes, steps: {}",
                blueprint.id,
                steps
                    .iter()
                    .map(|el| if let Some(v) = el {
                        v.to_string()
                    } else {
                        String::from("_")
                    })
                    .join(",")
            );
        }

        total_quality += blueprint.id * geode_count;
    }

    total_quality.to_string()
}

fn part2(filename: &Path) -> String {
    let blueprints = iter_lines(filename)
        .map(Blueprint::from)
        .take(3) // Only keep the first 3 blueprints
        .collect::<Vec<_>>();

    let mut quality_product = 1;

    for blueprint in blueprints.into_iter() {
        let (geode_count, steps) = blueprint.solve(32);

        if cfg!(debug_assertions) {
            println!(
                "===== Blueprint {} done, {geode_count} geodes, steps: {}",
                blueprint.id,
                steps
                    .iter()
                    .map(|el| if let Some(v) = el {
                        v.to_string()
                    } else {
                        String::from("_")
                    })
                    .join(",")
            );
        }

        quality_product *= geode_count;
    }

    quality_product.to_string()
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
        aoc_test("19", part1, "1092")
    }

    #[test]
    fn test2() {
        aoc_test("19", part2, "3542")
    }
}
