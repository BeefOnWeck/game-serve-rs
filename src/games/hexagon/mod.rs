use std::collections::HashMap;
use crate::games::core::{Player,Phase,PossibleActions,CoreConfigType};

struct HexagonIsland {
    phase: Phase,
    round: u16,
    players: Vec<Player>,
    active_player_key: Option<String>,
    num_players: usize,
    possible_actions: PossibleActions,
    config: HashMap<String, CoreConfigType>
}

#[cfg(test)]
mod test;

impl HexagonIsland {
    fn new() -> HexagonIsland {
        HexagonIsland {
            phase: Phase::Boot,
            round: 0,
            players: Vec::new(),
            active_player_key: None,
            num_players: 0,
            possible_actions: PossibleActions::None,
            config: HashMap::new()
        }
    }
}