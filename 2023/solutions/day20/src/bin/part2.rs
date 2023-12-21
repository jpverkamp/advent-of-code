use anyhow::Result;
use fxhash::FxHashMap;
use std::{collections::VecDeque, io};

use day20::{parse, types::*};

// #[aoc_test("data/test/20.txt", "no result")]
// #[aoc_test("data/20.txt", "240162699605221")]
fn main() -> Result<()> {
    env_logger::init();

    let stdin = io::stdin();
    let input = io::read_to_string(stdin.lock())?;
    let (s, mut modules) = parse::modules(&input).unwrap();
    assert_eq!(s.trim(), "");

    let mut state = modules
        .keys()
        .map(|label| (*label, Pulse::Low))
        .collect::<FxHashMap<_, _>>();

    // rx comes from exactly one node
    let targets = modules
        .iter()
        .filter(|(_, module)| module.outputs.iter().any(|output| *output == "rx"))
        .map(|(label, _)| *label)
        .collect::<Vec<_>>();

    // That node in turn comes from 4 that each turn on every so many frames
    let mut targets = modules
        .iter()
        .filter(|(_, module)| {
            module
                .outputs
                .iter()
                .any(|output| targets.iter().any(|target| target == output))
        })
        .map(|(label, _)| *label)
        .collect::<Vec<_>>();

    // Collect the lengths of those cycles
    let mut cycles = Vec::new();

    let mut push_i = 0;
    'simulation: loop {
        push_i += 1;
        log::info!("=== Push {push_i} ===");

        let mut queue = VecDeque::from(vec![("button", "broadcaster", Pulse::Low)]);

        while let Some((src, dst, pulse)) = queue.pop_front() {
            log::info!("{src} -{pulse:?}-> {dst}");

            if let Some(i) = targets.iter().position(|node| *node == dst) {
                if pulse == Pulse::Low {
                    targets.remove(i);
                    cycles.push(push_i as usize);
                }
            }

            if targets.is_empty() {
                break 'simulation;
            }

            let module = modules.get_mut(dst).unwrap();
            state.insert(dst, pulse);

            match module.module_type {
                // Broadcast modules send the received pulse to all outputs
                ModuleType::Broadcast => {
                    for output in &module.outputs {
                        queue.push_back((dst, *output, pulse));
                    }
                }
                // Flip-flops flip on low pulses
                // If it was off, it turns on and sends high
                // If it was on, it turns off and sends low
                ModuleType::FlipFlop(ref mut is_on) => {
                    if pulse == Pulse::Low {
                        let output_pulse = if *is_on { Pulse::Low } else { Pulse::High };
                        for output in &module.outputs {
                            queue.push_back((dst, *output, output_pulse));
                        }

                        *is_on = !*is_on;
                    }
                }
                // Conjunctions remember previous inputs
                // If all inputs are high, sends a low
                // Otherwise, send a high
                ModuleType::Conjunction(ref mut inputs) => {
                    inputs.insert(src, pulse);

                    let output_pulse = if inputs.values().all(|pulse| *pulse == Pulse::High) {
                        Pulse::Low
                    } else {
                        Pulse::High
                    };

                    for output in &module.outputs {
                        queue.push_back((dst, *output, output_pulse));
                    }
                }
                // Output modules do nothing
                ModuleType::Output => {}
            }
        }
    }

    log::info!("cycles: {cycles:?}");

    fn gcd(a: usize, b: usize) -> usize {
        if b == 0 {
            a
        } else {
            gcd(b, a % b)
        }
    }

    fn lcm(a: usize, b: usize) -> usize {
        a / gcd(a, b) * b
    }

    if let Some(result) = cycles.into_iter().reduce(lcm) {
        println!("{result}");
    } else {
        println!("no result");
    }

    Ok(())
}
