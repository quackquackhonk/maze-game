use std::collections::VecDeque;

use crate::config::Config;
use crate::json::JsonGameResult;
use crate::player::Player;
use common::board::{Board, DefaultBoard};
use common::grid::{squared_euclidian_distance, Position};
use common::{FullPlayerInfo, PlayerInfo, PrivatePlayerInfo, PubPlayerInfo, State};
use players::player::PlayerApi;
use players::strategy::PlayerMove;
use rand::{Rng, RngCore, SeedableRng};
use rand_chacha::ChaChaRng;
use serde::Serialize;

use crate::observer::Observer;

/// The Result of calling `Referee::run_game(...)`.
/// - The `winners` field contains all the winning players.
/// - The `kicked` field contains all the players who misbehaved during the game.
#[derive(Debug, Default, Clone, Serialize)]
#[serde(into = "JsonGameResult")]
pub struct GameResult {
    pub winners: Vec<Player>,
    pub kicked: Vec<Player>,
}

/// Represents the winner of the game.
/// Some(PlayerInfo) -> This `PlayerInfo` was the first player to reach their goal and then their
/// home.
/// None -> The game ended without a single winner, the Referee will calculate winners another way.
pub type GameWinner = Option<Player>;

#[derive(Debug, PartialEq, Eq)]
pub enum GameStatus {
    NoMoreRounds,
    Tie,
    Winner,
}

trait RefereeState {
    fn to_player_state(&self) -> State<PubPlayerInfo>;
    fn to_full_state(&self) -> State<FullPlayerInfo>;
}

impl RefereeState for State<Player> {
    fn to_player_state(&self) -> State<PubPlayerInfo> {
        State {
            board: self.board.clone(),
            player_info: self
                .player_info
                .iter()
                .map(|pl| pl.info.clone().into())
                .collect(),
            previous_slide: self.previous_slide,
        }
    }

    fn to_full_state(&self) -> State<FullPlayerInfo> {
        State {
            board: self.board.clone(),
            player_info: self.player_info.iter().map(|pl| pl.info.clone()).collect(),
            previous_slide: self.previous_slide,
        }
    }
}

pub struct Referee {
    config: Config,
    rand: Box<dyn RngCore>,
}

impl Referee {
    pub fn new(seed: u64) -> Self {
        Self {
            rand: Box::new(ChaChaRng::seed_from_u64(seed)),
            config: Config::default(),
        }
    }

    /// Asks each `Player` in `players` to propose a `Board` and returns the chosen `Board`
    ///
    /// # Panics  
    /// This method will panic is `player` is an empty vector
    fn get_player_boards(&self, _players: &[Box<dyn PlayerApi>]) -> Board {
        // FIXME: this should actually ask every player for a board
        //let board = players[0].propose_board0(7, 7).unwrap();
        // DOUBLE FIXME: We dont actually ask players to propose a board
        DefaultBoard::<7, 7>::default_board()
    }

    /// Creates a vector of alternate goals based on `self.config.multiple_goals` and the given
    /// State.
    ///
    /// If `multiple_goals` is `true`, returns a vector of all possible goals in the `State`, with
    /// the goals assigned to the `Players` in the game removed.
    pub fn get_initial_goals(&self, state: &State<Player>) -> Vec<Position> {
        if self.config.multiple_goals {
            let assigned_goals: Vec<Position> =
                state.player_info.iter().map(|pi| pi.goal()).collect();

            state
                .board
                .possible_goals()
                .filter(|g| !assigned_goals.contains(g))
                .collect()
        } else {
            vec![]
        }
    }

    /// Given a `Board` and the list of `Player`s, creates an initial `State` for this game.
    ///
    /// This will assign each player a Goal and a home tile, and set each `Player`'s current
    /// position to be their home tile.
    fn make_initial_state(
        &mut self,
        players: Vec<Box<dyn PlayerApi>>,
        board: Board,
    ) -> State<Player> {
        // The possible locations for homes
        let mut possible_homes = board.possible_homes().collect::<Vec<_>>();

        // The possible locations for goals, remove the filter here if goals become movable tiles.
        let mut possible_goals = board.possible_goals().collect::<VecDeque<_>>();
        let player_info = players
            .into_iter()
            .map(|player| {
                let home: Position =
                    possible_homes.remove(self.rand.gen_range(0..possible_homes.len()));
                let goal: Position = possible_goals
                    .pop_front()
                    .expect("Did not have enough goals");
                let info = FullPlayerInfo::new(
                    home,
                    home, /* players start on their home tile */
                    goal,
                    (self.rand.gen(), self.rand.gen(), self.rand.gen()).into(),
                );
                Player::new(player, info)
            })
            .collect();

        State::new(board, player_info)
    }

