use anyhow::Result;
use std::io;

use day22::{parse, types::*};

aoc_test::generate!{day22_part1_test_22 as "test/22.txt" => "5"}
aoc_test::generate!{day22_part1_22 as "22.txt" => "509"}

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

    for block in blocks.iter() {
        let name = block.name(&blocks);
        log::info!("- {name}: {block:?}");
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

    // Safe blocks are those that never are the only support for any other block
    Ok(blocks
        .iter()
        .filter(|block| {
            !supported_by
                .iter()
                .any(|(_, supports)| supports.contains(block) && supports.len() == 1)
        })
        .count()
        .to_string())
}
