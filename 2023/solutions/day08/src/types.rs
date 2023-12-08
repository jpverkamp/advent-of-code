use std::collections::BTreeMap;

#[derive(Debug, Clone, Copy)]
pub enum Move {
    Left,
    Right,
}

pub type Label = [char; 3];

#[derive(Debug, Clone, Copy)]
pub struct Neighbors {
    pub left: Label,
    pub right: Label,
}

#[derive(Debug, Clone)]
pub struct Simulation {
    pub moves: Vec<Move>,
    pub neighbors: BTreeMap<Label, Neighbors>,
}
