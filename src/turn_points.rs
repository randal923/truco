pub struct TurnPoints {
    team_1_score: u32,
    team_2_score: u32,
}

impl TurnPoints {
    pub fn new() -> TurnPoints {
        TurnPoints {
            team_1_score: 0,
            team_2_score: 0,
        }
    }

    pub fn team_1_score(&self) -> u32 {
        self.team_1_score
    }

    pub fn team_2_score(&self) -> u32 {
        self.team_2_score
    }

    pub fn add_team_1_score(&mut self, score: u32) {
        self.team_1_score += score;
    }

    pub fn add_team_2_score(&mut self, score: u32) {
        self.team_2_score += score;
    }

    pub fn reset(&mut self) {
        self.team_1_score = 0;
        self.team_2_score = 0;
    }
}
