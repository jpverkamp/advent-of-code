use aoc_runner_derive::{aoc, aoc_generator};
use hashbrown::{HashMap, HashSet};
use itertools::Itertools;

use crate::strgraph::StrGraph;

#[aoc_generator(day23)]
fn parse(input: &str) -> String {
    input.to_string()
}

#[aoc(day23, part1, v1)]
fn part1_v1(input: &str) -> usize {
    let g = StrGraph::from(input);
    let mut count = 0;

    let mut sorted_nodes = g.nodes().collect::<Vec<_>>();
    sorted_nodes.sort();

    for (i, a) in sorted_nodes.iter().enumerate() {
        for (j, b) in sorted_nodes.iter().skip(i + 1).enumerate() {
            if !g.has_edge(a, b) {
                continue;
            }

            for c in sorted_nodes.iter().skip(i + j + 1) {
                if !g.has_edge(a, c) || !g.has_edge(b, c) {
                    continue;
                }

                if a.starts_with('t') || b.starts_with('t') || c.starts_with('t') {
                    count += 1;
                }
            }
        }
    }

    count
}

#[aoc(day23, part2, sorted_complete)]
fn part2_sorted_complete(input: &str) -> String {
    let g = StrGraph::from(input);
    let mut checked = HashSet::new();

    // NOTE: This would not actually work in general

    // Basically, if we have something like this:

    //     D
    //     |
    //     C
    //    / \
    // A-B---F-E

    // Assume our HashSet returns things in alphabetical order (as a worst case)
    // 1. We'll start with A, get A,B; so we'll skip B
    // 2. Then we'll do D, get C,D; so we'll skip D
    // 3. Then we'll do E, get E,F; so we'll skip F
    // We never return the component B,C,F

    // Even removing the checked.contains doesn't actually fix this, in the worst case
    // Since if we get an ordering such that A and B return A,B and C and D return C,D; etc
    // We'll still never see the component B,C,F
    // What we need is completely_connected to return the *largest* component for a node
    // But this is significantly more expensive

    g.nodes()
        .sorted() // This at least guarantees we'll get the same ordering
        .filter_map(|n| {
            if checked.contains(n) {
                None
            } else {
                let component = g.completely_connected(n);
                for &n in component.iter() {
                    checked.insert(n);
                }
                Some(component)
            }
        })
        .max_by(|a, b| a.len().cmp(&b.len()))
        .map(|c| c.iter().sorted().join(","))
        .unwrap()
}

// #[aoc(day23, part2, largest_complete)]
#[allow(dead_code)]
fn part2_largest_complete(input: &str) -> String {
    let g = StrGraph::from(input);
    let mut checked = HashSet::new();

    // The 'significantly more expensive' version

    g.nodes()
        .filter_map(|n| {
            if checked.contains(n) {
                None
            } else {
                let component = g.largest_completely_connected(n);
                for &n in component.iter() {
                    checked.insert(n);
                }
                Some(component)
            }
        })
        .max_by(|a, b| a.len().cmp(&b.len()))
        .map(|c| c.iter().sorted().join(","))
        .unwrap()
}

// #[aoc(day23, part2, recur_memo)]
#[allow(dead_code)]
fn part2_recur_memo(input: &str) -> String {
    let graph = StrGraph::from(input);

    fn largest_completely_connected_subgraph<'a>(
        graph: &StrGraph,
        cache: &mut HashMap<Vec<&'a str>, Vec<&'a str>>,
        nodes: Vec<&'a str>,
    ) -> Vec<&'a str> {
        // We've already cached this result
        if let Some(result) = cache.get(&nodes) {
            return result.clone();
        }

        // Check if we're already completely connected
        if graph.is_completely_connected(&nodes) {
            cache.insert(nodes.clone(), nodes.clone());
            return nodes;
        }

        // Otherwise, for each node, try removing it and recurring
        let mut largest: Option<Vec<&'a str>> = None;
        for (i, _) in nodes.iter().enumerate() {
            let mut new_nodes = nodes.clone();
            new_nodes.remove(i);
            let result = largest_completely_connected_subgraph(graph, cache, new_nodes);

            if largest.is_none() || result.len() > largest.as_ref().unwrap().len() {
                largest = Some(result);
            }
        }
        let largest = largest.unwrap();

        cache.insert(nodes, largest.clone());
        largest
    }

    let nodes = graph.nodes().sorted().collect::<Vec<_>>();
    let mut cache = HashMap::new();

    largest_completely_connected_subgraph(&graph, &mut cache, nodes)
        .iter()
        .sorted()
        .join(",")
}

#[aoc(day23, part2, most_connected)]
fn part2_most_connected(input: &str) -> String {
    let graph = StrGraph::from(input);

    // Order nodes by descending number of neighbors
    // For each, check if removing any single neighbor is connected
    // Any nodes with higher order than the connected component will be checked first
    // But due to the structure of our graph, this way is efficient
    graph
        .nodes()
        .map(|node| (node, graph.neighbors(node).count()))
        .sorted_by(|a, b| a.1.cmp(&b.1))
        .rev()
        .find_map(|(node, _)| {
            let neighbors = graph.neighbors(node).collect::<Vec<_>>();

            // Try removing each single neighbor
            for neighbor in neighbors.iter() {
                let mut neighbors_without = neighbors.clone();
                neighbors_without.retain(|&n| n != *neighbor);

                if graph.is_completely_connected(&neighbors_without) {
                    neighbors_without.push(node);
                    return Some(neighbors_without.iter().sorted().join(","));
                }
            }

            // If we made it here, we couldn't find a solution removing more than 1 neighbor
            None
        })
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::make_test;

    const EXAMPLE: &str = "\
kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn";

    // Constructed to fail part2_largest_complete
    // See the comment in that function
    //
    //     D
    //     |
    //     C
    //    / \
    // A-B---F-E
    const EXAMPLE2: &str = "\
a-b
b-c
c-d
c-f
f-e
b-f";

    make_test!([part1_v1] => "day23.txt", 7, 1467);
    make_test!([part2_sorted_complete, part2_largest_complete, part2_recur_memo, part2_most_connected] => "day23.txt", "co,de,ka,ta", "di,gs,jw,kz,md,nc,qp,rp,sa,ss,uk,xk,yn");

    macro_rules! make_example2_tests {
        ($($function:ident),*) => {
            $(
                paste::paste! {
                    #[test]
                    fn [<test_ $function _example2>]() {
                        assert_eq!($function(EXAMPLE2), "b,c,f");
                    }
                }
            )*
        }
    }

    make_example2_tests!(
        part2_sorted_complete,
        part2_recur_memo,
        part2_most_connected
    );

    // This is constructed to fail due to the ordering of the graph
    #[test]
    #[ignore]
    fn test_part2_largest_complete_example2() {
        assert_eq!(part2_largest_complete(EXAMPLE2), "b,c,f");
    }
}
