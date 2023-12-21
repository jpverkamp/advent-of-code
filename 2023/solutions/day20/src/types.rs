use fxhash::FxHashMap;

#[derive(Debug, Clone)]
pub enum ModuleType<'a> {
    Broadcast,
    FlipFlop(bool),
    Conjunction(FxHashMap<&'a str, Pulse>),
    Output,
}

#[derive(Debug, Clone)]
pub struct Module<'a> {
    pub label: &'a str,
    pub module_type: ModuleType<'a>,
    pub outputs: Vec<&'a str>,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Pulse {
    High,
    Low,
}
