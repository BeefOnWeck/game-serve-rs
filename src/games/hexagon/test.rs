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
fn board_setup() {
    let mut game = HexagonIsland::new();
    let config = Config {
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

    let mut bad = 0;
    let number_counts = game.board.hexagons.iter().fold(
        (0,0,0,0,0,0,0,0,0,0,0,0),
        | num_cnt, hex | {
            let (
                mut desert, 
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
    assert_eq!(bad, 0);
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
    let action = PossibleActions::RollDice;
    game.process_action(action).unwrap();
    assert!(game.roll_result != (0,0));
}

// The sum total of rolling two dice can range between 2 and 12.
// There are 6 * 6 = 36 possible combinations of the two die rolls.
// The histogram (counts vs. dice total) of the 36 possible 
// combinations should look like the following if die are random:
//
//                                      x
//                                  x   x   x
//                              x   x   x   x   x
// ^                        x   x   x   x   x   x    x
// |                    x   x   x   x   x   x   x    x    x
// Counts           x   x   x   x   x   x   x   x    x    x    x
// Dice total --> | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 |
#[test]
fn dice_are_right_random() {
    let mut game = HexagonIsland::new();
    game.phase = Phase::Play;

    const NUM_TRIALS: usize = 10000;

    struct Bin {
        bin: usize,
        count: usize,
        expected_probability: f64,
        expected_value: f64,
        standard_deviation: f64
    };

    let mut histogram: Vec<Bin> = (0..=10).map(
        | idx | {
            let fidx = idx as f64;
            let expected_probability = if idx + 2 < 8 { fidx + 1.0 } else { 11.0 - fidx } / 36.0;
            let expected_value = NUM_TRIALS as f64 * expected_probability;
            let standard_deviation = f64::sqrt(expected_value * (1.0 - expected_probability));
            Bin {
                bin: idx + 2,
                count: 0,
                expected_probability,
                expected_value,
                standard_deviation
            }
        }
    ).collect();

    for _trial in 0..NUM_TRIALS {
        game.process_action(PossibleActions::RollDice).unwrap();
        let roll_sum = game.roll_result.0 + game.roll_result.1;
        histogram[roll_sum as usize - 2].count += 1;
    }

    let number_of_outliers = histogram.iter().fold(
        0,
        | acc, cv | {
            let lower_bound = f64::round(cv.expected_value - 4.0 * cv.standard_deviation) as usize;
            let upper_bound = f64::round(cv.expected_value + 4.0 * cv.standard_deviation) as usize;
            if cv.count < lower_bound || cv.count > upper_bound { acc + 1}
            else { acc }
        }
    );

    assert!(number_of_outliers == 0);
}