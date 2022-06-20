use std::collections::HashMap;
use crate::games::core::{
    Player,
    Phase,
    PossibleActions,
    CoreConfigType,
    traits::Game
};

struct HexagonIsland {
    phase: Phase,
    round: u16,
    players: Vec<Player>,
    active_key: Option<String>,
    cardinality: usize,
    possible_actions: PossibleActions,
    config: HashMap<String, CoreConfigType>
}

impl HexagonIsland {
    fn new() -> HexagonIsland {
        HexagonIsland {
            phase: Phase::Boot,
            round: 0,
            players: Vec::new(),
            active_key: None,
            cardinality: 0,
            possible_actions: PossibleActions::None,
            config: HashMap::new()
        }
    }
}

#[cfg(test)]
mod test;