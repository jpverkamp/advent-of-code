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

aoc::test!(
    text = "\
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
", 
    [part1] => "13",
    [part2, part2_no_map] => "43"
);

aoc::test!(
    file = "input/2025/day4.txt",
    [part1] => "1393",
    [part2, part2_no_map] => "8643"
);
