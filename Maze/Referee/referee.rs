use common::board::Board;
use common::{ColorName, PlayerInfo, State, BOARD_SIZE};
use players::player::Player;
use rand::SeedableRng;

/// The Result of calling `Referee::run_game(...)`.
/// - The `winners` field contains all the winning players.
/// - The `kicked` field contains all the players who misbehaved during the game.
struct GameResult {
    winners: Vec<Box<dyn Player>>,
    kicked: Vec<Box<dyn Player>>,
}

struct Referee<'a> {
    state: State,
    kicked_players: Vec<Box<dyn Player>>,
    reached_goals: Vec<&'a dyn Player>,
    rand: Box<dyn SeedableRng<Seed = usize>>,
}

impl<'a> Referee<'a> {
    /// Asks each `Player` in `players` to propose a `Board` and returns the chosen `Board`
    ///
    /// # Panics  
    /// This method will panic is `player` is an empty vector
    fn get_player_boards(&self, players: Vec<Box<dyn Player>>) -> Board {
        // FIXME: this should actually ask every player for a board
        players[0].propose_board0(BOARD_SIZE as u32, BOARD_SIZE as u32)
    }

    /// Given a `Board` and the list of `Player`s, creates an initial `State` for this game.
    ///
    /// This will assign each player a Goal and a home tile, and set each `Player`'s current
    /// position to be their home tile.
    fn make_initial_state(&self, players: Vec<Box<dyn Player>>, board: Board) -> State {
        todo!();
    }

    /// Communicates all public information of the current `state` and each `Player`'s private goal
    /// to all `Player`s in `players`.
    fn broadcast_state(&self, players: Vec<Box<dyn Player>>) {
        todo!();
    }

    /// Has `player` won?
    fn check_player_won(&self, player: Box<dyn Player>) -> bool {
        todo!();
    }

    /// Runs the game given the age-sorted `Vec<impl Players>`, `players`.
    pub fn run_game(&mut self, players: Vec<Box<dyn Player>>) -> GameResult {
        // Iterate over players to get their proposed boards
        // - for now, use the first players proposed board
        let board = self.get_player_boards(players);

        // Create `State` from the chosen board
        // Assign each player a home + goal + current position
        // communicate initial state to all players
        // FIXME: these positions needs to be random
        let player_info = players
            .iter()
            .map(|_| PlayerInfo::new((1, 1), (1, 1), board[(1, 1)].gems, ColorName::Red.into()));

        self.state = State::new(board, player_info);

        // loop until game is over
        // - ask each player for a turn
        // - check if that player won
        //
        // make the game result
        //
        // Communicate winners to all players
        //
        // return GameResult
        todo!();
    }
}

fn main() {
    println!("Hello World!");
}
