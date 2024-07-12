pub struct Turn {
    round: u32,
}

impl Turn {
    pub fn new() -> Turn {
        Turn { round: 0 }
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
}
