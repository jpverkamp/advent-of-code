use aoc2024::day17;

fn main() {
    let input = day17::parse(include_str!("../../input/2024/day17.txt"));

    println!("{}\n", input.decompile());

    let start = std::time::Instant::now();

    for i in 0.. {
        if i % 1_000_000 == 0 {
            println!("{} {:?}", i, start.elapsed());
        }

        let mut machine = input.clone();
        machine.a = i;
        machine.run();

        if machine.output == input.ram {
            break;
        }
    }
}
