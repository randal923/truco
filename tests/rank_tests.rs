use truco::rank::Rank;

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
fn test_rank_next() {
    assert_eq!(Rank::Four.next(), Rank::Five);
    assert_eq!(Rank::Five.next(), Rank::Six);
    assert_eq!(Rank::Six.next(), Rank::Seven);
    assert_eq!(Rank::Seven.next(), Rank::Queen);
    assert_eq!(Rank::Queen.next(), Rank::Jack);
    assert_eq!(Rank::Jack.next(), Rank::King);
    assert_eq!(Rank::King.next(), Rank::Ace);
    assert_eq!(Rank::Ace.next(), Rank::Two);
    assert_eq!(Rank::Two.next(), Rank::Three);
    assert_eq!(Rank::Three.next(), Rank::Four);
}
