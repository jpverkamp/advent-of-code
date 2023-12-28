use anyhow::Result;
use fxhash::FxHashMap;
use std::{collections::VecDeque, io};

use day20::{parse, types::*};

aoc_test::generate!{day20_part1_test_20 as "test/20.txt" => "32000000"}
aoc_test::generate!{day20_part1_test_20b as "test/20b.txt" => "11687500"}
aoc_test::generate!{day20_part1_20 as "20.txt" => "832957356"}

fn main() {
    env_logger::init();

    let stdin = io::stdin();
    let input = io::read_to_string(stdin.lock()).expect("read input");
    let result = process(input.as_str()).expect("no errors");
    println!("{}", result);
}

fn process(input: &str) -> Result<String> {
    let (s, mut modules) = parse::modules(input).unwrap();
    assert_eq!(s.trim(), "");

    let mut state = modules
        .keys()
        .map(|label| (*label, Pulse::Low))
        .collect::<FxHashMap<_, _>>();

    let mut low_sent = 0;
    let mut high_sent = 0;

    for push_i in 1..=1000 {
        log::info!("=== Push {push_i} ===");
        let mut queue = VecDeque::from(vec![("button", "broadcaster", Pulse::Low)]);

        while let Some((src, dst, pulse)) = queue.pop_front() {
            log::info!("{src} -{pulse:?}-> {dst}");

            match pulse {
                Pulse::Low => low_sent += 1,
                Pulse::High => high_sent += 1,
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

    log::info!(" low_sent: {low_sent}");
    log::info!("high_sent: {high_sent}");

    Ok((low_sent * high_sent).to_string())
}
