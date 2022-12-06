use std::collections::VecDeque;

use board::Board;
use board::Slide;
use grid::Position;
use hex::ToHex;
use thiserror::Error;

/// Contains all the types needed for the Board State and mutating the `Board`
pub mod board;
/// Contains the enum including all the possible Gems
pub mod gem;
/// Contains types for the `Grid` type and its `Position` type for indexing
pub mod grid;
/// Contains all the utilities for serializing and deserializing from JSON
pub mod json;
/// Contains the Tile type for use in the `Board`
pub mod tile;

#[derive(Debug, Error)]
pub enum StateError {
    #[error("{0:?} undoes the previous slide!")]
    SlideUndo(Slide),
    #[error("Player cannot go to {0:?}!")]
    PositionUnreachable(Position),
    #[error("No more players are in the game!")]
    NoPlayersLeft,
    #[error(transparent)]
    BoardError(#[from] board::OutOfBounds),
}

pub type StateResult<T> = Result<T, StateError>;

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct Color {
    /// The original name of the color.
    /// Is either the name of a color, like "red", or the Hex Color code for that color
    pub name: String,
    /// Represents a Hex color value
    /// contains values for (red, green, blue).
    pub code: (u8, u8, u8),
}

impl From<(u8, u8, u8)> for Color {
    fn from((r, g, b): (u8, u8, u8)) -> Self {
        Color {
            name: [r, g, b].encode_hex_upper::<String>(),
            code: (r, g, b),
        }
    }
}

/// Convenience Enum for making named colors
pub enum ColorName {
    Purple,
    Orange,
    Pink,
    Red,
    Green,
    Blue,
    Yellow,
    White,
    Black,
}

/// Converts from a `ColorName` enum to the corresponding `Color`
impl From<ColorName> for Color {
    fn from(cn: ColorName) -> Self {
        match cn {
            ColorName::Purple => Color {
                name: "purple".to_string(),
                code: (128, 0, 128),
            },
            ColorName::Orange => Color {
                name: "orange".to_string(),
                code: (255, 165, 0),
            },
            ColorName::Pink => Color {
                name: "pink".to_string(),
                code: (255, 192, 203),
            },
            ColorName::Red => Color {
                name: "red".to_string(),
                code: (255, 0, 0),
            },
            ColorName::Green => Color {
                name: "green".to_string(),
                code: (0, 255, 0),
            },
            ColorName::Blue => Color {
                name: "blue".to_string(),
                code: (0, 0, 255),
            },
            ColorName::Yellow => Color {
                name: "yellow".to_string(),
                code: (255, 255, 0),
            },
            ColorName::White => Color {
                name: "white".to_string(),
                code: (255, 255, 255),
            },
            ColorName::Black => Color {
                name: "black".to_string(),
                code: (0, 0, 0),
            },
        }
    }
}

/// Describes types that can be used as the information a `State` stores on its `Player`s
pub trait PlayerInfo {
    fn position(&self) -> Position;
    fn set_position(&mut self, dest: Position);
    fn home(&self) -> Position;
    /// Has this Player reached their home tile?
    fn reached_home(&self) -> bool;
    fn color(&self) -> Color;
}

pub trait PrivatePlayerInfo: PlayerInfo {
    fn reached_goal(&self) -> bool;
    fn set_goal(&mut self, goal: Position);
    fn goal(&self) -> Position;
    fn get_goals_reached(&self) -> u64;
    fn inc_goals_reached(&mut self);
}

/// Represents a Player and the `Position` of their home and themselves. Also holds their goal
/// `Gem` and their `Color`.
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct FullPlayerInfo {
    home: Position,
    position: Position,
    pub goal: Position,
    // Invariant: Every Player should have their own color
    color: Color,
    goals_reached: u64,
}

impl FullPlayerInfo {
    /// Constructs a new `Player` from its fields.
    pub fn new(home: Position, position: Position, goal: Position, color: Color) -> Self {
        Self {
            home,
            position,
            goal,
            color,
            goals_reached: 0,
        }
    }
}

impl PlayerInfo for FullPlayerInfo {
    fn position(&self) -> Position {
        self.position
    }

    fn set_position(&mut self, dest: Position) {
        self.position = dest;
    }

    fn home(&self) -> Position {
        self.home
    }

