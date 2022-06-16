
pub struct GameCore {
    phase: String
}

impl GameCore {
    pub fn new() -> GameCore {
        GameCore {
            phase: String::from("boot")
        }
    }
}


#[cfg(test)]
mod test;