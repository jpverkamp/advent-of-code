use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum Condition {
    Operational,
    Damaged,
    Unknown,
}
impl Condition {
    pub fn is_known(&self) -> bool {
        !matches!(self, Condition::Unknown)
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Spring {
    pub conditions: Vec<Condition>,
    pub groups: Vec<u64>,
}

impl Spring {
    pub fn is_valid(&self) -> bool {
        let (groups, next_group) = self.current_groups();

        if !self.groups.starts_with(&groups) {
            return false;
        }

        if groups.len() < self.groups.len()
            && next_group > 0
            && self.groups[groups.len()] < next_group
        {
            return false;
        }

        true
    }

    pub fn is_known(&self) -> bool {
        self.conditions.iter().all(|c| c.is_known())
    }

    pub fn is_correct(&self) -> bool {
        self.is_known() && self.groups == self.current_groups().0
    }

    pub fn current_groups(&self) -> (Vec<u64>, u64) {
        use Condition::*;

        let mut groups = vec![];
        let mut group: u64 = 0;
        let mut previous = Condition::Damaged;

        for current in self.conditions.iter().chain(std::iter::once(&Damaged)) {
            match (previous, current) {
                // Continuing this group
                (Operational, Operational) => group += 1,
                // Ending a group
                (Operational, Damaged) => {
                    groups.push(group);
                    group = 0;
                }
                // Starting a new group
                (Damaged, Operational) => group = 1,
                // Currently not in a group
                (Damaged, Damaged) => {}
                // If we hit an unknown, bail early with the current groups
                (_, Unknown) => break,
                _ => panic!("Invalid state"),
            }
            previous = *current;
        }

        (groups, group)
    }
}

impl Display for Spring {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.conditions
                .iter()
                .map(|c| match c {
                    Condition::Operational => "#",
                    Condition::Damaged => ".",
                    Condition::Unknown => "?",
                })
                .collect::<String>()
        )
    }
}
