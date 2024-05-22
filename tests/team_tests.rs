use truco::{player::Player, team::Team};

#[test]
fn team_new() {
    let team = Team::new();

    assert_eq!(team.players.len(), 0);
    assert_eq!(team.score, 0);
}

#[test]
fn team_add_player() {
    let mut team = Team::new();

    let player1 = Player::new("Player 1".to_string());
    team.add_player(player1);

    assert_eq!(team.players.len(), 1);
}

#[test]
fn team_get_score() {
    let team = Team::new();

    assert_eq!(team.get_score(), 0);
}

#[test]
fn team_increase_score() {
    let mut team = Team::new();

    team.increase_score(3);

    assert_eq!(team.get_score(), 3);
}

#[test]
fn team_reset_score() {
    let mut team = Team::new();

    team.increase_score(3);
    team.reset_score();

    assert_eq!(team.get_score(), 0);
}