    /// Communicates all public information of the current `state` and each `Player`'s private goal
    /// to all `Player`s in `players`.
    pub fn broadcast_initial_state(&self, state: &mut State<Player>, kicked: &mut Vec<Player>) {
        let mut player_state = state.to_player_state();
        let mut kicked_idx = 0;
        let total_players = state.player_info.len();
        for idx in 0..total_players {
            let player = state.current_player_info_mut();
            let goal = player.goal();
            match player.setup(Some(player_state.clone()), goal) {
                Ok(_) => {
                    state.next_player();
                    player_state.next_player()
                }
                Err(_) => {
                    kicked_idx += 1;
                    kicked.push(state.remove_player().unwrap())
                }
            }
            if idx + kicked_idx >= total_players {
                break;
            }
        }
    }

    /// Communicates the current state to all observers
    fn broadcast_state_to_observers(
        &self,
        state: &State<Player>,
        observers: &mut Vec<Box<dyn Observer>>,
    ) {
        for observer in observers {
            observer.recieve_state(state.to_full_state());
        }
    }

    /// Communicates that the game has ended to all observers
    fn broadcast_game_over_to_observers(&self, observers: &mut Vec<Box<dyn Observer>>) {
        for observer in observers {
            observer.game_over();
        }
    }

    /// Runs a single round. If the game ends during that round, return true.
    fn run_round(
        &mut self,
        state: &mut State<Player>,
        observers: &mut Vec<Box<dyn Observer>>,
        kicked: &mut Vec<Player>,
        remaining_goals: &mut VecDeque<Position>,
    ) -> Option<GameStatus> {
        let mut num_kicked = 0;
        let mut num_passed = 0;
        let players_in_round = state.player_info.len();
        for idx in 0..players_in_round {
            let should_kick = match state
                .current_player_info()
                .take_turn(state.to_player_state())
            {
                Ok(Some(PlayerMove {
                    slide,
                    rotations,
                    destination,
                })) => {
                    let valid_move =
                        state
                            .to_full_state()
                            .is_valid_move(slide, rotations, destination);
                    if !valid_move {
                        eprintln!(
                            "received invalid move from {}",
                            state.current_player_info().name().unwrap()
                        );
                        true
                    } else {
                        // eprintln!(
                        //     "received [{:?}, {:?}, {:?}] from {}",
                        //     slide,
                        //     rotations,
                        //     destination,
                        //     state.current_player_info().name().expect("valid")
                        // );
                        state.rotate_spare(rotations);
                        state
                            .slide_and_insert(slide)
                            .expect("move has already been validated");
                        state
                            .move_player(destination)
                            .expect("move has already been validated");

                        if state.player_reached_home()
                            && state.player_reached_goal()
                            && remaining_goals.is_empty()
                            // TODO: This is hack awaiting spec clarification
                            && state.current_player_info().get_goals_reached() > 0
                        {
                            self.broadcast_state_to_observers(state, observers);
                            // this player wins
                            return Some(GameStatus::Winner);
                        }

                        if state.to_full_state().player_reached_goal() {
                            state.current_player_info_mut().inc_goals_reached();
                            if !remaining_goals.is_empty() {
                                // player needs to go home
                                let goal = remaining_goals
                                    .pop_front()
                                    .expect("We checked it is not empty");
                                state.current_player_info_mut().set_goal(goal);
                                state.current_player_info_mut().setup(None, goal).is_err()
                            } else {
                                let home = state.current_player_info().home();
                                state.current_player_info_mut().set_goal(home);
                                state.current_player_info_mut().setup(None, home).is_err()
                            }
                        } else {
                            false
                        }
                    }
                }
                Ok(None) => {
                    eprintln!(
                        "received PASS from {}",
                        state.current_player_info().name().expect("valid")
                    );
                    num_passed += 1;
                    false
                }
                Err(_) => true,
            };

            if should_kick {
                num_kicked += 1;
                match state.remove_player() {
                    Ok(kicked_player) => kicked.push(kicked_player),
                    Err(_) => return Some(GameStatus::Tie),
                };
            } else {
                state.next_player();
            }

            self.broadcast_state_to_observers(state, observers);

            if idx + num_kicked >= players_in_round {
                break;
            }
        }

        if num_passed == players_in_round - num_kicked {
            return Some(GameStatus::Tie);
        }

        None
    }

