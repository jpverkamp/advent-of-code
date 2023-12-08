#[allow(dead_code)]
#[derive(Debug)]
pub struct Card {
    pub id: u32,
    pub winning_numbers: Vec<u8>,
    pub guesses: Vec<u8>,
}

impl Card {
    pub fn matches(&self) -> usize {
        self.guesses
            .iter()
            .filter(|guess: &&u8| self.winning_numbers.contains(guess))
            .count()
    }
}
