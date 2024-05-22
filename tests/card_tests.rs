use truco::card::Card;
use truco::rank::Rank;
use truco::suit::Suit;

#[test]
fn test_card_creation() {
    let card = Card::new(Suit::Clubs, Rank::Ace);
    assert_eq!(card.rank, Rank::Ace, "Card rank should be Ace");
    assert_ne!(card.suit, Suit::Hearts, "Card suit should not be Hearts");
}

#[test]
fn test_rank_order() {
    assert!(Rank::Four < Rank::Five);
    assert!(Rank::Five < Rank::Six);
    assert!(Rank::Six < Rank::Seven);
    assert!(Rank::Seven < Rank::Queen);
    assert!(Rank::Queen < Rank::Jack);
    assert!(Rank::Jack < Rank::King);
    assert!(Rank::King < Rank::Ace);
    assert!(Rank::Ace < Rank::Two);
    assert!(Rank::Two < Rank::Three);
}

#[test]
fn test_suit_order() {
    assert!(Suit::Diamonds < Suit::Spades);
    assert!(Suit::Spades < Suit::Hearts);
    assert!(Suit::Hearts < Suit::Clubs);
}
