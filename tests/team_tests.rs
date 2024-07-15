use truco::{player::Player, team::Team};

fn create_team() -> Team {
    let player_names = vec![
        String::from("Alice"),
        String::from("Bob"),
        String::from("Charlie"),
        String::from("David"),
    ];
    let players: Vec<Player> = player_names.into_iter().map(Player::new).collect();
    let team = Team::new(players);

    team
}

#[test]
fn team_new() {
    let team = create_team();

    assert_eq!(team.players.len(), 4);
    assert_eq!(team.score, 0);
}

#[test]
fn team_add_player() {
    let player1 = Player::new("Player 1".to_string());
    let mut team = Team::new(vec![player1]);

    assert_eq!(team.players.len(), 1);

    let player2 = Player::new("Player 2".to_string());
    team.add_player(player2);

    assert_eq!(team.players.len(), 2);
}

#[test]
fn team_get_score() {
    let team = create_team();

    assert_eq!(team.get_score(), 0);
}

#[test]
fn team_increase_score() {
    let mut team = create_team();

    team.increase_score(3);

    assert_eq!(team.get_score(), 3);
}

#[test]
fn team_reset_score() {
    let mut team = create_team();

    team.increase_score(3);
    team.reset_score();

    assert_eq!(team.get_score(), 0);
}
