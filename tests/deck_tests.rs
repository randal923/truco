use truco::deck::Deck;

#[test]
fn deck_contains_all_cards() {
    let deck = Deck::new();

    assert_eq!(deck.cards.len(), 40, "Deck should have 40 cards");
}

#[test]
fn deck_shuffles_all_cards() {
    let mut deck = Deck::new();
    let original_deck = deck.cards.clone();

    deck.shuffle();

    assert_ne!(
        deck.cards, original_deck,
        "Deck should be shuffled and different from the original"
    );
}

#[test]
fn deck_shuffles_in_random_order() {
    let mut deck = Deck::new();
    let mut shuffled_deck = Deck::new();

    deck.shuffle();
    shuffled_deck.shuffle();

    assert_ne!(
        deck.cards, shuffled_deck.cards,
        "Two shuffled decks should not be the same"
    );
}
