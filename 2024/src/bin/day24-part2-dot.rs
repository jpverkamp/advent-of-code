use std::{
    io::Write,
    process::{Command, Stdio},
};

use aoc2024::day24::Machine;

fn main() {
    let input = include_str!("../../input/2024/day24.txt");
    let graph = Machine::from(input);

    for limit in [7, 45] {
        let dot = graph.to_graphviz_limited(limit);
        let filename = format!("day24-limit{}.dot", limit);

        println!("Generating {filename}");
        std::fs::write(filename, dot.clone()).unwrap();

        for format in ["png", "svg"] {
            let filename = format!("day24-limit{}.{}", limit, format);
            println!("Rendering {filename}");

            let output = std::fs::File::create(filename).unwrap();
            let mut child = Command::new("dot")
                .arg(format!("-T{format}"))
                .stdin(Stdio::piped())
                .stdout(output)
                .spawn()
                .unwrap();

            let stdin = child.stdin.as_mut().unwrap();
            stdin.write_all(dot.as_bytes()).unwrap();
        }
    }
}
