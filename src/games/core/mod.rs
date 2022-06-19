
#[derive(Debug, PartialEq)]
pub struct GameCore {
    phase: Phase,
    round: u16,
    players: Vec<Player>,
    active_player_key: Option<String>,
    num_players: usize
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
            active_player_key: None,
            num_players: 0
        }
    }

    /// For progressing the phase of the game
    pub fn next_phase(&mut self) -> &mut GameCore {
        self.phase = match self.phase {
            Phase::Boot => Phase::Setup,
            Phase::Setup => Phase::Play,
            Phase::Play => Phase::End,
            Phase::End => Phase::End
        };

        self
    }

    /// For moving the game to the next round
    pub fn next_round(&mut self) -> &mut GameCore {
        self.round += 1;

        self
    }

    /// For resetting the game to the initial state
    pub fn reset(&mut self) -> &mut GameCore {
        self.phase = Phase::Boot;
        self.round = 0;
        self.players.truncate(0);
        self.active_player_key = None;
        self.num_players = 0;

        self
    }

    pub fn add_player(&mut self, key: &str, name: &str, socket_id: &str) -> &mut GameCore {
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
        self.num_players += 1;

        self
    }

    pub fn set_active_player(&mut self, key: &str) -> Result<&mut GameCore, &'static str> {
        let pki: Vec<_> = self.players.iter().filter(|p| p.key.as_str() == key).collect();
        match pki.len() {
            0 => Err("Player key not found!"),
            1 => {
                self.active_player_key = Some(String::from(key));
                Ok(self)
            },
            _ => Err("Non-unique player key found!")
        }  
    }
    
    pub fn next_player(&mut self) -> Result<&mut GameCore, &'static str> {
        let active_player_key = self.active_player_key.clone().unwrap();
        let active_player_index = self.players.iter().position(|p| p.key == active_player_key);
        match active_player_index {
            Some(idx) => {
                let next_player_index = (idx + 1) % self.num_players;
                self.active_player_key = Some(self.players[next_player_index].key.clone());
                Ok(self)
            },
            None => Err("Cannot index of active player!")
        }
    }
}


#[cfg(test)]
mod test;