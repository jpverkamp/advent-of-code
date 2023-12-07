use std::{cmp::Ordering, collections::HashMap};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Hash)]
pub enum Card {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl From<char> for Card {
    fn from(c: char) -> Self {
        match c {
            '*' => Card::Joker,
            '2' => Card::Two,
            '3' => Card::Three,
            '4' => Card::Four,
            '5' => Card::Five,
            '6' => Card::Six,
            '7' => Card::Seven,
            '8' => Card::Eight,
            '9' => Card::Nine,
            'T' => Card::Ten,
            'J' => Card::Jack,
            'Q' => Card::Queen,
            'K' => Card::King,
            'A' => Card::Ace,
            _ => panic!("Invalid card: {}", c),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Hand {
    pub cards: [Card; 5],
    pub bid: u64,
}

impl Hand {
    fn counts(&self) -> Vec<usize> {
        let mut counts: HashMap<Card, usize> =
            self.cards
                .into_iter()
                .fold(HashMap::new(), |mut counts, c| {
                    *counts.entry(c).or_default() += 1;
                    counts
                });

        // Special case for part 2, if we have any jokers assign them to the otherwise largest group
        // For 5 jokers, treat as 5 Aces (but this won't actually matter)
        if counts.contains_key(&Card::Joker) {
            let best_type = counts.iter().fold(Card::Ace, |best_type, (&k, &v)| {
                // Update if non-joker with more than current
                // If there's nothing but jokers, replace with Aces
                if k != Card::Joker && v > *(counts.get(&best_type).unwrap_or(&0)) {
                    k
                } else {
                    best_type
                }
            });

            *counts.entry(best_type).or_default() += counts[&Card::Joker];
            counts.remove(&Card::Joker);
        }

        let mut counts = counts.values().cloned().collect::<Vec<_>>();
        counts.sort();
        counts.reverse();
        counts
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_counts = self.counts();
        let other_counts = other.counts();

        // Counts are sorted in descending order, so we can compare them directly
        // IE five of a kind is <5>, four of a is <4, 1>, full house is <3, 2>, three of a is <3, 1, 1> etc
        // If two counts are the same, compare the cards lexicographically (using Ord on Card)
        if self_counts == other_counts {
            self.cards.cmp(&other.cards)
        } else {
            self_counts.cmp(&other_counts)
        }
    }

    fn max(self, other: Self) -> Self
    where
        Self: Sized,
    {
        std::cmp::max_by(self, other, Ord::cmp)
    }

    fn min(self, other: Self) -> Self
    where
        Self: Sized,
    {
        std::cmp::min_by(self, other, Ord::cmp)
    }

    fn clamp(self, _min: Self, _max: Self) -> Self
    where
        Self: Sized,
        Self: PartialOrd,
    {
        unimplemented!("Hand::clamp")
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
