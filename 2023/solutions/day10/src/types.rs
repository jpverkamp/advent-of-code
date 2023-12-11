#[derive(Debug)]
pub struct Map {
    nodes: Vec<Node>,
    start_index: usize,
}

// Parse a Map from a &str
impl From<&str> for Map {
    fn from(value: &str) -> Self {
        let raw_nodes = value
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .filter(|(_, c)| *c != '.')
                    .map(move |(x, value)| (x as isize, y as isize, value))
            })
            .collect::<Vec<_>>();

        let start_index = raw_nodes
            .iter()
            .position(|(_, _, value)| *value == 'S')
            .unwrap();

        fn index_offset(
            raw_nodes: &[(isize, isize, char)],
            x: isize,
            y: isize,
            xd: isize,
            yd: isize,
        ) -> Option<usize> {
            raw_nodes
                .iter()
                .position(|(x2, y2, _)| x + xd == *x2 && y + yd == *y2)
        }

        // Forward is first clockwise from up
        // Backwards is second

        let mut nodes = raw_nodes
            .iter()
            .map(|(x, y, value)| Node {
                x: *x,
                y: *y,
                value: *value,
                neighbor_a_index: match *value {
                    // Up
                    '|' | 'L' | 'J' => index_offset(&raw_nodes, *x, *y, 0, -1),
                    // Right (but no up)
                    '-' | 'F' => index_offset(&raw_nodes, *x, *y, 1, 0),
                    // Down (but not right or up)
                    '7' => index_offset(&raw_nodes, *x, *y, 0, 1),
                    // Left (can't have the first one go only left)
                    // Ignore S (we'll figure this out later)
                    'S' => None,
                    // Break on anything else
                    _ => panic!("Invalid value: {}", value),
                },
                neighbor_b_index: match *value {
                    // Up (can't have the second one go up)
                    // Right (first must have gone up)
                    'L' => index_offset(&raw_nodes, *x, *y, 1, 0),
                    // Down (first must have gone up or right)
                    '|' | 'F' => index_offset(&raw_nodes, *x, *y, 0, 1),
                    // Left (anything else really)
                    '-' | 'J' | '7' => index_offset(&raw_nodes, *x, *y, -1, 0),
                    // Ignore S (we'll figure this out later)
                    'S' => None,
                    // Break on anything else
                    _ => panic!("Invalid value: {}", value),
                },
            })
            .collect::<Vec<_>>();

        // The start node has exactly two neighbors; find them
        let start_neighbors = nodes
            .iter()
            .enumerate()
            .filter_map(|(i, node)| {
                if node.neighbor_a_index.is_some_and(|j| j == start_index)
                    || node.neighbor_b_index.is_some_and(|j| j == start_index)
                {
                    Some(i)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        assert_eq!(start_neighbors.len(), 2);

        nodes[start_index].neighbor_a_index = Some(start_neighbors[0]);
        nodes[start_index].neighbor_b_index = Some(start_neighbors[1]);

        Map { nodes, start_index }
    }
}

impl Map {
    pub fn bounds(&self) -> (isize, isize, isize, isize) {
        let mut min_x = isize::MAX;
        let mut min_y = isize::MAX;
        let mut max_x = isize::MIN;
        let mut max_y = isize::MIN;

        for node in self.nodes.iter() {
            min_x = min_x.min(node.x);
            min_y = min_y.min(node.y);
            max_x = max_x.max(node.x);
            max_y = max_y.max(node.y);
        }

        (min_x, min_y, max_x, max_y)
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Node {
    x: isize,
    y: isize,
    value: char,
    neighbor_a_index: Option<usize>,
    neighbor_b_index: Option<usize>,
}

impl Node {
    pub fn value(&self) -> char {
        self.value
    }

    pub fn x(&self) -> isize {
        self.x
    }

    pub fn y(&self) -> isize {
        self.y
    }
}

// An iterator over the nodes in Map
// Starts at the start node
// Returns each node (on the loop) once
#[derive(Debug, Copy, Clone)]
pub struct MapIterator<'a> {
    map: &'a Map,
    current_index: Option<usize>,
    previous_index: Option<usize>,
    fresh: bool,
}

impl<'a> Iterator for MapIterator<'a> {
    type Item = &'a Node;

    fn next(&mut self) -> Option<Self::Item> {
        // If we manage to run off a trail, something went wrong, but this will stop iter
        self.current_index?;

        // The node we're about to return
        let node = &self.map.nodes[self.current_index.unwrap()];

        // Only return the Start node once
        if !self.fresh && node.value == 'S' {
            return None;
        }

        // Find the next node, if 'a' points to the one we were just at, use 'b' instead
        let mut next_index = node.neighbor_a_index;
        if next_index == self.previous_index {
            next_index = node.neighbor_b_index;
        }

        self.previous_index = self.current_index;
        self.current_index = next_index;
        self.fresh = false;

        Some(node)
    }
}

impl Map {
    pub fn iter(&self) -> MapIterator {
        MapIterator {
            map: self,
            current_index: Some(self.start_index),
            previous_index: None,
            fresh: true,
        }
    }
}