    fn color(&self) -> Color {
        self.color.clone()
    }
    /// Has this `Player` reached their home?
    fn reached_home(&self) -> bool {
        self.home == self.position
    }
}

impl PrivatePlayerInfo for FullPlayerInfo {
    fn reached_goal(&self) -> bool {
        self.goal == self.position
    }

    fn set_goal(&mut self, goal: Position) {
        self.goal = goal;
    }

    fn goal(&self) -> Position {
        self.goal
    }

    fn get_goals_reached(&self) -> u64 {
        self.goals_reached
    }

    fn inc_goals_reached(&mut self) {
        self.goals_reached += 1;
    }
}

#[derive(Debug, Default, Clone)]
pub struct PubPlayerInfo {
    pub current: Position,
    pub home: Position,
    pub color: Color,
}

impl PlayerInfo for PubPlayerInfo {
    fn position(&self) -> Position {
        self.current
    }

    fn set_position(&mut self, dest: Position) {
        self.current = dest
    }

    fn home(&self) -> Position {
        self.home
    }

    fn reached_home(&self) -> bool {
        self.position() == self.home()
    }

    fn color(&self) -> Color {
        self.color.clone()
    }
}

impl From<FullPlayerInfo> for PubPlayerInfo {
    fn from(pi: FullPlayerInfo) -> Self {
        PubPlayerInfo {
            current: pi.position(),
            home: pi.home(),
            color: pi.color(),
        }
    }
}

/// Represents the State of a single Maze Game.
#[derive(Debug, PartialEq, Eq)]
pub struct State<PInfo: PlayerInfo> {
    pub board: Board,
    pub player_info: VecDeque<PInfo>,
    pub previous_slide: Option<Slide>,
}

impl<PInfo: PlayerInfo> State<PInfo> {
    pub fn new(board: Board, player_info: Vec<PInfo>) -> Self {
        State {
            board,
            player_info: player_info.into(),
            previous_slide: None,
        }
    }

    /// Rotates the spare `Tile` in the `board` by a given number of 90 degree turns
    ///
    /// Does nothing if we do not currently have a spare tile
    pub fn rotate_spare(&mut self, num_turns: usize) {
        // The modulo operator saves us from doing extraneous turns
        (0..num_turns % 4).for_each(|_| self.board.rotate_spare());
    }

    fn slide_players(&mut self, &slide: &Slide) {
        self.player_info.iter_mut().for_each(|player_info| {
            player_info.set_position(slide.move_position(
                player_info.position(),
                self.board.grid[0].len(),
                self.board.grid.len(),
            ))
        });
    }

    /// Performs a slide and insert action
    ///
    /// # Errors
    /// Errors if the given slide action would nullify the previous slide action
    /// ```
    /// # use common::State;
    /// # use common::board::Slide;
    /// # use common::tile::CompassDirection;
    /// # use common::FullPlayerInfo;
    /// let mut state = State::<FullPlayerInfo>::default();
    ///
    /// // This is fine
    /// let res = state.slide_and_insert(state.board.new_slide(0, CompassDirection::North).unwrap());
    /// assert!(res.is_ok());
    ///  
    /// // This is not
    /// let res = state.slide_and_insert(state.board.new_slide(0, CompassDirection::South).unwrap());
    /// assert!(res.is_err());
    ///
    /// // This would however be fine
    /// let res = state.slide_and_insert(state.board.new_slide(2, CompassDirection::South).unwrap());
    /// assert!(res.is_ok());
    ///
    /// ```
    pub fn slide_and_insert(&mut self, slide: Slide) -> StateResult<()> {
        if let Some(prev) = self.previous_slide {
            if prev.direction.opposite() == slide.direction && prev.index == slide.index {
                // Kicking player out code can go here
                Err(StateError::SlideUndo(slide))?;
            }
        }
        self.board.slide_and_insert(slide)?;
        self.slide_players(&slide);
        self.previous_slide = Some(slide);
        Ok(())
    }

    /// Attempts to move the active player to `destination`.
    ///
    /// # Errors
    ///
    /// Errors if the active player is not able to reach `destination` from their current position.
    pub fn move_player(&mut self, destination: Position) -> StateResult<()> {
        if !self.can_reach_position(destination) || self.player_info[0].position() == destination {
            Err(StateError::PositionUnreachable(destination))?;
        }
        self.player_info[0].set_position(destination);
        Ok(())
    }

