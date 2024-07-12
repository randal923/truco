use colored::*;

use crate::{card::Card, deck::Deck, player::Player};

pub struct Game {
    players: Vec<Player>,
}

impl Game {
    pub fn new(player_names: Vec<String>) -> Self {
        let mut players = player_names.into_iter().map(Player::new).collect();
        let mut deck = Deck::new();
        deck.shuffle();
        deck.deal_cards(&mut players);

        Game { players }
    }

    pub fn run(&mut self) {
        println!("\n{}\n", "Players' hands:".yellow().bold().underline());
        self.display_hands();
        println!("\n{}", "Game is ready to start!".green().bold());
    }

    fn display_hands(&self) {
        for (i, player) in self.players.iter().enumerate() {
            let hand = player.get_hand();
            let formatted_hand: Vec<String> = hand.iter().map(Card::format_card).collect();

            println!(
                "{}. {}:",
                (i + 1).to_string().bright_black().bold(),
                player.name.cyan().bold()
            );

            for line in 0..5 {
                for card in &formatted_hand {
                    print!("{} ", card.lines().nth(line).unwrap());
                }
                println!();
            }
            println!();
        }
    }
}
