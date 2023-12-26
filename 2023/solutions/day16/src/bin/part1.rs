use anyhow::Result;
use std::io;

use day16::types::*;

use grid::Grid;
use point::Point;

// #[aoc_test("data/test/16.txt", "")]
// #[aoc_test("data/16.txt", "")]
#[allow(dead_code)]
fn main() {
    let stdin = io::stdin();

    let input = io::read_to_string(stdin.lock()).expect("read input");
    let result = process(input.as_str()).expect("no errors");
    println!("{}", result);
}

fn process(input: &str) -> Result<String> {
    let mirrors = Grid::read(input, |c| match c {
        '|' => Some(Mirror::VerticalSplitter),
        '-' => Some(Mirror::HorizontalSplitter),
        '/' => Some(Mirror::ForwardReflector),
        '\\' => Some(Mirror::BackwardReflector),
        _ => None,
    });

    Ok(illuminate(&mirrors, (Point::new(0, 0), Direction::East))
        .iter()
        .count()
        .to_string())
}

pub(crate) fn illuminate(mirrors: &Grid<Mirror>, start: (Point, Direction)) -> Grid<bool> {
    use Direction::*;
    use Mirror::*;

    let mut queue = Vec::new();
    queue.push(start);

    let mut visited = fxhash::FxHashSet::default();
    let mut illuminated = Grid::new();

    while let Some((p, d)) = queue.pop() {
        // Ignore points that have gone out of bounds
        if !mirrors.bounds.contains(&p) {
            continue;
        }

        // Don't evaluate the same point + direction more than once
        if visited.contains(&(p, d)) {
            continue;
        }
        visited.insert((p, d));

        illuminated.insert(p, true);

        match (mirrors.get(&p), d) {
            // If you hit a splitter side on (ex >-), you continue in the same direction.
            (Some(VerticalSplitter), North) | (Some(VerticalSplitter), South) => {
                queue.push((p + d.into(), d));
            }
            (Some(HorizontalSplitter), East) | (Some(HorizontalSplitter), West) => {
                queue.push((p + d.into(), d));
            }
            // Otherwise (ex >|), split to the two directions it points
            (Some(VerticalSplitter), _) => {
                queue.push((p + North.into(), North));
                queue.push((p + South.into(), South));
            }
            (Some(HorizontalSplitter), _) => {
                queue.push((p + East.into(), East));
                queue.push((p + West.into(), West));
            }
            // Diagonal reflectors just change, so >\ goes South, >/ goes North etc
            (Some(ForwardReflector), North) => queue.push((p + East.into(), East)),
            (Some(ForwardReflector), East) => queue.push((p + North.into(), North)),
            (Some(ForwardReflector), South) => queue.push((p + West.into(), West)),
            (Some(ForwardReflector), West) => queue.push((p + South.into(), South)),

            (Some(BackwardReflector), North) => queue.push((p + West.into(), West)),
            (Some(BackwardReflector), East) => queue.push((p + South.into(), South)),
            (Some(BackwardReflector), South) => queue.push((p + East.into(), East)),
            (Some(BackwardReflector), West) => queue.push((p + North.into(), North)),
            // If there's nothing there, keep going
            (None, _) => queue.push((p + d.into(), d)),
        }
    }

    illuminated
}