    pub fn run_from_state(
        &mut self,
        state: &mut State<Player>,
        observers: &mut Vec<Box<dyn Observer>>,
    ) -> GameResult {
        let mut kicked = vec![];
        // loop until game is over
        // - ask each player for a turn
        // - check if that player won
        let mut remaining_goals: VecDeque<Position> = self.get_initial_goals(state).into();
        self.broadcast_initial_state(state, &mut kicked);
        self.broadcast_state_to_observers(state, observers);

        const ROUNDS: usize = 1000;

        let mut ended_early = GameStatus::NoMoreRounds;

        for _ in 0..ROUNDS {
            if let Some(status) =
                self.run_round(state, observers, &mut kicked, &mut remaining_goals)
            {
                ended_early = status;
                break;
            };
        }
        self.broadcast_game_over_to_observers(observers);
        let (mut winners, losers) = Referee::calculate_winners(state, ended_early);
        Referee::broadcast_winners(&mut winners, losers, &mut kicked);
        GameResult { winners, kicked }
    }

    /// Returns a tuple of two `Vec<Box<dyn Player>>`. The first of these vectors contains all
    /// `Box<dyn Player>`s who won the game, and the second vector contains all the losers.
    #[allow(clippy::type_complexity)]
    pub fn calculate_winners(
        state: &State<Player>,
        ended_early: GameStatus,
    ) -> (Vec<Player>, Vec<Player>) {
        let mut losers = vec![];

        // the current player in `state` is the player who made a move and ended the game.
        let game_ender = state.current_player_info();

        let players_to_check = {
            let max_goals = state
                .player_info
                .iter()
                .map(|pi| pi.get_goals_reached())
                .max()
                .unwrap_or(0);
            state
                .player_info
                .iter()
                .cloned()
                .fold(vec![], |mut acc, player| {
                    if player.get_goals_reached() == max_goals {
                        acc.push(player);
                    } else {
                        losers.push(player);
                    }
                    acc
                })
        };

        // If the game ended early, check if the `game_ender` has the highest number of goals
        // reached. If they do, they are the sole winner and everyone else loses
        if players_to_check.contains(game_ender) && ended_early == GameStatus::Winner {
            let losers = state
                .player_info
                .iter()
                .cloned()
                .filter(|pi| pi.color() != game_ender.color())
                .collect();
            return (vec![game_ender.clone()], losers);
        }

        let min_dist = players_to_check
            .iter()
            .map(|pi| squared_euclidian_distance(&pi.position(), &pi.goal()))
            .min()
            .unwrap_or(usize::MAX);
        dbg!(min_dist);

        players_to_check
            .into_iter()
            .fold((vec![], losers), |(mut winners, mut losers), player| {
                let goal_to_measure = player.goal();

                if min_dist
                    == dbg!(squared_euclidian_distance(
                        &player.position(),
                        &goal_to_measure
                    ))
                {
                    winners.push(player);
                } else {
                    losers.push(player);
                }
                (winners, losers)
            })
    }

    /// Communicates if a player won to all `Player`s in the given tuple of winners and losers
    fn broadcast_winners(
        winners: &mut Vec<Player>,
        mut losers: Vec<Player>,
        kicked: &mut Vec<Player>,
    ) {
        let mut kicked_winners = vec![];
        for (idx, player) in winners.iter_mut().enumerate() {
            if player.won(true).is_err() {
                kicked_winners.push(idx);
            }
        }
        for idx in kicked_winners.into_iter().rev() {
            kicked.push(winners.remove(idx));
        }

        let mut kicked_losers = vec![];
        for (idx, player) in losers.iter_mut().enumerate() {
            if player.won(false).is_err() {
                kicked_losers.push(idx);
            }
        }
        for idx in kicked_losers.into_iter().rev() {
            kicked.push(losers.remove(idx));
        }
    }

