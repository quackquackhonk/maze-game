#![allow(dead_code)]

use std::collections::{HashMap, HashSet, VecDeque};

use common::board::Board;
use common::grid::{squared_euclidian_distance, Position};
use common::{Color, ColorName, PlayerInfo, State, BOARD_SIZE};
use players::player::Player;
use players::strategy::{PlayerAction, PlayerMove};
use rand::distributions::uniform::SampleRange;
use rand::{Rng, RngCore};

/// The Result of calling `Referee::run_game(...)`.
/// - The `winners` field contains all the winning players.
/// - The `kicked` field contains all the players who misbehaved during the game.
#[derive(Default)]
pub struct GameResult {
    pub winners: Vec<Box<dyn Player>>,
    pub kicked: Vec<Box<dyn Player>>,
}

/// Represents the winner of the game.
/// Some(PlayerInfo) -> This `PlayerInfo` was the first player to reach their goal and then their
/// home.
/// None -> The game ended without a single winner, the Referee will calculate winners another way.
pub type GameWinner = Option<PlayerInfo>;

pub struct Referee {
    rand: Box<dyn RngCore>,
}

impl Referee {
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
                let goal: Position = gen_immovable_tile_pos(&mut self.rand);
                PlayerInfo::new(
                    home,
                    home, /* players start on their home tile */
                    goal,
                    ColorName::Red.into(),
                )
            })
            .collect();

        State::new(board, player_info)
    }

    /// Communicates all public information of the current `state` and each `Player`'s private goal
    /// to all `Player`s in `players`.
    fn broadcast_initial_state(&self, state: &State, players: &mut [Box<dyn Player>]) {
        for player in players {
            let goal = state.current_player_info().goal;
            player.setup(Some(state.clone().into()), goal);
        }
    }

    /// Has `player` won?
    fn check_player_won(&self, _player: Box<dyn Player>) -> bool {
        todo!();
    }

    /// Advances to the next player.
    ///
    /// # Effect
    ///
    /// rotates `players` to the left once, and does the same to the internal `Vec<PlayerInfo>`
    /// stored inside `state`.
    fn next_player(players: &mut [Box<dyn Player>], state: &mut State) {
        players.rotate_left(1);
        state.next_player();
    }

    /// Returns a tuple of two `Vec<Box<dyn Player>>`. The first of these vectors contains all
    /// `Box<dyn Player>`s who won the game, and the second vector contains all the losers.
    fn calculate_winners(
        winner: GameWinner,
        players: Vec<Box<dyn Player>>,
        state: &State,
    ) -> (Vec<Box<dyn Player>>, Vec<Box<dyn Player>>) {
        let zipped_players = players.into_iter().zip(state.player_info.iter());
        match winner {
            Some(winner) => zipped_players.fold(
                (vec![], vec![]),
                |(mut winners, mut losers), (api, info)| {
                    if info.color == winner.color {
                        winners.push(api);
                    } else {
                        losers.push(api);
                    }
                    (winners, losers)
                },
            ),
            None => {
                let min_dist = state.player_info.iter().fold(usize::MAX, |prev, info| {
                    usize::min(prev, squared_euclidian_distance(&info.position, &info.goal))
                });

                zipped_players.fold(
                    (vec![], vec![]),
                    |(mut winners, mut losers), (api, info)| {
                        if min_dist == squared_euclidian_distance(&info.position, &info.goal) {
                            winners.push(api);
                        } else {
                            losers.push(api);
                        }
                        (winners, losers)
                    },
                )
            }
        }
    }

    /// Communicates if a player won to all `Player`s in the given tuple of winners and losers
    fn broadcast_winners(winners: &[Box<dyn Player>], losers: Vec<Box<dyn Player>>) {
        for player in winners {
            player.won(true);
        }
        for player in losers {
            player.won(false);
        }
    }

    /// Runs the game given the age-sorted `Vec<Box<dyn Player>>`, `players`.
    pub fn run_game(&mut self, mut players: Vec<Box<dyn Player>>) -> GameResult {
        // Iterate over players to get their proposed boards
        // - for now, use the first players proposed board
        let board = self.get_player_boards(&players);

        // Create `State` from the chosen board
        // Assign each player a home + goal + current position
        // communicate initial state to all players
        let mut state = self.make_initial_state(&players, board);

        self.broadcast_initial_state(&state, &mut players);

        // loop until game is over
        // - ask each player for a turn
        // - check if that player won
        let mut game_result = GameResult::default();
        let mut reached_goal: HashSet<Color> = HashSet::default();
        let mut round = 0;
        let mut first_player = state.current_player_info().clone();
        let mut num_passed = 0;
        let winner = loop {
            let turn: PlayerAction = players[0].take_turn(state.clone().into());
            match turn {
                Some(PlayerMove {
                    slide,
                    rotations,
                    destination,
                }) => {
                    num_passed = 0;
                    if state.is_valid_move(slide, rotations, destination) {
                        state.rotate_spare(rotations);
                        state
                            .slide_and_insert(slide)
                            .expect("Slide has already been verified");
                        state
                            .move_player(destination)
                            .expect("Error case is already checked by is_valid_move");

                        let pi = state.current_player_info();
                        if state.player_reached_goal() {
                            // player has reached their goal
                            let pi = pi.clone();
                            players[0].setup(None, pi.home);
                            reached_goal.insert(pi.color);
                        } else if state.player_reached_home() && reached_goal.contains(&pi.color) {
                            // current player won
                            break Some(pi.clone());
                        }
                    } else {
                        players.rotate_left(1);
                        match players.pop() {
                            Some(player) => {
                                let pi = state.remove_player().expect("Player list is non-empty");

                                // if we kick out the first player, we should update first_player
                                // to be the new current player
                                if pi == first_player {
                                    first_player = state.current_player_info().clone();
                                }

                                reached_goal.remove(&pi.color);
                                game_result.kicked.push(player);
                            }
                            None => {
                                // Ran out of players
                                break None;
                            }
                        };
                    }
                }
                None => {
                    num_passed += 1;

                    if num_passed == state.player_info.len() {
                        //  game should end, all players passed
                        break None;
                    }
                }
            }

            // advance to the next player
            Referee::next_player(&mut players, &mut state);

            // One round has completed
            if &first_player == state.current_player_info() {
                round += 1;

                if round >= 1000 {
                    // game should end, 1000 turns passed
                    break None;
                }
            }
        };

        // Communicate winners to all players
        let (winners, losers) = Referee::calculate_winners(winner, players, &state);
        Referee::broadcast_winners(&winners, losers);

        // return GameResult
        game_result.winners = winners;
        game_result
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
