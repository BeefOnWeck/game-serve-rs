use std::sync::Arc;

#[derive(Clone, Debug, PartialEq)]
pub struct Player {
    pub key: String,
    pub name: String,
    pub socket_id: String
}

#[derive(Clone, Debug, PartialEq)]
pub struct Players {
    pub list: Vec<Arc<Player>>,
    pub active_player: Option<Arc<Player>>,
    pub cardinality: usize
}

impl Players {
    pub fn new() -> Players {
        Players { 
            list: Vec::new(), 
            active_player: None,
            cardinality: 0
        }
    }

    pub fn add_player(&mut self, key: &str, name: &str, socket_id: &str) -> &mut Players {
        self.list.push(
            Arc::new(
                Player { 
                    key: String::from(key), 
                    name: String::from(name), 
                    socket_id: String::from(socket_id) 
                }
            )
        );
        if self.list.len() == 1 {
            self.active_player = Some(Arc::clone(&self.list[0]));
        }
        self.cardinality += 1;

        self
    }

    pub fn set_active_player(&mut self, key: &str) -> Result<&mut Players, &'static str> {
        let pki = self.list.iter().position(|p| p.key.as_str() == key);
        match pki {
            Some(pki) => {
                self.active_player = Some(Arc::clone(&self.list[pki]));
                Ok(self)
            },
            None => Err("Player key not found!")
        }  
    }
    
    pub fn next_player(&mut self, advance: i8) -> Result<&mut Players, &'static str> {
        let active_player = match self.active_player.as_ref() {
            Some(ap) => ap,
            None => return Err("There is no active player.")
        };
        let active_player_index = self.list.iter().position(|p| p == active_player);
        match active_player_index {
            Some(idx) => {
                let next_player_index = (idx as i8 + advance) as usize % self.cardinality;
                self.active_player = Some(Arc::clone(&self.list[next_player_index]));
                Ok(self)
            },
            None => Err("Cannot get index of active player!")
        }
    }

    pub fn reset(&mut self) -> &mut Players {
        self.list.truncate(0);
        self.active_player = None;
        self.cardinality = 0;

        self
    }
}