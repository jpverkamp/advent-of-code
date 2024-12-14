use criterion::{criterion_group, criterion_main, Criterion};
use paste::paste;

/// Get input for a single day
macro_rules! get_day_input {
    ($day_num:literal) => {
        include_str!(concat!("../input/2024/day", $day_num, ".txt"))
    };
}

/// Define benchmarks for a single day with part1 and part2
macro_rules! benches_day {
    ($day_num:literal) => {
        paste! {
            use aoc2024::[<day $day_num>]; // Replace `aoc24` with your crate name

            pub fn [<bench_day $day_num>](c: &mut Criterion) {
                let mut group = c.benchmark_group(concat!("day", $day_num));
                let input = get_day_input!($day_num);
                group.bench_function(format!("day{}_part1", $day_num), |b| b.iter(|| [<day $day_num>]::part1(input)));
                group.bench_function(format!("day{}_part2", $day_num), |b| b.iter(|| [<day $day_num>]::part2(input)));
            }
        }
    };
}

/// Create benchmarks for included days
macro_rules! benches {
    ($($day_num:literal),*) => {
        paste! {
            $(
                benches_day!($day_num);
            )*

            criterion_group!(benches, $([<bench_day $day_num>]),*);
            criterion_main!(benches);
        }
    };
}

benches!(1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14); // Add more days here