    /// Returns a Vec of positions reachable by the active player
    pub fn reachable_by_player(&self) -> Vec<Position> {
        self.board
            .reachable(self.player_info[0].position())
            .expect("Positions in `self.player_info` are never out of bounds")
    }

    /// Determines if the currently active `Player` can reach the `Tile` at the given `Position`
    #[must_use]
    pub fn can_reach_position(&self, target: Position) -> bool {
        self.board
            .reachable(self.player_info[0].position())
            .expect("Active player positions are always in bounds")
            .contains(&target)
    }

    /// Checks if the currently active `Player` has landed on its home tile
    #[must_use]
    pub fn player_reached_home(&self) -> bool {
        let player_info = &self.player_info[0];
        player_info.reached_home()
    }

    /// Adds a `Player` to the end of the list of currently active players
    pub fn add_player(&mut self, to_add: PInfo) {
        self.player_info.push_back(to_add);
    }

    /// Sets `self.active_player` to be the next player by indexing `self.player_info`
    pub fn next_player(&mut self) {
        if !self.player_info.is_empty() {
            self.player_info.rotate_left(1);
        }
    }

    /// Removes the currently active `Player` from game.
    pub fn remove_player(&mut self) -> StateResult<PInfo> {
        self.player_info
            .pop_front()
            .ok_or(StateError::NoPlayersLeft)
    }

    /// Returns a reference to the currently active `PlayerInfo`
    pub fn current_player_info(&self) -> &PInfo {
        &self.player_info[0]
    }

    pub fn current_player_info_mut(&mut self) -> &mut PInfo {
        &mut self.player_info[0]
    }
}

impl<Info: PlayerInfo + Clone> Clone for State<Info> {
    fn clone(&self) -> Self {
        Self {
            board: self.board.clone(),
            player_info: self.player_info.clone(),
            previous_slide: self.previous_slide,
        }
    }
}

impl<Info: PlayerInfo + Clone> State<Info> {
    /// Can the active player make the move represented by these arguments?
    pub fn is_valid_move(&self, slide: Slide, rotations: usize, destination: Position) -> bool {
        let rows = self.board.grid.len();
        let cols = self.board.grid[0].len();
        let mut state = self.clone();
        let start = slide.move_position(self.player_info[0].position(), cols, rows);
        state.rotate_spare(rotations);
        match state.slide_and_insert(slide) {
            Ok(_) => destination != start && state.move_player(destination).is_ok(),
            Err(_) => false,
        }
    }

    /// After sliding the row specified by `slide` and inserting the spare tile after rotating it
    /// `rotations` times, can the player go from `start` to `destination`
    pub fn reachable_after_move(
        &self,
        slide: Slide,
        rotations: usize,
        destination: Position,
        start: Position,
    ) -> bool {
        let mut state = self.clone();
        (0..rotations).for_each(|_| state.board.rotate_spare());
        state
            .board
            .slide_and_insert(slide)
            .expect("Slides we create are always in bounds?");
        let start = slide.move_position(start, state.board.grid[0].len(), state.board.grid.len());
        state
            .board
            .reachable(start)
            .expect("Start must be in bounds")
            .into_iter()
            .filter(|curr| curr != &start)
            .any(|curr| curr == destination)
    }
}

/// Methods for `State<FullPlayerInfo>` types
impl<Info: PrivatePlayerInfo + Clone> State<Info> {
    /// Checks if the currently active `Player` has landed on its goal tile
    #[must_use]
    pub fn player_reached_goal(&self) -> bool {
        let player_info = &self.player_info[0];
        player_info.reached_goal()
    }
}

impl<PInfo: PlayerInfo + Clone> Default for State<PInfo> {
    fn default() -> Self {
        Self {
            board: Default::default(),
            player_info: Default::default(),
            previous_slide: Default::default(),
        }
    }
}

impl From<State<FullPlayerInfo>> for State<PubPlayerInfo> {
    fn from(full_state: State<FullPlayerInfo>) -> Self {
        Self {
            board: full_state.board,
            player_info: full_state
                .player_info
                .into_iter()
                .map(PubPlayerInfo::from)
                .collect(),
            previous_slide: full_state.previous_slide,
        }
    }
}

#[cfg(test)]
mod state_tests {
    use crate::tile::{CompassDirection::*, ConnectorShape::*, PathOrientation::*};

    use super::*;

