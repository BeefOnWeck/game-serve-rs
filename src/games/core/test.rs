use super::*;

#[test]
fn phase() {
    let game = GameCore::new();
    assert_eq!(game.phase, "boot");
}