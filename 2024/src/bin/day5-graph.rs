// Attempt to graph the input for day 5
// It ... doesn't really show anything useful, it's too large
fn main() {
    let filename = std::env::args().nth(1).expect("missing filename argument");
    let content = std::fs::read_to_string(filename).expect("cannot read file");

    println!("graph G {{");

    for line in content.lines() {
        if line.is_empty() {
            break;
        }

        let (a, b) = line.split_once('|').unwrap();
        println!("    {} -- {};", a, b);
    }

    println!("}}");
}