    #[test]
    fn test_add_player() {
        let mut state = State::default();

        assert!(state.player_info.is_empty());

        state.add_player(FullPlayerInfo {
            home: (0, 0),
            position: (0, 0),
            goal: (1, 1),
            color: ColorName::Red.into(),
            goals_reached: 0,
        });

        assert!(!state.player_info.is_empty());

        assert_eq!(state.player_info.len(), 1);

        state.add_player(FullPlayerInfo::new(
            (0, 1),
            (1, 0),
            (1, 1),
            ColorName::Blue.into(),
        ));

        assert_eq!(state.player_info.len(), 2);
    }

    #[test]
    fn test_remove_player() {
        let mut state = State::default();
        state.add_player(FullPlayerInfo::new(
            (0, 0),
            (0, 0),
            (1, 1),
            ColorName::Green.into(),
        ));

        assert_eq!(state.player_info.len(), 1);
        // Should not panic because the player exists in the HashMap
        assert!(state.remove_player().is_ok());

        assert_eq!(state.player_info.len(), 0);
        // Should not panic because `remove_player` ignores if players are actually in the game
        assert!(state.remove_player().is_err());
        assert_eq!(state.player_info.len(), 0);
    }

    #[test]
    fn test_next_player() {
        let mut state = State::default();
        // Does not fail
        state.next_player();
        let p1 = FullPlayerInfo::new((0, 0), (0, 0), (1, 1), ColorName::Red.into());
        state.add_player(p1.clone());
        assert_eq!(state.player_info[0], p1);
        state.next_player();
        assert_eq!(state.player_info[0], p1);

        let p2 = FullPlayerInfo::new((0, 0), (0, 0), (1, 3), ColorName::Green.into());
        state.add_player(p2.clone());
        assert_eq!(state.player_info[0], p1);
        state.next_player();
        assert_eq!(state.player_info[0], p2);
        state.next_player();
        assert_eq!(state.player_info[0], p1);

        let p3 = FullPlayerInfo::new((0, 0), (0, 0), (1, 5), ColorName::Yellow.into());
        state.add_player(p3.clone());
        assert_eq!(state.player_info[0], p1);
        state.next_player();
        assert_eq!(state.player_info[0], p2);
        state.next_player();
        assert_eq!(state.player_info[0], p3);
        state.next_player();
        assert_eq!(state.player_info[0], p1);
    }

    #[test]
    fn test_slide_and_insert() {
        let mut state: State<FullPlayerInfo> = State::default();

        let res = state.slide_and_insert(state.board.new_slide(0, North).unwrap());
        assert!(res.is_ok());

        // Sliding without inserting will not do anything
        let res = state.slide_and_insert(state.board.new_slide(0, South).unwrap());
        assert!(res.is_err());
    }

    #[test]
    fn test_slide_no_undo() {
        let mut state: State<FullPlayerInfo> = State::default();

        let res = state.slide_and_insert(state.board.new_slide(0, North).unwrap());
        assert!(res.is_ok());

        let res = state.slide_and_insert(state.board.new_slide(0, South).unwrap());
        assert!(res.is_err());

        // Doing it twice should not matter
        let res = state.slide_and_insert(state.board.new_slide(0, South).unwrap());
        assert!(res.is_err());

        // Doing it in another index is fine
        let res = state.slide_and_insert(state.board.new_slide(2, South).unwrap());
        assert!(res.is_ok());
    }

    #[test]
    fn test_slide_players() {
        let mut state = State::default();
        state.player_info.push_back(FullPlayerInfo::new(
            (0, 0),
            (0, 0),
            (1, 1),
            ColorName::Red.into(),
        ));
        state.player_info.push_back(FullPlayerInfo::new(
            (0, 0),
            (1, 2),
            (1, 3),
            ColorName::Yellow.into(),
        ));
        assert_eq!(state.player_info[0].position, (0, 0));
        assert_eq!(state.player_info[1].position, (1, 2));

        // Only player 1 is in the sliding column so it should move
        state.slide_players(&state.board.new_slide(0, South).unwrap());

        assert_eq!(state.player_info[0].position, (0, 1));
        assert_eq!(state.player_info[1].position, (1, 2));

        // Only player 2 is in the sliding row so it should move
        state.slide_players(&state.board.new_slide(2, East).unwrap());

        assert_eq!(state.player_info[0].position, (0, 1));
        assert_eq!(state.player_info[1].position, (2, 2));

        // Only player 1 is in the sliding column so it should move
        // but it should also wrap
        state.slide_players(&state.board.new_slide(0, North).unwrap());
        state.slide_players(&state.board.new_slide(0, North).unwrap());

        assert_eq!(state.player_info[0].position, (0, 6));
        assert_eq!(state.player_info[1].position, (2, 2));

        // Only player 2 is in the sliding row so it should move
        // but it should also wrap
        state.slide_players(&state.board.new_slide(2, West).unwrap());
        state.slide_players(&state.board.new_slide(2, West).unwrap());
        state.slide_players(&state.board.new_slide(2, West).unwrap());

        assert_eq!(state.player_info[0].position, (0, 6));
        assert_eq!(state.player_info[1].position, (6, 2));
    }

