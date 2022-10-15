use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use serde_json::to_string;

use crate::games::{ core::Phase };
use crate::games::core::playe::Players;
use crate::games::core::traits::Game;

pub mod actions;
mod board;
mod colo;
mod resources;
mod bonuses;

use actions::{ 
    Actions, 
    Target, 
    Command,
    next_allowed_actions,
    roll_dice, 
    build_road,
    build_node,
    count_player_nodes,
    count_player_roads
};
use board::GameBoard;
use colo::get_player_color;
use resources::{ Resource, ResourceList };
use bonuses::{ find_most_bugs };

#[derive(Serialize)]
pub struct Status {
    key: String,
    phase: Phase,
    round: u16,
    colors: HashMap<String, String>,
    resources: ResourceList
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Config {
    num_players: usize,
    score_to_win: u8,
    game_board_width: u8
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct HexagonIsland {
    phase: Phase,
    round: u16,
    pub players: Players,
    last_action: Actions,
    config: Config,
    roll_result: (u8,u8),
    player_colors: HashMap<String, String>,
    player_resources: HashMap<String, ResourceList>,
    bugs: HashMap<String, u8>,
    has_most_bugs: Option<String>,
    board: GameBoard,
    the_winner: Option<String>
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
            bugs: HashMap::new(),
            has_most_bugs: None,
            board: GameBoard::new(),
            the_winner: None
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
        self.bugs.clear();
        self.has_most_bugs = None;
        self.roll_result = (0,0);
        self.the_winner = None;
        self.last_action = Actions::None;

        self
    }

    fn add_player(&mut self, key: &str, name: &str) -> Result<&mut HexagonIsland, &'static str> {

        if self.players.cardinality == self.config.num_players {
            return Err("Cannot add player; exceeds maximum number of players.");
        }

        println!("Added player");

        self.player_colors.insert(String::from(key), get_player_color(self.players.cardinality));
        self.player_resources.insert(String::from(key), ResourceList::new());
        self.bugs.insert(String::from(key), 0);
        self.players.add_player(key, name);

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
                let active_player = self.players.active_player
                    .as_ref()
                    .ok_or("Can't get active player")?;
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

    // TODO: Can I just use serde to serialize this?
    fn get_game_status(&self, key: &str) -> String {
        let mut allowed_actions = Vec::<Actions>::new();
        if let Some(active_player) = &self.players.active_player {
            if active_player.key == key {
                if self.phase == Phase::Setup {
                    allowed_actions = vec![Actions::PlaceVillageAndRoad];
                } else if self.phase == Phase::Play {
                    let roll_sum = self.roll_result.0 + self.roll_result.1;
                    allowed_actions = next_allowed_actions(&self.last_action, roll_sum);
                } else {
                    allowed_actions = vec![Actions::None];
                }
            } else {
                allowed_actions = vec![Actions::None];
            }
        }
        let resources: ResourceList;
        match self.player_resources.get(key) {
            Some(list) => resources = *list,
            None => resources = ResourceList::new()
        }
        let bugs: u8;
        match self.bugs.get(key) {
            Some(bug) => bugs = *bug,
            None => bugs = 0
        }
        let status = String::new() + 
            "{" +
                "\"key\": " + "\"" + key + "\"," +
                "\"phase\": " + "\"" + &self.phase.to_string() + "\"," +
                "\"round\": " + &self.round.to_string() + "," +
                "\"active_player\": " + &to_string(&self.players.active_player).unwrap() + "," +
                "\"roll_result\": " + &to_string(&self.roll_result).unwrap() + "," +
                "\"allowed_actions\": " + &to_string(&allowed_actions).unwrap() + "," +
                "\"the_winner\": " + &to_string(&self.the_winner).unwrap() + "," +
                "\"colors\": " + &to_string(&self.player_colors).unwrap() + "," +
                "\"resources\": " + &to_string(&resources).unwrap() + "," +
                "\"bugs\": " + &to_string(&bugs).unwrap() + "," +
                "\"has_most_bugs\": " + &to_string(&self.has_most_bugs).unwrap() + "," +
                "\"board\": " + &to_string(&self.board).unwrap() +
            "}";

        status
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

    fn find_the_winner(&mut self) -> &mut HexagonIsland {
        for player in self.players.list.iter() {
            let building_score = count_player_nodes(&player.key, &self.board.nodes);
            // TODO: Longest road bonus
            // TODO: Most bugs bonus
            let score = building_score;
            if score >= self.config.score_to_win {
                self.the_winner = Some(player.key.clone());
            }
        }
        self
    }

    fn process_action(&mut self, command: Self::Command) -> Result<&mut HexagonIsland, &'static str> {
        
        let active_player = self.players.active_player
            .as_ref()
            .ok_or("Can't get active player")?;
        if command.player != active_player.key { return Err("It is not your turn."); }
        
        println!("{:?}", command);

        match self.phase {
            Phase::Setup => match command.action {
                Actions::PlaceVillageAndRoad => {
                    println!("{:?}", self.last_action);
                    if self.last_action != Actions::None && self.last_action != Actions::EndTurn {
                        return Err("That is not an allowed action right now.");
                    }
                    let (num_nodes, node_index) = command.get_first(Target::Node);
                    let (num_roads, road_index) = command.get_first(Target::Road);
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
                    println!("{:?}", self.last_action);
                    if self.last_action != Actions::PlaceVillageAndRoad {
                        return Err("That is not an allowed action right now.");
                    }
                    // TODO: Refactor into a function in board
                    let (
                        all_players_have_exactly_one,
                        all_players_have_at_least_one,
                        all_players_have_exactly_two
                    ) = self.players.list.iter().fold(
                        (true,true,true),
                        | acc, cv | {
                            let num_nodes = count_player_nodes(&cv.key, &self.board.nodes);
                            let num_roads = count_player_roads(&cv.key, &self.board.roads);
                            (
                                acc.0 && num_nodes == 1 && num_roads == 1,
                                acc.1 && num_nodes >= 1 && num_roads >= 1,
                                acc.2 && num_nodes == 2 && num_roads == 2
                            )
                        }
                    );

                    if all_players_have_exactly_one { }
                    else if all_players_have_exactly_two {
                        // TODO: Refactor into a function in board
                        let spoils = self.board.resolve_setup();
                        for (player_key,resource) in spoils {
                            let resources = self.player_resources
                                .get_mut(&player_key)
                                .ok_or("Can't get player resources.")?;
                            if resource != Resource::Desert { resources.deposit([resource])?; }
                        }
                        self.next_phase();
                        self.next_round();
                    }
                    else if all_players_have_at_least_one { self.previous_player()?; }
                    else { self.next_player()?; }

                    self.last_action = Actions::EndTurn;

                    Ok(self)
                },
                _ => Err("That is not an allowed action during the Setup Phase.")
            }, 
            Phase::Play => {
                // Check if command.action is allowed
                let roll_sum = self.roll_result.0 + self.roll_result.1;
                let allowed_actions = next_allowed_actions(&self.last_action, roll_sum);
                let valid_action = allowed_actions.iter().any(|&a| a == command.action);
                if !valid_action {
                    return Err("That is not an allowed action right now.");
                }

                match command.action {
                    Actions::RollDice => {
                        self.roll_result = roll_dice();
                        let roll_sum = self.roll_result.0 + self.roll_result.1;
                        match roll_sum {
                            7 => (), // Move the scorpion
                            _ => {
                                let spoils = self.board.resolve_roll(roll_sum);
                                for (player_key, resource) in spoils {
                                    let resources = self.player_resources
                                        .get_mut(&player_key)
                                        .ok_or("Can't get player resources.")?;
                                    resources.deposit([resource])?;
                                }
                            }
                        }
                        self.last_action = command.action;
                        Ok(self)
                    },
                    Actions::BuildStuff => {
                        let resources = self.player_resources
                            .get_mut(&command.player)
                            .ok_or("Can't get player resources.")?;

                        let roads = command.get_all(Target::Road);
                        for road in roads {
                            resources.check([Resource::Block, Resource::Timber])?;
                            build_road(
                                road, 
                                command.player.clone(), 
                                &self.board.nodes,
                                &mut self.board.roads,
                                false
                            )?;
                            resources.deduct([Resource::Block, Resource::Timber])?;
                        }

                        let nodes = command.get_all(Target::Node);
                        for node in nodes {
                            resources.check([Resource::Block, Resource::Timber, Resource::Fiber, Resource::Cereal])?;
                            build_node(
                                node,
                                command.player.clone(),
                                &mut self.board.nodes,
                                &self.board.roads,
                                false
                            )?;
                            resources.deduct([Resource::Block, Resource::Timber, Resource::Fiber, Resource::Cereal])?;
                        }

                        // TODO: Find the longest road
                        self.find_the_winner();
                        
                        self.last_action = command.action;
                        Ok(self)
                    },
                    Actions::MoveScorpion => {
                        let (num_hex, hex_index) = command.get_first(Target::Hex);
                        
                        if num_hex != 1 {
                            return Err("Must select one hexagon when moving the scorpion.");
                        }

                        if hex_index >= self.board.hexagons.len() {
                            return Err("Cannot move scorpion; invalid hexagon index.");
                        }

                        self.board.scorpion_index = Some(hex_index);

                        self.last_action = command.action;
                        Ok(self)
                    },
                    Actions::Trade => {
                        let resources = self.player_resources
                            .get_mut(&command.player)
                            .ok_or("Can't get player resources.")?;

                        let trades = command.get_trade()?;

                        resources.trade(trades.0, trades.1)?;

                        self.last_action = command.action;
                        Ok(self)
                    },
                    Actions::BuyBug => {
                        let resources = self.player_resources
                            .get_mut(&command.player)
                            .ok_or("Can't get player resources.")?;

                        resources.check([Resource::Rock, Resource::Fiber, Resource::Cereal])?;
                        
                        let bugs = self.bugs
                            .get_mut(&command.player)
                            .ok_or("Can't get player bugs")?;

                        *bugs = *bugs + 1;

                        resources.deduct([Resource::Rock, Resource::Fiber, Resource::Cereal])?;

                        self.has_most_bugs = find_most_bugs(&self.bugs, &self.has_most_bugs);
                        self.find_the_winner();

                        self.last_action = command.action;
                        Ok(self)
                    },
                    Actions::EndTurn => {
                        match &self.the_winner {
                            Some(_) => { self.next_phase(); }
                            None => { self.next_player()?; }
                        }
                        
                        self.last_action = command.action;
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