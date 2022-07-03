use rand::{thread_rng, Rng};

use super::board::{ Road, Node, ResourceList };

#[derive(Clone, Debug, PartialEq)]
pub enum PossibleActions {
    RollDice,
    BuildRoad,
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
    roads: &mut Vec<Road>, 
    nodes: &Vec<Node>, 
    resources: &mut ResourceList, 
    require_payment: bool
) {
    
    if require_payment {
        // TODO: Credit check
    }

    // TODO: Check for valid road index

    // TODO: Check if there is already a built road on this index

    // TODO: Do either of the nodes connected by this road contain a building by this player?

     // TODO: Is there an adjacent road owned by this player?

     // TODO: Deduct resources

     roads[road_index].player_key = Some(player_key);

}

#[cfg(test)]
mod test;