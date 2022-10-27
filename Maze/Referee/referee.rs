#![allow(dead_code)]

use common::board::Board;
use common::grid::Position;
use common::{ColorName, PlayerInfo, State, BOARD_SIZE};
use players::player::Player;
use players::strategy::PlayerAction;
use rand::{Rng, RngCore};

/// The Result of calling `Referee::run_game(...)`.
/// - The `winners` field contains all the winning players.
/// - The `kicked` field contains all the players who misbehaved during the game.
pub struct GameResult {
    winners: Vec<Box<dyn Player>>,
    kicked: Vec<Box<dyn Player>>,
}

pub struct Referee<'a> {
    state: State,
    kicked_players: Vec<Box<dyn Player>>,
    reached_goals: Vec<&'a dyn Player>,
    rand: Box<dyn RngCore>,
}

impl<'a> Referee<'a> {
    /// Asks each `Player` in `players` to propose a `Board` and returns the chosen `Board`
    ///
    /// # Panics  
    /// This method will panic is `player` is an empty vector
    fn get_player_boards(&self, players: &[Box<dyn Player>]) -> Board {
        // FIXME: this should actually ask every player for a board
        players[0].propose_board0(BOARD_SIZE as u32, BOARD_SIZE as u32)
    }

    /// Given a `Board` and the list of `Player`s, creates an initial `State` for this game.
    ///
    /// This will assign each player a Goal and a home tile, and set each `Player`'s current
    /// position to be their home tile.
    fn make_initial_state(&mut self, players: &[Box<dyn Player>], board: Board) -> State {
        let player_info = players
            .iter()
            .map(|_| {
                let home: Position = gen_immovable_tile_pos(&mut self.rand);
                PlayerInfo::new(
                    home,
                    home, /* players start on their home tile */
                    board[home].gems,
                    ColorName::Red.into(),
                )
            })
            .collect();

        State::new(board, player_info)
    }

    /// Communicates all public information of the current `state` and each `Player`'s private goal
    /// to all `Player`s in `players`.
    fn broadcast_initial_state(&self, players: &[Box<dyn Player>]) {
        for _player in players {
            //player.setup(state.into(), goal);
            todo!();
        }
    }

    /// Has `player` won?
    fn check_player_won(&self, _player: Box<dyn Player>) -> bool {
        todo!();
    }

    /// Runs the game given the age-sorted `Vec<impl Players>`, `players`.
    pub fn run_game(&mut self, players: Vec<Box<dyn Player>>) -> GameResult {
        // Iterate over players to get their proposed boards
        // - for now, use the first players proposed board
        let board = self.get_player_boards(&players);

        self.state = self.make_initial_state(&players, board);

        self.broadcast_initial_state(&players);

        // Create `State` from the chosen board
        // Assign each player a home + goal + current position
        // communicate initial state to all players

        for _round in 0..1000 {
            for _player in &players {
                let turn: PlayerAction = None; //player.take_turn(self.state);
                match turn {
                    Some(pmove) => {
                        let mut test = self.state.clone();
                        test.rotate_spare(pmove.rotations);
                        match test.slide_and_insert(pmove.slide) {
                            Ok(_) => {
                                self.state.rotate_spare(pmove.rotations);
                                self.state
                                    .slide_and_insert(pmove.slide)
                                    .expect("Slide has already been verified");
                            }
                            Err(_) => {
                                //self.kicked_players.push(self.state.remove_player());
                            }
                        }
                    }
                    None => todo!("Passing stuff"),
                }
            }
        }
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

fn gen_immovable_tile_pos(rng: &mut impl Rng) -> Position {
    (
        rng.gen_range(0..BOARD_SIZE / 2) * 2 + 1,
        rng.gen_range(0..BOARD_SIZE / 2) * 2 + 1,
    )
}

fn main() {
    println!("Hello World!");
}
