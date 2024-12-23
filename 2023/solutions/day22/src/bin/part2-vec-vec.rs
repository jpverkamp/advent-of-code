use anyhow::Result;
use std::io;

use day22::{parse, types::*};

aoc_test::generate!{day22_part2_vec_vec_test_22 as "test/22.txt" => "7"}
aoc_test::generate!{day22_part2_vec_vec_22 as "22.txt" => "102770"}

fn main() {
    env_logger::init();

    let stdin = io::stdin();
    let input = io::read_to_string(stdin.lock()).expect("read input");
    let result = process(input.as_str()).expect("no errors");
    println!("{}", result);
}

fn process(input: &str) -> Result<String> {
    let (s, mut blocks) = parse::blocks(input).unwrap();
    assert!(s.trim().is_empty());

    let gravity: Point = Point::new(0, 0, -1);

    log::info!("Dropping blocks");
    for step in 0.. {
        log::info!("- Step {}", step);
        let mut updated = false;

        for i in 0..blocks.len() {
            // Cannot fall through the floor
            if blocks[i].min.z == 1 {
                continue;
            }

            // Cannot fall through another block
            let fallen = blocks[i] + gravity;
            if blocks
                .iter()
                .enumerate()
                .any(|(j, block)| i != j && block.intersects(&fallen))
            {
                continue;
            }

            // If we've made it this far, drop the block and mark updated
            blocks[i] = fallen;
            updated = true;
        }

        if !updated {
            break;
        }
    }

    log::info!("Calculating supports");
    let supported_by = blocks
        .iter()
        .map(|block| {
            (
                block,
                blocks
                    .iter()
                    .filter(|other| block != *other && (*block + gravity).intersects(other))
                    .collect::<Vec<_>>(),
            )
        })
        .collect::<Vec<_>>();

    log::info!("Supported blocks:");
    for (block, supported_by) in supported_by.iter() {
        let name = block.name(&blocks);
        log::info!("- Block {name}: {block:?}");
        for other in supported_by.iter() {
            let name = other.name(&blocks);
            log::info!("  - {name}: {other:?}");
        }
    }

    // A helper function to remove blocks from a supported_by structure
    // This will remove the block from each sublist and the main list
    fn remove_from_supports(supported_by: &mut Vec<(&Block, Vec<&Block>)>, block: &Block) {
        supported_by.iter_mut().for_each(|supports| {
            let index = supports.1.iter().position(|b| *b == block);
            if let Some(index) = index {
                supports.1.remove(index);
            }
        });

        let index = supported_by.iter().position(|(b, _)| *b == block).unwrap();
        supported_by.remove(index);
    }

    Ok(blocks
        .iter()
        .map(|block| {
            let name = block.name(&blocks);
            log::info!("Attempting to remove {name}: {block:?}");

            // Make a local copy of supported blocks
            let mut supported_by = supported_by.clone();

            // Remove that block from any supports
            let name = block.name(&blocks);
            log::info!("- Removing {name}: {block:?}");
            remove_from_supports(&mut supported_by, block);

            // Repeatedly remove blocks that are unsupported
            log::info!("- Settling unsupported blocks");
            loop {
                let mut changed = false;

                // Find blocks that are now unsupported (and not on the floor)
                let to_remove = supported_by
                    .iter()
                    .filter(|(block, supports)| block.min.z > 1 && supports.is_empty())
                    .map(|(block, _)| **block)
                    .collect::<Vec<_>>();

                for block in to_remove.iter() {
                    let name = block.name(&blocks);
                    log::info!("  - Removing {name}: {block:?}");

                    remove_from_supports(&mut supported_by, block);
                    changed = true;
                }

                if !changed {
                    break;
                }
            }

            // Return the number of blocks that were removed
            // Off by 1 because the original 'block' doesn't count as fallen
            blocks.len() - supported_by.len() - 1
        })
        .sum::<usize>()
        .to_string())
}
