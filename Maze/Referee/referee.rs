use std::collections::HashSet;

use common::board::Board;
use common::grid::{squared_euclidian_distance, Position};
use common::{Color, FullPlayerInfo, PlayerInfo, State, BOARD_SIZE};
use players::player::Player;
use players::strategy::{PlayerAction, PlayerMove};
use rand::{Rng, RngCore, SeedableRng};
use rand_chacha::ChaChaRng;

use crate::observer::Observer;

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
pub type GameWinner = Option<FullPlayerInfo>;

pub struct Referee {
    rand: Box<dyn RngCore>,
}

impl Referee {
    pub fn new(seed: u64) -> Self {
        Self {
            rand: Box::new(ChaChaRng::seed_from_u64(seed)),
        }
    }

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
    fn make_initial_state(
        &mut self,
        players: &[Box<dyn Player>],
        board: Board,
    ) -> State<FullPlayerInfo> {
        let player_info = players
            .iter()
            .map(|_| {
                let home: Position = gen_immovable_tile_pos(&mut self.rand);
                let goal: Position = gen_immovable_tile_pos(&mut self.rand);
                FullPlayerInfo::new(
                    home,
                    home, /* players start on their home tile */
                    goal,
                    (self.rand.gen(), self.rand.gen(), self.rand.gen()).into(),
                )
            })
            .collect();

        State::new(board, player_info)
    }

    /// Communicates all public information of the current `state` and each `Player`'s private goal
    /// to all `Player`s in `players`.
    pub fn broadcast_initial_state(
        &self,
        state: &State<FullPlayerInfo>,
        players: &mut [Box<dyn Player>],
    ) {
        let mut state = state.clone();
        for player in players {
            let goal = state.current_player_info().goal;
            player.setup(Some(state.clone().into()), goal);
            state.next_player();
        }
    }

    /// Communicates the current state to all observers
    fn broadcast_state_to_observers(
        &self,
        state: &State<FullPlayerInfo>,
        observers: &mut Vec<Box<dyn Observer>>,
    ) {
        for observer in observers {
            observer.recieve_state(state.clone());
        }
    }

    /// Communicates that the game has ended to all observers
    fn broadcast_game_over_to_observers(&self, observers: &mut Vec<Box<dyn Observer>>) {
        for observer in observers {
            observer.game_over();
        }
    }

    /// Advances to the next player.
    ///
    /// # Effect
    ///
    /// rotates `players` to the left once, and does the same to the internal `Vec<PlayerInfo>`
    /// stored inside `state`.
    fn next_player(players: &mut [Box<dyn Player>], state: &mut State<FullPlayerInfo>) {
        players.rotate_left(1);
        state.next_player();
    }

