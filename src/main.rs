use colored::*;
use truco::{card::Card, deck::Deck, player::Player};

fn main() {
    println!("{}", "Starting game!".green().bold());

    let mut deck = Deck::new();
    println!("{}", "Shuffling".blue().italic());
    deck.shuffle();

    let mut players = vec![
        Player::new(String::from("Alice")),
        Player::new(String::from("Bob")),
        Player::new(String::from("Charlie")),
        Player::new(String::from("David")),
    ];

    println!("{}", "Dealing cards...".blue().italic());
    deck.deal_cards(&mut players);

    println!("\n{}", "Players' hands:".yellow().bold().underline());
    for (i, player) in players.iter().enumerate() {
        let hand = player.get_hand();
        let formatted_hand: Vec<String> = hand.iter().map(Card::format_card).collect();

        println!(
            "{}. {}:",
            (i + 1).to_string().bright_black().bold(),
            player.name.cyan().bold()
        );

        // Print cards in rows
        for line in 0..5 {
            for card in &formatted_hand {
                print!("{} ", card.lines().nth(line).unwrap());
            }
            println!();
        }
        println!(); // Add a blank line between players
    }

    println!("\n{}", "Game is ready to start!".green().bold());
}
