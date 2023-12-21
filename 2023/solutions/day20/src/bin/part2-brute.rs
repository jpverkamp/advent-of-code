use anyhow::Result;
use fxhash::FxHashMap;
use std::{collections::VecDeque, io};

use day20::{parse, types::*};

// #[aoc_test("data/test/20.txt", "")]
// #[aoc_test("data/test/20b.txt", "")]
// #[aoc_test("data/20.txt", "")]
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

    let start = std::time::Instant::now();

    let mut push_i = 0;
    'simulation: loop {
        push_i += 1;
        log::info!("=== Push {push_i} ===");

        if push_i % 1_000_000 == 0 {
            println!("On push {push_i} after {:?}", start.elapsed());
        }

        let mut queue = VecDeque::from(vec![("button", "broadcaster", Pulse::Low)]);

        while let Some((src, dst, pulse)) = queue.pop_front() {
            log::info!("{src} -{pulse:?}-> {dst}");

            if dst == "rx" && pulse == Pulse::Low {
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

    log::info!("   pushes: {push_i}");

    let result = push_i;

    println!("{result}");
    Ok(())
}
