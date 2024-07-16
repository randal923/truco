use colored::*;
use std::io::{self, Write};

use crate::{
    card::Card, deck::Deck, player::Player, round::Round, round_points::RoundPoints, team::Team,
};

pub struct Game {
    pub players: Vec<Player>,
    pub deck: Deck,
    pub round: Round,
    pub team1: Team,
    pub team2: Team,
    pub round_points: RoundPoints,
}

impl Game {
    pub fn new(player_names: Vec<String>) -> Self {
        let players: Vec<Player> = player_names.into_iter().map(Player::new).collect();
        let deck = Deck::new();
        let round = Round::new();
        let team1 = Team::new(players[0..2].to_vec());
        let team2 = Team::new(players[2..4].to_vec());
        let round_points = RoundPoints::new();

        Game {
            players,
            deck,
            round,
            team1,
            team2,
            round_points,
        }
    }

    pub fn start(&mut self) {
        println!("\n{}", "Game is starting!".green().bold());
        while self.team1.get_score() < 12 && self.team2.get_score() < 12 {
            self.play_hand();
        }

        self.announce_winner();
    }

    fn play_hand(&mut self) {
        println!("\n{}", "Starting a new hand!".green().bold());
        self.reset_for_new_hand();
        self.deck.deal_cards(&mut self.players);
        self.deck.set_flip_card();
        let flip_card = self.deck.flip_card.expect("Flip card is not set");
        let manilha_rank = self.deck.get_manilha_rank();

        println!("\n{}\n", "Players' hands:".yellow().bold().underline());
        self.display_hands();

        println!("Flip card: \n");
        println!("{}", flip_card.format_card());
        println!("\nManilha: ");
        println!("{}", Card::format_manilha_rank(manilha_rank));

        let mut round = 1;
        let mut is_draw = false;

        while round <= 3 && !self.is_hand_over() {
            println!("\n{} {}", "Round".green().bold(), round);
            match self.play_round(round, is_draw, &flip_card) {
                Ok(draw) => is_draw = draw,
                Err(_) => break, // End the hand if there's an error (e.g., no cards left)
            }

            if is_draw && round == 1 {
                println!(
                    "\n{}",
                    "It's a draw! Playing a special round.".yellow().bold()
                );
                self.play_special_round(&flip_card);
                break;
            }

            round += 1;
        }

        self.update_team_scores();
        self.display_game_scores();
    }

    fn play_round(
        &mut self,
        round: u32,
        is_draw: bool,
        flip_card: &Card,
    ) -> Result<bool, &'static str> {
        let mut played_cards: Vec<Card> = Vec::new();

        for i in 0..4 {
            let player = &mut self.players[i];
            if player.hand.is_empty() {
                println!("{} has no cards left to play.", player.name.cyan().bold());
                continue;
            }
            if !is_draw || (is_draw && round == 1 && i < 2) {
                let card_index = Game::get_player_card_choice(player);
                let played_card = player.play_card(card_index);
                println!("{} played:", player.name.cyan().bold());
                println!("{}", played_card.format_card());
                played_cards.push(played_card);
            }
        }

        if played_cards.is_empty() {
            return Err("No cards were played this round");
        }

        let winning_card = Card::compare_cards(&played_cards, flip_card);
        let winning_player_index = played_cards
            .iter()
            .position(|&card| card == *winning_card)
            .unwrap();

        println!(
            "\n{} {} with",
            self.players[winning_player_index].name.cyan().bold(),
            "won the round".green().bold(),
        );
        println!("{}", winning_card.format_card());

        if winning_player_index % 2 == 0 {
            self.round_points.add_team_1_score(1);
        } else {
            self.round_points.add_team_2_score(1);
        }

        println!("\nCurrent hand score:");
        println!("Team 1: {}", self.round_points.team_1_score());
        println!("Team 2: {}", self.round_points.team_2_score());

        Ok(self.round_points.team_1_score() == self.round_points.team_2_score())
    }

    fn play_special_round(&mut self, flip_card: &Card) {
        let mut played_cards: Vec<Card> = Vec::new();

        for i in 0..2 {
            let player = &mut self.players[i];
            if player.hand.is_empty() {
                println!("{} has no cards left to play.", player.name.cyan().bold());
                continue;
            }
            let card_index = Game::get_player_card_choice(player);
            let played_card = player.play_card(card_index);
            println!("{} played:", player.name.cyan().bold());
            println!("{}", played_card.format_card());
            played_cards.push(played_card);
        }

        if played_cards.is_empty() {
            println!("No cards were played in the special round. The hand ends in a draw.");
            return;
        }

        let winning_card = Card::compare_cards(&played_cards, flip_card);
        let winning_player_index = played_cards
            .iter()
            .position(|&card| card == *winning_card)
            .unwrap();

        println!(
            "\n{} {} with",
            self.players[winning_player_index].name.cyan().bold(),
            "won the special round".green().bold(),
        );
        println!("{}", winning_card.format_card());

        if winning_player_index % 2 == 0 {
            self.team1.increase_score(1);
        } else {
            self.team2.increase_score(1);
        }
    }

    fn update_team_scores(&mut self) {
        if self.round_points.team_1_score() > self.round_points.team_2_score() {
            self.team1.increase_score(1);
            println!("\n{}", "Team 1 wins the hand!".green().bold());
        } else if self.round_points.team_2_score() > self.round_points.team_1_score() {
            self.team2.increase_score(1);
            println!("\n{}", "Team 2 wins the hand!".green().bold());
        } else {
            println!("\n{}", "The hand ended in a draw!".yellow().bold());
        }
    }

    fn is_hand_over(&self) -> bool {
        self.round_points.team_1_score() == 2 || self.round_points.team_2_score() == 2
    }

    fn reset_for_new_hand(&mut self) {
        self.deck.reset();
        self.deck.shuffle();
        self.round_points.reset();
        for player in &mut self.players {
            player.reset();
        }
    }

    fn display_game_scores(&self) {
        println!("\n{}", "Current game score:".yellow().bold());
        println!("Team 1: {}", self.team1.get_score());
        println!("Team 2: {}", self.team2.get_score());
    }

    fn announce_winner(&self) {
        if self.team1.get_score() >= 12 {
            println!("\n{}", "Team 1 wins the game!".green().bold());
        } else {
            println!("\n{}", "Team 2 wins the game!".green().bold());
        }
    }

    fn get_player_card_choice(player: &Player) -> usize {
        if player.hand.is_empty() {
            println!("{} has no cards left to play.", player.name.cyan().bold());
            return 0;
        }

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
