pub struct Turn {
    round: u32,
    current_player_turn: u32,
}

impl Turn {
    pub fn new() -> Turn {
        Turn {
            round: 0,
            current_player_turn: 0,
        }
    }

    pub fn round(&self) -> u32 {
        self.round
    }

    pub fn next_round(&mut self) {
        self.round += 1;
    }

    pub fn reset(&mut self) {
        self.round = 0;
    }

    pub fn get_current_player_turn(&self) -> u32 {
        self.current_player_turn
    }

    pub fn set_current_player_turn(&mut self, player_index: u32) {
        self.current_player_turn = player_index;
    }
}
