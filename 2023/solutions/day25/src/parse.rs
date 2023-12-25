use fxhash::FxHashMap;
use nom::{
    character::complete::{self, alpha1, line_ending, space0, space1},
    multi::separated_list1,
    sequence::{separated_pair, terminated},
    IResult,
};
use petgraph::prelude::*;

fn label(input: &str) -> IResult<&str, &str> {
    alpha1(input)
}

fn edges(input: &str) -> IResult<&str, (&str, Vec<&str>)> {
    separated_pair(
        label,
        terminated(complete::char(':'), space0),
        separated_list1(space1, label),
    )(input)
}

pub fn read(input: &str) -> UnGraph<&str, ()> {
    let (s, lines) = separated_list1(line_ending, edges)(input).unwrap();
    assert!(s.trim().is_empty());

    let mut graph = UnGraph::new_undirected();
    let mut nodes = FxHashMap::default();
    let mut edges = Vec::new();

    for (label, targets) in lines {
        if !nodes.contains_key(label) {
            nodes.insert(label, graph.add_node(label));
        }
        
        for target in targets { 
            if !nodes.contains_key(target) {
                nodes.insert(target, graph.add_node(target));
            }

            graph.add_edge(nodes[label], nodes[target], ());
            edges.push((label, target));
        }
    }

    graph
}
