use aoc::*;
use std::{
    collections::{HashMap, VecDeque},
    path::Path,
};
use Instruction::*;

/* ----- A single instruction for the virtual machine ----- */
#[derive(Copy, Clone, Debug)]
enum Instruction {
    Noop,
    AddX(isize),
}

impl Instruction {
    fn cycles(self) -> usize {
        match self {
            Noop => 1,
            AddX(_) => 2,
        }
    }
}

impl From<String> for Instruction {
    fn from(line: String) -> Self {
        let mut parts = line.split_ascii_whitespace();

        match parts.next().expect("must have a first part") {
            "noop" => Noop,
            "addx" => {
                let v = parts
                    .next()
                    .expect("addx must have a value")
                    .parse::<isize>()
                    .expect("addx value must be numeric");

                AddX(v)
            }
            _ => panic!("unknown instruction format {:?}", line),
        }
    }
}

/* ----- Implement a simple virtual machine ----- */
#[derive(Debug)]
struct VM {
    instructions: Vec<Instruction>,
    program_counter: usize,
    time_counter: usize,
    delayed_instructions: VecDeque<Vec<Instruction>>,
    registers: HashMap<String, isize>,
    previous_registers: HashMap<String, isize>,
}

impl VM {
    fn new(instructions: Vec<Instruction>) -> Self {
        VM {
            instructions,
            program_counter: 0,
            time_counter: 0,
            delayed_instructions: VecDeque::new(),
            registers: HashMap::new(),
            previous_registers: HashMap::new(),
        }
    }

    fn step(&mut self) {
        self.time_counter += 1;

        match self.delayed_instructions.get(0) {
            // We have a current instruction, don't queue any more
            Some(v) if !v.is_empty() => {}

            // We don't have a current instruction, queue one
            _ => {
                let instruction = self.instructions.get(self.program_counter).unwrap();
                let cycles = instruction.cycles();

                while self.delayed_instructions.len() < cycles {
                    self.delayed_instructions.push_back(Vec::new());
                }

                self.delayed_instructions
                    .get_mut(cycles - 1)
                    .unwrap()
                    .push(*instruction);

                self.program_counter += 1;
            }
        }

        // Copy the registers
        for (k, v) in self.registers.iter() {
            self.previous_registers.insert(k.clone(), *v);
        }

        // Run any current instructions
        for instructions in self.delayed_instructions.pop_front() {
            for instruction in instructions {
                self.eval(instruction);
            }
        }
    }

    #[allow(dead_code)]
    fn pipelined_step(&mut self) {
        // Add the current instruction to the correct delay cycle
        if self.program_counter < self.instructions.len() {
            let instruction = self.instructions.get(self.program_counter).unwrap();
            let cycles = instruction.cycles();

            while self.delayed_instructions.len() < cycles + 1 {
                self.delayed_instructions.push_back(Vec::new());
            }

            self.delayed_instructions
                .get_mut(cycles)
                .unwrap()
                .push(*instruction);
        }

        // Copy the registers
        for (k, v) in self.registers.iter() {
            self.previous_registers.insert(k.clone(), *v);
        }

        // Pop and run all currently delay instructions
        for instructions in self.delayed_instructions.pop_front() {
            for instruction in instructions {
                self.eval(instruction);
            }
        }

        // Increment program counter
        self.program_counter += 1;
    }

    fn is_finished(&self) -> bool {
        self.program_counter >= self.instructions.len() && self.delayed_instructions.is_empty()
    }

    fn eval(&mut self, instruction: Instruction) {
        match instruction {
            Noop => {}
            AddX(v) => {
                self.registers.insert(
                    String::from("X"),
                    self.registers.get("X").or(Some(&(1 as isize))).unwrap() + v,
                );
            }
        }
    }
}

fn part1(filename: &Path) -> String {
    let instructions = iter_lines(filename).map(Instruction::from).collect();
    let mut vm = VM::new(instructions);

    let mut sample_sum = 0;

    loop {
        vm.step();

        if cfg!(debug_assertions) {
            println!(
                "[{:4}] [{:4}] {:?}, {:?}",
                vm.time_counter, vm.program_counter, vm.registers, vm.delayed_instructions
            );
        }

        match vm.time_counter {
            20 | 60 | 100 | 140 | 180 | 220 => {
                let signal = vm.time_counter as isize * *vm.previous_registers.get("X").unwrap();
                sample_sum += signal;
            }
            _ => {}
        }

        if vm.is_finished() {
            break;
        }
    }

    sample_sum.to_string()
}

fn part2(filename: &Path) -> String {
    let instructions = iter_lines(filename).map(Instruction::from).collect();
    let mut vm = VM::new(instructions);

    let mut output_buffer = String::new();
    let mut crt_x = 0;

    loop {
        vm.step();

        let sprite_center_x = *vm.previous_registers.get("X").or(Some(&1)).unwrap();
        let c = if crt_x >= sprite_center_x - 1 && crt_x <= sprite_center_x + 1 {
            '#'
        } else {
            '.'
        };

        output_buffer.push(c);

        crt_x += 1;
        if crt_x >= 40 {
            output_buffer.push('\n');
            crt_x = 0;
        }

        if vm.is_finished() {
            break;
        }
    }

    output_buffer.to_string()
}

fn main() {
    aoc_main(part1, part2);
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};
    use aoc::aoc_test;

    #[test]
    fn test1() {
        aoc_test("10", part1, "15140")
    }

    #[test]
    fn test2() {
        aoc_test(
            "10",
            part2,
            "\
###..###....##..##..####..##...##..###..
#..#.#..#....#.#..#....#.#..#.#..#.#..#.
###..#..#....#.#..#...#..#....#..#.#..#.
#..#.###.....#.####..#...#.##.####.###..
#..#.#....#..#.#..#.#....#..#.#..#.#....
###..#.....##..#..#.####..###.#..#.#....
",
        );
    }
}
