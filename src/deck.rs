use crate::card::{Card, Rank, Suit};
use rand::prelude::{thread_rng, SliceRandom};

pub struct Deck {
    pub cards: Vec<Card>,
}

impl Deck {
    pub fn new() -> Deck {
        let mut cards = Vec::new();

        let suits = [Suit::Clubs, Suit::Hearts, Suit::Spades, Suit::Diamonds];
        let ranks = [
            Rank::Three,
            Rank::Two,
            Rank::Ace,
            Rank::King,
            Rank::Jack,
            Rank::Queen,
            Rank::Seven,
            Rank::Six,
            Rank::Five,
            Rank::Four,
        ];

        for &suit in &suits {
            for &rank in &ranks {
                cards.push(Card::new(suit, rank));
            }
        }

        Deck { cards }
    }

    pub fn shuffle(&mut self) {
        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng);
    }
}
