use std::collections::HashMap;

use crate::games::core::Phase;
use crate::games::core::playe::Players;
use crate::games::core::traits::Game;

mod actions;
mod board;
mod colo;
mod resources;

use actions::{ 
    Actions, 
    Target, 
    Command,
    next_allowed_actions,
    roll_dice, 
    build_road,
    count_player_nodes,
    count_player_roads
};
use board::{ GameBoard };
use colo::get_player_color;
use resources::{ Resource, ResourceList };

use self::actions::build_node;

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

#[derive(Debug, PartialEq)]
struct HexagonIsland {
    phase: Phase,
    round: u16,
    players: Players,
    last_action: Actions,
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
            last_action: Actions::None,
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

    fn add_player(&mut self, key: &str, name: &str, socket_id: &str) -> Result<&mut HexagonIsland, &'static str> {
        let idx = self.players.cardinality;
        if idx == self.config.num_players {
            return Err("Cannot add player; exceeds maximum number of players.");
        }

        self.players.add_player(key, name, socket_id);
        self.player_colors.insert(
            String::from(key),
            get_player_color(idx)
        );
        self.player_resources.insert(
            String::from(key),
            ResourceList::new()
        );

        if self.players.cardinality == self.config.num_players { 
            self.next_phase();
            self.board.setup(self.config.game_board_width);
        }

