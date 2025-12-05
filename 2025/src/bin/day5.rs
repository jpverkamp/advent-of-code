aoc::main!(day5);

#[derive(Debug, Clone, PartialEq, Eq)]
struct Puzzle {
    ranges: Vec<(usize, usize)>,
    ids: Vec<usize>,
}

impl From<&str> for Puzzle {
    fn from(s: &str) -> Self {
        let mut ranges = Vec::new();
        let mut ids = Vec::new();

        let mut lines = s.lines();

        for line in lines.by_ref() {
            if line.trim().is_empty() {
                break;
            }

            let (a, b) = line.split_once('-').unwrap();
            let a = a.parse().unwrap();
            let b = b.parse().unwrap();
            ranges.push((a, b));
        }

        for line in lines {
            let id: usize = line.trim().parse().unwrap();
            ids.push(id);
        }

        Puzzle { ranges, ids }
    }
}

#[aoc::register]
fn part1(input: &str) -> impl Into<String> {
    let puzzle = Puzzle::from(input);

    puzzle
        .ids
        .into_iter()
        .filter(|id| puzzle.ranges.iter().any(|(a, b)| id >= a && id <= b))
        .count()
        .to_string()
}

#[aoc::register]
fn part2(input: &str) -> impl Into<String> {
    let puzzle = Puzzle::from(input);

    let mut ranges = puzzle.ranges.clone();

    // Merge overlapping and included ranges until nothing more can be merged
    'main: loop {
        for i in 0..ranges.len() {
            for j in (i + 1)..ranges.len() {
                let (a1, b1) = ranges[i];
                let (a2, b2) = ranges[j];

                // Completely included
                if a1 >= a2 && b1 <= b2 {
                    ranges.remove(i);
                    continue 'main;
                }

                if a2 >= a1 && b2 <= b1 {
                    ranges.remove(j);
                    continue 'main;
                }

                // Overlapping
                if a1 <= b2 && a2 <= b1 {
                    let new_range = (a1.min(a2), b1.max(b2));
                    ranges[i] = new_range;
                    ranges.remove(j);
                    continue 'main;
                }
            }
        }

        break;
    }

    ranges
        .into_iter()
        .map(|(a, b)| b - a + 1)
        .sum::<usize>()
        .to_string()
}

#[allow(dead_code)]
// #[aoc::register]
fn part2_bruteforce(input: &str) -> impl Into<String> {
    let puzzle = Puzzle::from(input);

    let min = puzzle.ranges.iter().map(|(a, _)| *a).min().unwrap();
    let max = puzzle.ranges.iter().map(|(_, b)| *b).max().unwrap();

    let start_time = std::time::Instant::now();

    (min..=max)
        .map(|id| {
            if id % 100_000_000 == 0 {
                let elapsed = start_time.elapsed().as_secs_f64();
                let rate = (id - min) as f64 / elapsed;
                let eta = (max  - id) as f64 / rate;
                
                println!("[{id}] Elapsed: {:.2} s, Rate: {:.2} ids/s, ETA: {:.2} s", elapsed, rate, eta);
            }
            id
        })
        .filter(|id| !puzzle.ranges.iter().any(|(a, b)| id >= a && id <= b))
        .count()
        .to_string()
}

aoc::test!(
    text = "\
3-5
10-14
16-20
12-18

1
5
8
11
17
32
", 
    [part1] => "3",
    [part2] => "14"
);

aoc::test!(
    file = "input/2025/day5.txt",
    [part1] => "874",
    [part2] => "348548952146313"
);
