use crate::{rank::Rank, suit::Suit};

#[derive(Debug, PartialEq, Clone, Copy, PartialOrd)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

impl Card {
    pub fn new(suit: Suit, rank: Rank) -> Self {
        Card { suit, rank }
    }

    pub fn compare_cards<'a>(cards: &'a [Card], flip_card: &Card) -> &'a Card {
        let manilha_card = flip_card.rank.next();
        println!("Manilha card: {:?}", manilha_card);
        cards.iter().max_by(|a, b| a.rank.cmp(&b.rank)).unwrap()
    }
}
