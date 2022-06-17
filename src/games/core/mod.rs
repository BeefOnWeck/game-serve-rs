
pub struct GameCore {
    phase: Phase,
    round: u16
}

#[derive(Debug, PartialEq)]
enum Phase {
    Boot,
    Setup,
    Play,
    End,
}

impl GameCore {
    /// GameCore constructor
    pub fn new() -> GameCore {
        GameCore {
            phase: Phase::Boot,
            round: 0
        }
    }

    /// For progressing the phase of the game
    pub fn next_phase(mut self) -> GameCore {
        self.phase = match self.phase {
            Phase::Boot => Phase::Setup,
            Phase::Setup => Phase::Play,
            Phase::Play => Phase::End,
            Phase::End => Phase::End
        };

        self
    }

    /// For moving the game to the next round
    pub fn next_round(mut self) -> GameCore {
        self.round += 1;

        self
    }

    /// For resetting the game to the initial state
    pub fn reset(mut self) -> GameCore {
        self = GameCore::new();

        self
    }
}


#[cfg(test)]
mod test;