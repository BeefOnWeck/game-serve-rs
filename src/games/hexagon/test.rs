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
                active_player: None,
                cardinality: 0,
            },
            possible_actions: PossibleActions::None,
            config: Config {
                num_players: 2,
                score_to_win: 10,
                game_board_width: 5
            },
            roll_result: (0,0), 
            player_colors: HashMap::new(),
            player_resources: HashMap::new(), 
            board: GameBoard::new()
        }
    )
}

#[test]
fn game_configuration() {
    let mut game = HexagonIsland::new();
    let num_players = 4;
    let score_to_win = 15;
    let game_board_width = 9;
    let config = Config {
        num_players,
        score_to_win,
        game_board_width
    };
    game.configure_game(config).unwrap();
    assert_eq!(game.config.num_players, 4);
    assert_eq!(game.config.score_to_win, 15);
    assert_eq!(game.config.game_board_width, 9);
    game.next_phase();
    let attempt = game.configure_game(Config {
        num_players: 2,
        score_to_win: 7,
        game_board_width: 7
    });
    assert_eq!(attempt, Err("Cannot configure game outside of boot phase!"));
}

#[test]
fn should_reset() {
    let mut game = HexagonIsland::new();
    let config = Config {
        num_players: 2,
        score_to_win: 10,
        game_board_width: 5
    };
    game.configure_game(config).unwrap();
    game.board.setup(5);

    game.reset();
    assert_eq!(
        game,
        HexagonIsland {
            phase: Phase::Boot,
            round: 0,
            players: Players {
                list: Vec::new(),
                active_player: None,
                cardinality: 0,
            },
            possible_actions: PossibleActions::None,
            config: Config {
                num_players: 2,
                score_to_win: 10,
                game_board_width: 5
            },
            roll_result: (0,0),
            player_colors: HashMap::new(),
            player_resources: HashMap::new(), 
            board: GameBoard::new()
        }
    )
}

#[test]
fn can_roll_the_dice() {
    let mut game = HexagonIsland::new();
    game.phase = Phase::Play;
    assert_eq!(game.roll_result, (0,0));
    // TODO: Verify error
    let command = Command {
        action: PossibleActions::RollDice,
        player: String::from("key1"),
        target: [(Target::None, None); 5]
    };
    game.process_action(command).unwrap();
    assert!(game.roll_result != (0,0));
}

#[test]
fn player_color() {
    let mut game = HexagonIsland::new();
    let config = Config {
        num_players: 2,
        score_to_win: 10,
        game_board_width: 5
    };
    game.configure_game(config).unwrap();

    game.add_player("key1", "name1", "socket_id1").unwrap()
        .add_player("key2", "name2", "socket_id2").unwrap();

    let mut expected_colors: HashMap<String, String> = HashMap::new();
    expected_colors.insert(String::from("key1"), String::from("#DC143C"));
    expected_colors.insert(String::from("key2"), String::from("#4169E1"));

    assert_eq!(game.players.cardinality, 2);
    assert_eq!(game.player_colors, expected_colors);
}

// #[test]
// fn building_nodes_and_roads() {
//     let mut game = HexagonIsland::new();
//     let config = Config {
//         num_players: 2,
//         score_to_win: 10,
//         game_board_width: 5
//     };
//     game.configure_game(config).unwrap();

//     game.add_player("key1", "name1", "socket_id1").unwrap()
//         .add_player("key2", "name2", "socket_id2").unwrap();

//     game.next_phase();

//     let num_built_nodes = game.board.nodes.iter().fold(
//         0, 
//         | acc, cv | if cv.player_key != None { acc + 1 } else { acc }
//     );
//     assert_eq!(num_built_nodes, 0);
//     let num_built_nodes = game.board.nodes.iter().fold(
//         0, 
//         | acc, cv | if cv.player_key != None { acc + 1 } else { acc }
//     );
//     assert_eq!(num_built_nodes, 0);

