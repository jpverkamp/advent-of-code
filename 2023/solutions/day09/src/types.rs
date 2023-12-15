use itertools::Itertools;

#[derive(Debug)]
pub struct Equation {
    pub terms: Vec<i64>,
}

impl Equation {
    // Generate a 'stack' of vecs, starting from the original terms
    // Each subsequent vec is the vec of differences in terms (and thus 1 element shorter)
    // Stop when that list is all 0s
    pub fn stack(&self) -> Vec<Vec<i64>> {
        let mut stack = vec![];
        stack.push(self.terms.clone());

        loop {
            let bottom = stack.last().unwrap();

            if bottom.iter().all(|t| *t == 0) {
                return stack;
            }

            stack.push(
                bottom
                    .iter()
                    .tuple_windows()
                    .map(|(a, b)| *b - *a)
                    .collect(),
            );
        }
    }
}
