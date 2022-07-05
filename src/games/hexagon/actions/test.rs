use super::*;
use crate::games::hexagon::board::{ GameBoard, ResourceList };

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
fn dice_distribution() {

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
        let roll_result = roll_dice();
        let roll_sum = roll_result.0 + roll_result.1;
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

#[test]
fn build_a_road() {
    let mut board = GameBoard::new();
    board.setup(5);

    let num_built_roads = board.roads.iter().fold(
        0, 
        | acc, cv | if cv.player_key != None { acc + 1 } else { acc }
    );
    assert!(num_built_roads == 0);

    let node_index = 0;
    let player_key = String::from("key1");
    let _status = build_node(node_index, player_key, &mut board.nodes, &board.roads, true);

    let road_index = 0;
    let player_key = String::from("key1");
    let _status = build_road(road_index, player_key, &board.nodes, &mut board.roads);

    let num_built_roads = board.roads.iter().fold(
        0, 
        | acc, cv | if cv.player_key != None { acc + 1 } else { acc }
    );
    assert_eq!(num_built_roads, 1);
}

#[test]
fn build_on_a_node() {
    let mut board = GameBoard::new();
    board.setup(5);

    let num_built_nodes = board.nodes.iter().fold(
        0, 
        | acc, cv | if cv.player_key != None { acc + 1 } else { acc }
    );
    assert!(num_built_nodes == 0);

    let node_index = 0;
    let player_key = String::from("key1");
    let status = build_node(node_index, player_key, &mut board.nodes, &board.roads, true);

    let num_built_nodes = board.nodes.iter().fold(
        0, 
        | acc, cv | if cv.player_key != None { acc + 1 } else { acc }
    );
    assert_eq!(num_built_nodes, 1);
}

#[test]
fn node_building_errors() {
    let mut board = GameBoard::new();
    board.setup(5);

    let node_index = 10000000;
    let player_key = String::from("key1");
    let is_setup = true;
    let status = build_node(node_index, player_key, &mut board.nodes, &board.roads, is_setup);
    assert_eq!(status, Err("Cannot make building; invalid node index."));

    let node_index = 0;
    let player_key = String::from("key1");
    let is_setup = true;
    let status = build_node(node_index, player_key, &mut board.nodes, &board.roads, is_setup);
    assert_eq!(status, Ok(()));

    let node_index = 0;
    let player_key = String::from("key1");
    let is_setup = true;
    let status = build_node(node_index, player_key, &mut board.nodes, &board.roads, is_setup);
    assert_eq!(status, Err("Cannot make building; there is already something there."));

    let node_index = 1;
    let player_key = String::from("key1");
    let is_setup = true;
    let status = build_node(node_index, player_key, &mut board.nodes, &board.roads, is_setup);
    assert_eq!(status, Err("Cannot make building; you must respect the two-space rule."));

    let node_index = 10;
    let player_key = String::from("key1");
    let is_setup = false;
    let status = build_node(node_index, player_key, &mut board.nodes, &board.roads, is_setup);
    assert_eq!(status, Err("Cannot make building; after initial setup you must build next to roads that you own."));

}

#[test]
fn road_building_errors() {
    let mut board = GameBoard::new();
    board.setup(5);

    let road_index = 10000000;
    let player_key = String::from("key1");
    let status = build_road(road_index, player_key, &board.nodes, &mut board.roads);
    assert_eq!(status, Err("Cannot build road; invalid road index."));

    let node_index = 0;
    let player_key = String::from("key1");
    let is_setup = true;
    let _status = build_node(node_index, player_key, &mut board.nodes, &board.roads, is_setup);

    let road_index = 0;
    let player_key = String::from("key1");
    let status = build_road(road_index, player_key, &board.nodes, &mut board.roads);
    assert_eq!(status, Ok(()));

    let road_index = 0;
    let player_key = String::from("key1");
    let status = build_road(road_index, player_key, &board.nodes, &mut board.roads);
    assert_eq!(status, Err("Cannot build road; there is already something there."));

    let road_index = 10;
    let player_key = String::from("key1");
    let status = build_road(road_index, player_key, &board.nodes, &mut board.roads);
    assert_eq!(status, Err("Roads have to be built next to other roads or buildings you own."));

    let road_index = 1;
    let player_key = String::from("key1");
    let status = build_road(road_index, player_key, &board.nodes, &mut board.roads);
    assert_eq!(status, Ok(()));
}