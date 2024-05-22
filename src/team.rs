use crate::player::Player;

pub struct Team {
    pub players: Vec<Player>,
    pub score: u32,
    pub name: Option<String>,
}

impl Team {
    pub fn new() -> Team {
        Team {
            players: Vec::new(),
            score: 0,
            name: None,
        }
    }

    pub fn add_player(&mut self, player: Player) {
        self.players.push(player);
    }

    pub fn get_score(&self) -> u32 {
        self.score
    }

    pub fn increase_score(&mut self, points: u32) {
        self.score += points;
    }

    pub fn reset_score(&mut self) {
        self.score = 0;
    }

    pub fn get_winning_team(&self) -> Option<String> {
        if self.score >= 12 {
            return self.name.clone();
        }

        return None;
    }
}
