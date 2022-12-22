use aoc::*;
use std::path::Path;

type INumber = i128;

#[derive(Clone, Debug)]
struct Message {
    data: Vec<(usize, INumber)>,
}

impl<I> From<&mut I> for Message
where
    I: Iterator<Item = String>,
{
    fn from(iter: &mut I) -> Self {
        Message {
            data: iter
                .map(|line| line.parse::<INumber>().expect("must be a number"))
                .enumerate()
                .collect::<Vec<_>>()
        }
    }
}

impl std::fmt::Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.data
                .iter()
                .map(|(_, v)| v.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}

impl Message {
    fn decrypt(&mut self) {
        for index_to_move in 0..self.len() {
            self.mix(index_to_move);
        }
    }

    fn mix(&mut self, index_to_move: usize) {
        let (index_current, (_, value)) = self.data
            .iter()
            .enumerate()
            .find(|(_, (index_original, _))| index_to_move == *index_original)
            .unwrap();

        let mut index_target = index_current as INumber + value;
        let ilen = self.data.len() as INumber;
        
        // If the index is negative, increase it into range
        // We can't jump directly there because we'll overflow even i128, so jump in decreasing steps
        let mut steps = 100000000;
        while steps > 1 {
            while index_target < -steps * ilen {
                let loops = (-index_target / ilen).min(steps);
                index_target += loops * ilen - loops;
            }
            steps /= 10;
        }
        while index_target < 0 {
            index_target += ilen - 1;
        }
        
        // Do the same if the index is too big, resetting steps
        steps = 10000000;
        while steps > 1 {
            while index_target >= steps * ilen {
                let loops = (index_target / ilen).min(steps); 
                index_target = index_target - loops * ilen + loops;
            }
            steps /= 10;
        }
        while index_target >= ilen {
            let loops = index_target / ilen; 
            index_target = index_target - loops * ilen + loops;
        }

        // If we were jumping downwards and ended up at 0, we actually want it at the end
        // Most to match the given test case, this doesn't matter for the real answers
        if index_target == 0 && *value < 0 {
            index_target = ilen - 1;
        }

        let index_target = index_target as usize;
        
        if cfg!(debug_assertions) {
            println!(
                "{} [moving, orig: {:2}, value: {:2}, curr: {:2}, next: {:2}]",
                self,
                index_to_move,
                value,
                index_current,
                index_target,
            );
        }

        let removed = self.data.remove(index_current);
        self.data.insert(index_target, removed);
    }

    fn len(&self) -> usize {
        self.data.len()
    }

    #[allow(dead_code)]
    fn data(&self) -> Vec<INumber> {
        self.data
            .iter()
            .map(|(_, v)| *v)
            .collect::<Vec<_>>()
    }
}

fn part1(filename: &Path) -> String {
    let mut message = Message::from(&mut iter_lines(filename));
    message.decrypt();
    
    if cfg!(debug_assertions) {
        println!("{message} [final]");
    }

    let index_zero = message
            .data
            .iter()
            .enumerate()
            .find(|(_, (_, v))| *v == 0)
            .unwrap()
            .0;

    (
        message.data[(index_zero + 1000) % message.len()].1 + 
        message.data[(index_zero + 2000) % message.len()].1 + 
        message.data[(index_zero + 3000) % message.len()].1
    ).to_string()
}

fn part2(filename: &Path) -> String {
    let mut message = Message::from(&mut iter_lines(filename));
    message.data.iter_mut().for_each(|p| p.1 *= 811589153);
    for _round in 0..10 {
        message.decrypt();
        println!("finished {_round}");
    }

    if cfg!(debug_assertions) {
        println!("{message} [final]");
    }

    let index_zero = message
            .data
            .iter()
            .enumerate()
            .find(|(_, (_, v))| *v == 0)
            .unwrap()
            .0;

    (
        message.data[(index_zero + 1000) % message.len()].1 + 
        message.data[(index_zero + 2000) % message.len()].1 + 
        message.data[(index_zero + 3000) % message.len()].1
    ).to_string()
}

