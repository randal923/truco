use crate::card::Card;

#[derive(Debug)]
pub struct Player {
    pub hand: Vec<Card>,
    pub name: String,
}

impl Player {
    pub fn new(name: String) -> Player {
        Player {
            hand: Vec::new(),
            name,
        }
    }

    pub fn add_card(&mut self, card: Card) {
        self.hand.push(card);
    }

    pub fn play_card(&mut self, index: usize) -> Card {
        self.hand.remove(index)
    }

    pub fn get_card(&self, index: usize) -> Card {
        self.hand[index]
    }

    pub fn get_hand(&self) -> &Vec<Card> {
        &self.hand
    }

    pub fn reset(&mut self) {
        self.hand.clear();
    }
}
