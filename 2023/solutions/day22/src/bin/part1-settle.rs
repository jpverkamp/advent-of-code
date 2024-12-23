use anyhow::Result;
use std::io;

use day22::{parse, types::*};

aoc_test::generate!{day22_part1_settle_test_22 as "test/22.txt" => "5"}
aoc_test::generate!{day22_part1_settle_22 as "22.txt" => "509"}

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
