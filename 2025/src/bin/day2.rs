use num::Integer;

#[aoc::register(day2, part1)]
fn part1(input: &str) -> impl Into<String> {
    input
        .trim_end()
        .split(",")
        .map(|s| {
            let (a, b) = s.split_once("-").expect("Invalid range");
            (
                a.parse::<usize>()
                    .unwrap_or_else(|_| panic!("failed to parse {a}")),
                b.parse::<usize>()
                    .unwrap_or_else(|_| panic!("failed to parse {b}")),
            )
        })
        .flat_map(|(start, end)| (start..=end))
        .filter(|num| {
            let s = num.to_string();
            s[..s.len() / 2] == s[s.len() / 2..]
        })
        .sum::<usize>()
        .to_string()
}

#[aoc::register(day2, part1_regex)]
fn part1_regex(input: &str) -> impl Into<String> {
    let re = fancy_regex::Regex::new(r"^(\d+)\1$").unwrap();

    input
        .trim_end()
        .split(",")
        .map(|s| {
            let (a, b) = s.split_once("-").expect("Invalid range");
            (
                a.parse::<usize>()
                    .unwrap_or_else(|_| panic!("failed to parse {a}")),
                b.parse::<usize>()
                    .unwrap_or_else(|_| panic!("failed to parse {b}")),
            )
        })
        .flat_map(|(start, end)| (start..=end))
        .filter(|num| re.is_match(&num.to_string()).unwrap_or(false))
        .sum::<usize>()
        .to_string()
}

#[aoc::register(day2, part1_intmatch)]
fn part1_intmatch(input: &str) -> impl Into<String> {
    input
        .trim_end()
        .split(",")
        .map(|s| {
            let (a, b) = s.split_once("-").expect("Invalid range");
            (
                a.parse::<usize>()
                    .unwrap_or_else(|_| panic!("failed to parse {a}")),
                b.parse::<usize>()
                    .unwrap_or_else(|_| panic!("failed to parse {b}")),
            )
        })
        .flat_map(|(start, end)| (start..=end))
        .filter(|num| {
            let digits = (*num as f64).log10() as usize + 1;
            num / 10_usize.pow((digits / 2) as u32) == num % 10_usize.pow((digits / 2) as u32)
        })
        .sum::<usize>()
        .to_string()
}

#[aoc::register(day2, part1_intmatch_divrem)]
fn part1_intmatch_divrem(input: &str) -> impl Into<String> {
    input
        .trim_end()
        .split(",")
        .map(|s| {
            let (a, b) = s.split_once("-").expect("Invalid range");
            (
                a.parse::<usize>()
                    .unwrap_or_else(|_| panic!("failed to parse {a}")),
                b.parse::<usize>()
                    .unwrap_or_else(|_| panic!("failed to parse {b}")),
            )
        })
        .flat_map(|(start, end)| (start..=end))
        .filter(|num| {
            let digits = (*num as f64).log10() as usize + 1;
            let (q, r) = num.div_mod_floor(&10_usize.pow((digits / 2) as u32));
            q == r
        })
        .sum::<usize>()
        .to_string()
}

#[aoc::register(day2, part1_chatgpt)]
fn part1_chatgpt(input: &str) -> impl Into<String> {
    fn sum_repeated_halves(a: u64, b: u64) -> u128 {
        let mut total_sum: u128 = 0u128;
        let mut h = 1;

        loop {
            let low = 10u64.pow(h - 1);
            let high = 10u64.pow(h) - 1;

            let power = 10u128.pow(h);

            for x in low..=high {
                let xx = x as u128 * (power + 1);
                if xx > b as u128 {
                    break; // all larger X will exceed b
                }
                if xx >= a as u128 {
                    total_sum += xx;
                }
            }

            // stop if the smallest XX for this h exceeds b
            if low as u128 * (power + 1) > b as u128 {
                break;
            }

            h += 1;
        }

        total_sum
    }

    input
        .trim_end()
        .split(",")
        .map(|s| {
            let (a, b) = s.split_once("-").expect("Invalid range");
            (
                a.parse::<usize>()
                    .unwrap_or_else(|_| panic!("failed to parse {a}")),
                b.parse::<usize>()
                    .unwrap_or_else(|_| panic!("failed to parse {b}")),
            )
        })
        .map(|(start, end)| sum_repeated_halves(start as u64, end as u64))
        .sum::<u128>()
        .to_string()
}

#[aoc::register(day2, part2)]
fn part2(input: &str) -> impl Into<String> {
    // Test if s is made up of n repeating chunks
    fn is_repeat(s: &str, n: usize) -> bool {
        let len = s.len();
        if len % n != 0 {
            return false;
        }

        let chunk_size = len / n;
        let chunk = &s[0..chunk_size];
        for i in 0..n {
            if &s[i * chunk_size..(i + 1) * chunk_size] != chunk {
                return false;
            }
        }
        true
    }

    input
        .trim_end()
        .split(",")
        .map(|s| {
            let (a, b) = s.split_once("-").expect("Invalid range");
            (
                a.parse::<usize>()
                    .unwrap_or_else(|_| panic!("failed to parse {a}")),
                b.parse::<usize>()
                    .unwrap_or_else(|_| panic!("failed to parse {b}")),
            )
        })
        .flat_map(|(start, end)| (start..=end))
        .filter(|num| {
            let s = num.to_string();
            (2..=s.len()).any(|chunk_size| is_repeat(&s, chunk_size))
        })
        .sum::<usize>()
        .to_string()
}

#[aoc::register(day2, part2_intmatch)]
fn part2_intmatch(input: &str) -> impl Into<String> {
    // Test if s is made up of n repeating chunks
    fn is_repeat(num: usize, n: usize) -> bool {
        let digits = (num as f64).log10() as usize + 1;
        if digits % n != 0 {
            return false;
        }

        let chunk_size = digits / n;
        let chunk_div = 10_usize.pow(chunk_size as u32);
        let chunk = num / chunk_div.pow((n - 1) as u32);

        for i in 0..n {
            if num / chunk_div.pow((n - 1 - i) as u32) % chunk_div != chunk {
                return false;
            }
        }
        true
    }

    input
        .trim_end()
        .split(",")
        .map(|s| {
            let (a, b) = s.split_once("-").expect("Invalid range");
            (
                a.parse::<usize>()
                    .unwrap_or_else(|_| panic!("failed to parse {a}")),
                b.parse::<usize>()
                    .unwrap_or_else(|_| panic!("failed to parse {b}")),
            )
        })
        .flat_map(|(start, end)| (start..=end))
        .filter(|num| {
            let digits = (*num as f64).log10() as usize + 1;
            (2..=digits).any(|chunk_size| is_repeat(*num, chunk_size))
        })
        .sum::<usize>()
        .to_string()
}

aoc::main!(day2);

aoc::test!(day2, text = "\
11-22,95-115,998-1012,1188511880-1188511890,222220-222224,\
1698522-1698528,446443-446449,38593856-38593862,565653-565659,\
824824821-824824827,2121212118-2121212124", [part1, part1_regex, part1_intmatch, part1_intmatch_divrem, part1_chatgpt] => "1227775554", [part2, part2_intmatch] => "4174379265");

aoc::test!(day2, file = "input/2025/day2.txt", [part1, part1_intmatch, part1_regex, part1_intmatch_divrem, part1_chatgpt] => "24157613387", [part2, part2_intmatch] => "33832678380");
