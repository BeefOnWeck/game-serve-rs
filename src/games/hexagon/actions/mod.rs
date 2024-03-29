use rand::{thread_rng, Rng};
use serde::{Serialize, Deserialize};

use super::board::{ Road, Node, BuildingType };
use super::resources::{Resource};

#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Actions {
    PlaceVillageAndRoad,
    RollDice,
    MoveScorpion,
    BuildStuff,
    Trade,
    BuyBug,
    EndTurn,
    None
}

#[derive(Copy, Clone, Debug, PartialEq, Deserialize, Serialize)]
pub enum Target {
    Road,
    Node,
    Hex
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Command {
    pub action: Actions,
    pub player: String,
    pub target: [Option<(Target,usize)>; 5],
    pub trade: Option<(Resource,Resource)>
}

impl Command {
    pub fn new(action: Actions, player: String) -> Command {
        Command { 
            action, 
            player,
            target: [None; 5],
            trade: None
        }
    }

    pub fn get_first(&self, cmd_tgt: Target) -> (usize,usize) {
        self.target.iter().fold(
            (0,0),
            | mut acc, cv | {
                if let Some(cmd) = cv {
                    if cmd.0 == cmd_tgt {
                        acc.0 += 1;
                        if acc.0 == 1 { acc.1 = cmd.1 }
                    }
                }
                acc
            }
        )
    }

    pub fn get_all(&self, cmd_tgt: Target) -> Vec<usize> {
        self.target.iter()
            .filter(|t| t.is_some() && t.unwrap().0 == cmd_tgt)
            .map(|t| t.unwrap().1)
            .collect()
    }

    pub fn get_trade(&self) -> Result<(Resource,Resource),&'static str> {
        self.trade.ok_or("No resources were specified in the trade.")
    }
}

pub fn next_allowed_actions(last_action: &Actions, roll_sum: u8) -> Vec<Actions> {
    match last_action {
        Actions::PlaceVillageAndRoad => vec![
            Actions::RollDice
        ],
        Actions::RollDice => {
            match roll_sum {
                7 => vec![
                    Actions::MoveScorpion
                ],
                _ =>vec![
                    Actions::Trade,
                    Actions::BuildStuff,
                    Actions::BuyBug,
                    Actions::EndTurn
                ]
            }
        },
        Actions::MoveScorpion => vec![
            Actions::Trade,
            Actions::BuildStuff,
            Actions::BuyBug,
            Actions::EndTurn
        ],
        Actions::BuildStuff => vec![
            Actions::Trade,
            Actions::BuildStuff,
            Actions::BuyBug,
            Actions::EndTurn
        ],
        Actions::Trade => vec![
            Actions::Trade,
            Actions::BuildStuff,
            Actions::BuyBug,
            Actions::EndTurn
        ],
        Actions::BuyBug => vec![
            Actions::MoveScorpion
        ],
        Actions::EndTurn => vec![
            Actions::RollDice
        ],
        Actions::None => vec![
            Actions::None
        ]
    }
}

pub fn roll_dice() -> (u8,u8) {
    let mut rng = thread_rng();
    (
        rng.gen_range(1..=6), 
        rng.gen_range(1..=6)
    )
}

pub fn build_road(
    road_index: usize, 
    player_key: String, 
    nodes: &[Node], 
    roads: &mut Vec<Road>,
    is_setup: bool
) -> Result<(), &'static str> {

    // Check for valid road index
    if road_index >= roads.len() {
        return Err("Cannot build road; invalid road index.");
    }

    // Check if there is already a built road on this index
    if roads[road_index].player_key != None {
        return Err("Cannot build road; there is already something there.");
    }

    if is_setup == false {
        // Do either of the nodes connected by this road contain a building by this player?
        let mut no_adjacent_building = true;
        let (idx1,idx2) = roads[road_index].inds;
        let some_player_key_clone = Some(player_key.clone());
        if nodes[idx1].player_key == some_player_key_clone || nodes[idx2].player_key == some_player_key_clone {
            no_adjacent_building = false;
        }

        // Is there an adjacent road owned by this player?
        let no_adjacent_road = !roads.iter().fold(
            false,
            | acc, cv | {
                let ind_align = 
                    cv.inds.0 == idx1 ||
                    cv.inds.1 == idx1 ||
                    cv.inds.0 == idx2 ||
                    cv.inds.1 == idx2;
                let player_align = cv.player_key == some_player_key_clone;
                acc || (ind_align && player_align)
            }
        );

        if no_adjacent_building && no_adjacent_road {
            return Err("Roads have to be built next to other roads or buildings you own.");
        }
    }

     roads[road_index].player_key = Some(player_key);

     Ok(())

}

pub fn build_node(
    node_index: usize, 
    player_key: String, 
    nodes: &mut Vec<Node>, 
    roads: &[Road],
    is_setup: bool
) -> Result<(), &'static str> {

    // Check for valid node index
    if node_index >= nodes.len() {
        return Err("Cannot make building; invalid node index.");
    }

    // Check if there is already a built node on this index
    if nodes[node_index].player_key != None {
        return Err("Cannot make building; there is already something there.");
    }

    // Find adjacent nodes
    let adjacent_nodes: Vec<usize> = roads.iter().filter(
        | r | r.inds.0 == node_index || r.inds.1 == node_index
    ).map(
        | r | if r.inds.0 == node_index { r.inds.1 } else { r.inds.0 }
    ).collect();

    // Check if there are buildings on adjacent nodes
    let mut is_adjacent_building = false;
    for idx in adjacent_nodes {
        if nodes[idx].player_key != None { is_adjacent_building = true; }
    }
    if is_adjacent_building {
        return Err("Cannot make building; you must respect the two-space rule.");
    }

    // Is there an adjacent road owned by this player?
    // NOTE: Only check this outside of the setup phase
    if is_setup == false {
        let some_player_key_clone = Some(player_key.clone());
        let no_adjacent_roads: bool = roads.iter().fold(
            true,
            | acc, cv | {
                if (
                    cv.inds.0 == node_index ||
                    cv.inds.1 == node_index
                ) && cv.player_key == some_player_key_clone {
                    false
                } else {
                    acc
                }
            }
        );

        if no_adjacent_roads {
            return Err("Cannot make building; after initial setup you must build next to roads that you own.");
        }
    }

     nodes[node_index].player_key = Some(player_key);
     nodes[node_index].building_type = BuildingType::Village;

     Ok(())

}

// TODO: Create a trait around having a player key and then make this function generic
pub fn count_player_nodes(player_key: &String, nodes: &[Node]) -> u8 {
    let num_player_nodes: u8 = nodes.iter().fold(
        0,
        | mut acc, cv | {
            match cv.player_key.as_ref() {
                Some(pk) => { if player_key == pk { acc += 1; } },
                None => ()
            };

            acc
        }
    );

    num_player_nodes
}

// TODO: Create a trait around having a player key and then make this function generic
pub fn count_player_roads(player_key: &String, roads: &[Road]) -> usize {
    let num_player_roads: usize = roads.iter().fold(
        0,
        | mut acc, cv | {
            match cv.player_key.as_ref() {
                Some(pk) => { if player_key == pk { acc += 1; } },
                None => ()
            };

            acc
        }
    );

    num_player_roads
}

#[cfg(test)]
mod test;