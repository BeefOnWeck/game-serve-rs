use super::*;
use serde_json::to_string;

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
            last_action: Actions::None,
            config: Config {
                num_players: 2,
                score_to_win: 10,
                game_board_width: 5
            },
            roll_result: (0,0), 
            player_colors: HashMap::new(),
            player_resources: HashMap::new(), 
            board: GameBoard::new(),
            the_winner: None
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
            last_action: Actions::None,
            config: Config {
                num_players: 2,
                score_to_win: 10,
                game_board_width: 5
            },
            roll_result: (0,0),
            player_colors: HashMap::new(),
            player_resources: HashMap::new(), 
            board: GameBoard::new(),
            the_winner: None
        }
    )
}

#[test]
fn can_roll_the_dice() {
    let mut game = HexagonIsland::new();
    game.add_player("key1", "name1", "socket_id1").unwrap()
        .add_player("key2", "name2", "socket_id2").unwrap();
    game.phase = Phase::Play;
    game.last_action = Actions::EndTurn;
    assert_eq!(game.roll_result, (0,0));
    // TODO: Verify error
    let command = Command {
        action: Actions::RollDice,
        player: String::from("key1"),
        target: [None; 5],
        trade: None
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

#[test]
fn too_many_players() {
    let mut game = HexagonIsland::new();

    game.add_player("key1", "name1", "socket_id1").unwrap();
    game.add_player("key2", "name2", "socket_id2").unwrap();
    let attempt = game.add_player("key3", "name3", "socket_id3");
    assert_eq!(attempt, Err("Cannot add player; exceeds maximum number of players."));
}

fn game_setup() -> HexagonIsland {
    let mut game = HexagonIsland::new();
    let config = Config {
        num_players: 2,
        score_to_win: 4,
        game_board_width: 5
    };
    game.configure_game(config).unwrap();

    assert_eq!(game.phase, Phase::Boot);

    game.add_player("key1", "name1", "socket_id1").unwrap();
    game.add_player("key2", "name2", "socket_id2").unwrap();

    assert_eq!(game.phase, Phase::Setup);

    let active_player = game.players.active_player.as_ref().unwrap();
    assert_eq!(active_player.key, String::from("key1"));

    let mut command = Command::new(
        Actions::PlaceVillageAndRoad,
        String::from("key1")
    );
    command.target[0] = Some( (Target::Node, 0) );
    let attempt = game.process_action(command);
    assert_eq!(attempt, Err("Must select one node and one road during setup."));

    let active_player = game.players.active_player.as_ref().unwrap();
    assert_eq!(active_player.key, String::from("key1"));

    let mut command = Command::new(
        Actions::PlaceVillageAndRoad,
        String::from("key1")
    );
    command.target[0] = Some( (Target::Node, 10) );
    command.target[1] = Some( (Target::Road, 26) );
    game.process_action(command).unwrap();

    let command = Command::new(
        Actions::EndTurn,
        String::from("key1")
    );
    game.process_action(command).unwrap();

    let active_player = game.players.active_player.as_ref().unwrap();
    assert_eq!(active_player.key, String::from("key2"));

    let mut command = Command::new(
        Actions::PlaceVillageAndRoad,
        String::from("key2")
    );
    command.target[0] = Some( (Target::Node, 15) );
    command.target[1] = Some( (Target::Road, 21) );
    game.process_action(command).unwrap();

    let command = Command::new(
        Actions::EndTurn,
        String::from("key2")
    );
    game.process_action(command).unwrap();

    let active_player = game.players.active_player.as_ref().unwrap();
    assert_eq!(active_player.key, String::from("key2"));

    let mut command = Command::new(
        Actions::PlaceVillageAndRoad,
        String::from("key2")
    );
    command.target[0] = Some( (Target::Node, 42) );
    command.target[1] = Some( (Target::Road, 55) );
    game.process_action(command).unwrap();

    let command = Command::new(
        Actions::EndTurn,
        String::from("key2")
    );
    game.process_action(command).unwrap();

    let active_player = game.players.active_player.as_ref().unwrap();
    assert_eq!(active_player.key, String::from("key1"));

    let mut command = Command::new(
        Actions::PlaceVillageAndRoad,
        String::from("key1")
    );
    command.target[0] = Some( (Target::Node, 20) );
    command.target[1] = Some( (Target::Road, 25) );
    game.process_action(command).unwrap();

    assert_eq!(game.phase, Phase::Setup);

    let command = Command::new(
        Actions::EndTurn,
        String::from("key1")
    );
    game.process_action(command).unwrap();

    assert_eq!(game.phase, Phase::Play);

    let active_player = game.players.active_player.as_ref().unwrap();
    assert_eq!(active_player.key, String::from("key1"));

    let num_player_one_resources = game.player_resources.get(&String::from("key1")).unwrap().count();
    let num_player_two_resources = game.player_resources.get(&String::from("key2")).unwrap().count();

    assert!(num_player_one_resources >= 3);
    assert!(num_player_two_resources >= 3);

    game

}

fn play_round_one(mut game: HexagonIsland) -> HexagonIsland {

    let player_key = String::from("key1");
    let resources = game.player_resources.get_mut(&player_key).unwrap();
    resources.deposit([
        Resource::Block,
        Resource::Timber,
        Resource::Block,
        Resource::Timber,
        Resource::Block,
        Resource::Timber,
        Resource::Fiber,
        Resource::Cereal
    ]).unwrap();

    let player_key = String::from("key2");
    let resources = game.player_resources.get_mut(&player_key).unwrap();
    resources.deposit([
        Resource::Block,
        Resource::Timber,
        Resource::Block,
        Resource::Timber,
        Resource::Block,
        Resource::Timber,
        Resource::Fiber,
        Resource::Cereal
    ]).unwrap();

    let command = Command::new(
        Actions::RollDice,
        String::from("key1")
    );
    game.process_action(command).unwrap();
    game.roll_result = (1,1); // In case we roll a 7

    let mut command = Command::new(
        Actions::BuildStuff,
        String::from("key1")
    );
    command.target[0] = Some( (Target::Road, 27) );
    command.target[1] = Some( (Target::Road, 44) );
    game.process_action(command).unwrap();

    let mut command = Command::new(
        Actions::BuildStuff,
        String::from("key1")
    );
    command.target[0] = Some( (Target::Node, 34) );
    game.process_action(command).unwrap();

    let command = Command::new(
        Actions::EndTurn,
        String::from("key1")
    );
    game.process_action(command).unwrap();

    let command = Command::new(
        Actions::RollDice,
        String::from("key2")
    );
    game.process_action(command).unwrap();
    game.roll_result = (1,1); // In case we roll a 7

    let mut command = Command::new(
        Actions::BuildStuff,
        String::from("key2")
    );
    command.target[0] = Some( (Target::Road, 38) );
    command.target[1] = Some( (Target::Road, 39) );
    game.process_action(command).unwrap();

    let mut command = Command::new(
        Actions::BuildStuff,
        String::from("key2")
    );
    command.target[0] = Some( (Target::Node, 30) );
    game.process_action(command).unwrap();

    let command = Command::new(
        Actions::EndTurn,
        String::from("key2")
    );
    game.process_action(command).unwrap();

    game
}

#[test]
fn game_progression() {
    let mut game = game_setup();
    game = play_round_one(game);

    let active_player = game.players.active_player.as_ref().unwrap();
    assert_eq!(active_player.key, String::from("key1"));

    assert_eq!(game.round, 2);
}

#[test]
fn action_errors() {
    let mut game = game_setup();

    let command = Command::new(
        Actions::EndTurn,
        String::from("key1")
    );
    let attempt = game.process_action(command);
    assert_eq!(attempt, Err("That is not an allowed action right now."));
}

#[test]
fn should_find_the_winner() {
    let mut game = game_setup();
    game = play_round_one(game);

    assert_eq!(game.the_winner, None);

    let player_key = String::from("key1");
    let resources = game.player_resources.get_mut(&player_key).unwrap();
    resources.deposit([
        Resource::Block,
        Resource::Timber,
        Resource::Block,
        Resource::Timber,
        Resource::Block,
        Resource::Timber,
        Resource::Fiber,
        Resource::Cereal
    ]).unwrap();

    let command = Command::new(
        Actions::RollDice,
        String::from("key1")
    );
    game.process_action(command).unwrap();
    game.roll_result = (1,1); // In case we roll a 7

    let mut command = Command::new(
        Actions::BuildStuff,
        String::from("key1")
    );
    command.target[0] = Some( (Target::Road, 43) );
    command.target[1] = Some( (Target::Road, 58) );
    // println!("{:?}", to_string(&command));
    game.process_action(command).unwrap();

    let mut command = Command::new(
        Actions::BuildStuff,
        String::from("key1")
    );
    command.target[0] = Some( (Target::Node, 44) );
    game.process_action(command).unwrap();

    let command = Command::new(
        Actions::EndTurn,
        String::from("key1")
    );
    game.process_action(command).unwrap();

    assert_eq!(game.the_winner.unwrap(), String::from("key1"));
    assert_eq!(game.phase, Phase::End);

}

#[test]
fn move_scorpion_when_seven_is_rolled() {
    let mut game = game_setup();

    let desert_index = game.board.hexagons.iter()
        .position(|hex| { hex.resource == Resource::Desert })
        .unwrap();

    assert_eq!(game.board.scorpion_index.unwrap(), desert_index);

    let command = Command::new(
        Actions::RollDice,
        String::from("key1")
    );
    game.process_action(command).unwrap();
    game.roll_result = (3,4); // Intentionally roll a seven

    let mut command = Command::new(
        Actions::MoveScorpion,
        String::from("key1")
    );
    command.target[0] = Some( (Target::Hex, 0) );
    game.process_action(command).unwrap();

    assert_eq!(game.board.scorpion_index.unwrap(), 0);
    
}