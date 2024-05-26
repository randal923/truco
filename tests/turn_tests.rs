use truco::{deck::Deck, player::Player, turn::Turn};

#[test]
fn test_new_turn() {
    let turn = Turn::new();
    assert_eq!(turn.round(), 0);
}

#[test]
fn test_next_round() {
    let mut turn = Turn::new();
    turn.next_round();
    assert_eq!(turn.round(), 1);
}

#[test]
fn test_reset() {
    let mut turn = Turn::new();
    turn.next_round();
    turn.reset();
    assert_eq!(turn.round(), 0);
}

#[test]
fn test_deal_cards() {
    let mut deck = Deck::new();
    deck.shuffle();

    let mut players = vec![
        Player::new("Player 1".to_string()),
        Player::new("Player 2".to_string()),
        Player::new("Player 3".to_string()),
        Player::new("Player 4".to_string()),
    ];

    let turn = Turn::new();
    turn.deal_cards(&mut players, &mut deck);

    for player in players.iter() {
        assert_eq!(player.hand.len(), 3);
    }
}
