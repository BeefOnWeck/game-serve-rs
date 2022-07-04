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
    roads: &Vec<Road>
) {

    // TODO: Check for valid road index

    // TODO: Check if there is already a built road on this index

    // TODO: Do either of the nodes connected by this road contain a building by this player?

     // TODO: Is there an adjacent road owned by this player?

     nodes[node_index].player_key = Some(player_key);
     nodes[node_index].building_type = BuildingType::Village

}

#[cfg(test)]
mod test;