    /// Runs the game given the age-sorted `Vec<Box<dyn Player>>`, `players`.
    pub fn run_game(
        &mut self,
        players: Vec<Box<dyn PlayerApi>>,
        mut observers: Vec<Box<dyn Observer>>,
    ) -> GameResult {
        // Iterate over players to get their proposed boards
        // - for now, use the first players proposed board
        let board = self.get_player_boards(&players);

        // Create `State` from the chosen board
        // Assign each player a home + goal + current position
        // communicate initial state to all players
        let mut state = self.make_initial_state(players, board);

        self.run_from_state(&mut state, &mut observers)
    }
}

#[cfg(test)]
mod tests {
    use std::{collections::VecDeque, sync::Arc};

    use common::{
        board::{Board, DefaultBoard},
        gem::Gem,
        grid::{Grid, Position},
        json::Name,
        tile::{CompassDirection, ConnectorShape, Tile},
        Color, ColorName, FullPlayerInfo, PlayerInfo, PubPlayerInfo, State,
    };
    use parking_lot::Mutex;
    use players::{
        player::{LocalPlayer, PlayerApi, PlayerApiResult},
        strategy::{NaiveStrategy, PlayerAction},
    };
    use rand::SeedableRng;
    use rand_chacha::ChaChaRng;

    use crate::{
        config::Config,
        referee::{GameResult, GameStatus, Player, PrivatePlayerInfo, Referee},
    };

    #[derive(Debug, Default, Clone)]
    struct MockPlayer {
        turns_taken: Arc<Mutex<usize>>,
        state: Arc<Mutex<Option<State<PubPlayerInfo>>>>,
        goal: Arc<Mutex<Option<Position>>>,
        won: Arc<Mutex<Option<bool>>>,
    }

    impl PlayerApi for MockPlayer {
        fn name(&self) -> PlayerApiResult<Name> {
            Ok(Name::from_static("bob"))
        }

        fn propose_board0(&self, _cols: u32, _rows: u32) -> PlayerApiResult<Board> {
            Ok(DefaultBoard::<3, 3>::default_board())
        }

        fn setup(
            &mut self,
            state: Option<State<PubPlayerInfo>>,
            goal: Position,
        ) -> PlayerApiResult<()> {
            *self.goal.lock() = Some(goal);
            *self.state.lock() = state;
            Ok(())
        }

        fn take_turn(&self, state: State<PubPlayerInfo>) -> PlayerApiResult<PlayerAction> {
            *self.turns_taken.lock() += 1;
            *self.state.lock() = Some(state);
            Ok(None)
        }

        fn won(&mut self, did_win: bool) -> PlayerApiResult<()> {
            *self.won.lock() = Some(did_win);
            Ok(())
        }
    }

    #[test]
    fn test_get_player_boards() {
        let referee = Referee {
            rand: Box::new(ChaChaRng::seed_from_u64(0)),
            config: Config::default(),
        };
        let mut players: Vec<Box<dyn PlayerApi>> = vec![Box::new(LocalPlayer::new(
            Name::from_static("bill"),
            NaiveStrategy::Euclid,
        ))];
        let board = referee.get_player_boards(&players);
        assert_eq!(board, DefaultBoard::<7, 7>::default_board());
        players.push(Box::new(MockPlayer::default()));
        players.rotate_left(1);
        let _board = referee.get_player_boards(&players);
        // TODO: fix this
        //  it should be a 3 by 3 board
        //assert_eq!(board, DefaultBoard::<7, 7>::default_board());
    }

    #[test]
    fn test_get_initial_goals() {
        let referee = Referee {
            rand: Box::new(ChaChaRng::seed_from_u64(0)),
            config: Config::default(),
        };

        let state = State::default();

        assert!(referee.get_initial_goals(&state).is_empty());
    }

    #[test]
    fn test_get_initial_goals_multiple() {
        let config = Config {
            multiple_goals: true,
        };

        let mut state = State::default();
        let bob = Player::new(
            Box::new(MockPlayer::default()),
            FullPlayerInfo::new((1, 1), (1, 1), (1, 5), Color::from(ColorName::Red)),
        );
        let jill = Player::new(
            Box::new(LocalPlayer::new(
                Name::from_static("jill"),
                NaiveStrategy::Riemann,
            )),
            FullPlayerInfo::new((1, 3), (1, 3), (1, 3), Color::from(ColorName::Blue)),
        );
        state.add_player(bob);
        state.add_player(jill);

        let referee = Referee {
            rand: Box::new(ChaChaRng::seed_from_u64(0)),
            config,
        };

        let init_goals = referee.get_initial_goals(&state);

        assert_eq!(init_goals.len(), 7);
    }

