use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Suit {
    Diamonds,
    Spades,
    Hearts,
    Clubs,
}

impl PartialOrd for Suit {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Suit {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_value = match self {
            Suit::Diamonds => 0,
            Suit::Spades => 1,
            Suit::Hearts => 2,
            Suit::Clubs => 3,
        };

        let other_value = match other {
            Suit::Diamonds => 0,
            Suit::Spades => 1,
            Suit::Hearts => 2,
            Suit::Clubs => 3,
        };

        self_value.cmp(&other_value)
    }
}
