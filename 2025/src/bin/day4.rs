use aoc2025::grid::Grid;

#[aoc::register(day4, part1)]
fn part1(input: &str) -> impl Into<String> {
    let g = Grid::read(input, |c| c == '@');

    g.iter()
        .filter(|(x, y, r)| *r && g.neighbors(*x, *y).filter(|v| *v == Some(true)).count() < 4)
        .count()
        .to_string()
}

#[aoc::register(day4, part2)]
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

#[aoc::register(day4, part2_no_map)]
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

#[aoc::register_render(day4, part2_render, scale = 4, fps = 10)]
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

aoc::main!(day4);

aoc::test!(
    day4,
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
    day4,
    file = "input/2025/day4.txt",
    [part1] => "1393",
    [part2, part2_no_map] => "8643"
);
