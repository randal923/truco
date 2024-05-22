use rand::prelude::{thread_rng, SliceRandom};

pub struct Deck {
    pub cards: Vec<i32>,
}

impl Deck {
    pub fn new() -> Deck {
        let mut cards = Vec::new();

        for i in 1..41 {
            cards.push(i);
        }

        Deck { cards }
    }

    pub fn shuffle(&mut self) {
        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng);
    }
}
