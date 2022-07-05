use rand::{thread_rng, Rng};

use super::board::{ Road, Node, ResourceList, BuildingType };

#[derive(Clone, Debug, PartialEq)]
pub enum PossibleActions {
    RollDice,
    BuildStuff,
    None
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
    roads: &mut Vec<Road>
) {

    // TODO: Check for valid road index

    // TODO: Check if there is already a built road on this index

    // TODO: Do either of the nodes connected by this road contain a building by this player?

     // TODO: Is there an adjacent road owned by this player?

     roads[road_index].player_key = Some(player_key);

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

#[cfg(test)]
mod test;