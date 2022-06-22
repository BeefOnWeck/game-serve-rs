use std::collections::HashMap;

use crate::games::core::{
    Players,
    Phase,
    PossibleActions,
    CoreConfigType,
    traits::Game
};

#[derive(Debug, PartialEq)]
struct Coordinate {
    x: f64,
    y: f64
}

impl Coordinate {
    fn new() -> Coordinate {
        Coordinate { x: 0.0, y: 0.0 }
    }
}

#[derive(Debug, PartialEq)]
struct Centroid {
    loc: Coordinate,
    number: u16
}

impl Centroid {
    fn new() -> Centroid {
        Centroid { 
            loc: Coordinate::new(), 
            number: 0
        }
    }
}

#[derive(Debug, PartialEq)]
enum Resource {
    Block,
    Rock,
    Timber,
    Fiber,
    Cereal
}

#[derive(Debug, PartialEq)]
struct Hexagon {
    vertices: Vec<Coordinate>,
    number: u16,
    resource: Resource
}

#[derive(Debug, PartialEq)]
struct ResourceList {
    block: u16,
    rock: u16,
    timber: u16,
    fiber: u16,
    cereal: u16
}

#[derive(Debug, PartialEq)]
struct HexagonState {
    centroids: Vec<Centroid>,
    nodes: Vec<Coordinate>,
    hexagons: Vec<Hexagon>,
    roads: Vec<(u32,u32)>,
    rollResult: (u8,u8),
    playerResources: HashMap<String, ResourceList>,
    bugs: HashMap<String, u8>,
    scorpionIndex: Option<u32>
}

impl HexagonState {
    fn new() -> HexagonState {
        HexagonState { 
            centroids: Vec::new(), 
            nodes: Vec::new(), 
            hexagons: Vec::new(), 
            roads: Vec::new(), 
            rollResult: (0,0), 
            playerResources: HashMap::new(), 
            bugs: HashMap::new(), 
            scorpionIndex: None
        }
    }
}

#[derive(Debug, PartialEq)]
struct HexagonIsland {
    phase: Phase,
    round: u16,
    players: Players,
    possible_actions: PossibleActions,
    config: HashMap<String, CoreConfigType>,
    state: HexagonState
}

impl HexagonIsland {
    fn new() -> HexagonIsland {
        HexagonIsland {
            phase: Phase::Boot,
            round: 0,
            players: Players::new(),
            possible_actions: PossibleActions::None,
            config: HashMap::new(),
            state: HexagonState::new()
        }
    }
}

#[cfg(test)]
mod test;