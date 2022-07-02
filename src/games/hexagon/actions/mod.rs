use rand::{thread_rng, Rng};

pub fn roll_dice() -> (u8,u8) {
    let mut rng = thread_rng();
    (
        rng.gen_range(1..=6), 
        rng.gen_range(1..=6)
    )
}

#[cfg(test)]
mod test;