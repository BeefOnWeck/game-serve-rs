use std::collections::HashMap;

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
pub struct ResourceList {
    block: u16,
    rock: u16,
    timber: u16,
    fiber: u16,
    cereal: u16
}

#[derive(Debug, PartialEq)]
pub struct GameBoard {
    centroids: Vec<Centroid>,
    nodes: Vec<Coordinate>,
    hexagons: Vec<Hexagon>,
    roads: Vec<(u32,u32)>,
    bugs: HashMap<String, u8>,
    scorpion_index: Option<u32>
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
}