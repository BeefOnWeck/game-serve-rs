use super::*;
use crate::games::hexagon::actions::roll_dice;
use crate::games::hexagon::resources::{ Resource, ResourceList };

#[test]
fn board_setup() {
    let mut board = GameBoard::new();
    board.setup(5);

    assert_eq!(board.centroids.len(), 19);
    assert_eq!(board.hexagons.len(), 19);
    assert_eq!(board.nodes.len(), 54);
    assert_eq!(board.roads.len(), 72);

    let resource_counts = board.hexagons.iter().fold(
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
    let number_counts = board.hexagons.iter().fold(
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
                1 => desert += 1,
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
fn should_find_neighboring_nodes() {
    let mut board = GameBoard::new();
    board.setup(5);

    let hex_idx = 0;
    let node_indices = board.find_neighboring_nodes(hex_idx);
    assert_eq!(node_indices, [0, 1, 2, 3, 4, 5]);

    let hex_idx = 1;
    let node_indices = board.find_neighboring_nodes(hex_idx);
    assert_eq!(node_indices, [1, 2, 6, 7, 8, 9]);
}

#[test]
fn should_find_neighboring_hexagons() {
    let mut board = GameBoard::new();
    board.setup(5);

    let node_idx = 0;
    let hexagon_indices = board.find_neighboring_hexagons(node_idx);
    assert_eq!(hexagon_indices, [0, 3, 4]);
}

#[test]
fn should_collect_rolled_resources() {
    let mut board = GameBoard::new();
    board.setup(5);

    // Build a village on each node
    let player_key = String::from("key_1");
    for ind in 0..board.nodes.len() {
        board.nodes[ind].player_key = Some(player_key.clone());
        board.nodes[ind].building_type = BuildingType::Village;
    }

    // Roll the dice
    let roll_result = roll_dice();
    let roll_sum = roll_result.0 + roll_result.1;

    // Find out what resources were rolled
    let mut rolled_resources = board.hexagons.iter().fold(
        ResourceList::new(),
        | mut acc, cv | {
            if cv.number == roll_sum {
                match cv.resource {
                    Resource::Block => acc.block += 1,
                    Resource::Rock => acc.rock += 1,
                    Resource::Timber => acc.timber += 1,
                    Resource::Fiber => acc.fiber += 1,
                    Resource::Cereal => acc.cereal += 1,
                    Resource::Desert => ()
                }
            }
            acc
        }
    );

    // Each hexagon that matches the roll should contribute six resources
    rolled_resources.block *= 6;
    rolled_resources.rock *= 6;
    rolled_resources.timber *= 6;
    rolled_resources.fiber *= 6;
    rolled_resources.cereal *= 6;

    // Call resolve_roll() and use this to decrement rolled_resources
    let spoils = board.resolve_roll(roll_sum);
    for (_player_key, resource) in spoils {
        match resource {
            Resource::Block => rolled_resources.block -= 1,
            Resource::Rock => rolled_resources.rock -= 1,
            Resource::Timber => rolled_resources.timber -= 1,
            Resource::Fiber => rolled_resources.fiber -= 1,
            Resource::Cereal => rolled_resources.cereal -= 1,
            Resource::Desert => ()
        }
    }

    assert_eq!(rolled_resources, ResourceList::new());

}

#[test]
fn scorpion_should_block_resources() {
    let mut board = GameBoard::new();
    board.setup(5);

    // Build a village on each node
    let player_key = String::from("key_1");
    for ind in 0..board.nodes.len() {
        board.nodes[ind].player_key = Some(player_key.clone());
        board.nodes[ind].building_type = BuildingType::Village;
    }

    for ind in 0..board.hexagons.len() {

        // Move the scorpion
        board.scorpion_index = Some(ind);

        // Set the die so that it equals the number of the hexagon with the scorpion
        let roll_sum = board.hexagons[ind].number;
        match roll_sum {
            2..=12 => {
                // Find out what resources were rolled
                let mut rolled_resources = board.hexagons.iter().enumerate().fold(
                    ResourceList::new(),
                    | mut acc, cv | {
                        let (i,val) = cv;
                        if val.number == roll_sum && Some(i) != board.scorpion_index {
                            match val.resource {
                                Resource::Block => acc.block += 1,
                                Resource::Rock => acc.rock += 1,
                                Resource::Timber => acc.timber += 1,
                                Resource::Fiber => acc.fiber += 1,
                                Resource::Cereal => acc.cereal += 1,
                                Resource::Desert => ()
                            }
                        }
                        acc
                    }
                );

                // Each hexagon that matches the roll should contribute six resources
                rolled_resources.block *= 6;
                rolled_resources.rock *= 6;
                rolled_resources.timber *= 6;
                rolled_resources.fiber *= 6;
                rolled_resources.cereal *= 6;

                // Call resolve_roll() and use this to decrement rolled_resources
                let spoils = board.resolve_roll(roll_sum);
                for (_player_key, resource) in spoils {
                    match resource {
                        Resource::Block => rolled_resources.block -= 1,
                        Resource::Rock => rolled_resources.rock -= 1,
                        Resource::Timber => rolled_resources.timber -= 1,
                        Resource::Fiber => rolled_resources.fiber -= 1,
                        Resource::Cereal => rolled_resources.cereal -= 1,
                        Resource::Desert => ()
                    }
                }

                assert_eq!(rolled_resources, ResourceList::new());
            },
            _ => ()
        }
    }    
}