    #[test]
    fn test_make_initial_state() {
        let mut referee = Referee {
            rand: Box::new(ChaChaRng::seed_from_u64(1)), // Seed 0 makes the first player have the
            config: Config::default(),
            // same home and goal tile
        };
        let player = Box::new(MockPlayer::default());
        let players: Vec<Box<dyn PlayerApi>> = vec![player, Box::new(MockPlayer::default())];
        let mut state = referee.make_initial_state(players, DefaultBoard::<7, 7>::default_board());
        assert_eq!(state.current_player_info().home(), (1, 3));
        assert_eq!(state.current_player_info().goal(), (1, 1));
        assert_eq!(state.current_player_info().position(), (1, 3));
        state.next_player();
        assert_eq!(state.current_player_info().home(), (5, 3));
        assert_eq!(state.current_player_info().goal(), (1, 3));
        assert_eq!(state.current_player_info().position(), (5, 3));
    }

    #[test]
    fn test_broadcast_inital_state() {
        let mut referee = Referee {
            rand: Box::new(ChaChaRng::seed_from_u64(0)),
            config: Config::default(),
        };
        let player = Box::new(MockPlayer::default());
        let players: Vec<Box<dyn PlayerApi>> = vec![player.clone()];
        let mut state = referee.make_initial_state(players, DefaultBoard::<7, 7>::default_board());
        assert_eq!(*player.goal.lock(), None);
        referee.broadcast_initial_state(&mut state, &mut vec![]);
        assert_eq!(
            state.current_player_info().goal(),
            player.goal.lock().unwrap()
        );
    }

    #[test]
    fn test_next_player() {
        let mut state = State::default();
        let bob = Player::new(
            Box::new(MockPlayer::default()),
            FullPlayerInfo::new((1, 1), (1, 1), (0, 5), Color::from(ColorName::Red)),
        );
        let jill = Player::new(
            Box::new(LocalPlayer::new(
                Name::from_static("jill"),
                NaiveStrategy::Riemann,
            )),
            FullPlayerInfo::new((1, 3), (1, 3), (0, 3), Color::from(ColorName::Blue)),
        );
        state.add_player(bob);
        state.add_player(jill);

        assert_eq!(state.player_info[0].name().unwrap(), "bob");
        assert_eq!(state.player_info[0].color(), Color::from(ColorName::Red));
        assert_eq!(state.player_info[1].name().unwrap(), "jill");
        assert_eq!(state.player_info[1].color(), Color::from(ColorName::Blue));
        state.next_player();
        assert_eq!(state.player_info[1].name().unwrap(), "bob");
        assert_eq!(state.player_info[1].color(), Color::from(ColorName::Red));
        assert_eq!(state.player_info[0].name().unwrap(), "jill");
        assert_eq!(state.player_info[0].color(), Color::from(ColorName::Blue));
    }

