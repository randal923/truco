use crate::{deck::Deck, player::Player};

pub struct Turn {
    round: u32,
}

impl Turn {
    pub fn new() -> Turn {
        Turn { round: 0 }
    }

    pub fn round(&self) -> u32 {
        self.round
    }

    pub fn next_round(&mut self) {
        self.round += 1;
    }

    pub fn reset(&mut self) {
        self.round = 0;
    }

    pub fn deal_cards(&self, players: &mut Vec<Player>, deck: &mut Deck) {
        for _ in 0..3 {
            for player in players.iter_mut() {
                let card = deck.cards.pop().expect("No more cards in the deck");
                player.add_card(card);
            }
        }
    }
}
