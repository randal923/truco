use crate::rank::Rank;

#[derive(Debug, PartialEq, Clone, Copy, PartialOrd)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

impl Card {
    pub fn new(suit: Suit, rank: Rank) -> Self {
        Card { suit, rank }
    }
}

#[derive(Debug, PartialEq, Clone, Copy, PartialOrd)]
pub enum Suit {
    Diamonds,
    Spades,
    Hearts,
    Clubs,
}
