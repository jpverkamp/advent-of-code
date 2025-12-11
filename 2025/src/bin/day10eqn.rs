use std::{
    collections::HashSet,
    sync::{Arc, Mutex},
    usize,
};

use rayon::prelude::*;

aoc::main!(day10);

#[derive(Debug)]
struct Machine {
    // The ID of the machine (for debugging)
    id: usize,
    // Cache the size of machine
    size: usize,
    // Sets of buttons, each button toggles the given index of wires
    // So if buttons[0] is [3, 4, 5], button 0 will toggle 3, 4, and 5
    buttons: Vec<Vec<usize>>,
    // Target joltage requirements
    joltage: Vec<usize>,
}

impl From<(usize, &str)> for Machine {
    fn from(value: (usize, &str)) -> Self {
        let (id, s) = value;
        let parts = s.split_ascii_whitespace().collect::<Vec<_>>();

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

        let size = joltage.len();

        Self {
            id,
            size,
            buttons,
            joltage,
        }
    }
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

    fn apply(&self, values: &Vec<Option<usize>>) -> Self {
        let mut new_constant = self.constant;
        let mut new_coefficients = self.coefficients.clone();

        for (i, &value_opt) in values.iter().enumerate() {
            if let Some(value) = value_opt {
                new_constant -= new_coefficients[i] * (value as isize);
                new_coefficients[i] = 0;
            }
        }

        Equation {
            constant: new_constant,
            coefficients: new_coefficients,
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

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Bound {
    Unknown,
    Bounded(isize, isize),
    Known(isize),
}

impl Machine {
    #[tracing::instrument(skip(self))]
    fn solve_joltage_eqn(&self) -> usize {
        let machine_id = self.id;
        log::info!("[{machine_id}] Working on {self:?}");

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
            log::debug!("[{machine_id}] Equation: {eq}");
        }

        let mut previous_possibilities = 0;
        let mut bounds = vec![Bound::Unknown; self.buttons.len()];

        for _i in 0.. {
            if _i >= 3 {
                break; // DEBUG
            }
            tracing::info!(
                "[{machine_id}] Expanding equations, iter={_i}, count={}",
                equations.len()
            );

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

            tracing::info!(
                "[{machine_id}] Found {} single-variable equations",
                single_var_eqs.len()
            );
            for eq in single_var_eqs.iter() {
                tracing::info!("  {eq}");
            }

            // Record known values
            for eq in single_var_eqs.iter() {
                let xi = eq.coefficients.iter().position(|&c| c != 0).unwrap();

                assert!(eq.constant % eq.coefficients[xi] == 0);
                bounds[xi] = Bound::Known(eq.constant / eq.coefficients[xi]);
            }
            tracing::info!("[{machine_id}] Known values so far: {bounds:?}");

            // Apply known values to all equations
            let known_values = bounds
                .iter()
                .map(|b| match b {
                    Bound::Known(v) => Some(*v as usize),
                    _ => None,
                })
                .collect::<Vec<_>>();
            let updated_equations: HashSet<Equation> =
                equations.iter().map(|eq| eq.apply(&known_values)).collect();
            equations = updated_equations;

            // For any equation with only positive co-efficients, we can set bounds
            // Assume that we put as much as possible into each variable for bound
            for eq in equations.iter() {
                // All negative is all positive, just... opposite!
                let eq = if eq.coefficients.iter().all(|&c| c <= 0) {
                    eq.negated()
                } else {
                    eq.clone()
                };

                if !eq.coefficients.iter().all(|&c| c >= 0) {
                    continue;
                }

                if eq.constant <= 0 {
                    continue;
                }

                for (i, coef) in eq.coefficients.iter().enumerate() {
                    if *coef == 0 {
                        continue;
                    }

                    let max_times = eq.constant / *coef;
                    match &bounds[i] {
                        // Previously unknown bounds now have a maximum value
                        Bound::Unknown => {
                            bounds[i] = Bound::Bounded(0, max_times);
                        }
                        // If this sets a better upper bound, yay?
                        Bound::Bounded(l, u) => {
                            if *u > max_times {
                                bounds[i] = Bound::Bounded(*l, max_times);
                            }
                        }
                        // Known bounds don't need to change
                        // Hopefully this doesn't prove our bounds are wrong :smile:
                        Bound::Known(_) => {}
                    };
                }
            }

            // Estimate how much work we have to do
            if !bounds.iter().any(|b| matches!(b, Bound::Unknown)) {
                let total_possibilities = bounds
                    .iter()
                    .map(|b| match b {
                        Bound::Known(_) => 1usize,
                        Bound::Bounded(l, u) => (u - l + 1) as usize,
                        Bound::Unknown => unreachable!("Should not have unknown here"),
                    })
                    .product::<usize>();
                log::debug!(
                    "[{machine_id}] Estimated total possibilities: {total_possibilities}"
                );

                if previous_possibilities == total_possibilities {
                    tracing::debug!("[{machine_id}] No change in possibilities, stopping");
                    break;
                }
                previous_possibilities = total_possibilities;
            }

            // Remove all equations that still depend on known variables
            // Because of the apply step above, these should be zeroed out
            let filtered_equations: HashSet<Equation> = equations
                .iter()
                .filter(|eq| {
                    eq.coefficients
                        .iter()
                        .enumerate()
                        .all(|(i, &c)| !(c != 0 && matches!(bounds[i], Bound::Known(_))))
                })
                .cloned()
                .collect();
            equations = filtered_equations;

            // Remove all equations with many variables, just cause we have too many
            // let filtered_equations: HashSet<Equation> = equations
            //     .iter()
            //     .filter(|eq| eq.coefficients.iter().filter(|&&c| c != 0).count() <= 4)
            //     .cloned()
            //     .collect();
            // equations = filtered_equations;
        }

        // Now, we have as many bounds as we can get
        // So from here, we want to solve with a recursive memoized approach
        log::info!("[{machine_id}] Final bounds: {bounds:?}");

        #[tracing::instrument(skip(machine, bounds, equations))]
        fn helper(
            machine: &Machine,
            presses: &Vec<Option<usize>>,
            bounds: &Vec<Bound>,
            equations: &HashSet<Equation>,
        ) -> Option<usize> {
            let machine_id = machine.id;

            // If the currently known presses make for an impossible voltage, fail
            // If we went beyond the bounds without finding an answer, fail
            let mut current = vec![0; machine.size];
            for (press, button) in presses.iter().zip(machine.buttons.iter()) {
                if let Some(p) = press {
                    for b in button {
                        current[*b] += p;
                    }
                }
            }

            log::debug!(
                "[{machine_id}] helper({presses:?}) => {current:?} vs {:?}",
                machine.joltage
            );

            // If we have exactly the right current, this is the correct solution
            if current == machine.joltage {
                log::debug!("[{machine_id}] Found an answer!: {presses:?}");
                let press_total = presses.iter().map(|v| v.unwrap_or(0)).sum::<usize>();
                return Some(press_total);
            }

            if current
                .iter()
                .zip(machine.joltage.iter())
                .any(|(c, j)| c > j)
            {
                log::debug!("[{machine_id}] OVER JOLTAGE");
                return None;
            }

            // If we made it this far and all presses are set, this solution is under joltage
            if presses.iter().all(|f| f.is_some()) {
                log::debug!("[{machine_id}] under joltage :(");
                return None;
            }

            // If we have any equation where all but 1 variable is known, we can set the last one
            for eq in equations.iter() {
                let unknown_vars: Vec<usize> = eq
                    .coefficients
                    .iter()
                    .enumerate()
                    .filter(|(i, _)| presses[*i].is_none() && eq.coefficients[*i] != 0)
                    .map(|(i, _)| i)
                    .collect();

                if unknown_vars.len() == 1 {
                    let xi = unknown_vars[0];
                    let mut sum_known = 0isize;
                    for (i, &coef) in eq.coefficients.iter().enumerate() {
                        if i != xi
                            && let Some(p) = presses[i] {
                                sum_known += coef * (p as isize);
                            }
                    }

                    let required_xi = (eq.constant - sum_known) / eq.coefficients[xi];
                    if required_xi < 0 {
                        log::debug!(
                            "[{machine_id}] Negative press violation on equation {eq} with {presses:?}"
                        );
                        return None;
                    }

                    let mut new_presses = presses.clone();
                    new_presses[xi] = Some(required_xi as usize);
                    log::debug!(
                        "[{machine_id}] Applying equation {eq}, {presses:?} => {new_presses:?}"
                    );
                    return helper(
                        machine,
                        &new_presses,
                        bounds,
                        equations,
                    );
                }
            }

            // The current index is the unset press with the lowest range
            let mut current_idx = 0;
            let mut best_size = isize::MAX;
            for (i, p) in presses.iter().enumerate() {
                if p.is_some() {
                    continue;
                }

                let range_size = match bounds[i] {
                    Bound::Unknown => isize::MAX,
                    Bound::Bounded(lo, hi) => hi - lo + 1,
                    Bound::Known(_) => 1,
                };

                if range_size < best_size {
                    current_idx = i;
                    best_size = range_size;
                }
            }
            tracing::debug!("[{machine_id}] Selected {current_idx} as the next index");

            // Test each value at the current idx, finding the best recursive answer
            let mut best = None;

            match bounds[current_idx] {
                Bound::Unknown => {
                    // Not sure how I can end up here, but it's possible?
                    // Hopefully we'll eventually find an answer
                    for value in 0.. {
                        let mut next_presses = presses.clone();
                        next_presses[current_idx] = Some(value as usize);
                        let result = helper(
                            machine,
                            &next_presses,
                            bounds,
                            equations,
                        );

                        if best.is_none() {
                            best = result;
                        } else if result.is_some() {
                            best = Some(best.unwrap().min(result.unwrap()));
                        }
                    }
                }
                Bound::Bounded(lo, hi) => {
                    for value in lo..=hi {
                        let mut next_presses = presses.clone();
                        next_presses[current_idx] = Some(value as usize);
                        let result = helper(
                            machine,
                            &next_presses,
                            bounds,
                            equations,
                        );

                        if best.is_none() {
                            best = result;
                        } else if result.is_some() {
                            best = Some(best.unwrap().min(result.unwrap()));
                        }
                    }
                }
                Bound::Known(v) => {
                    let mut next_presses = presses.clone();
                    next_presses[current_idx] = Some(v as usize);
                    best = helper(
                        machine,
                        &next_presses,
                        bounds,
                        equations,
                    );
                }
            }

            best
        }

        let machine_id = self.id;

        tracing::info!("[{machine_id}] Starting recursive search");
        let result = helper(
            self,
            &vec![None; self.buttons.len()],
            &bounds,
            &equations,
        );

        log::info!("[{machine_id}] Found answer: {result:?}");
        result.expect("Didn't find an answer?")
    }
}

#[aoc::register]
pub fn part2_eqn(input: &str) -> impl Into<String> {
    input
        .lines()
        .enumerate()
        .map(Machine::from)
        .map(|m| m.solve_joltage_eqn())
        .sum::<usize>()
        .to_string()
}

#[aoc::register]
pub fn part2_eqn_rayon(input: &str) -> impl Into<String> {
    let count = input.lines().count();
    let finished = Arc::new(Mutex::new(0));
    let start = std::time::Instant::now();

    let slowest_time = Arc::new(Mutex::new(std::time::Duration::ZERO));
    let slowest_machine_id = Arc::new(Mutex::new(0));

    let result = input
        .lines()
        .enumerate()
        .map(Machine::from)
        .par_bridge()
        .map(|m| {
            let my_start = std::time::Instant::now();
            let result = m.solve_joltage_eqn();

            let mut finished = finished.lock().unwrap();
            *finished += 1;

            if my_start.elapsed() > *slowest_time.lock().unwrap() {
                let mut slowest_time_lock = slowest_time.lock().unwrap();
                let mut slowest_machine_id_lock = slowest_machine_id.lock().unwrap();
                *slowest_time_lock = my_start.elapsed();
                *slowest_machine_id_lock = m.id;
            }

            log::info!(
                "Completed {}/{} machines after {:.2?} ({:.2?} for machine {})",
                *finished,
                count,
                start.elapsed(),
                my_start.elapsed(),
                m.id
            );

            result
        })
        .sum::<usize>();

    log::info!("Total time: {:.2?}", start.elapsed());

    log::info!(
        "Slowest machine was {} taking {:.2?}",
        *slowest_machine_id.lock().unwrap(),
        *slowest_time.lock().unwrap()
    );

    result.to_string()
}
