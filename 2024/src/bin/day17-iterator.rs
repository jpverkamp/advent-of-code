use aoc2024::day17;

fn main() {
    let input = day17::parse(include_str!("../../input/2024/day17.txt"));

    println!("{}\n", input.decompile());

    println!("{l1:<10} {l2:<10} Output", l1 = "Input A", l2 = "Octal",);

    for i in 0.. {
        let mut machine = input.clone();
        machine.a = i;
        machine.run();

        println!("{i:<10} {i:<10o} {output:?}", output = machine.output);

        if machine.output == input.ram {
            break;
        }

        if i > 100_000 {
            break;
        }
    }
}
