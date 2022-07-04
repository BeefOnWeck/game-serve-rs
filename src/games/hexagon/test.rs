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
                active_key: None,
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

    game.add_player("key1", "name1", "socket_id1")
        .add_player("key2", "name2", "socket_id2");

    let mut expected_colors: HashMap<String, String> = HashMap::new();
    expected_colors.insert(String::from("key1"), String::from("#DC143C"));
    expected_colors.insert(String::from("key2"), String::from("#4169E1"));

    assert_eq!(game.players.cardinality, 2);
    assert_eq!(game.player_colors, expected_colors);
}

#[test]
fn build_nodes_and_roads() {
    let mut game = HexagonIsland::new();
    let config = Config {
        num_players: 2,
        score_to_win: 10,
        game_board_width: 5
    };
    game.configure_game(config).unwrap();

    game.add_player("key1", "name1", "socket_id1")
        .add_player("key2", "name2", "socket_id2");

    let mut command = Command::new(
        PossibleActions::BuildStuff,
        String::from("key1")
    );
    command.target[0] = (Target::Road, Some(0));
    game.process_action(command).unwrap();

    let num_built_roads = game.board.roads.iter().fold(
        0, 
        | acc, cv | if cv.player_key != None { acc + 1 } else { acc }
    );
    assert_eq!(num_built_roads, 1);
}