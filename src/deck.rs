use crate::card::Card;
use crate::rank::Rank;
use crate::suit::Suit;
use rand::prelude::{thread_rng, SliceRandom};
use rand::Rng;

pub struct Deck {
    pub cards: Vec<Card>,
    pub flip_card: Option<Card>,
}

impl Deck {
    pub fn new() -> Deck {
        Deck {
            cards: Deck::create_cards(),
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

    pub fn get_manilha(&self) -> Rank {
        let flip_card = self.flip_card.expect("Flip card is not set");
        let manilha_rank = flip_card.rank.next();

        manilha_rank
    }

    pub fn reset(&mut self) {
        self.cards = Deck::create_cards();
        self.flip_card = None;
    }

    fn create_cards() -> Vec<Card> {
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

        cards
    }
}
