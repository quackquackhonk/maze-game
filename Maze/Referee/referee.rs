use players::player::Player;

/// The Result of calling `Referee::run_game(...)`.
/// - The `winners` field contains all the winning players.
/// - The `losers` field contains all the players who misbehaved during the game.
struct GameResult<P: Player> {
    winners: Vec<P>,
    losers: Vec<P>,
}

struct Referee {}

impl Referee {
    /// Runs the game given the age-sorted `Vec<impl Players>`, `players`.
    pub fn run_game<P: Player>(&mut self, players: Vec<P>) -> GameResult<P> {
        todo!();
    }
}

fn main() {
    println!("Hello World!");
}
