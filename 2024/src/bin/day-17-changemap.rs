use aoc2024::{day17, Grid};

fn main() {
    let input = day17::parse(include_str!("../../input/2024/day17.txt"));

    println!("{}", input.decompile());

    fn decimalize(ram: &[u8]) -> u128 {
        let mut a = 0;
        for nibble in ram.iter() {
            a = (a << 3) | (*nibble as u128);
        }
        a
    }

    let mut a = input.ram.clone();
    let mut change_map = Grid::new(input.ram.len(), input.ram.len());

    // Which bytes change which bytes
    for index in 0..input.ram.len() {
        let mut machine = input.clone();
        machine.a = decimalize(&a);
        machine.run();
        let output_1 = machine.output.clone();

        for new_value in 0..8 {
            let mut machine = input.clone();
            a[index] = new_value;
            machine.a = decimalize(&a);
            machine.run();
            let output_2 = machine.output.clone();

            output_1
                .iter()
                .zip(output_2.iter())
                .enumerate()
                .filter(|(_, (a, b))| a != b)
                .for_each(|(i, _)| {
                    change_map.set((index, i), true);
                });
        }
    }

    println!(
        "{}",
        change_map.to_string(&|b| (if *b { 'X' } else { '.' }).to_string())
    );
}
