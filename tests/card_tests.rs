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
fn test_compare_cards() {
    let card1 = Card::new(Suit::Clubs, Rank::Four);
    let card2 = Card::new(Suit::Clubs, Rank::Five);
    let card3 = Card::new(Suit::Clubs, Rank::Six);
    let card4 = Card::new(Suit::Clubs, Rank::Seven);

    let flip_card = Card::new(Suit::Clubs, Rank::Queen);

    let cards = vec![card1, card2, card3, card4];

    assert_eq!(Card::compare_cards(&cards, &flip_card), &card4);
}

#[test]
fn test_compare_cards_with_one_manilha() {
    let card1 = Card::new(Suit::Clubs, Rank::Ace);
    let card2 = Card::new(Suit::Clubs, Rank::King);
    let card3 = Card::new(Suit::Clubs, Rank::Six);
    let card4 = Card::new(Suit::Clubs, Rank::Seven);

    let flip_card = Card::new(Suit::Diamonds, Rank::Six);

    let cards = vec![card1, card2, card3, card4];

    assert_eq!(Card::compare_cards(&cards, &flip_card), &card4);
}

#[test]
fn test_compare_cards_with_two_manilha() {
    let card1 = Card::new(Suit::Diamonds, Rank::Six);
    let card2 = Card::new(Suit::Spades, Rank::Six);
    let card3 = Card::new(Suit::Hearts, Rank::Two);
    let card4 = Card::new(Suit::Spades, Rank::Three);

    let flip_card = Card::new(Suit::Diamonds, Rank::Five);

    let cards = vec![card1, card2, card3, card4];

    assert_eq!(Card::compare_cards(&cards, &flip_card), &card2);
}

#[test]
fn test_compare_cards_with_three_manilha() {
    let card1 = Card::new(Suit::Diamonds, Rank::Six);
    let card2 = Card::new(Suit::Spades, Rank::Six);
    let card3 = Card::new(Suit::Hearts, Rank::Six);
    let card4 = Card::new(Suit::Spades, Rank::Three);

    let flip_card = Card::new(Suit::Diamonds, Rank::Five);

    let cards = vec![card1, card2, card3, card4];

    assert_eq!(Card::compare_cards(&cards, &flip_card), &card3);
}

#[test]
fn test_compare_cards_with_four_manilha() {
    let card1 = Card::new(Suit::Clubs, Rank::Seven);
    let card2 = Card::new(Suit::Diamonds, Rank::Seven);
    let card3 = Card::new(Suit::Hearts, Rank::Seven);
    let card4 = Card::new(Suit::Spades, Rank::Seven);

    let flip_card = Card::new(Suit::Diamonds, Rank::Six);

    let cards = vec![card1, card2, card3, card4];

    assert_eq!(Card::compare_cards(&cards, &flip_card), &card1);
}
