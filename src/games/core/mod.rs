
pub struct GameCore {
    phase: Phase,
    round: u16,
    players: Vec<Player>,
    active_player_key: Option<String>
}

#[derive(Debug, PartialEq)]
enum Phase {
    Boot,
    Setup,
    Play,
    End,
}

#[derive(Debug, PartialEq)]
struct Player {
    key: String,
    name: String,
    socket_id: String
}

impl GameCore {
    /// GameCore constructor
    pub fn new() -> GameCore {
        GameCore {
            phase: Phase::Boot,
            round: 0,
            players: Vec::new(),
            active_player_key: None
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
        self.phase = Phase::Boot;
        self.round = 0;
        self.players.truncate(0);
        self.active_player_key = None;

        self
    }

    pub fn add_player(mut self, key: &str, name: &str, socket_id: &str) -> GameCore {
        self.players.push(
            Player { 
                key: String::from(key), 
                name: String::from(name), 
                socket_id: String::from(socket_id) 
            }
        );
        if self.players.len() == 1 {
            self.active_player_key = Some(String::from(key));
        }

        self
    }

    pub fn set_active_player(mut self, key: &str) -> GameCore {
        self.active_player_key = Some(String::from(key));

        self
    }
}


#[cfg(test)]
mod test;