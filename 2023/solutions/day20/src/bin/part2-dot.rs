use std::io;

use day20::{parse, types::*};

fn main() {
    env_logger::init();

    let stdin = io::stdin();
    let input = io::read_to_string(stdin.lock()).expect("read input");

    let (s, modules) = parse::modules(&input).unwrap();
    assert_eq!(s.trim(), "");

    println!("digraph G {{");

    // Nodes with labels
    modules.iter().for_each(|(label, module)| {
        println!(
            "{}",
            match module.module_type {
                ModuleType::Broadcast => format!("  {label}"),
                ModuleType::FlipFlop(_) =>
                    format!("  {label} [label=\"%{label}\", color=\"blue\"];"),
                ModuleType::Conjunction(_) =>
                    format!("  {label} [label=\"&{label}\", color=\"green\"];"),
                ModuleType::Output => format!("  {label}"),
            }
        )
    });

    // Edges
    modules.iter().for_each(|(label, module)| {
        module
            .outputs
            .iter()
            .for_each(|output| println!("  {} -> {};", label, output));
    });

    println!("}}");
}
