use std::collections::HashMap;
use rand::thread_rng;
use rand::seq::SliceRandom;

use crate::games::hexagon::resources::Resource;

#[derive(Debug, PartialEq)]
pub struct Coordinate {
    x: f64,
    y: f64
}

#[derive(Debug, PartialEq)]
pub struct Centroid {
    loc: Coordinate,
    number: u8
}

#[derive(Debug, PartialEq)]
pub struct Hexagon {
    pub vertices: Vec<Coordinate>,
    pub number: u8,
    pub resource: Resource
}

#[derive(Debug, PartialEq)]
pub enum BuildingType {
    Village,
    Empty
}

#[derive(Debug, PartialEq)]
pub struct Node {
    pub loc: Coordinate,
    pub player_key: Option<String>,
    pub building_type: BuildingType
}

#[derive(Debug, PartialEq)]
pub struct Road {
    pub inds: (usize,usize),
    pub player_key: Option<String>
}

#[derive(Debug, PartialEq)]
pub struct GameBoard {
    pub centroids: Vec<Centroid>,
    pub nodes: Vec<Node>,
    pub hexagons: Vec<Hexagon>,
    pub roads: Vec<Road>,
    pub bugs: HashMap<String, u8>,
    pub scorpion_index: Option<u32>
}

impl GameBoard {
    pub fn new() -> GameBoard {
        GameBoard { 
            centroids: Vec::new(), 
            nodes: Vec::new(), 
            hexagons: Vec::new(), 
            roads: Vec::new(),
            bugs: HashMap::new(), 
            scorpion_index: None
        }
    }

    pub fn reset(&mut self) -> &mut GameBoard {
        self.centroids.truncate(0);
        self.nodes.truncate(0);
        self.hexagons.truncate(0);
        self.roads.truncate(0);
        self.bugs.clear();
        self.scorpion_index = None;

        self
    }

    pub fn setup(&mut self, game_board_width: i8) {
        const CENTROID_SPACING: u8 = 100;
        self.compute_hex_grid_centroids(CENTROID_SPACING, game_board_width);
        self.assign_resources_and_rolls();
        self.compute_nodes_and_roads(CENTROID_SPACING);
    }

    fn compute_hex_grid_centroids(&mut self, centroid_spacing: u8, game_board_width: i8) {
        let num_off_center_rows: i8 = (game_board_width - 1) / 2;
        for row in (-1 * num_off_center_rows)..=num_off_center_rows {
            let num_hex_in_row = game_board_width - row.abs();
            let vertical_offset: f64 = f64::from(row) * f64::sqrt(3.0/4.0);
            let horizontal_offset: f64 = f64::from(row).abs() / 2.0;
            for hex in 0..num_hex_in_row {
                self.centroids.push( Centroid { 
                    loc: Coordinate { 
                        x: f64::from(centroid_spacing) * (horizontal_offset + f64::from(hex)), 
                        y: f64::from(centroid_spacing) * (vertical_offset + f64::from(num_off_center_rows))
                    }, 
                    number: 0
                });
            }
        }
    }