    #[test]
    fn test_rotate_spare() {
        let mut state: State<FullPlayerInfo> = State::default();

        assert_eq!(state.board.extra.connector, Crossroads);
        state.rotate_spare(1);
        assert_eq!(state.board.extra.connector, Crossroads);

        let res = state.slide_and_insert(state.board.new_slide(0, North).unwrap());
        assert!(res.is_ok());

        assert_eq!(state.board.extra.connector, Path(Horizontal));
        state.rotate_spare(1);
        assert_eq!(state.board.extra.connector, Path(Vertical));

        let res = state.slide_and_insert(state.board.new_slide(0, North).unwrap());
        assert!(res.is_ok());

        assert_eq!(state.board.extra.connector, Fork(East));
        state.rotate_spare(1);
        assert_eq!(state.board.extra.connector, Fork(North));
        state.rotate_spare(3);
        assert_eq!(state.board.extra.connector, Fork(East));
        state.rotate_spare(8);
        assert_eq!(state.board.extra.connector, Fork(East));
    }

    #[test]
    fn test_can_reach_position() {
        let mut state = State::default();
        state.player_info.push_back(FullPlayerInfo {
            home: (1, 1),
            position: (1, 1),
            goal: (1, 1),
            color: ColorName::Yellow.into(),
            goals_reached: 0,
        });
        state.player_info.push_back(FullPlayerInfo {
            home: (3, 1),
            position: (1, 3),
            goal: (1, 1),
            color: ColorName::Red.into(),
            goals_reached: 0,
        });

        // Default Board<7> is:
        //   0123456
        // 0 ─│└┌┐┘┴
        // 1 ├┬┤┼─│└
        // 2 ┌┐┘┴├┬┤
        // 3 ┼─│└┌┐┘
        // 4 ┴├┬┤┼─│
        // 5 └┌┐┘┴├┬
        // 6 ┤┼─│└┌┐
        //
        // extra = ┼
        // player can reach their own position
        assert!(state.can_reach_position((1, 1)));
        assert!(state.can_reach_position((0, 1)));
        assert!(state.can_reach_position((2, 1)));
        assert!(state.can_reach_position((2, 2)));
        assert!(!state.can_reach_position((0, 2)));
        // the second player can reach this position, but they are not the active player
        assert!(!state.can_reach_position((0, 3)));
        assert!(!state.can_reach_position((3, 3)));

        let res = state.slide_and_insert(state.board.new_slide(0, North).unwrap());
        assert!(res.is_ok());

        // Board after slide and insert:
        //   0123456
        // 0 ├│└┌┐┘┴
        // 1 ┌┬┤┼─│└
        // 2 ┼┐┘┴├┬┤
        // 3 ┴─│└┌┐┘
        // 4 └├┬┤┼─│
        // 5 ┤┌┐┘┴├┬
        // 6 ┼┼─│└┌┐
        //
        // extra = ─
        assert!(state.can_reach_position((0, 2)));
        assert!(state.can_reach_position((0, 3)));

        let res = state.slide_and_insert(state.board.new_slide(2, South).unwrap());
        assert!(res.is_ok());

        // Board after slide and insert:
        //   0123456
        // 0 ├│─┌┐┘┴
        // 1 ┌┬└┼─│└
        // 2 ┼┐┤┴├┬┤
        // 3 ┴─┘└┌┐┘
        // 4 └├│┤┼─│
        // 5 ┤┌┬┘┴├┬
        // 6 ┼┼┐│└┌┐
        //
        // extra = ─

        assert!(!state.can_reach_position((2, 1)));
        assert!(state.can_reach_position((2, 2)));
    }

