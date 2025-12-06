use aoc2025::grid::Grid;

aoc::main!(day6);

#[aoc::register]
fn part1(input: &str) -> impl Into<String> {
    let lines = input.lines().collect::<Vec<_>>();

    let numbers = lines[..lines.len() - 1]
        .iter()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|value| value.parse::<usize>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    lines
        .last()
        .unwrap()
        .split_ascii_whitespace()
        .enumerate()
        .map(|(i, op)| {
            let values = numbers.iter().map(|line| line[i]);
            match op {
                "*" => values.product::<usize>(),
                "+" => values.sum::<usize>(),
                _ => unimplemented!("Unknown operator {op}"),
            }
        })
        .sum::<usize>()
        .to_string()
}

#[aoc::register]
fn part1_grid(input: &str) -> impl Into<String> {
    let grid = Grid::read(input, |c| c);

    let mut col_start = 0;
    let mut sum = 0;

    for x in 0..=grid.width() {
        // Detected a vertical column of empty spaces at x
        if x == grid.width() || (0..grid.height()).all(|y| grid.get(x, y) == Some(' ')) {
            // Parse out each number in the column, ignore empty spaces
            let numbers = (0..grid.height() - 1).map(|y| {
                (col_start..x).fold(0_usize, |a, x| match grid.get(x, y).unwrap() {
                    '0'..='9' => a * 10 + (grid.get(x, y).unwrap() as usize - '0' as usize),
                    ' ' => a,
                    c => unreachable!("Unknown value {c:?}"),
                })
            });

            // Apply the matching operator, assume it's left aligned
            sum += match grid.get(col_start, grid.height() - 1) {
                Some('*') => numbers.product::<usize>(),
                Some('+') => numbers.sum::<usize>(),
                c => unreachable!("Unknown operator {c:?}"),
            };

            col_start = x + 1;
        }
    }

    sum.to_string()
}

#[aoc::register]
fn part2(input: &str) -> impl Into<String> {
    let grid = Grid::read(input, |c| c);

    let mut col_start = 0;
    let mut sum = 0;

    for x in 0..=grid.width() {
        // Detected a vertical column of empty spaces at x
        if x == grid.width() || (0..grid.height()).all(|y| grid.get(x, y) == Some(' ')) {
            // Parse out each number in the column, ignore empty spaces
            // This time, numbers run from bottom to top, right to left (although order doesn't matter)
            // So the only change is swapping the two iterators (x/y)
            let numbers = (col_start..x).map(|x| {
                (0..grid.height() - 1).fold(0_usize, |a, y| match grid.get(x, y).unwrap() {
                    '0'..='9' => a * 10 + (grid.get(x, y).unwrap() as usize - '0' as usize),
                    ' ' => a,
                    c => unreachable!("Unknown value {c:?}"),
                })
            });

            // Apply the matching operator, assume it's left aligned
            sum += match grid.get(col_start, grid.height() - 1) {
                Some('*') => numbers.product::<usize>(),
                Some('+') => numbers.sum::<usize>(),
                c => unreachable!("Unknown operator {c:?}"),
            };

            col_start = x + 1;
        }
    }

    sum.to_string()
}

aoc::test!(
    text = "\
123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  
", 
    [part1, part1_grid] => "4277556",
    [part2] => "3263827"
);

aoc::test!(
    file = "input/2025/day6.txt",
    [part1] => "4309240495780",
    [part2] => "9170286552289"
);
