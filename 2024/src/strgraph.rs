use hashbrown::{HashMap, HashSet};
use itertools::Itertools;

#[derive(Debug)]
pub struct StrGraph<'a> {
    nodes: HashSet<&'a str>,
    edges: HashSet<(&'a str, &'a str)>,
    neighbors: HashMap<&'a str, HashSet<&'a str>>,
}

impl<'a> From<&'a str> for StrGraph<'a> {
    fn from(input: &'a str) -> Self {
        let mut nodes = HashSet::new();
        let mut edges = HashSet::new();
        let mut neighbors = HashMap::new();

        for line in input.lines() {
            let mut parts = line.split('-');
            let a = parts.next().unwrap();
            let b = parts.next().unwrap();

            nodes.insert(a);
            nodes.insert(b);

            edges.insert((a, b));
            edges.insert((b, a));

            neighbors.entry(a).or_insert_with(HashSet::new).insert(b);
            neighbors.entry(b).or_insert_with(HashSet::new).insert(a);
        }

        StrGraph {
            nodes,
            edges,
            neighbors,
        }
    }
}

impl<'a> StrGraph<'a> {
    pub fn nodes(&'a self) -> impl Iterator<Item = &'a str> + 'a {
        self.nodes.iter().copied()
    }

    pub fn edges(&'a self) -> impl Iterator<Item = (&'a str, &'a str)> + 'a {
        self.edges.iter().copied()
    }

    pub fn has_edge(&self, a: &'a str, b: &'a str) -> bool {
        // NOTE: This should never hit the second case, since I'm inserting both earlier
        self.edges.contains(&(a, b)) || self.edges.contains(&(b, a))
    }

    pub fn neighbors(&'a self, node: &'a str) -> impl Iterator<Item = &'a str> + 'a {
        self.neighbors.get(node).unwrap().iter().copied()
    }

    pub fn is_completely_connected(&self, nodes: &[&'a str]) -> bool {
        nodes
            .iter()
            .all(|&n| nodes.iter().all(|&c| n == c || self.has_edge(n, c)))
    }

    // This will return *a* completely connected component for a node
    pub fn completely_connected(&self, node: &'a str) -> HashSet<&'a str> {
        let mut connected = HashSet::new();
        connected.insert(node);

        // For each node, add it if it's connected to all other added nodes
        for &n in self.nodes.iter().sorted() {
            if !connected.iter().all(|&c| self.has_edge(n, c)) {
                continue;
            }

            connected.insert(n);
        }

        connected
    }

    // This will return the *largest* completely connected component for a node
    pub fn largest_completely_connected(&self, node: &'a str) -> HashSet<&'a str> {
        fn recur<'a>(
            nodes: &HashSet<&'a str>,
            neighbors: &HashMap<&'a str, HashSet<&'a str>>,
            component: Vec<&'a str>,
        ) -> Vec<&'a str> {
            nodes
                .iter()
                // Don't check nodes we've already done
                .filter(|&n| !component.contains(n))
                // Check if all neighbors are in the component
                .filter(|&n| {
                    component
                        .iter()
                        .all(|&c| neighbors.get(n).unwrap().contains(c))
                })
                // Recur adding that component
                .map(|n| {
                    recur(nodes, neighbors, {
                        let mut component = component.clone();
                        component.push(n);
                        component
                    })
                })
                // Which is the largest
                .max_by(|a, b| a.len().cmp(&b.len()))
                // If we didn't find a larger child, return all
                .unwrap_or_else(|| component.to_vec())
        }

        recur(&self.nodes, &self.neighbors, vec![node])
            .into_iter()
            .collect()
    }
}
