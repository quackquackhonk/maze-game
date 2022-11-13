use common::board::Board;
use common::grid::{squared_euclidian_distance, Position};
use common::json::Name;
use common::{Color, FullPlayerInfo, PlayerInfo, PrivatePlayerInfo, PubPlayerInfo, State};
use players::player::{PlayerApi, PlayerApiResult};
use players::strategy::{PlayerAction, PlayerMove};
use rand::{Rng, RngCore, SeedableRng};
use rand_chacha::ChaChaRng;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::collections::HashSet;
use std::fmt::Debug;
use std::hash::Hash;
use std::rc::Rc;

use crate::observer::Observer;

/// The Result of calling `Referee::run_game(...)`.
/// - The `winners` field contains all the winning players.
/// - The `kicked` field contains all the players who misbehaved during the game.
#[derive(Default)]
pub struct GameResult {
    pub winners: Vec<Player>,
    pub kicked: Vec<Player>,
}

/// Represents the winner of the game.
/// Some(PlayerInfo) -> This `PlayerInfo` was the first player to reach their goal and then their
/// home.
/// None -> The game ended without a single winner, the Referee will calculate winners another way.
pub type GameWinner = Option<Player>;

#[derive(Clone)]
pub struct Player {
    pub api: Rc<RefCell<Box<dyn PlayerApi>>>,
    pub info: FullPlayerInfo,
}

impl Debug for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Player").field("info", &self.info).finish()
    }
}

impl Player {
    fn new(api: Box<dyn PlayerApi>, info: FullPlayerInfo) -> Self {
        Player {
            api: Rc::new(RefCell::new(api)),
            info,
        }
    }
}

impl PlayerInfo for Player {
    fn position(&self) -> Position {
        self.info.position()
    }

    fn set_position(&mut self, dest: Position) {
        self.info.set_position(dest);
    }

    fn home(&self) -> Position {
        self.info.home()
    }

    fn reached_home(&self) -> bool {
        self.info.reached_home()
    }

    fn color(&self) -> Color {
        self.info.color()
    }
}

impl PrivatePlayerInfo for Player {
    fn reached_goal(&self) -> bool {
        self.info.reached_goal()
    }

    fn goal(&self) -> Position {
        self.info.goal
    }
}

impl PlayerApi for Player {
    fn name(&self) -> PlayerApiResult<Name> {
        self.api.borrow().name()
    }

    fn propose_board0(&self, cols: u32, rows: u32) -> PlayerApiResult<Board> {
        self.api.borrow().propose_board0(cols, rows)
    }

    fn setup(
        &mut self,
        state: Option<State<PubPlayerInfo>>,
        goal: Position,
    ) -> PlayerApiResult<()> {
        self.api.borrow_mut().setup(state, goal)
    }

    fn take_turn(&self, state: State<PubPlayerInfo>) -> PlayerApiResult<PlayerAction> {
        self.api.borrow().take_turn(state)
    }

    fn won(&mut self, did_win: bool) -> PlayerApiResult<()> {
        self.api.borrow_mut().won(did_win)
    }
}

impl PartialEq for Player {
    fn eq(&self, other: &Self) -> bool {
        self.info.color() == other.info.color()
    }
}

impl PartialEq<Color> for Player {
    fn eq(&self, other: &Color) -> bool {
        &self.info.color() == other
    }
}

impl PartialEq<Player> for Color {
    fn eq(&self, other: &Player) -> bool {
        self == &other.color()
    }
}

impl Eq for Player {}

