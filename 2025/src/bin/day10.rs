use rayon::prelude::*;
use std::collections::{HashSet, VecDeque};
use std::io::Write;
use std::sync::{Arc, Mutex};
use tempfile::NamedTempFile;

aoc::main!(day10);

#[derive(Debug)]
struct Machine {
    // Cache the size of machine
    size: usize,
    // The target states of lights, true means light should be turned on
    lights: Vec<bool>,
    // Sets of buttons, each button toggles the given index of wires
    // So if buttons[0] is [3, 4, 5], button 0 will toggle 3, 4, and 5
    buttons: Vec<Vec<usize>>,
    // Target joltage requirements
    joltage: Vec<usize>,
}

impl From<&str> for Machine {
    fn from(value: &str) -> Self {
        let parts = value.split_ascii_whitespace().collect::<Vec<_>>();

        let light_part = parts[0];
        let lights = light_part[1..light_part.len() - 1]
            .chars()
            .map(|c| c == '#')
            .collect::<Vec<_>>();

        let size = lights.len();

        let button_parts = &parts[1..parts.len() - 1];
        let buttons = button_parts
            .iter()
            .map(|button| {
                button[1..button.len() - 1]
                    .split(',')
                    .map(|v| v.parse::<usize>().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let joltage_part = parts[parts.len() - 1];
        let joltage = joltage_part[1..joltage_part.len() - 1]
            .split(',')
            .map(|v| v.parse::<usize>().unwrap())
            .collect::<Vec<_>>();

        Self {
            size,
            lights,
            buttons,
            joltage,
        }
    }
}

impl Machine {
    fn solve_lights(&self) -> usize {
        log::info!("Working on {self:?}");

        let mut queue = VecDeque::new();
        queue.push_back((0, vec![false; self.size]));

        while let Some((presses, lights)) = queue.pop_front() {
            log::debug!("[{presses}, {}] {lights:?}", queue.len());

            if lights == self.lights {
                log::info!("Found solution {presses}");
                return presses;
            }

            for button in self.buttons.iter() {
                let new_lights = lights
                    .iter()
                    .enumerate()
                    .map(|(i, on)| if button.contains(&i) { !on } else { *on })
                    .collect::<Vec<_>>();

                queue.push_back((presses + 1, new_lights));
            }
        }

        unreachable!("no solution found");
    }
}

#[aoc::register]
fn part1(input: &str) -> impl Into<String> {
    input
        .lines()
        .map(Machine::from)
        .map(|m| m.solve_lights())
        .sum::<usize>()
        .to_string()
}

#[aoc::register]
fn part1_rayon(input: &str) -> impl Into<String> {
    input
        .lines()
        .map(Machine::from)
        .par_bridge()
        .map(|m| m.solve_lights())
        .sum::<usize>()
        .to_string()
}

impl Machine {
    fn solve_joltage(&self) -> usize {
        log::info!("Working on {self:?}");

        let mut queue = VecDeque::new();
        queue.push_back((0, vec![0; self.size]));

        while let Some((presses, joltages)) = queue.pop_front() {
            log::debug!("[{presses}, {}] {joltages:?}", queue.len());

            if joltages == self.joltage {
                log::info!("Found solution {presses}");
                return presses;
            }

            if joltages
                .iter()
                .zip(self.joltage.iter())
                .any(|(current, target)| current > target)
            {
                log::debug!("OVER JOLTAGE!");
                continue;
            }

            for button in self.buttons.iter() {
                let new_joltages = joltages
                    .iter()
                    .enumerate()
                    .map(|(i, v)| if button.contains(&i) { v + 1 } else { *v })
                    .collect::<Vec<_>>();

                queue.push_back((presses + 1, new_joltages));
            }
        }

        unreachable!("no solution found");
    }
}

#[aoc::register]
fn part2(input: &str) -> impl Into<String> {
    input
        .lines()
        .map(Machine::from)
        .map(|m| m.solve_joltage())
        .sum::<usize>()
        .to_string()
}

#[aoc::register]
fn part2_rayon(input: &str) -> impl Into<String> {
    input
        .lines()
        .map(Machine::from)
        .par_bridge()
        .map(|m| m.solve_joltage())
        .sum::<usize>()
        .to_string()
}

impl Machine {
    fn solve_joltage_z3(&self) -> usize {
        log::info!("Working on {self:?}");

        let mut buffer = String::new();

        buffer.push_str("(set-option :produce-models true)\n");
        buffer.push_str("(set-logic QF_LIA)\n");

        // Our variables are how many times each button is pressed
        for i in 0..self.buttons.len() {
            buffer.push_str(&format!("(declare-const p{i} Int)\n"));
            buffer.push_str(&format!("(assert (>= p{i} 0))\n"));
        }

        // Each joltage has to end up exactly matching pi * if button[i] contains it
        for idx in 0..self.size {
            let mut terms: Vec<String> = vec![];
            for (bi, button) in self.buttons.iter().enumerate() {
                if button.contains(&idx) {
                    terms.push(format!("p{bi}"));
                }
            }
            let sum = if terms.is_empty() {
                "0".to_string()
            } else {
                format!("(+ {})", terms.join(" "))
            };
            buffer.push_str(&format!("(assert (= {} {}))\n", sum, self.joltage[idx]));
        }

        // Our objective is to minimize the total presses
        let total = (0..self.buttons.len())
            .map(|i| format!("p{i}"))
            .collect::<Vec<_>>()
            .join(" ");

        buffer.push_str(&format!("(minimize (+ {total}))\n"));

        buffer.push_str("(check-sat)\n");
        buffer.push_str("(get-model)\n");

        log::debug!("Z3 input:\n{buffer}");

        let f = {
            let mut f = NamedTempFile::new().expect("failed to create temp file for z3");
            f.write_all(buffer.as_bytes())
                .expect("failed to write z3 input to temp file");
            f
        };

        // Now call Z3 on it
        let output = std::process::Command::new("z3")
            .arg(f.path().to_str().unwrap())
            .output()
            .expect("failed to execute z3");

        // Parse output
        // sat
        // (
        //   (define-fun p5 () Int
        //     0)
        // ...
        let stdout = String::from_utf8_lossy(&output.stdout);
        let mut total_presses = 0;

        log::debug!("Z3 output:\n{stdout}");

        let mut lines = stdout.lines();
        while let Some(line) = lines.next() {
            if line.contains("(define-fun") {
                let next_line = lines.next().unwrap();
                let presses: usize = next_line
                    .trim()
                    .trim_end_matches(')')
                    .parse()
                    .expect("failed to parse presses");

                total_presses += presses;
            }
        }
        log::info!("Found solution: {total_presses}");

        total_presses
    }
}

#[aoc::register]
fn part2_z3(input: &str) -> impl Into<String> {
    input
        .lines()
        .map(Machine::from)
        .map(|m| m.solve_joltage_z3())
        .sum::<usize>()
        .to_string()
}

#[aoc::register]
fn part2_z3_rayon(input: &str) -> impl Into<String> {
    input
        .lines()
        .map(Machine::from)
        .par_bridge()
        .map(|m| m.solve_joltage_z3())
        .sum::<usize>()
        .to_string()
}

// System of equations approach

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Equation {
    constant: isize,
    coefficients: Vec<isize>,
}

impl std::ops::Add for Equation {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Equation {
            constant: self.constant + other.constant,
            coefficients: {
                let mut result = vec![0; self.coefficients.len()];
                for i in 0..self.coefficients.len() {
                    result[i] = self.coefficients[i] + other.coefficients[i];
                }
                result
            },
        }
        .reduced()
    }
}

impl Equation {
    fn negated(&self) -> Self {
        Equation {
            constant: -self.constant,
            coefficients: self.coefficients.iter().map(|&c| -c).collect::<Vec<_>>(),
        }
    }

    fn reduced(&self) -> Self {
        let gcd = self
            .coefficients
            .iter()
            .cloned()
            .filter(|&c| c != 0)
            .fold(0, num::integer::gcd);
        let reduced = if gcd == 0 || gcd == 1 {
            self.clone()
        } else {
            Equation {
                constant: self.constant / gcd,
                coefficients: self
                    .coefficients
                    .iter()
                    .map(|&c| c / gcd)
                    .collect::<Vec<_>>(),
            }
        };

        if reduced.constant < 0 {
            reduced.negated()
        } else {
            reduced
        }
    }
}

impl std::fmt::Display for Equation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let terms: Vec<String> = self
            .coefficients
            .iter()
            .enumerate()
            .filter(|(_, coef)| **coef != 0)
            .map(|(i, &coef)| format!("{coef} * x{i}"))
            .collect();
        write!(f, "{} = {}", terms.join(" + "), self.constant)
    }
}