fn main() {
    aoc_main(part1, part2);
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2, Message};
    use aoc::aoc_test;

    fn make_test() -> Message {
        Message { data: vec![
            (0, 1),
            (1, 2),
            (2, -3), 
            (3, 3),
            (4, -2),
            (5, 0),
            (6, 4),
        ]}
    }

    fn make_zeroes(size: usize) -> Message {
        Message {
            data: (0..size)
                .map(|v| (v, 0))
                .collect::<Vec<_>>()
        }
    }

    fn make_singleton(size: usize, index: usize, value: crate::INumber) -> Message{
        let mut message = make_zeroes(size);
        message.data[index].1 = value;
        message
    }

    
    #[test]
    fn test_mix() {
        let mut message = make_test();
        assert_eq!(message.data(), vec![1, 2, -3, 3, -2, 0, 4]);

        message.mix(0);
        assert_eq!(message.data(), vec![2, 1, -3, 3, -2, 0, 4]);

        message.mix(1);
        assert_eq!(message.data(), vec![1, -3, 2, 3, -2, 0, 4]);
        
        message.mix(2);
        assert_eq!(message.data(), vec![1, 2, 3, -2, -3, 0, 4]);

        message.mix(3);
        assert_eq!(message.data(), vec![1, 2, -2, -3, 0, 3, 4]);

        message.mix(4);
        assert_eq!(message.data(), vec![1, 2, -3, 0, 3, 4, -2]);

        message.mix(5);
        assert_eq!(message.data(), vec![1, 2, -3, 0, 3, 4, -2]);

        message.mix(6);
        assert_eq!(message.data(), vec![1, 2, -3, 4, 0, 3, -2]);
    }

    #[test]
    fn test_decrypt() {
        let mut message = make_test();
        message.decrypt();
        assert_eq!(message.data(), vec![1, 2, -3, 4, 0, 3, -2]);
    }

    #[test]
    fn test_zeros() {
        let mut message = make_zeroes(9);
        assert_eq!(message.data(), vec![0, 0, 0, 0, 0, 0, 0, 0, 0]);

        message.decrypt();
        assert_eq!(message.data(), vec![0, 0, 0, 0, 0, 0, 0, 0, 0]);
    }

    #[test]
    fn test_singleton() {
        let mut message = make_singleton(9, 4, 1);
        assert_eq!(message.data(), vec![0, 0, 0, 0, 1, 0, 0, 0, 0]);

        message.decrypt();
        assert_eq!(message.data(), vec![0, 0, 0, 0, 0, 1, 0, 0, 0]);
    }

    #[test]
    fn test_small_forward() {
        let mut message = make_singleton(9, 4, 2);
        message.decrypt();
        assert_eq!(message.data(), vec![0, 0, 0, 0, 0, 0, 2, 0, 0]);
    }

    #[test]
    fn test_looped_forward() {
        let mut message = make_singleton(9, 4, 5);
        message.decrypt();
        assert_eq!(message.data(), vec![0, 5, 0, 0, 0, 0, 0, 0, 0]);

        let mut message = make_singleton(9, 4, 4);
        message.decrypt();
        assert_eq!(message.data(), vec![0, 0, 0, 0, 0, 0, 0, 0, 4]);
    }

    #[test]
    fn test_double_looped_forward() {
        let mut message = make_singleton(9, 4, 14);
        message.decrypt();
        assert_eq!(message.data(), vec![0, 0, 14, 0, 0, 0, 0, 0, 0]);
    }

    #[test]
    fn test_small_backward() {
        let mut message = make_singleton(9, 4, -2);
        message.decrypt();
        assert_eq!(message.data(), vec![0, 0, -2, 0, 0, 0, 0, 0, 0]);
    }

    #[test]
    fn test_exact_loop_backward() {
        let mut message = make_singleton(9, 2, -2);
        message.decrypt();
        assert_eq!(message.data(), vec![0, 0, 0, 0, 0, 0, 0, 0, -2]);
    }

    #[test]
    fn test_looped_backward() {
        let mut message = make_singleton(9, 4, -4);
        message.decrypt();
        assert_eq!(message.data(), vec![0, 0, 0, 0, 0, 0, 0, 0, -4]);

        let mut message = make_singleton(9, 4, -8);
        message.decrypt();
        assert_eq!(message.data(), vec![0, 0, 0, 0, -8, 0, 0, 0, 0]);

        let mut message = make_singleton(9, 4, -5);
        message.decrypt();
        assert_eq!(message.data(), vec![0, 0, 0, 0, 0, 0, 0, -5, 0]);

        let mut message = make_singleton(9, 1, -3);
        message.decrypt();
        assert_eq!(message.data(), vec![0, 0, 0, 0, 0, 0, -3, 0, 0]);
    }

    #[test]
    fn test_double_looped_backward() {
        let mut message = make_singleton(9, 4, -14);
        message.decrypt();
        assert_eq!(message.data(), vec![0, 0, 0, 0, 0, 0, -14, 0, 0]);
    }

    #[test]
    fn test1() {
        aoc_test("20", part1, "19070")
    }

    #[test]
    fn test2() {
        aoc_test("20", part2, "14773357352059")
    }
}
