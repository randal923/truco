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
