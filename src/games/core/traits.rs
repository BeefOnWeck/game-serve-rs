
pub trait Game {
    type Status;
    type Command;
    type Config;

    fn new() -> Self;
    fn next_phase(&mut self) -> &mut Self;
    fn next_round(&mut self) -> &mut Self;
    fn reset(&mut self) -> &mut Self;
    fn add_player(&mut self, key: &str, name: &str, socket_id: &str) -> Result<&mut Self, &'static str>;
    fn set_active_player(&mut self, key: &str) -> Result<&mut Self, &'static str>;
    fn next_player(&mut self) -> Result<&mut Self, &'static str>;
    fn previous_player(&mut self) -> Result<&mut Self, &'static str>;
    fn get_game_status(&self, key: &str) -> String;
    fn find_the_winner(&mut self) -> &mut Self;
    fn process_action(&mut self, command: Self::Command) -> Result<&mut Self, &'static str>;
    fn configure_game(&mut self, config: Self::Config) -> Result<&mut Self, &'static str>;
}