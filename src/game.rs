use colored::*;
use std::io::{self, Write};

use crate::{
    card::Card, deck::Deck, player::Player, team::Team, turn::Turn, turn_points::TurnPoints,
};

pub struct Game {
    players: Vec<Player>,
    deck: Deck,
    turn: Turn,
    team1: Team,
    team2: Team,
    turn_points: TurnPoints,
}

impl Game {
    pub fn new(player_names: Vec<String>) -> Self {
        let players: Vec<Player> = player_names.into_iter().map(Player::new).collect();
        let deck = Deck::new();
        let turn = Turn::new();
        let team1 = Team::new(players[0..2].to_vec());
        let team2 = Team::new(players[2..4].to_vec());
        let turn_points = TurnPoints::new();

        Game {
            players,
            deck,
            turn,
            team1,
            team2,
            turn_points,
        }
    }

    pub fn start(&mut self) {
        self.deck.shuffle();
        self.deck.deal_cards(&mut self.players);
        println!("\n{}\n", "Players' hands:".yellow().bold().underline());
        self.display_hands();
        println!("\n{}", "Game is ready to start!".green().bold());

        self.play_turn();
    }

    fn play_turn(&mut self) {
        println!("\n{}", "Starting a new turn!".green().bold());
        self.deck.set_flip_card();
        let flip_card = self.deck.flip_card.expect("Flip card is not set");
        let manilha_rank = self.deck.get_manilha_rank();
        println!("Flip card: \n");
        println!("{}", flip_card.format_card());
        println!("\nManilha: ");
        println!("{}", Card::format_manilha_rank(manilha_rank));

        let mut played_cards: Vec<Card> = Vec::new();

        for i in 0..4 {
            let player = &mut self.players[i];
            let card_index = Game::get_player_card_choice(player);
            let played_card = player.play_card(card_index);

            println!("{} played:", player.name.cyan().bold(),);
            println!("{}", played_card.format_card());
            played_cards.push(played_card);
        }

        let winning_card = Card::compare_cards(&played_cards, &flip_card);
        let winning_player_index = played_cards
            .iter()
            .position(|&card| card == *winning_card)
            .unwrap();

        println!(
            "\n{} {} with",
            self.players[winning_player_index].name.cyan().bold(),
            "won the turn".green().bold(),
        );
        println!("{}", winning_card.format_card());

        if winning_player_index % 2 == 0 {
            self.turn_points.add_team_1_score(1);
        } else {
            self.turn_points.add_team_2_score(1);
        }

        println!("\nCurrent score:");
        println!("Team 1: {}", self.turn_points.team_1_score());
        println!("Team 2: {}", self.turn_points.team_2_score());
    }

    fn get_player_card_choice(player: &Player) -> usize {
        loop {
            print!(
                "{}, choose a card to play (1-{}): ",
                player.name.cyan().bold(),
                player.hand.len()
            );
            io::stdout().flush().unwrap();

            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");

            match input.trim().parse::<usize>() {
                Ok(index) if index > 0 && index <= player.hand.len() => return index - 1,
                _ => println!(
                    "Invalid choice. Please enter a number between 1 and {}.",
                    player.hand.len()
                ),
            }
        }
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
