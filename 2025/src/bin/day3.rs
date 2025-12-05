aoc::main!(day3);

#[aoc::register]
fn part1(input: &str) -> impl Into<String> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let digits = line
                .chars()
                .map(|c| ((c as u8) - b'0') as usize)
                .collect::<Vec<_>>();

            let mut max_v = 0;

            for (i, a) in digits.iter().enumerate() {
                for (j, b) in digits.iter().enumerate() {
                    if i >= j {
                        continue;
                    }

                    let v = *a * 10 + *b;
                    max_v = v.max(max_v);
                }
            }

            max_v
        })
        .sum::<usize>()
        .to_string()
}

// Find the maximum digit string of count digits in the given string
#[tracing::instrument(ret)]
fn max_digits(digits: &[u8], count: usize) -> Option<usize> {
    // Break early if we don't have enough digits left to solve the problem
    if count > digits.len() {
        return None;
    }

    // Base case: we don't need any more digits
    if count == 0 {
        return Some(0);
    }

    // Try each digit from 9 down to 0
    for target in (0..=9u8).rev() {
        // Find the first index of that digit
        for (index, digit) in digits.iter().enumerate() {
            if *digit != target {
                continue;
            }

            // Such that there is a recursive answer in the remaining digits
            if let Some(recur) = max_digits(&digits[index + 1..], count - 1) {
                return Some((*digit as usize) * 10_usize.pow((count as u32) - 1) + recur);
            }
        }
    }

    None
}

#[aoc::register]
fn part1_max_digits(input: &str) -> impl Into<String> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            tracing::info!("Working on {line:?}");
            let digits = line.chars().map(|c| ((c as u8) - b'0')).collect::<Vec<_>>();

            max_digits(&digits, 2).unwrap()
        })
        .sum::<usize>()
        .to_string()
}

#[aoc::register]
fn part2(input: &str) -> impl Into<String> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let digits = line.chars().map(|c| ((c as u8) - b'0')).collect::<Vec<_>>();

            max_digits(&digits, 12).unwrap()
        })
        .sum::<usize>()
        .to_string()
}

#[allow(dead_code)]
// #[aoc::register]
fn part2_bruteforce(input: &str) -> impl Into<String> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let digits = line.chars().map(|c| ((c as u8) - b'0')).collect::<Vec<_>>();
            let n = digits.len();

            let mut max_value = 0;

            for i1 in 0..n {
                for i2 in (i1 + 1)..n {
                    for i3 in (i2 + 1)..n {
                        for i4 in (i3 + 1)..n {
                            for i5 in (i4 + 1)..n {
                                for i6 in (i5 + 1)..n {
                                    for i7 in (i6 + 1)..n {
                                        for i8 in (i7 + 1)..n {
                                            for i9 in (i8 + 1)..n {
                                                for i10 in (i9 + 1)..n {
                                                    for i11 in (i10 + 1)..n {
                                                        for i12 in (i11 + 1)..n {
                                                            let value: usize = format!(
                                                                "{}{}{}{}{}{}{}{}{}{}{}{}",
                                                                digits[i1],
                                                                digits[i2],
                                                                digits[i3],
                                                                digits[i4],
                                                                digits[i5],
                                                                digits[i6],
                                                                digits[i7],
                                                                digits[i8],
                                                                digits[i9],
                                                                digits[i10],
                                                                digits[i11],
                                                                digits[i12]
                                                            )
                                                            .parse()
                                                            .unwrap();

                                                            max_value = value.max(max_value);
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            max_value
        })
        .sum::<usize>()
        .to_string()
}

aoc::test!(
    text = "\
987654321111111
811111111111119
234234234234278
818181911112111", 
    [part1, part1_max_digits] => "357",
    [part2, part2_bruteforce] => "3121910778619"
);

aoc::test!(
    file = "input/2025/day3.txt",
    [part1, part1_max_digits] => "16927",
    [part2, part2_bruteforce] => "167384358365132"
);
