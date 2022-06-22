use std::collections::HashMap;

pub mod traits;
use traits::Game;

#[derive(Clone, Debug, PartialEq)]
pub enum Phase {
    Boot,
    Setup,
    Play,
    End,
}

impl Phase {
    fn next_phase(&mut self) {
        *self = match *self {
            Phase::Boot => Phase::Setup,
            Phase::Setup => Phase::Play,
            Phase::Play => Phase::End,
            Phase::End => Phase::End
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Player {
    key: String,
    name: String,
    socket_id: String
}

#[derive(Clone, Debug, PartialEq)]
pub struct Players {
    pub list: Vec<Player>,
    pub active_key: Option<String>,
    pub cardinality: usize
}

impl Players {
    fn add_player(&mut self, key: &str, name: &str, socket_id: &str) -> &mut Players {
        self.list.push(
            Player { 
                key: String::from(key), 
                name: String::from(name), 
                socket_id: String::from(socket_id) 
            }
        );
        if self.list.len() == 1 {
            self.active_key = Some(String::from(key));
        }
        self.cardinality += 1;

        self
    }

    fn set_active_player(&mut self, key: &str) -> Result<&mut Players, &'static str> {
        let pki: Vec<_> = self.list.iter().filter(|p| p.key.as_str() == key).collect();
        match pki.len() {
            0 => Err("Player key not found!"),
            1 => {
                self.active_key = Some(String::from(key));
                Ok(self)
            },
            _ => Err("Non-unique player key found!")
        }  
    }
    
    fn next_player(&mut self) -> Result<&mut Players, &'static str> {
        let active_key = self.active_key.clone().unwrap();
        let active_player_index = self.list.iter().position(|p| p.key == active_key);
        match active_player_index {
            Some(idx) => {
                let next_player_index = (idx + 1) % self.cardinality;
                self.active_key = Some(self.list[next_player_index].key.clone());
                Ok(self)
            },
            None => Err("Cannot index of active player!")
        }
    }

    fn reset(&mut self) -> &mut Players {
        self.list.truncate(0);
        self.active_key = None;
        self.cardinality = 0;

        self
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum PossibleActions {
    None
}

pub struct CoreCommand {
    action: PossibleActions
}

#[derive(Clone, Debug, PartialEq)]
pub enum CoreConfigType {
    Int(i32)
}

#[derive(Debug, PartialEq)]
pub struct Core {
    phase: Phase,
    round: u16,
    players: Players,
    possible_actions: PossibleActions,
    config: HashMap<String, CoreConfigType>
}

impl Game for Core {
    type Status = Core;
    type Command = CoreCommand;
    type Config = HashMap<String, CoreConfigType>;

    /// Core constructor
    fn new() -> Core {
        Core {
            phase: Phase::Boot,
            round: 0,
            players: Players {
                list: Vec::new(),
                active_key: None,
                cardinality: 0
            },
            possible_actions: PossibleActions::None,
            config: HashMap::new()
        }
    }

    /// For progressing the phase of the game
    fn next_phase(&mut self) -> &mut Core {
        self.phase.next_phase();

        self
    }

    /// For moving the game to the next round
    fn next_round(&mut self) -> &mut Core {
        self.round += 1;

        self
    }

    /// For resetting the game to the initial state
    fn reset(&mut self) -> &mut Core {
        self.phase = Phase::Boot;
        self.round = 0;
        self.players.reset();

        self
    }

    fn add_player(&mut self, key: &str, name: &str, socket_id: &str) -> &mut Core {
        self.players.add_player(key, name, socket_id);

        self
    }

    fn set_active_player(&mut self, key: &str) -> Result<&mut Core, &'static str> {
        match self.players.set_active_player(key) {
            Ok(_) => Ok(self),
            Err(e) => Err(e)
        }
    }
    
    fn next_player(&mut self) -> Result<&mut Core, &'static str> {
        match self.players.next_player() {
            Ok(_) => Ok(self),
            Err(e) => Err(e)
        }
    }

    fn get_game_status(&self) -> Core {
        Core { 
            phase: self.phase.clone(),
            round: self.round.clone(),
            players: self.players.clone(),
            possible_actions: self.possible_actions.clone(),
            config: self.config.clone()
        }
    }

    fn process_action(&mut self, command: Self::Command) -> Result<&mut Core, &'static str> {
        match self.phase {
            Phase::Setup | Phase::Play => match command.action {
                PossibleActions::None => Ok(self)
            },
            _ => Err("Can only take action during the Setup or Play phases!")
        }
    }

    fn configure_game(&mut self, config: Self::Config) -> Result<&mut Self, &'static str> {
        match self.phase {
            Phase::Boot => {
                self.config = config;
                Ok(self)
            },
            _ => Err("Cannot configure game outside of boot phase!")
        }
    }
}


#[cfg(test)]
mod test;