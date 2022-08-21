use std::sync::Arc;
use super::*;
use playe::Player;

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
    game.add_player("key", "name", "socket_id").unwrap();
    assert_eq!(game.phase, Phase::Play);
    assert_eq!(game.round, 1);
    assert_eq!(game.players.list.len(), 1);
    game.reset();
    assert_eq!(game.phase, Phase::Boot);
    assert_eq!(game.round, 0);
    assert_eq!(game.players.list.len(), 0);
}

#[test]
fn add_players() {
    let mut game = Core::new();
    assert_eq!(game.players.list.len(), 0);
    game.add_player("key", "name", "socket_id").unwrap();
    assert_eq!(game.players.list.len(), 1);
    assert_eq!(game.players.list[0].key.as_str(), "key");
    assert_eq!(game.players.list[0].name.as_str(), "name");
    assert_eq!(game.players.list[0].socket_id.as_str(), "socket_id");
}

#[test]
fn active_player() {
    let mut game = Core::new();
    assert_eq!(game.players.list.len(), 0);
    assert_eq!(game.players.active_player, None);
    game.add_player("key1", "name1", "socket_id1").unwrap().add_player("key2", "name2", "socket_id2").unwrap();
    assert_eq!(game.players.list.len(), 2);
    assert_eq!(game.players.active_player.as_ref().unwrap().key, String::from("key1"));
    game.set_active_player("key2").unwrap(); // NOTE: Using unwrap() because function returns a Result
    // println!("{:?}", game);
    assert_eq!(game.players.active_player.as_ref().unwrap().key, String::from("key2"));
    let attempt = game.set_active_player("not_a_valid_key"); // NOTE: No unwrap() to avoid a panic
    // println!("{:?}", attempt);
    assert_eq!(attempt, Err("Player key not found!"));
    game.reset();
    assert_eq!(game.players.active_player, None);
}

#[test]
fn next_player() {
    let mut game = Core::new();
    game
        .add_player("key1", "name1", "socket_id1").unwrap()
        .add_player("key2", "name2", "socket_id2").unwrap()
        .add_player("key3", "name3", "socket_id3").unwrap()
        .add_player("key4", "name4", "socket_id4").unwrap();
    assert_eq!(game.players.cardinality, 4);
    assert_eq!(game.players.active_player.as_ref().unwrap().key, String::from("key1"));
    game.next_player().unwrap(); // NOTE: Using unwrap() because function returns a Result
    assert_eq!(game.players.active_player.as_ref().unwrap().key, String::from("key2"));
    game.next_player().unwrap();
    assert_eq!(game.players.active_player.as_ref().unwrap().key, String::from("key3"));
    game.next_player().unwrap();
    assert_eq!(game.players.active_player.as_ref().unwrap().key, String::from("key4"));
    game.next_player().unwrap();
    assert_eq!(game.players.active_player.as_ref().unwrap().key, String::from("key1"));
}

#[test]
fn game_status() {
    let mut game = Core::new();
    game.next_phase().next_phase().next_round();
    game
        .add_player("key1", "name1", "socket_id1").unwrap()
        .add_player("key2", "name2", "socket_id2").unwrap()
        .add_player("key3", "name3", "socket_id3").unwrap()
        .add_player("key4", "name4", "socket_id4").unwrap();
    let game_status = game.get_game_status();
    let player_list = vec![
        Arc::new( Player { 
            key: String::from("key1"), 
            name: String::from("name1"), 
            socket_id: String::from("socket_id1") 
        } ),
        Arc::new( Player { 
            key: String::from("key2"),
            name: String::from("name2"),
            socket_id: String::from("socket_id2")
        } ),
        Arc::new( Player { 
            key: String::from("key3"),
            name: String::from("name3"),
            socket_id: String::from("socket_id3")
        } ),
        Arc::new( Player { 
            key: String::from("key4"),
            name: String::from("name4"),
            socket_id: String::from("socket_id4")
        } )
    ];
    assert_eq!(
        game_status,
        Core {
            phase: Phase::Play,
            round: 1,
            players: Players {
                list: player_list.iter().map(| p | { Arc::clone(&p) }).collect(),
                active_player: Some(Arc::clone(&player_list[0])),
                cardinality: 4,
            },
            last_action: Actions::None,
            config: HashMap::new()
        }
    )
}

#[test]
fn process_actions() {
    let mut game = Core::new();
    let command = CoreCommand {
        action: Actions::None
    };
    let attempt = game.process_action(command);
    assert_eq!(attempt, Err("Can only take action during the Setup or Play phases!"));
    game.next_phase().next_phase();
    let command = CoreCommand {
        action: Actions::None
    };
    let attempt = game.process_action(command);
    let mut expected_result = Core { 
        phase: Phase::Play, 
        round: 0, 
        players: Players { 
            list: [].to_vec(), 
            active_player: None, 
            cardinality: 0
        },
        last_action: Actions::None, 
        config: HashMap::new() 
    };
    assert_eq!(
        attempt,
        Ok(&mut expected_result)
    );
}

#[test]
fn game_configuration() {
    // Can set config during boot
    let mut game = Core::new();
    let mut config = HashMap::new();
    config.insert(String::from("config_num_players"), CoreConfigType::Int(2));
    let config_copy = config.clone();
    let attempt = game.configure_game(config);
    let mut expected_result = Core { 
        phase: Phase::Boot, 
        round: 0, 
        players: Players { 
            list: [].to_vec(), 
            active_player: None, 
            cardinality: 0
        },
        last_action: Actions::None, 
        config: config_copy 
    };
    assert_eq!(
        attempt,
        Ok(&mut expected_result)
    );

    // Cannot set config outside of boot
    let mut game = Core::new();
    let mut config = HashMap::new();
    config.insert(String::from("config_num_players"), CoreConfigType::Int(2));
    game.next_phase();
    let attempt = game.configure_game(config);
    assert_eq!(
        attempt,
        Err("Cannot configure game outside of boot phase!")
    );
}