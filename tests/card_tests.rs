use truco::card::Card;
use truco::rank::Rank;
use truco::suit::Suit;

#[test]
fn test_card_creation() {
    let card = Card::new(Suit::Clubs, Rank::Ace);
    assert_eq!(card.rank, Rank::Ace, "Card rank should be Ace");
    assert_ne!(card.suit, Suit::Hearts, "Card suit should not be Hearts");
}