    #[test]
    fn test_calculate_winners() {
        let mut state = State::default();
        let bob = Player {
            api: Arc::new(Mutex::new(Box::new(MockPlayer::default()))),
            info: FullPlayerInfo::new((1, 1), (0, 0), (1, 5), Color::from(ColorName::Red)),
        };
        let jill = Player::new(
            Box::new(LocalPlayer::new(
                Name::from_static("jill"),
                NaiveStrategy::Riemann,
            )),
            FullPlayerInfo::new((1, 5), (1, 0), (1, 1), Color::from(ColorName::Blue)),
        );
        state.add_player(bob);
        state.add_player(jill);

        // as is, jill wins because it is closer to 1, 1
        let (winners, losers) = Referee::calculate_winners(&state, GameStatus::Tie);
        assert_eq!(winners.len(), 1);
        assert_eq!(winners[0].name().unwrap(), "jill");
        assert_eq!(losers.len(), 1);

        let mut state = State::default();
        let mut bob = Player {
            api: Arc::new(Mutex::new(Box::new(MockPlayer::default()))),
            info: FullPlayerInfo::new((1, 1), (0, 0), (1, 5), Color::from(ColorName::Red)),
        };
        let jill = Player::new(
            Box::new(LocalPlayer::new(
                Name::from_static("jill"),
                NaiveStrategy::Riemann,
            )),
            FullPlayerInfo::new((1, 5), (1, 0), (1, 1), Color::from(ColorName::Blue)),
        );
        bob.inc_goals_reached();
        state.add_player(bob);
        state.add_player(jill);
        // if bob has collected a goal, bob wins
        let (winners, losers) = Referee::calculate_winners(&state, GameStatus::Tie);
        assert_eq!(winners.len(), 1);
        assert_eq!(winners[0].name().unwrap(), "bob");
        assert_eq!(losers.len(), 1);

        let mut state = State::default();
        let mut bob = Player {
            api: Arc::new(Mutex::new(Box::new(MockPlayer::default()))),
            info: FullPlayerInfo::new((1, 1), (1, 1), (3, 3), Color::from(ColorName::Red)),
        };
        let mut jill = Player::new(
            Box::new(LocalPlayer::new(
                Name::from_static("jill"),
                NaiveStrategy::Riemann,
            )),
            FullPlayerInfo::new((1, 5), (1, 1), (5, 5), Color::from(ColorName::Blue)),
        );
        bob.inc_goals_reached();
        bob.inc_goals_reached();
        jill.inc_goals_reached();
        jill.inc_goals_reached();
        state.add_player(bob);
        state.add_player(jill);
        // bob wins because it is closer
        let (winners, losers) = Referee::calculate_winners(&state, GameStatus::Tie);
        assert_eq!(winners.len(), 1);
        assert_eq!(winners[0].name().unwrap(), "bob");
        assert_eq!(losers.len(), 1);

        let mut state = State::default();
        let mut bob = Player {
            api: Arc::new(Mutex::new(Box::new(MockPlayer::default()))),
            info: FullPlayerInfo::new((1, 1), (1, 1), (3, 3), Color::from(ColorName::Red)),
        };
        let mut jill = Player::new(
            Box::new(LocalPlayer::new(
                Name::from_static("jill"),
                NaiveStrategy::Riemann,
            )),
            FullPlayerInfo::new((1, 5), (1, 1), (3, 3), Color::from(ColorName::Blue)),
        );
        bob.inc_goals_reached();
        jill.inc_goals_reached();
        state.add_player(bob);
        state.add_player(jill);
        // both players win
        let (winners, losers) = Referee::calculate_winners(&state, GameStatus::Tie);
        assert_eq!(winners[0].name().unwrap(), "bob");
        assert_eq!(winners.len(), 2);
        assert_eq!(losers.len(), 0);
    }

    #[test]
    fn test_broadcast_winners() {
        let mut referee = Referee {
            rand: Box::new(ChaChaRng::seed_from_u64(0)),
            config: Config::default(),
        };

        let player = Box::new(MockPlayer::default());
        let players: Vec<Box<dyn PlayerApi>> = vec![player.clone()];
        assert_eq!(*player.won.lock(), None);
        referee.run_game(players, vec![]);
        assert_eq!(*player.won.lock(), Some(true));

        let player = Box::new(MockPlayer::default());
        let players: Vec<Box<dyn PlayerApi>> = vec![
            Box::new(LocalPlayer::new(
                Name::from_static("joe"),
                NaiveStrategy::Euclid,
            )),
            player.clone(),
        ];
        assert_eq!(*player.won.lock(), None);
        referee.run_game(players, vec![]);
        assert_eq!(*player.won.lock(), Some(false));
    }

