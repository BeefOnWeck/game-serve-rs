use super::*;

#[test]
fn keep_track_of_phase() {
    let mut game = Core::new();
    assert_eq!(game.phase, Phase::Boot);
    game.next_phase();
    assert_eq!(game.phase, Phase::Setup);
    game.next_phase();
    assert_eq!(game.phase, Phase::Play);
    game.next_phase();
    assert_eq!(game.phase, Phase::End);
    game.next_phase();
    assert_eq!(game.phase, Phase::End);
}

#[test]
fn keep_track_of_round() {
    let mut game = Core::new();
    assert_eq!(game.round, 0);
    game.next_round();
    assert_eq!(game.round, 1);
    game.next_round();
    assert_eq!(game.round, 2);
}

#[test]
fn reset_the_game() {
    let mut game = Core::new();
    game.next_phase().next_phase().next_round();
    game.add_player("key", "name", "socket_id");
    assert_eq!(game.phase, Phase::Play);
    assert_eq!(game.round, 1);
    assert_eq!(game.players.len(), 1);
    game.reset();
    assert_eq!(game.phase, Phase::Boot);
    assert_eq!(game.round, 0);
    assert_eq!(game.players.len(), 0);
}

#[test]
fn add_players() {
    let mut game = Core::new();
    assert_eq!(game.players.len(), 0);
    game.add_player("key", "name", "socket_id");
    assert_eq!(game.players.len(), 1);
    assert_eq!(game.players[0].key.as_str(), "key");
    assert_eq!(game.players[0].name.as_str(), "name");
    assert_eq!(game.players[0].socket_id.as_str(), "socket_id");
}

#[test]
fn active_player() {
    let mut game = Core::new();
    assert_eq!(game.players.len(), 0);
    assert_eq!(game.active_player_key, None);
    game.add_player("key1", "name1", "socket_id1").add_player("key2", "name2", "socket_id2");
    assert_eq!(game.players.len(), 2);
    assert_eq!(game.active_player_key, Some(String::from("key1")));
    game.set_active_player("key2").unwrap(); // NOTE: Using unwrap() because function returns a Result
    // println!("{:?}", game);
    assert_eq!(game.active_player_key, Some(String::from("key2")));
    let attempt = game.set_active_player("not_a_valid_key"); // NOTE: No unwrap() to avoid a panic
    // println!("{:?}", attempt);
    assert_eq!(attempt, Err("Player key not found!"));
    game.reset();
    assert_eq!(game.active_player_key, None);
}

#[test]
fn next_player() {
    let mut game = Core::new();
    game
        .add_player("key1", "name1", "socket_id1")
        .add_player("key2", "name2", "socket_id2")
        .add_player("key3", "name3", "socket_id3")
        .add_player("key4", "name4", "socket_id4");
    assert_eq!(game.num_players, 4);
    assert_eq!(game.active_player_key, Some(String::from("key1")));
    game.next_player().unwrap(); // NOTE: Using unwrap() because function returns a Result
    assert_eq!(game.active_player_key, Some(String::from("key2")));
    game.next_player().unwrap();
    assert_eq!(game.active_player_key, Some(String::from("key3")));
    game.next_player().unwrap();
    assert_eq!(game.active_player_key, Some(String::from("key4")));
    game.next_player().unwrap();
    assert_eq!(game.active_player_key, Some(String::from("key1")));
}

#[test]
fn game_status() {
    let mut game = Core::new();
    game.next_phase().next_phase().next_round();
    game
        .add_player("key1", "name1", "socket_id1")
        .add_player("key2", "name2", "socket_id2")
        .add_player("key3", "name3", "socket_id3")
        .add_player("key4", "name4", "socket_id4");
    let game_status = game.get_game_status();
    assert_eq!(
        game_status,
        Core {
            phase: Phase::Play,
            round: 1,
            players: vec![
                Player { 
                    key: String::from("key1"), 
                    name: String::from("name1"), 
                    socket_id: String::from("socket_id1") 
                },
                Player { 
                    key: String::from("key2"),
                    name: String::from("name2"),
                    socket_id: String::from("socket_id2")
                },
                Player { 
                    key: String::from("key3"),
                    name: String::from("name3"),
                    socket_id: String::from("socket_id3")
                },
                Player { 
                    key: String::from("key4"),
                    name: String::from("name4"),
                    socket_id: String::from("socket_id4")
                }
            ],
            active_player_key: Some(String::from("key1")),
            num_players: 4
        }
    )
}