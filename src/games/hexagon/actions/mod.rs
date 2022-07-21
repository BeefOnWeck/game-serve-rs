use rand::{thread_rng, Rng};

use super::board::{ Road, Node, BuildingType };

#[derive(Clone, Debug, PartialEq)]
pub enum PossibleActions {
    PlaceVillageAndRoad,
    RollDice,
    BuildStuff,
    EndTurn,
    None
}

#[derive(Copy, Clone, PartialEq)]
pub enum Target {
    Road,
    Node,
    None
}

pub struct Command {
    pub action: PossibleActions,
    pub player: String,
    pub target: [( Target, Option<usize> ); 5] // TODO: Const Generic
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
    nodes: & Vec<Node>, 
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
        let no_adjacent_road = roads.iter().fold(
            true,
            | acc, cv | {
                let ind_align = 
                    cv.inds.0 == idx1 ||
                    cv.inds.1 == idx1 ||
                    cv.inds.0 == idx2 ||
                    cv.inds.1 == idx2;
                let player_align = cv.player_key == some_player_key_clone;
                return acc || (ind_align && player_align);
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
    roads: &Vec<Road>,
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
        let no_adjacent_roads: bool = roads.iter().fold(
            true,
            | acc, cv | {
                if (
                    cv.inds.0 == node_index ||
                    cv.inds.1 == node_index
                ) && cv.player_key == Some(player_key.clone()) {
                    return false;
                } else {
                    return acc;
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

pub fn count_player_nodes(player_key: &String, nodes: &Vec<Node>) -> usize {
    let num_player_nodes: usize = nodes.iter().fold(
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
pub fn count_player_roads(player_key: &String, roads: &Vec<Road>) -> usize {
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