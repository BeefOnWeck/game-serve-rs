
pub trait Game {
    fn new() -> Self;
    fn next_phase(&mut self) -> &mut Self;
    fn next_round(&mut self) -> &mut Self;
    fn reset(&mut self) -> &mut Self;
    fn add_player(&mut self, key: &str, name: &str, socket_id: &str) -> &mut Self;
    fn set_active_player(&mut self, key: &str) -> Result<&mut Self, &'static str>;
    fn next_player(&mut self) -> Result<&mut Self, &'static str>;
}