//     let mut command = Command::new(
//         PossibleActions::BuildStuff,
//         String::from("key1")
//     );
//     command.target[0] = (Target::Node, Some(0));
//     let attempt = game.process_action(command);
//     assert_eq!(attempt, Err("Not enough resources to build."));

//     let resources = game.player_resources.get_mut("key1").unwrap();
//     let _status = resources.deposit([Resource::Block, Resource::Timber, Resource::Fiber, Resource::Cereal]);

//     let mut command = Command::new(
//         PossibleActions::BuildStuff,
//         String::from("key1")
//     );
//     command.target[0] = (Target::Node, Some(0));
//     game.process_action(command).unwrap();

//     let mut command = Command::new(
//         PossibleActions::BuildStuff,
//         String::from("key1")
//     );
//     command.target[0] = (Target::Road, Some(0));
//     let attempt = game.process_action(command);
//     assert_eq!(attempt, Err("Not enough resources to build."));

//     let resources = game.player_resources.get_mut("key1").unwrap();
//     let _status = resources.deposit([Resource::Block, Resource::Timber]);

//     let mut command = Command::new(
//         PossibleActions::BuildStuff,
//         String::from("key1")
//     );
//     command.target[0] = (Target::Road, Some(0));
//     game.process_action(command).unwrap();

//     let num_built_roads = game.board.roads.iter().fold(
//         0, 
//         | acc, cv | if cv.player_key != None { acc + 1 } else { acc }
//     );
//     assert_eq!(num_built_roads, 1);

//     let num_built_nodes = game.board.nodes.iter().fold(
//         0, 
//         | acc, cv | if cv.player_key != None { acc + 1 } else { acc }
//     );
//     assert_eq!(num_built_nodes, 1);
// }

#[test]
fn too_many_players() {
    let mut game = HexagonIsland::new();

    game.add_player("key1", "name1", "socket_id1").unwrap();
    game.add_player("key2", "name2", "socket_id2").unwrap();
    let attempt = game.add_player("key3", "name3", "socket_id3");
    assert_eq!(attempt, Err("Cannot add player; exceeds maximum number of players."));
}

#[test]
fn game_progression() {
    let mut game = HexagonIsland::new();
    let config = Config {
        num_players: 2,
        score_to_win: 10,
        game_board_width: 5
    };
    game.configure_game(config).unwrap();

    assert_eq!(game.phase, Phase::Boot);

    game.add_player("key1", "name1", "socket_id1").unwrap();
    game.add_player("key2", "name2", "socket_id2").unwrap();

    assert_eq!(game.phase, Phase::Setup);

    let mut command = Command::new(
        PossibleActions::PlaceVillageAndRoad,
        String::from("key1")
    );
    command.target[0] = (Target::Node, Some(0));
    let attempt = game.process_action(command);
    assert_eq!(attempt, Err("Must select one node and one road during setup."));

    // TODO: Check that it is player 1's turn

    let mut command = Command::new(
        PossibleActions::PlaceVillageAndRoad,
        String::from("key1")
    );
    command.target[0] = (Target::Node, Some(0));
    command.target[1] = (Target::Road, Some(0));
    game.process_action(command).unwrap();

    // TODO: Check that it is player 2's turn

    let mut command = Command::new(
        PossibleActions::PlaceVillageAndRoad,
        String::from("key2")
    );
    command.target[0] = (Target::Node, Some(9));
    command.target[1] = (Target::Road, Some(10));
    game.process_action(command).unwrap();

    // TODO: Check that it is player 2's turn again

    let mut command = Command::new(
        PossibleActions::PlaceVillageAndRoad,
        String::from("key2")
    );
    command.target[0] = (Target::Node, Some(20));
    command.target[1] = (Target::Road, Some(25));
    game.process_action(command).unwrap();

    // TODO: Check that it is player 1's turn

    let mut command = Command::new(
        PossibleActions::PlaceVillageAndRoad,
        String::from("key1")
    );
    command.target[0] = (Target::Node, Some(4));
    command.target[1] = (Target::Road, Some(5));
    game.process_action(command).unwrap();

    // TODO: Check that it is the play phase
    // TODO: Check that it is still player 1's turn

}