    fn assign_resources_and_rolls(&mut self) {

        let num_centroids = self.centroids.len() as f64;
        let canonical_count = 18.0;

        // Resource ratios
        let block_ratio = 3.0 / canonical_count;
        let rock_ratio = 3.0 / canonical_count;
        let timber_ratio = 4.0 / canonical_count;
        let cereal_ratio = 4.0 / canonical_count;
        let fiber_ratio = 4.0 / canonical_count;

        // Determine number of hexagons per resource type
        let num_block = f64::round(block_ratio * num_centroids) as u32;
        let num_rock = f64::floor(rock_ratio * num_centroids) as u32;
        let num_timber = f64::round(timber_ratio * num_centroids) as u32;
        let num_cereal = f64::floor(cereal_ratio * num_centroids) as u32;
        let num_fiber = f64::floor(fiber_ratio * num_centroids) as u32;

        // Generate a randomly-shuffled vector of resources
        let mut resources = Vec::new();
        for _ in 0..num_block { resources.push(Resource::Block); }
        for _ in 0..num_rock { resources.push(Resource::Rock); }
        for _ in 0..num_timber { resources.push(Resource::Timber); }
        for _ in 0..num_cereal { resources.push(Resource::Cereal); }
        for _ in 0..num_fiber { resources.push(Resource::Fiber); }
        resources.push(Resource::Desert);
        let mut rng = thread_rng();
        resources.shuffle(&mut rng);

        // Number ratios
        let two_ratio = 1.0 / canonical_count;
        let three_ratio = 2.0 / canonical_count;
        let four_ratio = 2.0 / canonical_count;
        let five_ratio = 2.0 / canonical_count;
        let six_ratio = 2.0 / canonical_count;
        let eight_ratio = 2.0 / canonical_count;
        let nine_ratio = 2.0 / canonical_count;
        let ten_ratio = 2.0 / canonical_count;
        let eleven_ratio = 2.0 / canonical_count;
        let twelve_ratio = 1.0 / canonical_count;

        // Determine number of hexagons per number
        let num_two = f64::round(two_ratio * num_centroids) as u32;
        let num_three = f64::round(three_ratio * num_centroids) as u32;
        let num_four = f64::round(four_ratio * num_centroids) as u32;
        let num_five = f64::round(five_ratio * num_centroids) as u32;
        let num_six = f64::round(six_ratio * num_centroids) as u32;
        let num_eight = f64::round(eight_ratio * num_centroids) as u32;
        let num_nine = f64::round(nine_ratio * num_centroids) as u32;
        let num_ten = f64::round(ten_ratio * num_centroids) as u32;
        let num_eleven = f64::round(eleven_ratio * num_centroids) as u32;
        let num_twelve = f64::round(twelve_ratio * num_centroids) as u32;

        // Generate a randomly-shuffed vector of numbers
        let mut numbers = Vec::<u8>::new();
        for _ in 0..num_two { numbers.push(2); }
        for _ in 0..num_three { numbers.push(3); }
        for _ in 0..num_four { numbers.push(4); }
        for _ in 0..num_five { numbers.push(5); }
        for _ in 0..num_six { numbers.push(6); }
        for _ in 0..num_eight { numbers.push(8); }
        for _ in 0..num_nine { numbers.push(9); }
        for _ in 0..num_ten { numbers.push(10); }
        for _ in 0..num_eleven { numbers.push(11); }
        for _ in 0..num_twelve { numbers.push(12); }
        numbers.push(1); // for the Desert
        numbers.shuffle(&mut rng);

        // Make sure the desert and -1 are at the same index
        let desert_index = resources.iter().position(|p| *p == Resource::Desert).unwrap();
        let minus_one_index = numbers.iter().position(|p| *p == 1).unwrap();
        if desert_index != minus_one_index {
            numbers.swap(desert_index, minus_one_index);
        }

        for (idx, &num) in numbers.iter().enumerate() {
            self.hexagons.push( Hexagon { 
                vertices: Vec::new(), 
                number: num, 
                resource: Resource::Desert
            });
            self.centroids[idx].number = num;
        }
        for (idx, rsc) in resources.drain(..).enumerate() {
            self.hexagons[idx].resource = rsc;
        }
    }