    #[test]
    fn test_reachable_by_player() {
        let mut state = State::default();
        state.player_info.push_back(FullPlayerInfo {
            home: (1, 1),
            position: (1, 1),
            goal: (1, 1),
            color: ColorName::Green.into(),
            goals_reached: 0,
        });
        state.player_info.push_back(FullPlayerInfo {
            home: (3, 1),
            position: (1, 3),
            goal: (1, 1),
            color: ColorName::Red.into(),
            goals_reached: 0,
        });
        state.player_info.push_back(FullPlayerInfo {
            home: (5, 1),
            position: (3, 6),
            goal: (1, 1),
            color: ColorName::Purple.into(),
            goals_reached: 0,
        });
        // Default Board<7> is:
        //   0123456
        // 0 ─│└┌┐┘┴
        // 1 ├┬┤┼─│└
        // 2 ┌┐┘┴├┬┤
        // 3 ┼─│└┌┐┘
        // 4 ┴├┬┤┼─│
        // 5 └┌┐┘┴├┬
        // 6 ┤┼─│└┌┐
        //
        // extra = ┼

        let from_1_1 = state.reachable_by_player();
        assert_eq!(from_1_1.len(), 4);
        // cannot go to (4, 1) from (1, 1)
        assert!(!from_1_1.contains(&(4, 1)));

        assert!(state
            .slide_and_insert(state.board.new_slide(0, West).unwrap())
            .is_ok());

        // Board after slide:
        //   0123456
        // 0 │└┌┐┘┴┼
        // 1 ├┬┤┼─│└
        // 2 ┌┐┘┴├┬┤
        // 3 ┼─│└┌┐┘
        // 4 ┴├┬┤┼─│
        // 5 └┌┐┘┴├┬
        // 6 ┤┼─│└┌┐
        //
        // extra = ─
        let from_1_1 = state.reachable_by_player();
        assert_eq!(from_1_1.len(), 10);
        // can now go to (4, 1) from (1, 1)
        assert!(from_1_1.contains(&(4, 1)));

        // tiles reachable by Red player at (1, 3)
        state.next_player();
        let from_1_3 = state.reachable_by_player();
        assert_eq!(from_1_3.len(), 5);

        // isolated tile
        state.next_player();
        let from_3_6 = state.reachable_by_player();
        assert_eq!(from_3_6.len(), 1);
    }

    #[test]
    fn test_reachable_after_move() {
        let state = State {
            player_info: vec![
                PubPlayerInfo {
                    current: (1, 1),
                    home: (1, 1),
                    color: ColorName::Red.into(),
                },
                PubPlayerInfo {
                    current: (2, 2),
                    home: (3, 1),
                    color: ColorName::Purple.into(),
                },
            ]
            .into(),
            ..Default::default()
        };
        // Default Board<7> is:
        //   0123456
        // 0 ─│└┌┐┘┴
        // 1 ├┬┤┼─│└
        // 2 ┌┐┘┴├┬┤
        // 3 ┼─│└┌┐┘
        // 4 ┴├┬┤┼─│
        // 5 └┌┐┘┴├┬
        // 6 ┤┼─│└┌┐
        //
        // extra = ┼
        assert_eq!(state.board.reachable((0, 0)).unwrap(), vec![(0, 0)]);
        // board state after `player_move` is:
        //   0123456
        // 0 ┼─│└┌┐┘
        // 1 ├┬┤┼─│└
        // 2 ┌┐┘┴├┬┤
        // 3 ┼─│└┌┐┘
        // 4 ┴├┬┤┼─│
        // 5 └┌┐┘┴├┬
        // 6 ┤┼─│└┌┐
        //
        // extra = ┴

        // slides the top row right, moves player to (1, 1)
        // can the player go from (0, 0) to (2, 2) after making the move?
        assert!(state.reachable_after_move(
            state.board.new_slide(0, East).unwrap(),
            0,
            (2, 2),
            (0, 0)
        ));

        // slide the bottom row left
        // starting at (2, 6) you can go to (1, 5)
        assert!(state.board.reachable((2, 6)).unwrap().contains(&(1, 5)));
        // If you start at (2, 6) can you go to (1, 5) after making move? no
        assert!(!state.reachable_after_move(
            state.board.new_slide(6, West).unwrap(),
            0,
            (1, 5),
            (2, 6)
        ));
    }

