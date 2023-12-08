// A game consists of an ID and some number of rounds each with some number of dice
#[derive(Debug, PartialEq)]
pub struct Game {
    pub id: u32,
    pub rounds: Vec<Round>,
}

// The power of a game is the product of the minumum number of cubes of each color
impl Game {
    pub fn power(&self) -> u32 {
        self.rounds
            .iter()
            .fold([0, 0, 0], |acc, round| {
                [
                    acc[0].max(round.red),
                    acc[1].max(round.green),
                    acc[2].max(round.blue),
                ]
            })
            .into_iter()
            .product()
    }
}

// A single round can have some number each of red/green/blue dice
#[derive(Debug, PartialEq)]
pub struct Round {
    pub red: u32,
    pub green: u32,
    pub blue: u32,
}

// Represents colors of dice
#[derive(Debug, PartialEq)]
pub enum Colors {
    Red,
    Green,
    Blue,
}
