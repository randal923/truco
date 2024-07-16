// use truco::{card::Card, game::Game, rank::Rank, suit::Suit};

// #[test]
// fn test_game_initialization() {
//     let player_names = vec![
//         String::from("Alice"),
//         String::from("Bob"),
//         String::from("Charlie"),
//         String::from("David"),
//     ];

//     let game = Game::new(player_names);

//     assert_eq!(game.players.len(), 4);
//     assert_eq!(game.team1.players.len(), 2);
//     assert_eq!(game.team2.players.len(), 2);
//     assert_eq!(game.round_points.team_1_score(), 0);
//     assert_eq!(game.round_points.team_2_score(), 0);
// }

// #[test]
// fn test_deal_cards() {
//     let player_names = vec![
//         String::from("Alice"),
//         String::from("Bob"),
//         String::from("Charlie"),
//         String::from("David"),
//     ];
//     let mut game = Game::new(player_names);
//     game.deck.deal_cards(&mut game.players);

//     for player in &game.players {
//         assert_eq!(player.hand.len(), 3);
//     }
// }

// #[test]
// fn test_play_round() {
//     let player_names = vec![
//         String::from("Alice"),
//         String::from("Bob"),
//         String::from("Charlie"),
//         String::from("David"),
//     ];
//     let mut game = Game::new(player_names);
//     game.deck.deal_cards(&mut game.players);
//     game.deck.set_flip_card();

//     let flip_card = game.deck.flip_card.unwrap();
//     let result = game.play_round(1, false, &flip_card);

//     assert!(result.is_ok());
//     assert!(game.round_points.team_1_score() + game.round_points.team_2_score() == 1);
// }

// #[test]
// fn test_is_hand_over() {
//     let player_names = vec![
//         String::from("Alice"),
//         String::from("Bob"),
//         String::from("Charlie"),
//         String::from("David"),
//     ];
//     let mut game = Game::new(player_names);

//     assert!(!game.is_hand_over());

//     game.round_points.add_team_1_score(2);
//     assert!(game.is_hand_over());

//     game.round_points.reset();
//     game.round_points.add_team_2_score(2);
//     assert!(game.is_hand_over());
// }

// #[test]
// fn test_update_team_scores() {
//     let player_names = vec![
//         String::from("Alice"),
//         String::from("Bob"),
//         String::from("Charlie"),
//         String::from("David"),
//     ];
//     let mut game = Game::new(player_names);

//     game.round_points.add_team_1_score(2);
//     game.update_team_scores();
//     assert_eq!(game.team1.get_score(), 1);
//     assert_eq!(game.team2.get_score(), 0);

//     game.round_points.reset();
//     game.round_points.add_team_2_score(2);
//     game.update_team_scores();
//     assert_eq!(game.team1.get_score(), 1);
//     assert_eq!(game.team2.get_score(), 1);
// }

// #[test]
// fn test_reset_for_new_hand() {
//     let player_names = vec![
//         String::from("Alice"),
//         String::from("Bob"),
//         String::from("Charlie"),
//         String::from("David"),
//     ];
//     let mut game = Game::new(player_names);

//     // Simulate a played hand
//     game.round_points.add_team_1_score(1);
//     game.players[0].add_card(Card::new(Suit::Hearts, Rank::Ace));
//     game.deck.cards.pop();

//     game.reset_for_new_hand();

//     assert_eq!(game.round_points.team_1_score(), 0);
//     assert_eq!(game.round_points.team_2_score(), 0);
//     assert_eq!(game.deck.cards.len(), 40);
//     for player in &game.players {
//         assert!(player.hand.is_empty());
//     }
// }

// // You might need to modify the Game struct to make some methods public for testing
// // or use #[cfg(test)] modules to access private methods in tests.