    pub fn run_from_state(
        &self,
        state: &mut State<FullPlayerInfo>,
        players: &mut Vec<Box<dyn Player>>,
        observers: &mut Vec<Box<dyn Observer>>,
        reached_goal: &mut HashSet<Color>,
        kicked: &mut Vec<Box<dyn Player>>,
    ) -> GameWinner {
        // loop until game is over
        // - ask each player for a turn
        // - check if that player won
        self.broadcast_initial_state(&state, players);
        self.broadcast_state_to_observers(&state, observers);

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

                        self.broadcast_state_to_observers(state, observers);
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
                                kicked.push(player);
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
            if players.is_empty() {
                break None;
            }
            Referee::next_player(players, state);

            // One round has completed
            if first_player.color == state.current_player_info().color {
                round += 1;

                if round >= 1000 {
                    // game should end, 1000 turns passed
                    break None;
                }
            }
        };
        self.broadcast_game_over_to_observers(observers);
        winner
    }

    /// Returns a tuple of two `Vec<Box<dyn Player>>`. The first of these vectors contains all
    /// `Box<dyn Player>`s who won the game, and the second vector contains all the losers.
    #[allow(clippy::type_complexity)]
    pub fn calculate_winners(
        winner: GameWinner,
        players: Vec<Box<dyn Player>>,
        state: &State<FullPlayerInfo>,
        reached_goal: HashSet<Color>,
    ) -> (Vec<Box<dyn Player>>, Vec<Box<dyn Player>>) {
        let mut losers = vec![];
        let zipped_players: Box<dyn Iterator<Item = (Box<dyn Player>, &PlayerInfo)>> =
            if reached_goal.is_empty() {
                Box::new(players.into_iter().zip(state.player_info.iter()))
            } else {
                Box::new(
                    players
                        .into_iter()
                        .zip(state.player_info.iter())
                        .fold(vec![], |mut acc, (api, info)| {
                            if reached_goal.contains(&info.color) {
                                acc.push((api, info));
                            } else {
                                losers.push(api);
                            }
                            acc
                        })
                        .into_iter(),
                )
            };
        match winner {
            Some(winner) => zipped_players.fold(
                (vec![], losers),
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
                    (vec![], losers),
                    |(mut winners, mut losers), (api, info)| {
                        if min_dist
                            == squared_euclidian_distance(
                                &info.position,
                                if reached_goal.is_empty() {
                                    &info.goal
                                } else {
                                    &info.home
                                },
                            )
                        {
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
    fn broadcast_winners(winners: &mut [Box<dyn Player>], mut losers: Vec<Box<dyn Player>>) {
        for player in winners {
            player.won(true);
        }
        for player in &mut losers {
            player.won(false);
        }
    }

    /// Runs the game given the age-sorted `Vec<Box<dyn Player>>`, `players`.
    pub fn run_game(
        &mut self,
        mut players: Vec<Box<dyn Player>>,
        mut observers: Vec<Box<dyn Observer>>,
    ) -> GameResult {
        // Iterate over players to get their proposed boards
        // - for now, use the first players proposed board
        let board = self.get_player_boards(&players);

        // Create `State` from the chosen board
        // Assign each player a home + goal + current position
        // communicate initial state to all players
        let mut state = self.make_initial_state(&players, board);

        let mut game_result = GameResult::default();
        let mut reached_goal: HashSet<Color> = HashSet::default();
        let winner = self.run_from_state(
            &mut state,
            &mut players,
            &mut observers,
            &mut reached_goal,
            &mut game_result.kicked,
        );

        // Communicate winners to all players
        let (mut winners, losers) =
            Referee::calculate_winners(winner, players, &state, reached_goal);
        Referee::broadcast_winners(&mut winners, losers);
        self.broadcast_game_over_to_observers(&mut observers);

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

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, collections::HashSet, rc::Rc};

    use common::{
        board::{Board, DefaultBoard},
        grid::Position,
        json::Name,
        ColorName, FullPlayerInfo, PlayerInfo, State,
    };
    use players::{
        player::{LocalPlayer, Player},
        strategy::{NaiveStrategy, PlayerAction, PlayerBoardState},
    };
    use rand::SeedableRng;
    use rand_chacha::ChaChaRng;

    use crate::referee::{GameResult, Referee};

    #[derive(Debug, Default, Clone)]
    struct MockPlayer {
        turns_taken: Rc<RefCell<usize>>,
        state: Rc<RefCell<Option<PlayerBoardState>>>,
        goal: Rc<RefCell<Option<Position>>>,
        won: Rc<RefCell<Option<bool>>>,
    }

    impl Player for MockPlayer {
        fn name(&self) -> Name {
            Name::from_static("bob")
        }

        fn propose_board0(&self, _cols: u32, _rows: u32) -> Board {
            DefaultBoard::<3, 3>::default_board()
        }

        fn setup(&mut self, state: Option<PlayerBoardState>, goal: Position) {
            *self
                .goal
                .try_borrow_mut()
                .expect("we are the only owners??") = Some(goal);
            *self
                .state
                .try_borrow_mut()
                .expect("we are the only owners?") = state;
        }

        fn take_turn(&self, state: PlayerBoardState) -> PlayerAction {
            *self
                .turns_taken
                .try_borrow_mut()
                .expect("we are the only owners?") += 1;
            *self
                .state
                .try_borrow_mut()
                .expect("we are the only owners?") = Some(state);
            None
        }

        fn won(&mut self, did_win: bool) {
            *self.won.try_borrow_mut().expect("we are the only owners?") = Some(did_win)
        }
    }

    #[test]
    fn test_get_player_boards() {
        let referee = Referee {
            rand: Box::new(ChaChaRng::seed_from_u64(0)),
        };
        let mut players: Vec<Box<dyn Player>> = vec![Box::new(LocalPlayer::new(
            Name::from_static("bill"),
            NaiveStrategy::Euclid,
        ))];
        let board = referee.get_player_boards(&players);
        assert_eq!(board, DefaultBoard::<7, 7>::default_board());
        players.push(Box::new(MockPlayer::default()));
        players.rotate_left(1);
        let board = referee.get_player_boards(&players);
        assert_eq!(board, DefaultBoard::<3, 3>::default_board());
    }

    #[test]
    fn test_make_initial_state() {
        let mut referee = Referee {
            rand: Box::new(ChaChaRng::seed_from_u64(1)), // Seed 0 makes the first player have the
                                                         // same home and goal tile
        };
        let player = Box::new(MockPlayer::default());
        let players: Vec<Box<dyn Player>> = vec![player, Box::new(MockPlayer::default())];
        let mut state = referee.make_initial_state(&players, DefaultBoard::<7, 7>::default_board());
        assert_eq!(state.current_player_info().home, (1, 3));
        assert_eq!(state.current_player_info().goal, (3, 3));
        assert_eq!(state.current_player_info().position, (1, 3));
        state.next_player();
        assert_eq!(state.current_player_info().home, (1, 1));
        assert_eq!(state.current_player_info().goal, (5, 3));
        assert_eq!(state.current_player_info().position, (1, 1));
    }

    #[test]
    fn test_broadcast_inital_state() {
        let mut referee = Referee {
            rand: Box::new(ChaChaRng::seed_from_u64(0)),
        };
        let player = Box::new(MockPlayer::default());
        let mut players: Vec<Box<dyn Player>> = vec![player.clone()];
        let state = referee.make_initial_state(&players, DefaultBoard::<7, 7>::default_board());
        assert_eq!(player.goal.borrow().to_owned(), None);
        referee.broadcast_initial_state(&state, &mut players);
        assert_eq!(
            state.current_player_info().goal,
            player.goal.borrow().unwrap()
        );
    }

    #[test]
    fn test_next_player() {
        let mut state = State::default();
        state.add_player(FullPlayerInfo {
            home: (1, 1),
            position: (1, 1),
            goal: (0, 5),
            color: ColorName::Red.into(),
        });
        state.add_player(FullPlayerInfo {
            home: (1, 3),
            position: (1, 3),
            goal: (0, 3),
            color: ColorName::Blue.into(),
        });

        let mock = MockPlayer::default();
        let mut players: Vec<Box<dyn Player>> = vec![
            Box::new(mock),
            Box::new(LocalPlayer::new(
                Name::from_static("jill"),
                NaiveStrategy::Riemann,
            )),
        ];
        assert_eq!(players[0].name(), "bob");
        assert_eq!(state.player_info[0].color, ColorName::Red.into());
        assert_eq!(players[1].name(), "jill");
        assert_eq!(state.player_info[1].color, ColorName::Blue.into());
        Referee::next_player(&mut players, &mut state);
        assert_eq!(players[1].name(), "bob");
        assert_eq!(state.player_info[1].color, ColorName::Red.into());
        assert_eq!(players[0].name(), "jill");
        assert_eq!(state.player_info[0].color, ColorName::Blue.into());
    }

    #[test]
    fn test_calculate_winners() {
        let mut state = State::default();
        state.add_player(FullPlayerInfo {
            home: (0, 0),
            position: (1, 0),
            goal: (0, 5),
            color: ColorName::Red.into(),
        });
        let won_player = FullPlayerInfo {
            home: (1, 0),
            position: (1, 6),
            goal: (6, 1),
            color: ColorName::Blue.into(),
        };
        state.add_player(won_player.clone());

        let (winners, losers) = Referee::calculate_winners(
            Some(won_player),
            vec![
                Box::new(MockPlayer::default()),
                Box::new(LocalPlayer::new(
                    Name::from_static("jill"),
                    NaiveStrategy::Euclid,
                )),
            ],
            &state,
            vec![ColorName::Blue.into()].into_iter().collect(),
        );
        assert_eq!(winners.len(), 1);
        assert_eq!(winners[0].name(), "jill");
        assert_eq!(losers.len(), 1);
        let (winners, losers) = Referee::calculate_winners(
            None,
            vec![
                Box::new(MockPlayer::default()),
                Box::new(LocalPlayer::new(
                    Name::from_static("jill"),
                    NaiveStrategy::Euclid,
                )),
            ],
            &state,
            HashSet::default(),
        );
        assert_eq!(winners.len(), 1);
        assert_eq!(winners[0].name(), "bob");
        assert_eq!(losers.len(), 1);
    }

    #[test]
    fn test_broadcast_winners() {
        let mut referee = Referee {
            rand: Box::new(ChaChaRng::seed_from_u64(0)),
        };

        let player = Box::new(MockPlayer::default());
        let players: Vec<Box<dyn Player>> = vec![player.clone()];
        assert_eq!(player.won.borrow().to_owned(), None);
        referee.run_game(players, vec![]);
        assert_eq!(player.won.borrow().to_owned(), Some(true));

        let player = Box::new(MockPlayer::default());
        let players: Vec<Box<dyn Player>> = vec![
            Box::new(LocalPlayer::new(
                Name::from_static("joe"),
                NaiveStrategy::Euclid,
            )),
            player.clone(),
        ];
        assert_eq!(player.won.borrow().to_owned(), None);
        referee.run_game(players, vec![]);
        assert_eq!(player.won.borrow().to_owned(), Some(false));
    }

    #[test]
    fn test_run_game() {
        let mut referee = Referee {
            rand: Box::new(ChaChaRng::seed_from_u64(1)),
        };

        let player = Box::new(MockPlayer::default());
        let players: Vec<Box<dyn Player>> = vec![player.clone()];
        let GameResult { winners, kicked } = referee.run_game(players, vec![]);
        assert_eq!(winners[0].name(), player.name());
        assert_eq!(player.turns_taken.borrow().to_owned(), 1);
        assert!(kicked.is_empty());

        let player = Box::new(MockPlayer::default());
        let players: Vec<Box<dyn Player>> = vec![
            Box::new(LocalPlayer::new(
                Name::from_static("joe"),
                NaiveStrategy::Euclid,
            )),
            player,
        ];
        let GameResult { winners, kicked } = referee.run_game(players, vec![]);
        assert_eq!(winners[0].name(), Name::from_static("joe"));
        //dbg!(&player);
        assert_eq!(winners.len(), 1);
        assert!(kicked.is_empty());

        let mock = MockPlayer::default();
        let players: Vec<Box<dyn Player>> = vec![
            Box::new(LocalPlayer::new(
                Name::from_static("jill"),
                NaiveStrategy::Riemann,
            )),
            Box::new(LocalPlayer::new(
                Name::from_static("joe"),
                NaiveStrategy::Euclid,
            )),
            Box::new(mock.clone()),
        ];
        let GameResult { winners, kicked } = referee.run_game(players, vec![]);
        dbg!(mock);
        assert_eq!(winners[0].name(), Name::from_static("joe"));
        assert_eq!(winners.len(), 1);
        assert!(kicked.is_empty());
    }
}