    #[test]
    fn test_run_game() {
        let mut referee = Referee {
            rand: Box::new(ChaChaRng::seed_from_u64(1)),
            config: Config::default(),
        };

        let player = Box::new(MockPlayer::default());
        let players: Vec<Box<dyn PlayerApi>> = vec![player.clone()];
        let GameResult { winners, kicked } = referee.run_game(players, vec![]);
        assert_eq!(winners[0].name().unwrap(), player.name().unwrap());
        assert_eq!(*player.turns_taken.lock(), 1);
        assert!(kicked.is_empty());

        let player = Box::new(MockPlayer::default());
        let players: Vec<Box<dyn PlayerApi>> = vec![
            Box::new(LocalPlayer::new(
                Name::from_static("joe"),
                NaiveStrategy::Euclid,
            )),
            player,
        ];
        let GameResult { winners, kicked } = referee.run_game(players, vec![]);
        assert_eq!(winners[0].name().unwrap(), Name::from_static("joe"));
        assert_eq!(winners.len(), 1);
        assert!(kicked.is_empty());

        let mock = MockPlayer::default();
        let players: Vec<Box<dyn PlayerApi>> = vec![
            Box::new(LocalPlayer::new(
                Name::from_static("jill"),
                NaiveStrategy::Riemann,
            )),
            Box::new(LocalPlayer::new(
                Name::from_static("joe"),
                NaiveStrategy::Euclid,
            )),
            Box::new(mock),
        ];
        assert_eq!(
            players[0].propose_board0(7, 7).unwrap(),
            referee.get_player_boards(&players)
        );
        assert_eq!(
            players[0].propose_board0(7, 7).unwrap(),
            DefaultBoard::<7, 7>::default_board()
        );
        let GameResult { winners, kicked } = referee.run_game(players, vec![]);
        assert_eq!(winners.len(), 1);
        assert_eq!(winners[0].name().unwrap(), Name::from_static("jill"));
        assert!(kicked.is_empty());
    }

    #[test]
    fn test_run_from_state() {
        let mut referee = Referee {
            rand: Box::new(ChaChaRng::seed_from_u64(1)),
            config: Config::default(),
        };
        let players = vec![
            Player::new(
                Box::new(LocalPlayer::new(
                    Name::from_static("bob"),
                    NaiveStrategy::Riemann,
                )),
                FullPlayerInfo::new((1, 3), (1, 1), (3, 3), ColorName::Red.into()),
            ),
            Player::new(
                Box::new(LocalPlayer::new(
                    Name::from_static("joe"),
                    NaiveStrategy::Riemann,
                )),
                FullPlayerInfo::new((1, 3), (1, 1), (3, 3), ColorName::Blue.into()),
            ),
        ];
        let mut state: State<Player> = State {
            player_info: players.into(),
            ..Default::default()
        };
        let mut idx = 0;
        let corner = ConnectorShape::Corner(CompassDirection::West);
        state.board.grid = Grid::from([[(); 7]; 7].map(|list| {
            list.map(|_| {
                let tile = Tile {
                    connector: corner,
                    gems: Gem::pair_from_num(idx),
                };
                idx += 1;
                tile
            })
        }));
        state.board.extra.connector = corner;
        state.previous_slide = state.board.new_slide(0, CompassDirection::West);

        let GameResult { winners, kicked } = referee.run_from_state(&mut state, &mut vec![]);
        assert_eq!(winners.len(), 2);
        assert_eq!(kicked.len(), 0);
    }

    #[test]
    fn test_run_from_state_multiple_goals() {
        let mut referee = Referee {
            rand: Box::new(ChaChaRng::seed_from_u64(1)),
            config: Config {
                multiple_goals: true,
            },
        };
        let players = vec![
            Player::new(
                Box::new(LocalPlayer::new(
                    Name::from_static("bob"),
                    NaiveStrategy::Riemann,
                )),
                FullPlayerInfo::new((1, 3), (1, 1), (5, 3), ColorName::Red.into()),
            ),
            Player::new(
                Box::new(LocalPlayer::new(
                    Name::from_static("joe"),
                    NaiveStrategy::Euclid,
                )),
                FullPlayerInfo::new((1, 3), (1, 1), (3, 3), ColorName::Blue.into()),
            ),
        ];
        let mut state: State<Player> = State {
            player_info: players.into(),
            ..Default::default()
        };
        let GameResult { winners, kicked } = dbg!(referee.run_from_state(&mut state, &mut vec![]));
        let (calculated_winners, losers) =
            dbg!(Referee::calculate_winners(&state, GameStatus::Tie));

        assert_eq!(winners.len(), 1);
        assert_eq!(calculated_winners.len(), 1);
        assert_eq!(losers.len(), 1);
        assert_eq!(kicked.len(), 0);
        assert_eq!(winners[0].name().unwrap(), "bob");
        assert_eq!(losers[0].name().unwrap(), "joe");
    }

