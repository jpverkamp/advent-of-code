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
            num / 10_usize.pow((num.to_string().len() / 2) as u32)
                == num % 10_usize.pow((num.to_string().len() / 2) as u32)
        })
        .sum::<usize>()
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

aoc::main!(day2);

aoc::test!(day2, text = "\
11-22,95-115,998-1012,1188511880-1188511890,222220-222224,\
1698522-1698528,446443-446449,38593856-38593862,565653-565659,\
824824821-824824827,2121212118-2121212124", [part1, part1_intmatch, part1_regex] => "1227775554", [part2] => "4174379265");

aoc::test!(day2, file = "input/2025/day2.txt", [part1, part1_intmatch, part1_regex] => "24157613387", [part2] => "33832678380");
