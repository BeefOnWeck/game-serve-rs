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
            state: HexagonState { 
                centroids: Vec::new(), 
                nodes: Vec::new(), 
                hexagons: Vec::new(), 
                roads: Vec::new(), 
                roll_result: (0,0), 
                player_resources: HashMap::new(), 
                bugs: HashMap::new(), 
                scorpion_index: None
            }
        }
    )
}

#[test]
fn board_setup() {
    let game = HexagonIsland::new();
    let mut config = HashMap::new();
    config.insert(String::from("config_num_players"), CoreConfigType::Int(2));
    // game.configure_game(config).unwrap();
    // game.setup(3);
}