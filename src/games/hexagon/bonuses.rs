use std::collections::HashMap;

use crate::games::core::playe::Players;
use super::board::Road;

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

pub fn find_longest_road(roads: &[Road], players: &Players, title_holder: &Option<String>) -> Option<String> {

    let road_lengths: HashMap<String,u8> = get_road_lengths(roads, players);

    let ( has_longest_road, longest_road ) = road_lengths.iter().fold(
        ( None, 0 ),
        | acc, ( key, val ) | {
            if *val > acc.1 {
                return ( Some(key.clone()), *val );
            } else {
                return acc;
            }
        }
    );

    match has_longest_road {
        Some(key) => {
            match title_holder {
                Some(champ) => {
                    if let Some(champ_road) = road_lengths.get(champ) {
                        if longest_road > *champ_road { return Some(key); }
                        else { return Some(champ.clone()); }
                    } else {
                        return Some(key);
                    }
                },
                None => {
                    if longest_road >= 3 { return Some(key); }
                    else { return None; }
                }
            }
        },
        None => { return None; }
    }
}

fn get_road_lengths(roads: &[Road], players: &Players) -> HashMap<String,u8> {

    let player_roads = players.list.iter().fold(
        HashMap::<String,Vec<(usize,usize)>>::new(),
        | mut acc, plyr | {

            let plyr_roads: Vec<(usize,usize)> = roads.iter()
                .filter(|r| r.player_key == Some(plyr.key.clone()))
                .map(|r| r.inds)
                .collect();

            acc.insert(plyr.key.clone(), plyr_roads);

            acc
        }
    );

    let player_road_length = player_roads.iter().fold(
        HashMap::<String,u8>::new(),
        | mut acc, (key, value) | {
            acc.insert(key.to_string(), find_max_road_length(value.to_vec()));
            acc
        }
    );

    player_road_length

}

fn find_max_road_length(roads: Vec<(usize,usize)>) -> u8 {
    let mut max_road_length = 0;
    for road in roads.iter() {
        let other_roads = roads.iter()
            .filter(|otr| otr.0 != road.0 || otr.1 != road.1)
            .map(|otr| *otr)
            .collect();
        let road_length = measure_road_segment(road, other_roads);
        if road_length > max_road_length { max_road_length = road_length; }
    }
    max_road_length
}

fn measure_road_segment(road: &(usize,usize), other_roads: Vec<(usize,usize)>) -> u8 {
    let mut max_connecting_length = 0;

    let connecting_roads = other_roads.iter().fold(
        Vec::<(usize,usize,usize)>::new(),
        | mut acc, otr | {
            if otr.0 == road.0 || otr.0 == road.1 {
                acc.push((otr.0,otr.1,otr.0));
                acc
            } else if otr.1 == road.0 || otr.1 == road.1 {
                acc.push((otr.0,otr.1,otr.1));
                acc
            } else {
                acc
            }
        }
    );

    for cr in connecting_roads.iter() {
        let other_other_roads = other_roads.iter().filter(
            | &oor | {
                oor.0 != cr.2 &&
                oor.1 != cr.2 &&
                (
                    oor.0 != cr.0 ||
                    oor.1 != cr.1
                )
            }
        ).map(|oor| *oor).collect();

        let segment_length = measure_road_segment(&(cr.0,cr.1), other_other_roads);
        if segment_length > max_connecting_length { max_connecting_length = segment_length; }
    }

    max_connecting_length
}