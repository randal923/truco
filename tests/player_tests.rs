use truco::{card::Card, player::Player, rank::Rank, suit::Suit};

#[test]
fn player_new() {
    let player = Player::new("Player 1".to_string());
    assert_eq!(player.name, "Player 1");
    assert_eq!(player.hand.len(), 0);
}

#[test]
fn player_add_card() {
    let mut player = Player::new("Player 1".to_string());
    player.add_card(Card::new(Suit::Clubs, Rank::Two));
    assert_eq!(player.hand.len(), 1);
}

#[test]
fn player_play_card() {
    let mut player = Player::new("Player 1".to_string());
    player.add_card(Card::new(Suit::Clubs, Rank::Two));
    let card = player.play_card(0);
    assert_eq!(card.suit, Suit::Clubs);
    assert_eq!(card.rank, Rank::Two);
    assert_eq!(player.hand.len(), 0);
}

#[test]
fn player_get_card() {
    let mut player = Player::new("Player 1".to_string());
    player.add_card(Card::new(Suit::Clubs, Rank::Two));
    let card = player.get_card(0);
    assert_eq!(card.suit, Suit::Clubs);
    assert_eq!(card.rank, Rank::Two);
}

#[test]
fn player_get_hand() {
    let mut player = Player::new("Player 1".to_string());
    player.add_card(Card::new(Suit::Clubs, Rank::Two));
    player.add_card(Card::new(Suit::Clubs, Rank::Three));
    let hand = player.get_hand();
    assert_eq!(hand.len(), 2);
    assert_eq!(hand[0].suit, Suit::Clubs);
    assert_eq!(hand[0].rank, Rank::Two);
    assert_eq!(hand[1].suit, Suit::Clubs);
    assert_eq!(hand[1].rank, Rank::Three);
}

#[test]
fn player_reset() {
    let mut player = Player::new("Player 1".to_string());
    player.add_card(Card::new(Suit::Clubs, Rank::Two));
    player.add_card(Card::new(Suit::Clubs, Rank::Three));
    player.reset();
    assert_eq!(player.hand.len(), 0);
}
