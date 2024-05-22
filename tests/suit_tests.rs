use truco::suit::Suit;

#[test]
fn test_suit_order() {
    assert!(Suit::Diamonds < Suit::Spades);
    assert!(Suit::Spades < Suit::Hearts);
    assert!(Suit::Hearts < Suit::Clubs);
}
