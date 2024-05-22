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

#[derive(Debug, PartialEq, Clone, Copy, PartialOrd, Ord, Eq, Hash)]
pub enum Rank {
    Four,
    Five,
    Six,
    Seven,
    Queen,
    Jack,
    King,
    Ace,
    Two,
    Three,
}

impl Rank {
    pub fn next(&self) -> Rank {
        match self {
            Rank::Four => Rank::Five,
            Rank::Five => Rank::Six,
            Rank::Six => Rank::Seven,
            Rank::Seven => Rank::Queen,
            Rank::Queen => Rank::Jack,
            Rank::Jack => Rank::King,
            Rank::King => Rank::Ace,
            Rank::Ace => Rank::Two,
            Rank::Two => Rank::Three,
            Rank::Three => Rank::Four,
        }
    }
}