impl Machine {
    fn solve_joltage_eqn(&self) -> usize {
        log::info!("Working on {self:?}");

        let mut equations: HashSet<Equation> = HashSet::new();
        for idx in 0..self.size {
            let mut coefficients = vec![0; self.buttons.len()];
            for (bi, button) in self.buttons.iter().enumerate() {
                if button.contains(&idx) {
                    coefficients[bi] = 1;
                }
            }
            equations.insert(Equation {
                constant: self.joltage[idx] as isize,
                coefficients,
            });
        }

        for eq in &equations {
            log::info!("Equation: {eq}");
        }

        let mut known_values = vec![None; self.buttons.len()];

        for _i in 0.. {
            if _i >= 3 {
                break; // DEBUG
            }
            println!("Expanding equations, iter={_i}, count={}", equations.len());

            let initial_equations = equations.clone();
            let initial_size = equations.len();

            for eq1 in initial_equations.iter() {
                for eq2 in initial_equations.iter() {
                    if eq1 != eq2 {
                        equations.insert(eq1.clone() + eq2.clone());
                        equations.insert(eq1.clone().negated() + eq2.clone());
                        equations.insert(eq2.clone().negated() + eq1.clone());
                    }
                }
            }
            if equations.len() == initial_size {
                break;
            }

            // Look for any single-variable equations
            let single_var_eqs: Vec<Equation> = equations
                .iter()
                .filter(|eq| eq.coefficients.iter().filter(|&&c| c != 0).count() == 1)
                .cloned()
                .collect();

            println!("Found {} single-variable equations", single_var_eqs.len());
            for eq in single_var_eqs.iter() {
                println!("  {eq}");
            }

            // Record known values
            for eq in single_var_eqs.iter() {
                let xi = eq
                    .coefficients
                    .iter()
                    .position(|&c| c != 0)
                    .unwrap();

                known_values[xi] = Some(eq.constant as usize);
            }
            println!("Known values so far: {:?}", known_values);


            // Remove all equations that use only known values
            equations = equations
                .into_iter()
                .filter(|eq| {
                    eq.coefficients
                        .iter()
                        .enumerate()
                        .any(|(i, &c)| c != 0 && known_values[i].is_none())
                })
                .collect();

            // Look for any equations with exactly two values and constant = 0
            let two_var_zero_eqs: Vec<Equation> = equations
                .iter()
                .filter(|eq| eq.coefficients.iter().filter(|&&c| c != 0).count() == 2)
                .cloned()
                .collect();

            println!("Found {} two-variable", two_var_zero_eqs.len());
            for eq in two_var_zero_eqs {
                println!("  {eq}");
            }
        }

        panic!()
    }
}