        Ok(self)
    }

    fn set_active_player(&mut self, key: &str) -> Result<&mut HexagonIsland, &'static str> {
        match self.players.set_active_player(key) {
            Ok(_) => Ok(self),
            Err(e) => Err(e)
        }
    }
    
    fn next_player(&mut self) -> Result<&mut HexagonIsland, &'static str> {
        match self.players.next_player(1) {
            Ok(_) => {
                let active_player = self.players.active_player.as_ref().unwrap();
                let active_player_index = self.players.list.iter().position(|p| p == active_player);
                if active_player_index == Some(0) { self.next_round(); }
                Ok(self)
            },
            Err(e) => Err(e)
        }
    }

    fn previous_player(&mut self) -> Result<&mut HexagonIsland, &'static str> {
        match self.players.next_player(-1) {
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

    fn configure_game(&mut self, config: Self::Config) -> Result<&mut Self, &'static str> {
        match self.phase {
            Phase::Boot => {
                self.config = config;
                Ok(self)
            },
            _ => Err("Cannot configure game outside of boot phase!")
        }
    }

    fn process_action(&mut self, command: Self::Command) -> Result<&mut HexagonIsland, &'static str> {
        // TODO: Throw error if player tries an action out of turn (need to augment Command)
        match self.phase {
            Phase::Setup => match command.action {
                Actions::PlaceVillageAndRoad => {
                    let (num_nodes, node_index, num_roads, road_index) = command.target.iter().fold(
                        (0,0,0,0),
                        | mut acc, cv | {
                            match cv.0 {
                                Target::Node => {
                                    acc.0 += 1;
                                    acc.1 = cv.1.unwrap();
                                },
                                Target::Road => {
                                    acc.2 += 1;
                                    acc.3 = cv.1.unwrap();
                                },
                                Target::None => ()
                            }
                            acc
                        }
                    );
                    if num_nodes != 1 || num_roads != 1 {
                        return Err("Must select one node and one road during setup.");
                    }
                    let adj_nodes = self.board.roads[road_index].inds;
                    if adj_nodes.0 != node_index && adj_nodes.1 != node_index {
                        return Err("Selected node and road must be next to each other.");
                    }
                    build_node(
                        node_index, 
                        command.player.clone(), 
                        &mut self.board.nodes,
                        &self.board.roads,
                        true
                    )?;
                    build_road(
                        road_index, 
                        command.player.clone(),
                        &self.board.nodes,
                        &mut self.board.roads,
                        true
                    )?;
                    self.last_action = Actions::PlaceVillageAndRoad;
                    Ok(self)
                },
                Actions::EndTurn => {
                    let (
                        all_players_have_exactly_one,
                        all_players_have_at_least_one,
                        all_players_have_exactly_two
                    ) = self.players.list.iter().fold(
                        (true,true,true),
                        | acc, cv | {
                            let num_nodes = count_player_nodes(&cv.key, &self.board.nodes);
                            let num_roads = count_player_roads(&cv.key, &self.board.roads);
                            return (
                                acc.0 && num_nodes == 1 && num_roads == 1,
                                acc.1 && num_nodes >= 1 && num_roads >= 1,
                                acc.2 && num_nodes == 2 && num_roads == 2
                            );
                        }
                    );

                    if all_players_have_exactly_one { }
                    else if all_players_have_exactly_two {
                        let spoils = self.board.hexagons.iter().enumerate().fold(
                            Vec::new(),
                            |mut acc, (ind,hex)| {
                                let neighboring_nodes = self.board.find_neighboring_nodes(ind);
                                for nn in neighboring_nodes {
                                    match &self.board.nodes[nn].player_key {
                                        Some(player) => acc.push( (player.clone(), hex.resource) ),
                                        None => ()
                                    }
                                }
                                acc
                            }
                        );
                        for (player_key,resource) in spoils {
                            let resources = self.player_resources.get_mut(&player_key).unwrap();
                            if resource != Resource::Desert {
                                resources.deposit([resource])?;
                            }
                        }
                        self.next_phase();
                        self.next_round();
                    }
                    else if all_players_have_at_least_one { self.previous_player()?; }
                    else { self.next_player()?; }

                    Ok(self)
                },
                _ => Err("That is not an allowed action during the Setup Phase.")
            }, 
            Phase::Play => {
                let roll_sum = self.roll_result.0 + self.roll_result.1;
                let allowed_actions = next_allowed_actions(&self.last_action, roll_sum);
                let valid_action = allowed_actions.iter().any(|&a| a == command.action);
                if valid_action == false {
                    return Err("That is not an allowed action right now.");
                }
                match command.action {
                    Actions::RollDice => {
                        self.roll_result = roll_dice();
                        let roll_sum = self.roll_result.0 + self.roll_result.1;
                        match roll_sum {
                            7 => (), // TODO: Move the scorpion
                            _ => {
                                let spoils = self.board.collect_resources(roll_sum);
                                for (player_key, resource) in spoils {
                                    let resources = self.player_resources.get_mut(&player_key).unwrap();
                                    resources.deposit([resource])?;
                                }
                            }
                        }
                        self.last_action = Actions::RollDice;
                        Ok(self)
                    },
                    Actions::BuildStuff => {
                        let resources = self.player_resources.get_mut(&command.player).unwrap();
                        let roads = command.target.iter().filter(|t| t.0 == Target::Road);
                        for r in roads {
                            resources.check([Resource::Block, Resource::Timber])?;
                            build_road(
                                r.1.unwrap(), 
                                command.player.clone(), 
                                &self.board.nodes,
                                &mut self.board.roads,
                                false
                            )?;
                            resources.deduct([Resource::Block, Resource::Timber])?;
                        }

                        let nodes = command.target.iter().filter(|t| t.0 == Target::Node);
                        for n in nodes {
                            resources.check([Resource::Block, Resource::Timber, Resource::Fiber, Resource::Cereal])?;
                            build_node(
                                n.1.unwrap(), 
                                command.player.clone(), 
                                &mut self.board.nodes,
                                &self.board.roads,
                                false
                            )?;
                            resources.deduct([Resource::Block, Resource::Timber, Resource::Fiber, Resource::Cereal])?;
                        }
                        
                        self.last_action = Actions::BuildStuff;
                        Ok(self)
                    },
                    Actions::EndTurn => {
                        self.next_player()?;
                        self.last_action = Actions::EndTurn;
                        Ok(self)
                    },
                    Actions::None => Ok(self),
                    _ => Err("That action is not supported during the Play phase.")
                }
            },
            _ => Err("Can only take action during the Setup or Play phases.")
        }
    }
}

#[cfg(test)]
mod test;