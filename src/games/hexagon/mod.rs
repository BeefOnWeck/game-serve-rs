use std::collections::HashMap;

use crate::games::core::{
    Phase,
    PossibleActions,
    CoreConfigType,
    CoreCommand
};
use crate::games::core::actors::Players;
use crate::games::core::traits::Game;

#[derive(Debug, PartialEq)]
struct Coordinate {
    x: f64,
    y: f64
}

impl Coordinate {
    fn new() -> Coordinate {
        Coordinate { x: 0.0, y: 0.0 }
    }
}

#[derive(Debug, PartialEq)]
struct Centroid {
    loc: Coordinate,
    number: u16
}

impl Centroid {
    fn new() -> Centroid {
        Centroid { 
            loc: Coordinate::new(), 
            number: 0
        }
    }
}

#[derive(Debug, PartialEq)]
enum Resource {
    Block,
    Rock,
    Timber,
    Fiber,
    Cereal
}

#[derive(Debug, PartialEq)]
struct Hexagon {
    vertices: Vec<Coordinate>,
    number: u16,
    resource: Resource
}

#[derive(Debug, PartialEq)]
struct ResourceList {
    block: u16,
    rock: u16,
    timber: u16,
    fiber: u16,
    cereal: u16
}

#[derive(Debug, PartialEq)]
struct HexagonState {
    centroids: Vec<Centroid>,
    nodes: Vec<Coordinate>,
    hexagons: Vec<Hexagon>,
    roads: Vec<(u32,u32)>,
    roll_result: (u8,u8),
    player_resources: HashMap<String, ResourceList>,
    bugs: HashMap<String, u8>,
    scorpion_index: Option<u32>
}

impl HexagonState {
    fn new() -> HexagonState {
        HexagonState { 
            centroids: Vec::new(), 
            nodes: Vec::new(), 
            hexagons: Vec::new(), 
            roads: Vec::new(), 
            roll_result: (0,0), 
            player_resources: HashMap::new(), 
            bugs: HashMap::new(), 
            scorpion_index: None
        }
    }
}

struct HexagonIslandStatus {
    phase: Phase,
    round: u16,
    players: Players
}

#[derive(Debug, PartialEq)]
struct HexagonIslandConfig {
    num_players: u8,
    score_to_win: u8,
    game_board_width: u8
}

#[derive(Debug, PartialEq)]
struct HexagonIsland {
    phase: Phase,
    round: u16,
    players: Players,
    possible_actions: PossibleActions,
    config: HexagonIslandConfig,
    state: HexagonState
}

impl Game for HexagonIsland {
    type Status = HexagonIslandStatus;
    type Command = CoreCommand;
    type Config = HexagonIslandConfig;

    fn new() -> HexagonIsland {
        HexagonIsland {
            phase: Phase::Boot,
            round: 0,
            players: Players::new(),
            possible_actions: PossibleActions::None,
            config: HexagonIslandConfig {
                num_players: 2,
                score_to_win: 10,
                game_board_width: 5
            },
            state: HexagonState::new()
        }
    }

    /// For progressing the phase of the game
    fn next_phase(&mut self) -> &mut HexagonIsland {
        self.phase.next_phase();

        self
    }

    /// For moving the game to the next round
    fn next_round(&mut self) -> &mut HexagonIsland {
        self.round += 1;

        self
    }

    /// For resetting the game to the initial state
    fn reset(&mut self) -> &mut HexagonIsland {
        self.phase = Phase::Boot;
        self.round = 0;
        self.players.reset();

        self
    }

    fn add_player(&mut self, key: &str, name: &str, socket_id: &str) -> &mut HexagonIsland {
        self.players.add_player(key, name, socket_id);

        self
    }

    fn set_active_player(&mut self, key: &str) -> Result<&mut HexagonIsland, &'static str> {
        match self.players.set_active_player(key) {
            Ok(_) => Ok(self),
            Err(e) => Err(e)
        }
    }
    
    fn next_player(&mut self) -> Result<&mut HexagonIsland, &'static str> {
        match self.players.next_player() {
            Ok(_) => Ok(self),
            Err(e) => Err(e)
        }
    }

    fn get_game_status(&self) -> HexagonIslandStatus {
        HexagonIslandStatus { 
            phase: self.phase.clone(),
            round: self.round.clone(),
            players: self.players.clone()
        }
    }

    fn process_action(&mut self, command: Self::Command) -> Result<&mut HexagonIsland, &'static str> {
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