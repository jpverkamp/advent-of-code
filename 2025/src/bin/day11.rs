use std::collections::HashMap;

aoc::main!(day11);

#[aoc::register]
fn part1(input: &str) -> impl Into<String> {
    let mut outputs = HashMap::new();

    for line in input.lines() {
        let (src, dsts) = line.split_once(": ").unwrap();
        for dst in dsts.split_whitespace() {
            outputs.entry(src).or_insert_with(Vec::new).push(dst);
        }
    }

    // Assuming no loops
    fn paths(outputs: &HashMap<&str, Vec<&str>>, src: &str, dst: &str) -> usize {
        if src == dst {
            return 1;
        }

        let mut total = 0;
        if let Some(dsts) = outputs.get(src) {
            for next in dsts {
                total += paths(outputs, next, dst);
            }
        }
        total
    }

    paths(&outputs, "you", "out").to_string()
}

#[aoc::register]
fn part2(input: &str) -> impl Into<String> {
    let mut outputs = HashMap::new();

    for line in input.lines() {
        let (src, dsts) = line.split_once(": ").unwrap();
        for dst in dsts.split_whitespace() {
            outputs.entry(src).or_insert_with(Vec::new).push(dst);
        }
    }

    // Assuming no loops
    fn paths(
        outputs: &HashMap<&str, Vec<&str>>,
        src: &str,
        dst: &str,
        targets: &Vec<&str>,
    ) -> usize {
        if src == dst {
            if targets.is_empty() {
                return 1;
            } else {
                return 0;
            }
        }

        // Side path: if we're on a target node, remove it from targets
        if targets.contains(&src) {
            let mut new_targets = targets.clone();
            new_targets.retain(|&x| x != src);
            return paths(outputs, src, dst, &new_targets);
        }

        // Otherwise, recur
        let mut total = 0;
        if let Some(dsts) = outputs.get(src) {
            for next in dsts {
                total += paths(outputs, next, dst, targets);
            }
        }
        total
    }

    paths(&outputs, "svr", "out", &vec!["dac", "fft"]).to_string()
}

#[aoc::register]
fn part2_memo(input: &str) -> impl Into<String> {
    let mut outputs = HashMap::new();

    let mut ids = vec![];

    for line in input.lines() {
        let (src, dsts) = line.split_once(": ").unwrap();

        if !ids.contains(&src) {
            ids.push(src);
        }
        let src_id = ids.iter().position(|&x| x == src).unwrap();

        for dst in dsts.split_whitespace() {
            if !ids.contains(&dst) {
                ids.push(dst);
            }
            let dst_id = ids.iter().position(|&x| x == dst).unwrap();

            outputs.entry(src_id).or_insert_with(Vec::new).push(dst_id);
        }
    }

    // Still assuming no loops
    // This time, we have to go from src to dst and hit all of targets
    // We'll memoize on (src, dst, remaining targets)
    // In order to memoize and not deal with lifetimes, we'll use indices instead of &strs
    fn paths(
        outputs: &HashMap<usize, Vec<usize>>,
        src: usize,
        dst: usize,
        targets: &Vec<usize>,
        memo: &mut HashMap<(usize, usize, Vec<usize>), usize>,
    ) -> usize {
        if let Some(&cached) = memo.get(&(src, dst, targets.clone())) {
            return cached;
        }

        if src == dst {
            if targets.is_empty() {
                return 1;
            } else {
                return 0;
            }
        }

        // Side path: if we're on a target node, remove it from targets
        if targets.contains(&src) {
            let mut new_targets = targets.clone();
            new_targets.retain(|&x| x != src);
            return paths(outputs, src, dst, &new_targets, memo);
        }

        // Otherwise, recur
        let mut total = 0;
        if let Some(dsts) = outputs.get(&src) {
            for next in dsts {
                total += paths(outputs, *next, dst, targets, memo);
            }
        }

        memo.insert((src, dst, targets.clone()), total);

        total
    }

    let svr_id = ids.iter().position(|&x| x == "svr").unwrap();
    let out_id = ids.iter().position(|&x| x == "out").unwrap();
    let dac_id = ids.iter().position(|&x| x == "dac").unwrap();
    let fft_id = ids.iter().position(|&x| x == "fft").unwrap();

    paths(
        &outputs,
        svr_id,
        out_id,
        &vec![dac_id, fft_id],
        &mut HashMap::new(),
    )
    .to_string()
}

aoc::test!(
    text = "\
aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out
", 
    [part1] => "5"
);

aoc::test!(
    text = "\
svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out
", 
    [part2] => "2"
);

aoc::test!(
    file = "input/2025/day11.txt",
    [part1] => "TODO",
    [part2] => "TODO"
);
