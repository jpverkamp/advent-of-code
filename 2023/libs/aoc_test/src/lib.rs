#[macro_export]
macro_rules! generate {
    ($name:tt as $input_file:tt => $expected:tt) => {
        #[cfg(test)]
        mod $name {
            use super::*;
            use std::fs::OpenOptions;
            use std::io::prelude::*;

            #[test]
            fn run_test() {
                let filename = format!("../../data/{}", $input_file);
                let input = std::fs::read_to_string(filename).unwrap();

                let start = std::time::Instant::now();
                let actual = process(input.as_str()).unwrap();
                let elapsed = start.elapsed();

                if std::env::var("AOC_TIMING").is_ok() {
                    let output = format!(
                        "{},{},{},{:?},{}",
                        stringify!($name),
                        $expected,
                        actual,
                        elapsed,
                        elapsed.as_nanos()
                    );

                    let mut file = OpenOptions::new()
                        .create(true)
                        .append(true)
                        .open("../timing.csv")
                        .unwrap();
                    writeln!(file, "{}", output).unwrap();
                }

                assert_eq!(actual, $expected);
            }
        }
    };
}
