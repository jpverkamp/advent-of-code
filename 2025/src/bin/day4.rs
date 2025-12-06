use aoc2025::grid::Grid;

aoc::main!(day4);

#[aoc::register]
fn part1(input: &str) -> impl Into<String> {
    let g = Grid::read(input, |c| c == '@');

    g.iter()
        .filter(|(x, y, r)| *r && g.neighbors(*x, *y).filter(|v| *v == Some(true)).count() < 4)
        .count()
        .to_string()
}

#[aoc::register]
fn part2(input: &str) -> impl Into<String> {
    let mut g = Grid::read(input, |c| c == '@');
    let initial_count = g.iter().filter(|(_, _, v)| *v).count();
    let mut previous_count = initial_count;

    loop {
        let new_g =
            g.map(|x, y, v| v && g.neighbors(x, y).filter(|v| *v == Some(true)).count() >= 4);

        let new_count = new_g.iter().filter(|(_, _, v)| *v).count();

        if previous_count == new_count {
            return (initial_count - new_count).to_string();
        }

        g = new_g;
        previous_count = new_count;
    }
}

#[aoc::register]
fn part2_no_map(input: &str) -> impl Into<String> {
    let mut g = Grid::read(input, |c| c == '@');
    let mut count = 0;

    loop {
        let mut changed = false;

        for x in 0..g.width() {
            for y in 0..g.height() {
                if g.get(x, y) == Some(true)
                    && g.neighbors(x, y).filter(|v| *v == Some(true)).count() < 4
                {
                    g.set(x, y, false);
                    changed = true;
                    count += 1;
                }
            }
        }

        if !changed {
            break;
        }
    }

    count.to_string()
}

// This is mostly here to test the render_image! macro
#[aoc::register_render]
fn part2_image(input: &str) {
    let g = Grid::read(input, |c| c == '@');

    aoc::render_image!(test, g.width() as usize, g.height() as usize, |x, y| {
        match g.get(x as isize, y as isize) {
            Some(true) => (0, 0, 0),
            _ => (255, 255, 255),
        }
    });
}

// Test the render_frame! macro, but also produce a useful animation of the process
#[aoc::register_render(scale = 4, fps = 10)]
fn part2_render(input: &str) {
    let mut g = Grid::read(input, |c| c == '@');

    loop {
        let mut removed = vec![];

        for x in 0..g.width() {
            for y in 0..g.height() {
                if g.get(x, y) == Some(true)
                    && g.neighbors(x, y).filter(|v| *v == Some(true)).count() < 4
                {
                    g.set(x, y, false);
                    removed.push((x, y));
                }
            }
        }

        aoc::render_frame!(g.width() as usize, g.height() as usize, |x, y| {
            if removed.contains(&(x as isize, y as isize)) {
                return (255, 0, 0);
            }

            match g.get(x as isize, y as isize) {
                Some(true) => (0, 0, 0),
                _ => (255, 255, 255),
            }
        });

        if removed.is_empty() {
            break;
        }
    }
}

#[aoc::register]
fn part2_floodfill(input: &str) -> impl Into<String> {
    let mut g = Grid::read(input, |c| c == '@');
    let mut count = 0;

    for y in 0..g.height() {
        for x in 0..g.width() {
            // At each point, flood fill points we can remove
            let mut stack = vec![(x, y)];
            while let Some((cx, cy)) = stack.pop() {
                // Any points we can't remove in the stack are ignored
                if g.get(cx, cy) != Some(true) {
                    continue;
                }
                if g.neighbors(cx, cy).filter(|v| *v == Some(true)).count() >= 4 {
                    continue;
                }

                // Remove point, add neighbors to stack
                g.set(cx, cy, false);
                count += 1;

                for nx in (cx - 1)..=(cx + 1) {
                    for ny in (cy - 1)..=(cy + 1) {
                        if nx == cx && ny == cy {
                            continue;
                        }
                        stack.push((nx, ny));
                    }
                }
            }
        }
    }

    count.to_string()
}

#[aoc::register_render(scale = 4, fps = 30)]
fn part2_render_floodfill(input: &str) {
    let mut g = Grid::read(input, |c| c == '@');

    for y in 0..g.height() {
        for x in 0..g.width() {
            // At each point, flood fill points we can remove
            let mut stack = vec![(x, y)];
            let mut removed = vec![];

            while let Some((cx, cy)) = stack.pop() {
                // Any points we can't remove in the stack are ignored
                if g.get(cx, cy) != Some(true) {
                    continue;
                }
                if g.neighbors(cx, cy).filter(|v| *v == Some(true)).count() >= 4 {
                    continue;
                }

                // Remove point, add neighbors to stack
                g.set(cx, cy, false);
                removed.push((cx, cy));

                for nx in (cx - 1)..=(cx + 1) {
                    for ny in (cy - 1)..=(cy + 1) {
                        if nx == cx && ny == cy {
                            continue;
                        }
                        stack.push((nx, ny));
                    }
                }
            }

            if !removed.is_empty() {
                aoc::render_frame!(g.width() as usize, g.height() as usize, |x, y| {
                    if removed.contains(&(x as isize, y as isize)) {
                        return (255, 0, 0);
                    }

                    match g.get(x as isize, y as isize) {
                        Some(true) => (0, 0, 0),
                        _ => (255, 255, 255),
                    }
                });
            }
        }
    }
}

aoc::test!(
    text = "\
123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  
", 
    [part1] => "13",
    [part2, part2_no_map] => "43"
);

aoc::test!(
    file = "input/2025/day4.txt",
    [part1] => "1393",
    [part2, part2_no_map] => "8643"
);
