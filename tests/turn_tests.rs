use truco::round::Round;

#[test]
fn test_new_turn() {
    let round = Round::new();
    assert_eq!(round.round(), 0);
}

#[test]
fn test_next_round() {
    let mut round = Round::new();
    round.next_round();
    assert_eq!(round.round(), 1);
}

#[test]
fn test_reset() {
    let mut round = Round::new();
    round.next_round();
    round.reset();
    assert_eq!(round.round(), 0);
}