    fn compute_nodes_and_roads(&mut self, centroid_spacing: u8) {
        // Distance from each centroid to their surrounding nodes (vertices)
        let radius = f64::from(centroid_spacing) / f64::sqrt(3.0);

        // Loop over centroids and construct the nodes, roads, and hexagon vertices
        for (idx, el) in self.centroids.iter().enumerate() {
            let node_idx = 6 * idx as usize;
            // Find the [non-unique] six nodes around each hexagon centroid
            for step in 0..6 {
                let angle = step as f64 * std::f64::consts::PI / 3.0;
                let x = f64::round( 1000.0 * ( radius * f64::sin(angle) + el.loc.x ) ) / 1000.0;
                let y = f64::round( 1000.0 * ( radius * f64::cos(angle) + el.loc.y ) ) / 1000.0;
                self.nodes.push( Node { loc: Coordinate { x, y }, player_key: None, building_type: BuildingType::Empty } );
                self.hexagons[idx].vertices.push( Coordinate{x, y} );
                if step == 0 { self.roads.push( Road { inds: (node_idx + 5, node_idx), player_key: None } ) }
                else { self.roads.push( Road { inds: (node_idx+step-1, node_idx+step), player_key: None } ) }
            }
        }

        // Now go through the list of nodes and reduce it down to the unique set
        self.nodes = self.nodes.drain(..).enumerate().fold(
            Vec::<Node>::new(),
            | mut unique, (index,value) | {
                let new_idx: Vec<usize> = unique.iter().enumerate().filter_map(
                    | (idx,val) | {
                        if val.loc.x == value.loc.x && val.loc.y == value.loc.y { Some(idx) }
                        else { None }
                    }
                ).collect();
                // Is `value` already in the `unique` vector?
                if new_idx.len() > 0 {
                    // Update the indices in `roads`
                    for ni in new_idx {
                        self.roads = self.roads.iter().map(
                            | segment | {
                                let s1 = if segment.inds.0 == index { ni } else { segment.inds.0 };
                                let s2 = if segment.inds.1 == index { ni } else { segment.inds.1 };
                                Road { inds: (s1,s2), player_key: None }
                            }
                        ).collect();
                    }
                    // And don't add the node to the `unique` list
                    unique
                } else { // If `value` is not already in `unique`, add it
                    let inc_idx = unique.len();
                    self.roads = self.roads.iter().map(
                        | segment | {
                            let s1 = if segment.inds.0 == index { inc_idx } else { segment.inds.0 };
                            let s2 = if segment.inds.1 == index { inc_idx } else { segment.inds.1 };
                            Road { inds: (s1,s2), player_key: None }
                        }
                    ).collect();
                    // Add this node to the `unique` list
                    unique.push(value);
                    unique
                }
            }
        );

        // Winnow roads down to a unique set
        self.roads = self.roads.drain(..).fold(
            Vec::<Road>::new(),
            | mut acc, cv | {
                let mut reversibly_unique = true;
                for a in &acc {
                    if cv.inds.0 == a.inds.0 && cv.inds.1 == a.inds.1 { reversibly_unique = false };
                    if cv.inds.0 == a.inds.1 && cv.inds.1 == a.inds.0 { reversibly_unique = false };
                }
                match reversibly_unique {
                    true => { acc.push(cv); acc }
                    false => acc
                }
            }
        );
    }

    pub fn find_neighboring_nodes(&self, hex_idx: usize) -> Vec<usize> {
        let neighboring_nodes_indices = self.nodes.iter().enumerate().fold(
            Vec::new(),
            | mut acc, (n,cv) | {
                let any_matches = self.hexagons[hex_idx].vertices.iter().any(
                    | v | {
                        v.x == cv.loc.x && v.y == cv.loc.y
                    }
                );
                if any_matches { acc.push(n) }
                acc
            }
        );

        neighboring_nodes_indices
    }

    pub fn find_neighboring_hexagons(&self, node_idx: usize) -> Vec<usize> {
        let neighboring_hexagon_indices = self.hexagons.iter().enumerate().fold(
            Vec::new(),
            | mut acc, (n,cv) | {
                let any_matches = cv.vertices.iter().any(
                    | v | {
                        v.x == self.nodes[node_idx].loc.x && 
                        v.y == self.nodes[node_idx].loc.y
                    }
                );
                if any_matches { acc.push(n) }
                acc
            }
        );

        neighboring_hexagon_indices
    }

    pub fn collect_resources(&self, roll_sum: u8) -> Vec<(String,Resource)> {
        let mut spoils = Vec::new();

        let rolled_hexagons: Vec<(usize,Resource)> = self.hexagons.iter().enumerate().filter(
            | (_ind, hex) | { hex.number == roll_sum }
        ).map(
            | (ind, hex) | { (ind, hex.resource) }
        ).collect();
        
        for (ind, resource) in rolled_hexagons {
            let neighboring_nodes = self.find_neighboring_nodes(ind);
            for nn in neighboring_nodes {
                match &self.nodes[nn].player_key {
                    Some(player) => spoils.push((player.clone(), resource)),
                    None => ()
                }
            }
        }

        spoils
    }

}

#[cfg(test)]
mod test;