use anyhow::Result;
use std::io;

use day22::{parse, types::*};

use fxhash::{FxHashMap, FxHashSet};

aoc_test::generate!{day22_part2_test_22 as "test/22.txt" => "7"}
aoc_test::generate!{day22_part2_22 as "22.txt" => "102770"}

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

    // Sort blocks ascending
    blocks.sort_by(|b1, b2| b1.min.z.cmp(&b2.min.z));

    // For each block, find all blocks in the region beneath it
    log::info!("Dropping blocks");
    for block_i in 0..blocks.len() {
        // Generate the region beneath this block
        let block = blocks[block_i];
        let beneath = Block::new(
            Point::new(block.min.x, block.min.y, 1),
            Point::new(block.max.x, block.max.y, block.min.z - 1),
        );

        // Find the height of the tallest block under it
        if let Some(fall_to) = blocks
            .iter()
            .filter_map(|b| {
                if beneath.intersects(b) {
                    Some(b.max.z)
                } else {
                    None
                }
            })
            .max()
        {
            // If we have a height, drop the block
            let block = &mut blocks[block_i];
            let fall_by = block.min.z - fall_to - 1;
            block.min.z -= fall_by;
            block.max.z -= fall_by;
        } else {
            // No blocks beneath, fall to the floor
            let block = &mut blocks[block_i];
            let fall_by = block.min.z - 1;
            block.min.z -= fall_by;
            block.max.z -= fall_by;
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
                    .collect::<FxHashSet<_>>(),
            )
        })
        .collect::<FxHashMap<_, _>>();

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
    fn remove_from_supports(
        supported_by: &mut FxHashMap<&Block, FxHashSet<&Block>>,
        block: &Block,
    ) {
        supported_by.values_mut().for_each(|supports| {
            supports.remove(block);
        });

        supported_by.remove(block);
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
            blocks.len() - supported_by.len() - 1
        })
        .sum::<usize>()
        .to_string())
}
