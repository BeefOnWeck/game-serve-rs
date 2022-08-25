use std::collections::HashMap;
use serde::{Serialize, Deserialize};

pub mod traits;
use traits::Game;

pub mod playe;
use playe::Players;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Phase {
    Boot,
    Setup,
    Play,
    End,
}

impl Phase {
    pub fn next_phase(&mut self) {
        *self = match *self {
            Phase::Boot => Phase::Setup,
            Phase::Setup => Phase::Play,
            Phase::Play => Phase::End,
            Phase::End => Phase::End
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Actions {
    None
}

pub struct CoreCommand {
    pub action: Actions
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
    last_action: Actions,
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
            players: Players::new(),
            last_action: Actions::None,
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

    fn add_player(&mut self, key: &str, name: &str, socket_id: &str) -> Result<&mut Core, &'static str> {
        self.players.add_player(key, name, socket_id);

        Ok(self)
    }

    fn set_active_player(&mut self, key: &str) -> Result<&mut Core, &'static str> {
        match self.players.set_active_player(key) {
            Ok(_) => Ok(self),
            Err(e) => Err(e)
        }
    }
    
    fn next_player(&mut self) -> Result<&mut Core, &'static str> {
        match self.players.next_player(1) {
            Ok(_) => Ok(self),
            Err(e) => Err(e)
        }
    }

    fn previous_player(&mut self) -> Result<&mut Core, &'static str> {
        match self.players.next_player(-1) {
            Ok(_) => Ok(self),
            Err(e) => Err(e)
        }
    }

    fn get_game_status(&self) -> Core {
        Core { 
            phase: self.phase.clone(),
            round: self.round,
            players: self.players.clone(),
            last_action: self.last_action.clone(),
            config: self.config.clone()
        }
    }

    fn find_the_winner(&mut self) -> &mut Self {
        self
    }

    fn process_action(&mut self, command: Self::Command) -> Result<&mut Core, &'static str> {
        match self.phase {
            Phase::Setup | Phase::Play => match command.action {
                Actions::None => Ok(self)
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