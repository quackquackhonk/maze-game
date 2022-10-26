use crate::strategy::{NaiveStrategy, PlayerAction, PlayerBoardState, Strategy};
use common::board::{Board, DefaultBoard};
use common::grid::Position;

/// Trait describing the methods that `Player`s must implement
pub trait Player {
    /// Returns the name of this Player
    fn name(&self) -> String;
    /// Returns a `Board` with at least `cols` columns and `rows` rows
    fn propose_board0(&self, cols: u32, rows: u32) -> Board;
    /// The player receives a `PlayerBoardState`, which is all the publicly available information
    /// in the game, and its own private goal tile.
    fn setup(&mut self, state: Option<PlayerBoardState>, goal: Position);
    /// Returns a `PlayerAction` based on the given `PlayerBoardState`
    fn take_turn(&self, state: PlayerBoardState) -> PlayerAction;
    /// The player is informed if they won or not.
    fn won(&self, did_win: bool);
}

/// Represents a Local AI Player
pub struct LocalPlayer<S: Strategy> {
    /// The name of the `LocalPlayer`
    name: String,
    /// The `strategy::Strategy` that this `LocalPlayer` will use to make moves
    strategy: S,
    /// The goal position of this `LocalPlayer`. This type is an `Option` because the `LocalPlayer`
    /// will not know their goal until the `Referee` communicates it to them.
    goal: Option<Position>,
}

impl<S: Strategy> Player for LocalPlayer<S> {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn propose_board0(&self, cols: u32, rows: u32) -> Board {
        // FIXME: this shouldn't just propose the default board
        DefaultBoard::<7, 7>::default_board()
    }

    /// # Effect
    /// Sets `self.goal = Some(goal)`.
    fn setup(&mut self, _state: Option<PlayerBoardState>, goal: Position) {
        self.goal = Some(goal);
    }

    fn take_turn(&self, state: PlayerBoardState) -> PlayerAction {
        let start = state.player_positions[0];
        self.strategy.get_move(
            state,
            start,
            self.goal
                .expect("setup() needs to be called before take_turn()"),
        )
    }

    /// Does nothing
    fn won(&self, _did_win: bool) {}
}
