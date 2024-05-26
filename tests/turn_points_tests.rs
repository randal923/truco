use truco::turn_points::TurnPoints;

#[test]
fn test_new_turn() {
    let turn_points = TurnPoints::new();
    assert_eq!(turn_points.team_1_score(), 0);
    assert_eq!(turn_points.team_2_score(), 0);
}

#[test]
fn test_add_team_1_score() {
    let mut turn_points = TurnPoints::new();
    turn_points.add_team_1_score(5);
    assert_eq!(turn_points.team_1_score(), 5);
}

#[test]
fn test_add_team_2_score() {
    let mut turn_points = TurnPoints::new();
    turn_points.add_team_2_score(3);
    assert_eq!(turn_points.team_2_score(), 3);
}

#[test]
fn test_reset() {
    let mut turn_points = TurnPoints::new();
    turn_points.add_team_1_score(5);
    turn_points.add_team_2_score(3);
    turn_points.reset();
    assert_eq!(turn_points.team_1_score(), 0);
    assert_eq!(turn_points.team_2_score(), 0);
}
