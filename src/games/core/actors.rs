#[derive(Clone, Debug, PartialEq)]
pub struct Player {
    pub key: String,
    pub name: String,
    pub socket_id: String
}

#[derive(Clone, Debug, PartialEq)]
pub struct Players {
    pub list: Vec<Player>,
    pub active_key: Option<String>,
    pub cardinality: usize
}

impl Players {
    pub fn new() -> Players {
        Players { 
            list: Vec::new(), 
            active_key: None, 
            cardinality: 0
        }
    }

    pub fn add_player(&mut self, key: &str, name: &str, socket_id: &str) -> &mut Players {
        self.list.push(
            Player { 
                key: String::from(key), 
                name: String::from(name), 
                socket_id: String::from(socket_id) 
            }
        );
        if self.list.len() == 1 {
            self.active_key = Some(String::from(key));
        }
        self.cardinality += 1;

        self
    }

    pub fn set_active_player(&mut self, key: &str) -> Result<&mut Players, &'static str> {
        let pki: Vec<_> = self.list.iter().filter(|p| p.key.as_str() == key).collect();
        match pki.len() {
            0 => Err("Player key not found!"),
            1 => {
                self.active_key = Some(String::from(key));
                Ok(self)
            },
            _ => Err("Non-unique player key found!")
        }  
    }
    
    pub fn next_player(&mut self) -> Result<&mut Players, &'static str> {
        // TODO: Consider rewriting using iterators and cycle()
        // https://stackoverflow.com/questions/47838596/how-do-i-have-a-structs-field-be-an-iterator-over-t-elements
        let active_key = self.active_key.clone().unwrap();
        let active_player_index = self.list.iter().position(|p| p.key == active_key);
        match active_player_index {
            Some(idx) => {
                let next_player_index = (idx + 1) % self.cardinality;
                self.active_key = Some(self.list[next_player_index].key.clone());
                Ok(self)
            },
            None => Err("Cannot index of active player!")
        }
    }

    pub fn reset(&mut self) -> &mut Players {
        self.list.truncate(0);
        self.active_key = None;
        self.cardinality = 0;

        self
    }
}