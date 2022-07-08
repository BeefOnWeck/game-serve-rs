use super::*;

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