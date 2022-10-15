use std::collections::HashMap;

pub fn find_most_bugs(bugs: &HashMap<String, u8>, title_holder: &Option<String>) -> Option<String> {

    let ( has_most_bugs, most_bugs ) = bugs.iter().fold(
        ( None, 0 ),
        | acc, ( key, val ) | {
            if *val > acc.1 {
                return ( Some(key.clone()), *val );
            } else {
                return acc;
            }
        }
    );

    match has_most_bugs {
        Some(key) => {
            match title_holder {
                Some(champ) => {
                    if let Some(champ_bugs) = bugs.get(champ) {
                        if most_bugs > *champ_bugs { return Some(key); }
                        else { return Some(champ.clone()); }
                    } else {
                        return Some(key);
                    }
                },
                None => {
                    if most_bugs >= 3 { return Some(key); }
                    else { return None; }
                }
            }
        },
        None => { return None; }
    }
}