    #[test]
    fn test_run_round() {
        let mut referee = Referee {
            rand: Box::new(ChaChaRng::seed_from_u64(1)),
            config: Config {
                multiple_goals: false,
            },
        };
        let players = vec![
            Player::new(
                Box::new(LocalPlayer::new(
                    Name::from_static("bob"),
                    NaiveStrategy::Riemann,
                )),
                FullPlayerInfo::new((1, 3), (1, 1), (5, 3), ColorName::Red.into()),
            ),
            Player::new(
                Box::new(LocalPlayer::new(
                    Name::from_static("joe"),
                    NaiveStrategy::Euclid,
                )),
                FullPlayerInfo::new((1, 3), (1, 1), (3, 3), ColorName::Blue.into()),
            ),
        ];
        let mut state: State<Player> = State {
            player_info: players.into(),
            ..Default::default()
        };

        let mut kicked = vec![];

        referee.broadcast_initial_state(&mut state, &mut kicked);

        // the game does not end
        assert!(referee
            .run_round(
                &mut state,
                &mut vec![],
                &mut kicked,
                &mut VecDeque::default()
            )
            .is_none());
        assert_eq!(state.player_info[0].position(), (0, 0));
        assert_eq!(state.player_info[0].goal(), (5, 3));
        assert_eq!(state.player_info[1].position(), (3, 3));
        assert_eq!(state.player_info[1].goal(), (1, 3));

        // the game does end
        assert!(referee
            .run_round(
                &mut state,
                &mut vec![],
                &mut kicked,
                &mut VecDeque::default()
            )
            .is_some());
        // joe is now the 0th player because it won
        assert_eq!(state.player_info[0].position(), (1, 3));
        assert_eq!(state.player_info[0].goal(), (1, 3));
        assert_eq!(state.player_info[1].position(), (5, 3));
        assert_eq!(state.player_info[1].goal(), (1, 3));
    }

    #[test]
    fn test_run_round_multiple() {
        let mut referee = Referee {
            rand: Box::new(ChaChaRng::seed_from_u64(1)),
            config: Config {
                multiple_goals: true,
            },
        };
        let players = vec![
            Player::new(
                Box::new(LocalPlayer::new(
                    Name::from_static("bob"),
                    NaiveStrategy::Riemann,
                )),
                FullPlayerInfo::new((1, 5), (1, 1), (5, 3), ColorName::Red.into()),
            ),
            Player::new(
                Box::new(LocalPlayer::new(
                    Name::from_static("joe"),
                    NaiveStrategy::Euclid,
                )),
                FullPlayerInfo::new((1, 3), (1, 1), (3, 3), ColorName::Blue.into()),
            ),
        ];
        let mut state: State<Player> = State {
            player_info: players.into(),
            ..Default::default()
        };

        let mut kicked = vec![];
        let mut remaining_goals: VecDeque<Position> = vec![(1, 1), (5, 5)].into();

        referee.broadcast_initial_state(&mut state, &mut kicked);

        // the game does not end
        assert_eq!(remaining_goals.len(), 2);
        assert!(referee
            .run_round(&mut state, &mut vec![], &mut kicked, &mut remaining_goals)
            .is_none());
        assert_eq!(remaining_goals.len(), 1);
        assert_eq!(state.player_info[0].position(), (0, 0));
        assert_eq!(state.player_info[0].goal(), (5, 3));
        assert_eq!(state.player_info[1].position(), (3, 3));
        assert_eq!(state.player_info[1].goal(), (1, 1));

        // the game does not end
        assert!(referee
            .run_round(&mut state, &mut vec![], &mut kicked, &mut remaining_goals)
            .is_none());
        assert_eq!(remaining_goals.len(), 0);
        assert_eq!(state.player_info[0].position(), (5, 3));
        assert_eq!(state.player_info[0].goal(), (5, 5));
        assert_eq!(state.player_info[1].position(), (1, 1));
        assert_eq!(state.player_info[1].goal(), (1, 3));

        // the game does end
        assert!(referee
            .run_round(&mut state, &mut vec![], &mut kicked, &mut remaining_goals)
            .is_some());
        assert_eq!(remaining_goals.len(), 0);
        // joe is the first player bc it won
        assert_eq!(state.player_info[0].position(), (1, 3));
        assert_eq!(state.player_info[0].goal(), (1, 3));
        assert_eq!(state.player_info[1].position(), (5, 5));
        assert_eq!(state.player_info[1].goal(), (1, 5));
    }
}