#[aoc::register]
fn part2_eqn(input: &str) -> impl Into<String> {
    input
        .lines()
        .map(Machine::from)
        .map(|m| m.solve_joltage_eqn())
        .sum::<usize>()
        .to_string()
}

impl Machine {
    fn solve_joltage_memo(&self) -> usize {
        log::info!("Working on {self:?}");

        let mut memo = std::collections::HashMap::new();
        let mut stats = (0usize, 0usize); // (hits, misses)

        fn helper(
            machine: &Machine,
            current: Vec<usize>,
            memo: &mut std::collections::HashMap<Vec<usize>, usize>,
            stats: &mut (usize, usize),
        ) -> usize {
            if let Some(&result) = memo.get(&current) {
                stats.0 += 1;
                return result;
            } else {
                stats.1 += 1;
            }

            if current == machine.joltage {
                return 0;
            }

            let mut min_presses = usize::MAX;

            // Find the joltage that is impacted by the fewest buttons taking into account current state
            // Break ties by highest remaining joltage
            let mut best_joltage_idx = None;
            let mut best_button_count = usize::MAX;
            let mut best_remaining_joltage = 0usize;
            for idx in 0..machine.size {
                // Skip any joltages that are already at target
                if current[idx] >= machine.joltage[idx] {
                    continue;
                }

                // Count how many buttons contribute to this joltage
                let button_count = machine
                    .buttons
                    .iter()
                    .filter(|button| button.contains(&idx))
                    .count();

                let remaining_joltage = machine.joltage[idx] - current[idx];
                if button_count < best_button_count
                    || (button_count == best_button_count
                        && remaining_joltage > best_remaining_joltage)
                {
                    best_button_count = button_count;
                    best_remaining_joltage = remaining_joltage;
                    best_joltage_idx = Some(idx);
                }
            }

            // Try pressing each button that affects that joltage only
            // This is still guaranteed to eventually find an optimal solution since all joltages must jolt
            for button in machine.buttons.iter() {
                if let Some(joltage_idx) = best_joltage_idx
                    && !button.contains(&joltage_idx)
                {
                    continue;
                }

                let new_joltages = current
                    .iter()
                    .enumerate()
                    .map(|(i, v)| if button.contains(&i) { v + 1 } else { *v })
                    .collect::<Vec<_>>();

                // Don't recur into cases that put any joltage over joltage
                if new_joltages
                    .iter()
                    .zip(machine.joltage.iter())
                    .any(|(current, target)| current > target)
                {
                    continue;
                }

                let recur_presses = helper(machine, new_joltages, memo, stats);
                if recur_presses != usize::MAX {
                    min_presses = min_presses.min(recur_presses + 1);
                }
            }

            memo.insert(current.clone(), min_presses);
            min_presses
        }

        let result = helper(self, vec![0; self.size], &mut memo, &mut stats);
        log::info!("Found solution: {result}");
        log::info!("Memo stats: hits={}, misses={}", stats.0, stats.1);
        result
    }
}

