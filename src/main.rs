use truco::game::Game;

fn main() {
    let player_names = vec![
        String::from("Alice"),
        String::from("Bob"),
        String::from("Charlie"),
        String::from("David"),
    ];

    let mut game = Game::new(player_names);
    game.start();
}
