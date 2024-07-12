use truco::turn::Turn;

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