#[aoc::register]
fn part2_memo(input: &str) -> impl Into<String> {
    input
        .lines()
        .map(Machine::from)
        .map(|m| m.solve_joltage_memo())
        .sum::<usize>()
        .to_string()
}

#[aoc::register]
fn part2_memo_rayon(input: &str) -> impl Into<String> {
    input
        .lines()
        .map(Machine::from)
        .par_bridge()
        .map(|m| m.solve_joltage_memo())
        .sum::<usize>()
        .to_string()
}

impl Machine {
    fn solve_joltage_astar(&self) -> usize {
        log::info!("Working on {self:?}");

        #[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
        struct State {
            presses: usize,
            joltages: Vec<usize>,
        }

        let mut heap = std::collections::BinaryHeap::new();
        heap.push(std::cmp::Reverse(State {
            presses: 0,
            joltages: vec![0; self.size],
        }));

        let mut visited: HashSet<Vec<usize>> = HashSet::new();

        while let Some(std::cmp::Reverse(state)) = heap.pop() {
            log::debug!("[{}, {}] {:?}", state.presses, heap.len(), state.joltages);

            if state.joltages == self.joltage {
                log::info!("Found solution {}", state.presses);
                return state.presses;
            }

            if visited.contains(&state.joltages) {
                continue;
            }
            visited.insert(state.joltages.clone());

            for button in self.buttons.iter() {
                let new_joltages = state
                    .joltages
                    .iter()
                    .enumerate()
                    .map(|(i, v)| if button.contains(&i) { v + 1 } else { *v })
                    .collect::<Vec<_>>();

                if new_joltages
                    .iter()
                    .zip(self.joltage.iter())
                    .any(|(current, target)| current > target)
                {
                    continue;
                }

                heap.push(std::cmp::Reverse(State {
                    presses: state.presses + 1,
                    joltages: new_joltages,
                }));
            }
        }

        unreachable!("no solution found");
    }
}

#[aoc::register]
fn part2_astar(input: &str) -> impl Into<String> {
    input
        .lines()
        .map(Machine::from)
        .map(|m| m.solve_joltage_astar())
        .sum::<usize>()
        .to_string()
}

