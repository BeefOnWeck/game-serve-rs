use super::*;

#[test]
fn initial_state() {
    let game = HexagonIsland::new();
    assert_eq!(
        game,
        HexagonIsland {
            phase: Phase::Boot,
            round: 0,
            players: Players {
                list: Vec::new(),
                active_key: None,
                cardinality: 0,
            },
            possible_actions: PossibleActions::None,
            config: HashMap::new(),
            state: HexagonState::new()
        }
    )
}