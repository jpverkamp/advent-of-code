use aoc_runner_derive::{aoc, aoc_generator};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Instruction {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

impl From<u8> for Instruction {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Adv,
            1 => Self::Bxl,
            2 => Self::Bst,
            3 => Self::Jnz,
            4 => Self::Bxc,
            5 => Self::Out,
            6 => Self::Bdv,
            7 => Self::Cdv,
            _ => panic!("Invalid instruction"),
        }
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Instruction {
    // True if the operand is always a literal value, false if it's a combo operand (below)
    fn is_literally_literal(&self) -> bool {
        match self {
            Self::Adv => false,
            Self::Bxl => true,
            Self::Bst => false,
            Self::Jnz => true,
            Self::Bxc => true, // Takes one but ignores it
            Self::Out => false,
            Self::Bdv => false,
            Self::Cdv => false,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Operand {
    Literal(u8),
    A,
    B,
    C,
}

impl From<u8> for Operand {
    fn from(value: u8) -> Self {
        match value {
            0..=3 => Self::Literal(value),
            4 => Self::A,
            5 => Self::B,
            6 => Self::C,
            _ => panic!("Invalid combo operand"),
        }
    }
}

impl fmt::Display for Operand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Literal(value) => write!(f, "{}", value),
            Self::A => write!(f, "A"),
            Self::B => write!(f, "B"),
            Self::C => write!(f, "C"),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct Machine {
    pub a: u128,
    pub b: u128,
    pub c: u128,
    pub ip: usize,
    pub ram: Vec<u8>,
    pub halted: bool,
    pub output: Vec<u8>,
}

impl Machine {
    pub fn decompile(&self) -> String {
        let mut output = String::new();

        for (i, &byte) in self.ram.iter().enumerate().step_by(2) {
            let instruction = Instruction::from(byte);
            let operand = if instruction.is_literally_literal() {
                Operand::Literal(self.ram[i + 1])
            } else {
                Operand::from(self.ram[i + 1])
            };

            output.push_str(&format!("{instruction} {operand}\n"));
        }

        output
    }

    fn value_of(&self, operand: Operand) -> u128 {
        match operand {
            Operand::Literal(value) => value as u128,
            Operand::A => self.a,
            Operand::B => self.b,
            Operand::C => self.c,
        }
    }

    pub fn run(&mut self) {
        while !self.halted {
            self.step();
        }
    }

    pub fn step(&mut self) {
        if self.halted {
            return;
        }

        // Always read an instruction + operand, out of bounds is an error
        if self.ip >= self.ram.len() - 1 {
            self.halted = true;
            return;
        }

        let instruction = Instruction::from(self.ram[self.ip]);
        let operand = if instruction.is_literally_literal() {
            Operand::Literal(self.ram[self.ip + 1])
        } else {
            Operand::from(self.ram[self.ip + 1])
        };

        // println!("[ip={:>4?} a={:>10} b={:>10} c={:>10}] {} {}", self.ip, self.a, self.b, self.c, instruction, operand);

        match instruction {
            // Division (actually a right shift)
            Instruction::Adv => {
                self.a >>= self.value_of(operand);
            }
            // Bitwise XOR
            Instruction::Bxl => {
                self.b ^= self.value_of(operand);
            }
            // Bitwise set
            Instruction::Bst => {
                self.b = self.value_of(operand) & 0b111;
            }
            // Jump (if not zero)
            Instruction::Jnz => {
                if self.a != 0 {
                    self.ip = self.value_of(operand) as usize;
                    return; // Don't increment the IP
                }
            }
            // Bitwise XOR between b and c (ignores operand)
            Instruction::Bxc => {
                self.b ^= self.c;
            }
            // Output
            Instruction::Out => {
                self.output.push((self.value_of(operand) as u8) & 0b111);
            }
            // Division (actually a right shift) to b, still reads from a
            Instruction::Bdv => {
                self.b = self.a >> self.value_of(operand);
            }
            // Division (actually a right shift) to c, still reads from a
            Instruction::Cdv => {
                self.c = self.a >> self.value_of(operand);
            }
        }

        self.ip += 2;
    }
}

#[aoc_generator(day17)]
pub fn parse(input: &str) -> Machine {
    let mut lines = input.lines();
    let a = lines
        .next()
        .unwrap()
        .rsplit_once(" ")
        .unwrap()
        .1
        .parse()
        .unwrap();

    let b = lines
        .next()
        .unwrap()
        .rsplit_once(" ")
        .unwrap()
        .1
        .parse()
        .unwrap();

    let c = lines
        .next()
        .unwrap()
        .rsplit_once(" ")
        .unwrap()
        .1
        .parse()
        .unwrap();

    lines.next(); // Skip the empty line

    let ram = lines
        .next()
        .unwrap()
        .rsplit_once(" ")
        .unwrap()
        .1
        .split(",")
        .map(|s| s.parse().unwrap())
        .collect();

    Machine {
        a,
        b,
        c,
        ip: 0,
        ram,
        halted: false,
        output: Vec::new(),
    }
}

#[aoc(day17, part1, v1)]
fn part1_v1(input: &Machine) -> String {
    let mut machine = input.clone();

    // println!("{}", machine.decompile());

    machine.run();

    machine
        .output
        .iter()
        .map(|b| b.to_string())
        .collect::<Vec<_>>()
        .join(",")
}

#[allow(dead_code)]
// #[aoc(day17, part2, bruteforce)]
fn part2_bruteforce(input: &Machine) -> u128 {
    for a in (8 ^ 15).. {
        let mut machine = input.clone();
        machine.a = a;
        machine.run();
        if machine.output == machine.ram {
            return a;
        }
    }

    panic!("No solution found");
}

#[aoc(day17, part2, backtrack)]
fn part2_backtrack(input: &Machine) -> u128 {
    fn recur(original_machine: &Machine, a: u128, index: usize) -> Option<u128> {
        for tribble in 0..8 {
            let mut machine = original_machine.clone();
            let next_a = (a << 3) | tribble;
            machine.a = next_a;
            machine.run();

            // println!("{a} {next_a} {tribble} {} {}", machine.output[0], machine.ram[index]);

            if machine.output[0] == machine.ram[index] {
                if index == 0 {
                    return Some(next_a);
                }

                if let Some(a) = recur(original_machine, next_a, index - 1) {
                    return Some(a);
                }
            }
        }

        None
    }

    recur(input, 0, input.ram.len() - 1).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::make_test;

    const EXAMPLE: &str = "\
Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";

    make_test!([part1_v1] => "day17.txt", "4,6,3,5,6,3,5,2,1,0", "2,3,6,2,1,6,1,2,1");

    #[test]
    fn test_part2_v1_example() {
        let example = "\
Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0";

        assert_eq!(part2_backtrack(&parse(example)), 117440);
    }

    #[test]
    fn test_part2_v1_final() {
        assert_eq!(
            part2_backtrack(&parse(include_str!("../input/2024/day17.txt"))),
            90938893795561
        );
    }

    // If register C contains 9, the program 2,6 would set register B to 1.
    #[test]
    fn test_instruction_1() {
        let mut machine = Machine::default();
        machine.c = 9;
        machine.ram = vec![2, 6];

        machine.step();

        assert_eq!(machine.b, 1);
    }

    // If register A contains 10, the program 5,0,5,1,5,4 would output 0,1,2.
    #[test]
    fn test_instruction_2() {
        let mut machine = Machine::default();
        machine.a = 10;
        machine.ram = vec![5, 0, 5, 1, 5, 4];

        machine.step();
        machine.step();
        machine.step();

        assert_eq!(machine.output, vec![0, 1, 2]);
    }

    // If register A contains 2024, the program 0,1,5,4,3,0 would output 4,2,5,6,7,7,7,7,3,1,0 and leave 0 in register A.
    #[test]
    fn test_instruction_3() {
        let mut machine = Machine::default();
        machine.a = 2024;
        machine.ram = vec![0, 1, 5, 4, 3, 0];

        while !machine.halted {
            machine.step();
        }

        assert_eq!(machine.output, vec![4, 2, 5, 6, 7, 7, 7, 7, 3, 1, 0]);
        assert_eq!(machine.a, 0);
    }

    // If register B contains 29, the program 1,7 would set register B to 26.
    #[test]
    fn test_instruction_4() {
        let mut machine = Machine::default();
        machine.b = 29;
        machine.ram = vec![1, 7];

        machine.step();

        assert_eq!(machine.b, 26);
    }

    // If register B contains 2024 and register C contains 43690, the program 4,0 would set register B to 44354.
    #[test]
    fn test_instruction_5() {
        let mut machine = Machine::default();
        machine.b = 2024;
        machine.c = 43690;
        machine.ram = vec![4, 0];

        machine.step();

        assert_eq!(machine.b, 44354);
    }
}

// For codspeed
pub fn part1(input: &str) -> String {
    part1_v1(&parse(input)).to_string()
}

pub fn part2(input: &str) -> String {
    part2_backtrack(&parse(input)).to_string()
}