impl Machine {
    fn solve_joltage_branch_and_bound(&self) -> usize {
        log::info!("Working on {self:?}");
        let start = std::time::Instant::now();

        let mut best_solution = usize::MAX;
        let mut memo = std::collections::HashMap::new();
        let mut stats = (0usize, 0usize); // (hits, misses)

        fn branch_and_bound_recursive(
            machine: &Machine,
            presses: usize,
            joltages: Vec<usize>,
            best_solution: &mut usize,
            memo: &mut std::collections::HashMap<Vec<usize>, Option<usize>>,
            stats: &mut (usize, usize),
        ) {
            // Check memoization first
            if let Some(cached) = memo.get(&joltages) {
                stats.0 += 1;
                match cached {
                    Some(min_remaining) => {
                        let total = presses + min_remaining;
                        if total < *best_solution {
                            *best_solution = total;
                        }
                    }
                    None => {
                        // State is infeasible
                    }
                }
                return;
            }
            stats.1 += 1;

            // Pruning: if current presses >= best known solution, stop
            if presses >= *best_solution {
                memo.insert(joltages, None);
                return;
            }

            // Check if we've found a solution
            if joltages == machine.joltage {
                *best_solution = (*best_solution).min(presses);
                memo.insert(joltages, Some(0));
                return;
            }

            // Check if we've exceeded the target (infeasible)
            if joltages
                .iter()
                .zip(machine.joltage.iter())
                .any(|(current, target)| current > target)
            {
                memo.insert(joltages, None);
                return;
            }

            // Estimate lower bound for remaining presses
            let remaining = machine
                .joltage
                .iter()
                .zip(joltages.iter())
                .map(|(target, current)| target.saturating_sub(*current))
                .max()
                .unwrap_or(0);

            // Pruning: if lower bound + current presses >= best solution, stop
            if presses + remaining >= *best_solution {
                memo.insert(joltages, None);
                return;
            }

            // Find the joltage with fewest buttons affecting it (and still needs to reach target)
            // This optimization significantly reduces the search space
            let mut best_joltage_idx = None;
            let mut best_button_count = usize::MAX;
            let mut best_remaining_joltage = 0usize;

            for idx in 0..machine.size {
                // Skip any joltages that are already at target
                if joltages[idx] >= machine.joltage[idx] {
                    continue;
                }

                // Count how many buttons contribute to this joltage
                let button_count = machine
                    .buttons
                    .iter()
                    .filter(|button| button.contains(&idx))
                    .count();

                let remaining_joltage = machine.joltage[idx] - joltages[idx];
                if button_count < best_button_count
                    || (button_count == best_button_count
                        && remaining_joltage > best_remaining_joltage)
                {
                    best_button_count = button_count;
                    best_remaining_joltage = remaining_joltage;
                    best_joltage_idx = Some(idx);
                }
            }

            let mut min_remaining_presses = usize::MAX;

            // Try pressing each button that affects the chosen joltage
            // This constrains the search to only promising branches
            for button in machine.buttons.iter() {
                if let Some(joltage_idx) = best_joltage_idx
                    && !button.contains(&joltage_idx)
                {
                    continue;
                }

                let new_joltages = joltages
                    .iter()
                    .enumerate()
                    .map(|(i, v)| if button.contains(&i) { v + 1 } else { *v })
                    .collect::<Vec<_>>();

                let initial_best = *best_solution;
                branch_and_bound_recursive(
                    machine,
                    presses + 1,
                    new_joltages,
                    best_solution,
                    memo,
                    stats,
                );

                // Track minimum remaining presses from this state
                if *best_solution != initial_best {
                    min_remaining_presses = min_remaining_presses.min(*best_solution - presses - 1);
                }
            }

            // Memoize the minimum remaining presses from this state
            if min_remaining_presses != usize::MAX {
                memo.insert(joltages, Some(min_remaining_presses));
            } else {
                memo.insert(joltages, None);
            }
        }

        branch_and_bound_recursive(
            self,
            0,
            vec![0; self.size],
            &mut best_solution,
            &mut memo,
            &mut stats,
        );

        log::info!("Found solution: {best_solution}");
        log::info!("Memo stats: hits={}, misses={}", stats.0, stats.1);
        log::info!("Elapsed time: {:.2?}", start.elapsed());

        if best_solution == usize::MAX {
            unreachable!("no solution found");
        }
        best_solution
    }
}

#[aoc::register]
fn part2_branch_and_bound(input: &str) -> impl Into<String> {
    input
        .lines()
        .map(Machine::from)
        .map(|m| m.solve_joltage_branch_and_bound())
        .sum::<usize>()
        .to_string()
}

#[aoc::register]
fn part2_branch_and_bound_rayon(input: &str) -> impl Into<String> {
    let count = input.lines().count();
    let finished = Arc::new(Mutex::new(0));
    let start = std::time::Instant::now();

    input
        .lines()
        .map(Machine::from)
        .par_bridge()
        .map(|m| {
            let result = m.solve_joltage_branch_and_bound();

            let mut finished = finished.lock().unwrap();
            *finished += 1;

            log::info!(
                "Completed {}/{} machines after {:.2?}",
                *finished,
                count,
                start.elapsed()
            );

            result
        })
        .sum::<usize>()
        .to_string()
}

aoc::test!(
    text = "\
[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
", 
    [part1] => "7",
    [part2, part2_rayon, part2_memo, part2_memo_rayon, part2_astar, part2_branch_and_bound] => "33"
);

aoc::test!(
    file = "input/2025/day10.txt",
    [part1] => "452",
    [part2_z3_rayon] => "17424"
);
