use anyhow::Result;
use std::io;
use fxhash::FxHashMap;

type Key<'a> = (&'a [u8], u8, u8, &'a [u64], u64);
struct Solver<'a> {
    cache: FxHashMap<Key<'a>, u128>,
}

impl<'a> Solver<'a> {
    fn new() -> Self {
        Self {
            cache: FxHashMap::default(),
        }
    }

    fn check(
        &mut self,
        s: &'a [u8],       // The remaining input string after current
        curr: u8,          // The current character to check
        prev: u8,          // The previous character to check
        groups: &'a [u64], // The remaining groups to match
        count: u64,        // The size of the current group
    ) -> u128 {
        let key = (s, curr, prev, groups, count);

        if let Some(value) = self.cache.get(&key) {
            return *value;
        }

        let result = {
            if groups.is_empty() {
                // Base case, we have no more groups to go
                // Everything else must not be #
                if curr == b'#' || s.iter().any(|c| *c == b'#') {
                    0
                } else {
                    1
                }
                // From here on out, we know groups is not empty
            } else if curr == b'?' {
                // Current is unknown, try both cases (without advancing s!)
                let if_d = self.check(s, b'.', prev, groups, count);
                let if_o = self.check(s, b'#', prev, groups, count);
                if_d + if_o
            } else if s.is_empty() {
                // This block seems wrong, but I need it to have curr and prev work with ?
                // We have no more input, check the last current
                // We have at least one group at this point
                if curr == b'#' {
                    // If the last current is operational, we need to match the last group
                    if groups.len() == 1 && count + 1 == groups[0] {
                        1
                    } else {
                        0
                    }
                } else if curr == b'.' {
                    // If we came from operational check the last group
                    if groups.len() == 1 && count == groups[0] {
                        1
                    } else {
                        0
                    }
                } else {
                    panic!("got something weird on empty input: {curr}")
                }
            } else if curr == b'#' {
                // Current is operational
                if prev == b'.' {
                    // After damaged, start a new group
                    self.check(&s[1..], s[0], curr, groups, 1)
                } else if prev == b'#' {
                    // After another operational, continue group
                    self.check(&s[1..], s[0], curr, groups, count + 1)
                } else {
                    panic!("got # after something weird: {prev}")
                }
            } else if curr == b'.' {
                // Current is damaged
                if prev == b'.' {
                    // After another damaged, nothing happens
                    self.check(&s[1..], s[0], curr, groups, 0)
                } else if prev == b'#' {
                    // After operational, finish the current group
                    // If the size doesn't match, this branch is immediately invalid
                    if count == groups[0] {
                        self.check(&s[1..], s[0], curr, &groups[1..], 0)
                    } else {
                        0
                    }
                } else {
                    panic!("got . after something weird: {prev}")
                }
            } else {
                panic!("got something weird: {curr}")
            }
        };

        // dbg!(result);

        self.cache.insert(key, result);
        result
    }
}

// #[aoc_test("data/test/12.txt", "21")]
// #[aoc_test("data/12.txt", "7025")]
fn main() -> Result<()> {
    let stdin = io::stdin();
    let input = io::read_to_string(stdin.lock())?;

    fn drep(s: &str, d: &str, n: usize) -> String {
        std::iter::repeat(s).take(n).collect::<Vec<_>>().join(d)
    }

    let input = input
        .lines()
        .map(|line| {
            let parts = line.split_once(' ').unwrap();
            drep(parts.0, "?", 5) + " " + &drep(parts.1, ",", 5)
        })
        .collect::<Vec<_>>()
        .join("\n");

    let result = input
        .lines()
        .map(|line| {
            let parts = line.split_once(' ').unwrap();
            let conditions = parts.0.as_bytes();
            let groups = parts
                .1
                .split(',')
                .map(|s| s.parse().unwrap())
                .collect::<Vec<_>>();

            Solver::new().check(conditions, b'.', b'.', &groups, 0)
        })
        .sum::<u128>();

    println!("{result}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test {
        ($s:expr, $g:expr, $e:expr) => {
            assert_eq!(Solver::new().check($s.as_bytes(), b'.', b'.', $g, 0), $e);
        };
    }

    #[test]
    fn no_questions() {
        test!("", &[], 1);
        test!(".", &[], 1);
        test!("#", &[1], 1);
        test!("#.#", &[1, 1], 1);
        test!("#.##.###", &[1, 2, 3], 1);
        test!("###.##.#", &[3, 2, 1], 1);
    }

    #[test]
    fn simple_questions() {
        test!("?", &[1], 1);
        test!(".?", &[1], 1);
        test!("??", &[1], 2);
        test!(".??", &[1], 2);
        test!("???", &[1], 3);
        test!(".???", &[1], 3);
    }

    #[test]
    fn hash_question() {
        test!("#?", &[1], 1);
        test!("#??", &[1], 1);
        test!("#???", &[1], 1);
    }
}
