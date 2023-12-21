use crate::types::*;
use fxhash::FxHashMap;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, alpha1, line_ending, space0},
    combinator::map,
    multi::separated_list1,
    sequence::{delimited, pair, terminated},
    IResult,
};

fn module_type(input: &str) -> IResult<&str, ModuleType> {
    alt((
        map(complete::char('%'), |_| ModuleType::FlipFlop(false)),
        map(complete::char('&'), |_| {
            ModuleType::Conjunction(FxHashMap::default())
        }),
    ))(input)
}

fn broadcast_module(input: &str) -> IResult<&str, (ModuleType, &str)> {
    let (input, name) = tag("broadcaster")(input)?;
    Ok((input, (ModuleType::Broadcast, name)))
}

fn other_module(input: &str) -> IResult<&str, (ModuleType, &str)> {
    pair(module_type, alpha1)(input)
}

fn module(input: &str) -> IResult<&str, Module> {
    let (input, (module_type, label)) = alt((broadcast_module, other_module))(input)?;
    let (input, _) = delimited(space0, tag("->"), space0)(input)?;
    let (input, outputs) = separated_list1(terminated(complete::char(','), space0), alpha1)(input)?;

    Ok((
        input,
        Module {
            label,
            module_type,
            outputs,
        },
    ))
}

pub fn modules(input: &str) -> IResult<&str, FxHashMap<&str, Module>> {
    let (input, modules) = separated_list1(line_ending, module)(input)?;

    let mut modules = modules
        .iter()
        .map(|module| (module.label, module.clone()))
        .collect::<FxHashMap<_, _>>();

    let inputs = modules
        .iter()
        .flat_map(|(label, module)| module.outputs.iter().map(|output| (*output, *label)))
        .collect::<Vec<_>>();

    for (output, label) in inputs {
        if let Some(module) = modules.get_mut(output) {
            // Conjunctions need a reference back to their inputs
            if let ModuleType::Conjunction(ref mut inputs) = module.module_type {
                inputs.insert(label, Pulse::Low);
            }
        } else {
            // If the output doesn't exist, create it as an output module
            modules.insert(
                output,
                Module {
                    label: output,
                    module_type: ModuleType::Output,
                    outputs: vec![],
                },
            );
        }
    }

    Ok((input, modules))
}
