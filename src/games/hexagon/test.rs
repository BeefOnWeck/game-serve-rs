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
            config: HexagonIslandConfig {
                num_players: 2,
                score_to_win: 10,
                game_board_width: 5
            },
            roll_result: (0,0), 
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
    let config = HexagonIslandConfig {
        num_players,
        score_to_win,
        game_board_width
    };
    game.configure_game(config).unwrap();
    assert_eq!(game.config.num_players, 4);
    assert_eq!(game.config.score_to_win, 15);
    assert_eq!(game.config.game_board_width, 9);
    game.next_phase();
    let attempt = game.configure_game(HexagonIslandConfig {
        num_players: 2,
        score_to_win: 7,
        game_board_width: 7
    });
    assert_eq!(attempt, Err("Cannot configure game outside of boot phase!"));
}

#[test]
fn board_setup() {
    let mut game = HexagonIsland::new();
    let config = HexagonIslandConfig {
        num_players: 2,
        score_to_win: 10,
        game_board_width: 5
    };
    game.configure_game(config).unwrap();
    game.board.setup(5);

    assert_eq!(game.board.centroids.len(), 19);
    assert_eq!(game.board.hexagons.len(), 19);
    assert_eq!(game.board.nodes.len(), 54);
    assert_eq!(game.board.roads.len(), 72);

    let resource_counts = game.board.hexagons.iter().fold(
        (0,0,0,0,0,0),
        | rsc_cnt, hex | {
            let (
                mut blk,
                mut rck,
                mut tmb,
                mut fib,
                mut crl,
                mut dst
            ) = rsc_cnt;
            match hex.resource {
                Resource::Block => blk += 1,
                Resource::Rock => rck += 1,
                Resource::Timber => tmb += 1,
                Resource::Fiber => fib += 1,
                Resource::Cereal => crl += 1,
                Resource::Desert => dst += 1,
            }

            (blk,rck,tmb,fib,crl,dst)
        }
    );

    assert_eq!(resource_counts, (3, 3, 4, 4, 4, 1));

    let number_counts = game.board.hexagons.iter().fold(
        (0,0,0,0,0,0,0,0,0,0,0,0),
        | num_cnt, hex | {
            let (
                mut desert, // doesn't get a number
                mut two,
                mut three,
                mut four,
                mut five,
                mut six,
                mut seven,
                mut eight,
                mut nine,
                mut ten,
                mut eleven,
                mut twelve
            ) = num_cnt;
            let mut bad = 0;
            match hex.number {
                -1 => desert += 1,
                2 => two += 1,
                3 => three += 1,
                4 => four += 1,
                5 => five += 1,
                6 => six += 1,
                7 => seven += 1,
                8 => eight += 1,
                9 => nine += 1,
                10 => ten += 1,
                11 => eleven += 1,
                12 => twelve += 1,
                _ => bad += 1
            }

            (desert,two,three,four,five,six,seven,eight,nine,ten,eleven,twelve)
        }
    );

    assert_eq!(number_counts, (1,1,2,2,2,2,0,2,2,2,2,1));
}

#[test]
fn should_reset() {
    let mut game = HexagonIsland::new();
    let config = HexagonIslandConfig {
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
            config: HexagonIslandConfig {
                num_players: 2,
                score_to_win: 10,
                game_board_width: 5
            },
            roll_result: (0,0), 
            player_resources: HashMap::new(), 
            board: GameBoard::new()
        }
    )
}