    #[test]
    fn test_move_player() {
        let mut state = State::default();
        state.player_info.push_back(FullPlayerInfo {
            home: (1, 1),
            position: (1, 1),
            goal: (1, 1),
            color: ColorName::Yellow.into(),
            goals_reached: 0,
        });
        state.player_info.push_back(FullPlayerInfo {
            home: (3, 1),
            position: (3, 1),
            goal: (1, 3),
            color: ColorName::Red.into(),
            goals_reached: 0,
        });
        state.player_info.push_back(FullPlayerInfo {
            home: (5, 1),
            position: (0, 4),
            goal: (1, 5),
            color: ColorName::Blue.into(),
            goals_reached: 0,
        });

        // Default Board<7> is:
        //   0123456
        // 0 ─│└┌┐┘┴
        // 1 ├┬┤┼─│└
        // 2 ┌┐┘┴├┬┤
        // 3 ┼─│└┌┐┘
        // 4 ┴├┬┤┼─│
        // 5 └┌┐┘┴├┬
        // 6 ┤┼─│└┌┐
        //
        // extra = ┼
        // active player is the Yellow player
        assert!(state.move_player((2, 1)).is_ok());
        assert_eq!(state.player_info[0].position, (2, 1));
        // try to move the player to the right
        // should error and not update the player's position
        assert!(state.move_player((4, 1)).is_err());
        assert_eq!(state.player_info[0].position, (2, 1));
        // try to move the player to its current position
        // should error and not update the player's position
        assert!(state.move_player((2, 1)).is_err());
        assert_eq!(state.player_info[0].position, (2, 1));
        // set active player to Red player
        // Red player can go right to (4, 1)
        state.next_player();
        assert!(state.move_player((4, 1)).is_ok());
        assert_eq!(state.player_info[0].position, (4, 1));
        // try and go left to where Yellow player is, should error
        assert!(state.move_player((2, 1)).is_err());
        assert_eq!(state.player_info[0].position, (4, 1));
        // set active player to the Blue player
        state.next_player();
        // tests for moving multiple tiles at a time
        assert!(state.move_player((1, 2)).is_ok());
        assert_eq!(state.player_info[0].position, (1, 2));
        assert!(state.move_player((1, 3)).is_ok());
        assert_eq!(state.player_info[0].position, (1, 3));
    }

    #[test]
    fn test_player_reached_home() {
        // home tile is not on the same connected component as active player
        let mut state = State::default();
        state.player_info.push_back(FullPlayerInfo {
            home: (1, 1),
            position: (2, 3),
            goal: (1, 1),
            color: ColorName::Blue.into(),
            goals_reached: 0,
        });
        assert!(!state.player_reached_home());

        // player is on the same connected component, but not on their home tile
        let mut state = State::default();
        state.player_info.push_back(FullPlayerInfo {
            home: (1, 1),
            position: (0, 1),
            goal: (1, 3),
            color: ColorName::Red.into(),
            goals_reached: 0,
        });
        state.next_player();
        assert!(!state.player_reached_home());

        state.next_player();
        // active player is not on a home tile, but another player is
        let mut state = State::default();
        state.player_info.push_front(FullPlayerInfo {
            home: (1, 1),
            position: (2, 3),
            goal: (1, 1),
            color: ColorName::Green.into(),
            goals_reached: 0,
        });
        state.player_info.push_front(FullPlayerInfo {
            home: (3, 1),
            position: (3, 1),
            goal: (1, 3),
            color: ColorName::Blue.into(),
            goals_reached: 0,
        });
        assert!(state.player_reached_home());
        state.next_player();
        assert!(!state.player_reached_home());
    }

    #[test]
    fn test_player_reached_goal() {
        // Current Implementation of the Default board has Garnets and Amethysts in every Tile
        let mut state = State::default();
        state.player_info.push_back(FullPlayerInfo {
            home: (1, 1),
            position: (2, 3),
            goal: (1, 3),
            color: ColorName::Red.into(),
            goals_reached: 0,
        });
        assert!(!state.player_reached_goal());

        let mut state = State::default();
        state.player_info.push_back(FullPlayerInfo {
            home: (1, 1),
            position: (2, 3),
            goal: (2, 3),
            color: ColorName::Green.into(),
            goals_reached: 0,
        });
        state.next_player();
        assert!(state.player_reached_goal());
    }
}