impl Hash for Player {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.info.color().hash(state);
    }
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
    fn get_player_boards(&self, players: &[Box<dyn PlayerApi>]) -> Board {
        // FIXME: this should actually ask every player for a board
        players[0].propose_board0(7, 7).unwrap()
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
        let possible_goals = board.possible_goals().collect::<Vec<_>>();

        let player_info = players
            .into_iter()
            .map(|player| {
                let home: Position =
                    possible_homes.remove(self.rand.gen_range(0..possible_homes.len()));
                let goal: Position = possible_goals[self.rand.gen_range(0..possible_goals.len())];
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

    fn run_round(
        &self,
        state: &mut State<Player>,
        observers: &mut Vec<Box<dyn Observer>>,
        reached_goal: &mut HashSet<Player>,
        kicked: &mut Vec<Player>,
    ) -> Option<GameWinner> {
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
                    if valid_move {
                        state.rotate_spare(rotations);
                        state
                            .slide_and_insert(slide)
                            .expect("move has already been validated");
                        state
                            .move_player(destination)
                            .expect("move has already been validated");

                        if state.player_reached_home()
                            && reached_goal.contains(state.current_player_info())
                        {
                            // this player wins
                            return Some(Some(state.remove_player().unwrap()));
                        }

                        if state.to_full_state().player_reached_goal() {
                            // player needs to go home
                            reached_goal.insert(state.current_player_info().clone());
                            let player = &mut state.player_info[0];
                            player.setup(None, player.home()).is_err()
                        } else {
                            false
                        }
                    } else {
                        true
                    }
                }
                Ok(None) => {
                    num_passed += 1;
                    false
                }
                Err(_) => true,
            };

            if should_kick {
                num_kicked += 1;
                match state.remove_player() {
                    Ok(kicked_player) => kicked.push(kicked_player),
                    Err(_) => return Some(None),
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
            return Some(None);
        }

        None
    }

    pub fn run_from_state(
        &self,
        state: &mut State<Player>,
        observers: &mut Vec<Box<dyn Observer>>,
        mut reached_goal: HashSet<Player>,
        mut kicked: Vec<Player>,
    ) -> GameResult {
        // loop until game is over
        // - ask each player for a turn
        // - check if that player won
        self.broadcast_initial_state(state, &mut kicked);
        self.broadcast_state_to_observers(state, observers);

        const ROUNDS: usize = 1000;

        let mut result = None;
        for _ in 0..ROUNDS {
            if let Some(game_winner) =
                self.run_round(state, observers, &mut reached_goal, &mut kicked)
            {
                result = game_winner;
                break;
            };
        }
        self.broadcast_game_over_to_observers(observers);
        let (mut winners, losers) = Referee::calculate_winners(result, state, reached_goal);
        Referee::broadcast_winners(&mut winners, losers, &mut kicked);
        GameResult { winners, kicked }
    }

    /// Returns a tuple of two `Vec<Box<dyn Player>>`. The first of these vectors contains all
    /// `Box<dyn Player>`s who won the game, and the second vector contains all the losers.
    #[allow(clippy::type_complexity)]
    pub fn calculate_winners(
        winner: GameWinner,
        state: &State<Player>,
        reached_goal: HashSet<Player>,
    ) -> (Vec<Player>, Vec<Player>) {
        let mut losers = vec![];

        let players_to_check: Box<dyn Iterator<Item = Player>> = if reached_goal.is_empty() {
            Box::new(state.player_info.iter().cloned())
        } else {
            Box::new(
                state
                    .player_info
                    .iter()
                    .cloned()
                    .fold(vec![], |mut acc, player| {
                        if reached_goal.contains(&player) {
                            acc.push(player);
                        } else {
                            losers.push(player);
                        }
                        acc
                    })
                    .into_iter(),
            )
        };

        match winner {
            Some(winner) => (vec![winner], state.player_info.clone().into()),
            None => {
                let min_dist = state.player_info.iter().fold(usize::MAX, |prev, player| {
                    usize::min(
                        prev,
                        squared_euclidian_distance(&player.position(), &player.goal()),
                    )
                });

                players_to_check.fold((vec![], losers), |(mut winners, mut losers), player| {
                    let goal_to_measure = if reached_goal.is_empty() {
                        player.goal()
                    } else {
                        player.home()
                    };
                    if min_dist == squared_euclidian_distance(&player.position(), &goal_to_measure)
                    {
                        winners.push(player);
                    } else {
                        losers.push(player);
                    }
                    (winners, losers)
                })
            }
        }
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

        let game_result = GameResult::default();
        let reached_goal: HashSet<Player> = HashSet::default();

        self.run_from_state(&mut state, &mut observers, reached_goal, game_result.kicked)
    }
}

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, collections::HashSet, rc::Rc};

    use common::{
        board::{Board, DefaultBoard},
        gem::Gem,
        grid::{Grid, Position},
        json::Name,
        tile::{CompassDirection, ConnectorShape, Tile},
        Color, ColorName, FullPlayerInfo, PlayerInfo, PubPlayerInfo, State,
    };
    use players::{
        player::{LocalPlayer, PlayerApi, PlayerApiResult},
        strategy::{NaiveStrategy, PlayerAction},
    };
    use rand::SeedableRng;
    use rand_chacha::ChaChaRng;

    use crate::referee::{GameResult, Player, PrivatePlayerInfo, Referee};

    #[derive(Debug, Default, Clone)]
    struct MockPlayer {
        turns_taken: Rc<RefCell<usize>>,
        state: Rc<RefCell<Option<State<PubPlayerInfo>>>>,
        goal: Rc<RefCell<Option<Position>>>,
        won: Rc<RefCell<Option<bool>>>,
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
            *self
                .goal
                .try_borrow_mut()
                .expect("we are the only owners??") = Some(goal);
            *self
                .state
                .try_borrow_mut()
                .expect("we are the only owners?") = state;
            Ok(())
        }

        fn take_turn(&self, state: State<PubPlayerInfo>) -> PlayerApiResult<PlayerAction> {
            *self
                .turns_taken
                .try_borrow_mut()
                .expect("we are the only owners?") += 1;
            *self
                .state
                .try_borrow_mut()
                .expect("we are the only owners?") = Some(state);
            Ok(None)
        }

        fn won(&mut self, did_win: bool) -> PlayerApiResult<()> {
            *self.won.try_borrow_mut().expect("we are the only owners?") = Some(did_win);
            Ok(())
        }
    }

    #[test]
    fn test_get_player_boards() {
        let referee = Referee {
            rand: Box::new(ChaChaRng::seed_from_u64(0)),
        };
        let mut players: Vec<Box<dyn PlayerApi>> = vec![Box::new(LocalPlayer::new(
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
        let players: Vec<Box<dyn PlayerApi>> = vec![player, Box::new(MockPlayer::default())];
        let mut state = referee.make_initial_state(players, DefaultBoard::<7, 7>::default_board());
        assert_eq!(state.current_player_info().home(), (1, 3));
        assert_eq!(state.current_player_info().goal(), (5, 5));
        assert_eq!(state.current_player_info().position(), (1, 3));
        state.next_player();
        assert_eq!(state.current_player_info().home(), (3, 1));
        assert_eq!(state.current_player_info().goal(), (5, 3));
        assert_eq!(state.current_player_info().position(), (3, 1));
    }

    #[test]
    fn test_broadcast_inital_state() {
        let mut referee = Referee {
            rand: Box::new(ChaChaRng::seed_from_u64(0)),
        };
        let player = Box::new(MockPlayer::default());
        let players: Vec<Box<dyn PlayerApi>> = vec![player.clone()];
        let mut state = referee.make_initial_state(players, DefaultBoard::<7, 7>::default_board());
        assert_eq!(player.goal.borrow().to_owned(), None);
        referee.broadcast_initial_state(&mut state, &mut vec![]);
        assert_eq!(
            state.current_player_info().goal(),
            player.goal.borrow().unwrap()
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
        state.add_player(Player {
            api: Rc::new(RefCell::new(Box::new(MockPlayer::default()))),
            info: FullPlayerInfo::new((0, 0), (1, 0), (0, 5), Color::from(ColorName::Red)),
        });
        let won_player = Player::new(
            Box::new(LocalPlayer::new(
                Name::from_static("jill"),
                NaiveStrategy::Riemann,
            )),
            FullPlayerInfo::new((1, 0), (1, 6), (6, 1), Color::from(ColorName::Blue)),
        );

        let (winners, losers) = Referee::calculate_winners(
            Some(won_player.clone()),
            &state,
            vec![won_player.clone()].into_iter().collect(),
        );
        assert_eq!(winners.len(), 1);
        assert_eq!(winners[0].name().unwrap(), "jill");
        assert_eq!(losers.len(), 1);
        state.add_player(won_player);
        let (winners, losers) = Referee::calculate_winners(None, &state, HashSet::default());
        assert_eq!(winners.len(), 1);
        assert_eq!(winners[0].name().unwrap(), "bob");
        assert_eq!(losers.len(), 1);
    }

    #[test]
    fn test_broadcast_winners() {
        let mut referee = Referee {
            rand: Box::new(ChaChaRng::seed_from_u64(0)),
        };

        let player = Box::new(MockPlayer::default());
        let players: Vec<Box<dyn PlayerApi>> = vec![player.clone()];
        assert_eq!(player.won.borrow().to_owned(), None);
        referee.run_game(players, vec![]);
        assert_eq!(player.won.borrow().to_owned(), Some(true));

        let player = Box::new(MockPlayer::default());
        let players: Vec<Box<dyn PlayerApi>> = vec![
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
        let players: Vec<Box<dyn PlayerApi>> = vec![player.clone()];
        let GameResult { winners, kicked } = referee.run_game(players, vec![]);
        assert_eq!(winners[0].name().unwrap(), player.name().unwrap());
        assert_eq!(player.turns_taken.borrow().to_owned(), 1);
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
        let GameResult { winners, kicked } = referee.run_game(players, vec![]);
        assert_eq!(winners.len(), 1);
        assert_eq!(winners[0].name().unwrap(), Name::from_static("joe"));
        assert!(kicked.is_empty());
    }

    #[test]
    fn test_run_from_state() {
        let referee = Referee {
            rand: Box::new(ChaChaRng::seed_from_u64(1)),
        };
        let players = vec![
            Player::new(
                Box::new(LocalPlayer::new(
                    Name::from_static("bob"),
                    NaiveStrategy::Riemann,
                )),
                FullPlayerInfo::new((1, 3), (0, 1), (3, 3), ColorName::Red.into()),
            ),
            Player::new(
                Box::new(LocalPlayer::new(
                    Name::from_static("bob"),
                    NaiveStrategy::Riemann,
                )),
                FullPlayerInfo::new((1, 3), (0, 1), (3, 3), ColorName::Red.into()),
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

        let GameResult { winners, kicked } =
            referee.run_from_state(&mut state, &mut vec![], HashSet::default(), vec![]);
        assert_eq!(winners.len(), 2);
        assert_eq!(kicked.len(), 0);
    }
}
