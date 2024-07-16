use truco::round_points::RoundPoints;

#[test]
fn test_new_turn() {
    let round_points = RoundPoints::new();
    assert_eq!(round_points.team_1_score(), 0);
    assert_eq!(round_points.team_2_score(), 0);
}

#[test]
fn test_add_team_1_score() {
    let mut round_points = RoundPoints::new();
    round_points.add_team_1_score(5);
    assert_eq!(round_points.team_1_score(), 5);
}

#[test]
fn test_add_team_2_score() {
    let mut round_points = RoundPoints::new();
    round_points.add_team_2_score(3);
    assert_eq!(round_points.team_2_score(), 3);
}

#[test]
fn test_reset() {
    let mut round_points = RoundPoints::new();
    round_points.add_team_1_score(5);
    round_points.add_team_2_score(3);
    round_points.reset();
    assert_eq!(round_points.team_1_score(), 0);
    assert_eq!(round_points.team_2_score(), 0);
}
