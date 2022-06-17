use super::*;

#[test]
fn keep_track_of_phase() {
    let mut game = GameCore::new();
    assert_eq!(game.phase, Phase::Boot);
    game = game.next_phase();
    assert_eq!(game.phase, Phase::Setup);
    game = game.next_phase();
    assert_eq!(game.phase, Phase::Play);
    game = game.next_phase();
    assert_eq!(game.phase, Phase::End);
    game = game.next_phase();
    assert_eq!(game.phase, Phase::End);
}

#[test]
fn keep_track_of_round() {
    let mut game = GameCore::new();
    assert_eq!(game.round, 0);
    game = game.next_round();
    assert_eq!(game.round, 1);
    game = game.next_round();
    assert_eq!(game.round, 2);
}

#[test]
fn reset_the_game() {
    let mut game = GameCore::new();
    game = game.next_phase().next_phase().next_round();
    assert_eq!(game.phase, Phase::Play);
    assert_eq!(game.round, 1);
    game = game.reset();
    assert_eq!(game.phase, Phase::Boot);
    assert_eq!(game.round, 0);
}