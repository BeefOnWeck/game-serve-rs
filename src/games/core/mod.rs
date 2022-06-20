mod traits;
use traits::Game;

#[derive(Debug, PartialEq)]
struct Core {
    phase: Phase,
    round: u16,
    players: Vec<Player>,
    active_player_key: Option<String>,
    num_players: usize,
    possible_actions: PossibleActions
}

#[derive(Clone, Debug, PartialEq)]
enum Phase {
    Boot,
    Setup,
    Play,
    End,
}

#[derive(Clone, Debug, PartialEq)]
struct Player {
    key: String,
    name: String,
    socket_id: String
}

#[derive(Clone, Debug, PartialEq)]
enum PossibleActions {
    None
}

struct CoreCommand {
    action: PossibleActions
}

impl Game for Core {
    type Status = Core;
    type Command = CoreCommand;

    /// Core constructor
    fn new() -> Core {
        Core {
            phase: Phase::Boot,
            round: 0,
            players: Vec::new(),
            active_player_key: None,
            num_players: 0,
            possible_actions: PossibleActions::None
        }
    }

    /// For progressing the phase of the game
    fn next_phase(&mut self) -> &mut Core {
        self.phase = match self.phase {
            Phase::Boot => Phase::Setup,
            Phase::Setup => Phase::Play,
            Phase::Play => Phase::End,
            Phase::End => Phase::End
        };

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
        self.players.truncate(0);
        self.active_player_key = None;
        self.num_players = 0;

        self
    }

    fn add_player(&mut self, key: &str, name: &str, socket_id: &str) -> &mut Core {
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

    fn set_active_player(&mut self, key: &str) -> Result<&mut Core, &'static str> {
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
    
    fn next_player(&mut self) -> Result<&mut Core, &'static str> {
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

    fn get_game_status(&self) -> Core {
        Core { 
            phase: self.phase.clone(),
            round: self.round.clone(),
            players: self.players.clone(),
            active_player_key: self.active_player_key.clone(),
            num_players: self.num_players.clone(),
            possible_actions: self.possible_actions.clone()
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
}


#[cfg(test)]
mod test;