use aoc2024::day5;

// Used to determine/verify that the ordering is *not* transitive
fn main() {
    let filename = std::env::args().nth(1).expect("missing filename argument");
    let content = std::fs::read_to_string(filename).expect("cannot read file");

    let (ordering, _) = day5::parse(&content);

    for a in ordering.values() {
        for b in ordering.values() {
            let proceeds = ordering.can_preceed(a, b);
            let proceeds_transitive = ordering.can_preceed_transitive(a, b);

            if proceeds_transitive && !proceeds {
                println!("{a} {b} {:?}", ordering.can_preceed_transitive_path(a, b));
            }
        }
    }
}
