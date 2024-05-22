use crate::card::{Card, Rank, Suit};
use rand::prelude::{thread_rng, SliceRandom};
use rand::Rng;

pub struct Deck {
    pub cards: Vec<Card>,
    pub flip_card: Option<Card>,
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

        Deck {
            cards,
            flip_card: None,
        }
    }

    pub fn shuffle(&mut self) {
        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng);
    }

    pub fn set_flip_card(&mut self) {
        let mut rng = rand::thread_rng();
        let index = rng.gen_range(0..self.cards.len());
        let flip_card = self.cards.swap_remove(index);
        self.flip_card = Some(flip_card);
    }
}
