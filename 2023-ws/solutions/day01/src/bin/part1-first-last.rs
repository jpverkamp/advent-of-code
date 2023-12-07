use anyhow::Result;
use std::io::{self, BufRead};

trait IteratorExt: Iterator {
    fn first_and_last(mut self) -> [Self::Item; 2]
    where
        Self: Sized,
        Self::Item: Clone,
    {
        let first = self.next().unwrap();
        let last = self.last().or_else(|| Some(first.clone())).unwrap();

        [first, last]
    }
}

impl<T: ?Sized> IteratorExt for T where T: Iterator {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_and_last() {
        assert_eq!(vec![1, 2, 3, 4, 5].into_iter().first_and_last(), [1, 5]);
        assert_eq!(vec![1].into_iter().first_and_last(), [1, 1]);
    }
}

// #[aoc_test("data/test/01.txt", 142)]
// #[aoc_test("data/test/01b.txt", 209)]
// #[aoc_test("data/01.txt", 53651)]
fn main() -> Result<()> {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();

    let result = lines
        .map(|l| {
            l.expect("have input")
                .chars()
                .filter(|c| c.is_numeric())
                .first_and_last()
                .iter()
                .collect::<String>()
                .parse::<u32>()
                .unwrap()
        })
        .sum::<u32>();

    println!("{result}");
    Ok(())
}
