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

#[test]
fn deck_has_no_flip_card() {
    let deck = Deck::new();

    assert_eq!(
        deck.flip_card, None,
        "Deck should not have a flip card on creation"
    );
}

#[test]
fn deck_has_flip_card_after_flipping() {
    let mut deck = Deck::new();

    deck.set_flip_card();

    assert_ne!(
        deck.flip_card, None,
        "Deck should have a flip card after flipping"
    );
}

#[test]
fn deck_has_one_less_card_after_flipping() {
    let mut deck = Deck::new();
    let original_deck_size = deck.cards.len();

    deck.set_flip_card();

    assert_eq!(
        deck.cards.len(),
        original_deck_size - 1,
        "Deck should have one less card after flipping"
    );
}

#[test]
fn deck_set_manilha_if_flip_card_is_set() {
    let mut deck = Deck::new();

    deck.set_flip_card();

    let flip_card = deck.flip_card.unwrap();
    let manilha = deck.get_manilha();

    assert_eq!(
        manilha,
        flip_card.rank.next(),
        "Manilha should be the next rank after the flip card"
    );
}

#[test]
#[should_panic(expected = "Flip card is not set")]
fn deck_get_manilha_panics_with_message_if_no_flip_card() {
    let deck = Deck::new();
    let _ = deck.get_manilha();
}
