use std::collections::HashMap;

use crate::games::core::Phase;
use crate::games::core::playe::Players;
use crate::games::core::traits::Game;

mod actions;
mod board;
mod colo;

use actions::{ PossibleActions, roll_dice, build_road };
use board::{ GameBoard, ResourceList };
use colo::get_player_color;

struct Status {
    phase: Phase,
    round: u16,
    players: Players
}

#[derive(Debug, PartialEq)]
struct Config {
    num_players: usize,
    score_to_win: u8,
    game_board_width: u8
}

#[derive(Copy, Clone, PartialEq)]
enum Target {
    Road,
    Village,
    None
}

struct Command {
    pub action: PossibleActions,
    player: String,
    pub target: [( Target, Option<usize> ); 5]
}

impl Command {
    pub fn new(action: PossibleActions, player: String) -> Command {
        Command { 
            action, 
            player: player.clone(),
            target: [( Target::None, None ); 5]
        }
    }
}

#[derive(Debug, PartialEq)]
struct HexagonIsland {
    phase: Phase,
    round: u16,
    players: Players,
    possible_actions: PossibleActions,
    config: Config,
    roll_result: (u8,u8),
    player_colors: HashMap<String, String>,
    player_resources: HashMap<String, ResourceList>,
    board: GameBoard
}

impl Game for HexagonIsland {
    type Status = Status;
    type Command = Command;
    type Config = Config;

    fn new() -> HexagonIsland {
        HexagonIsland {
            phase: Phase::Boot,
            round: 0,
            players: Players::new(),
            possible_actions: PossibleActions::None,
            config: Config {
                num_players: 2,
                score_to_win: 10,
                game_board_width: 5
            },
            roll_result: (0,0),
            player_colors: HashMap::new(),
            player_resources: HashMap::new(), 
            board: GameBoard::new()
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
        self.board.reset();
        self.player_resources.clear();
        self.player_colors.clear();
        self.roll_result = (0,0);

        self
    }

    fn add_player(&mut self, key: &str, name: &str, socket_id: &str) -> &mut HexagonIsland {
        let idx = self.players.cardinality;
        // TODO: Throw error if we already have enough players
        // TODO: Throw error if we're already in the play phase
        self.players.add_player(key, name, socket_id);
        self.player_colors.insert(
            String::from(key),
            get_player_color(idx)
        );
        self.player_resources.insert(
            String::from(key),
            ResourceList { block: 0, rock: 0, timber: 0, fiber: 0, cereal: 0 }
        );

        if self.players.cardinality == self.config.num_players { 
            self.next_phase();
            self.board.setup(5);
        }

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

    fn get_game_status(&self) -> Status {
        Status { 
            phase: self.phase.clone(),
            round: self.round.clone(),
            players: self.players.clone()
        }
    }

    fn process_action(&mut self, command: Self::Command) -> Result<&mut HexagonIsland, &'static str> {
        // TODO: Throw error if player tries an action out of turn (need to augment Command)
        match self.phase {
            Phase::Setup | Phase::Play => match command.action {
                PossibleActions::RollDice => {
                    self.roll_result = roll_dice();
                    Ok(self)
                },
                PossibleActions::BuildStuff => {
                    // 1. Loop over node and road indices (roads first)
                    // 2. Get cost to build
                    // 3. Check credit
                    // 4. If check passes, build
                    // 5. If build passes, deduct cost
                    // let resources = self.player_resources.get_mut(&command.player).unwrap();
                    let roads = command.target.iter().filter(|t| t.0 == Target::Road);
                    for r in roads {
                        build_road(
                            r.1.unwrap(), 
                            command.player.clone(), 
                            &mut self.board.roads, 
                            &self.board.nodes
                        );
                    }
                    
                    Ok(self)
                }
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