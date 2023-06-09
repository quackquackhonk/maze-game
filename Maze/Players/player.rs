use std::io;

use crate::strategy::{PlayerAction, Strategy};
use common::{
    board::{Board, DefaultBoard},
    grid::Position,
    json::{JsonError, Name},
    state::{PlayerInfo, State},
};
use thiserror::Error;

pub type PlayerApiResult<T> = Result<T, PlayerApiError>;

#[derive(Error, Debug)]
pub enum PlayerApiError {
    #[error("IO error occured when communicating to player")]
    IoError(#[from] io::Error),
    #[error("response is not Json")]
    NotJson(#[from] serde_json::Error),
    #[error("response has incorrect format")]
    WrongJson(#[from] JsonError),
    #[error("timeout reached when attempting to recieve a response")]
    Timeout,
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

/// Trait describing the methods that `Player`s must implement
pub trait PlayerApi: Send {
    /// Returns the name of this Player
    fn name(&self) -> Name;
    /// Returns a `Board` with at least `cols` columns and `rows` rows
    fn propose_board0(&self, cols: u32, rows: u32) -> PlayerApiResult<Board>;
    /// The player receives a `PlayerBoardState`, which is all the publicly available information
    /// in the game, and its own private goal tile.
    fn setup(&mut self, state: Option<State<PlayerInfo>>, goal: Position) -> PlayerApiResult<()>;
    /// Returns a `PlayerAction` based on the given `PlayerBoardState`
    fn take_turn(&self, state: State<PlayerInfo>) -> PlayerApiResult<PlayerAction>;
    /// The player is informed if they won or not.
    fn won(&mut self, did_win: bool) -> PlayerApiResult<()>;
}

/// Represents a Local AI Player
pub struct LocalPlayer<S: Strategy + Send> {
    /// The name of the `LocalPlayer`
    name: Name,
    /// The `strategy::Strategy` that this `LocalPlayer` will use to make moves
    strategy: S,
    /// The goal position of this `LocalPlayer`. This type is an `Option` because the `LocalPlayer`
    /// will not know their goal until the `Referee` communicates it to them.
    goal: Option<Position>,
}

impl<S: Strategy + Send> LocalPlayer<S> {
    pub fn new(name: Name, strategy: S) -> Self {
        Self {
            name,
            strategy,
            goal: None,
        }
    }
}

impl<S: Strategy + Send> PlayerApi for LocalPlayer<S> {
    fn name(&self) -> Name {
        self.name.clone()
    }

    fn propose_board0(&self, _cols: u32, _rows: u32) -> PlayerApiResult<Board> {
        // FIXME: this shouldn't just propose the default board
        Ok(DefaultBoard::<7, 7>::default_board())
    }

    /// # Effect
    /// Sets `self.goal = Some(goal)`.
    fn setup(&mut self, _state: Option<State<PlayerInfo>>, goal: Position) -> PlayerApiResult<()> {
        self.goal = Some(goal);
        Ok(())
    }

    fn take_turn(&self, state: State<PlayerInfo>) -> PlayerApiResult<PlayerAction> {
        let start = state.player_info[0].current;
        Ok(self.strategy.get_move(
            state,
            start,
            self.goal.unwrap_or_else(|| {
                panic!(
                    "{} :setup() needs to be called before take_turn()",
                    self.name
                )
            }),
        ))
    }

    /// Does nothing
    fn won(&mut self, _did_win: bool) -> PlayerApiResult<()> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use common::color::ColorName;

    use super::*;

    use crate::{
        player::PlayerApi,
        strategy::{NaiveStrategy, Strategy},
    };

    use super::LocalPlayer;

    #[test]
    fn test_name() {
        let player = LocalPlayer {
            name: Name::from_static("bill"),
            strategy: NaiveStrategy::Euclid,
            goal: None,
        };

        assert_eq!(player.name(), Name::from_static("bill"));
    }

    #[test]
    fn test_propose_board() {
        let player = LocalPlayer {
            name: Name::from_static("bill"),
            strategy: NaiveStrategy::Euclid,
            goal: None,
        };

        assert_eq!(
            player.propose_board0(7, 7).unwrap(),
            DefaultBoard::<7, 7>::default_board()
        );
    }

    #[test]
    fn test_setup() {
        let mut player = LocalPlayer {
            name: Name::from_static("bill"),
            strategy: NaiveStrategy::Euclid,
            goal: None,
        };

        assert!(player.goal.is_none());

        let state = Some(State::default());
        player
            .setup(state, (1, 1))
            .expect("LocalPlayers are infallible");
        assert!(player.goal.is_some());
        assert_eq!(player.goal.unwrap(), (1, 1));

        player
            .setup(None, (2, 1))
            .expect("LocalPlayers are infallible");
        assert_eq!(player.goal.unwrap(), (2, 1));
    }

    #[test]
    fn test_take_turn() {
        let mut player = LocalPlayer {
            name: Name::from_static("bill"),
            strategy: NaiveStrategy::Euclid,
            goal: None,
        };

        let state = Some(State::default());
        player
            .setup(state, (1, 1))
            .expect("LocalPlayers are infallible");

        let state = State {
            player_info: vec![PlayerInfo {
                current: (0, 0),
                home: (0, 0),
                color: ColorName::Red.into(),
            }]
            .into(),
            ..Default::default()
        };

        let turn = player.take_turn(state.clone()).unwrap();
        assert_eq!(turn, NaiveStrategy::Euclid.get_move(state, (0, 0), (1, 1)));
    }
}
