use std::collections::VecDeque;

use board::Board;
use board::BoardResult;
use board::Slide;
use gem::Gem;
use grid::Position;
use unordered_pair::UnorderedPair;

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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Color {
    /// The original name of the color.
    /// Is either the name of a color, like "red", or the Hex Color code for that color
    name: String,
    /// Represents a Hex color value
    /// contains values for (red, green, blue).
    code: (u8, u8, u8),
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

/// Represents a Player and the `Position` of their home and themselves. Also holds their goal
/// `Gem` and their `Color`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlayerInfo {
    home: Position,
    pub(crate) position: Position,
    goal: UnorderedPair<Gem>,
    // Invariant: Every Player should have their own color
    color: Color,
}

impl PlayerInfo {
    /// Constructs a new `Player` from its fields.
    pub fn new(
        home: Position,
        position: Position,
        goal: impl Into<UnorderedPair<Gem>>,
        color: Color,
    ) -> Self {
        Self {
            home,
            position,
            goal: goal.into(),
            color,
        }
    }

    /// Is the given `Gem` this `Player`'s goal?
    fn reached_goal(&self, gem: impl Into<UnorderedPair<Gem>>) -> bool {
        self.goal == gem.into()
    }

    /// Has this `Player` reached their home?
    fn reached_home(&self) -> bool {
        self.home == self.position
    }
}

/// Represents the State of a single Maze Game.
#[derive(Debug, Default, Clone)]
pub struct State {
    pub(crate) board: Board,
    pub(crate) player_info: VecDeque<PlayerInfo>,
    /// Invariant: active_player must be < player_info.len();
    /// its unsigned so it will always be <= 0.
    pub(crate) previous_slide: Option<Slide>,
}

pub const BOARD_SIZE: usize = 7;

impl State {
    pub fn new(board: Board, player_info: Vec<PlayerInfo>) -> Self {
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
            player_info.position = slide.move_position(
                player_info.position,
                self.board.grid[0].len(),
                self.board.grid.len(),
            )
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
    /// let mut state = State::default();
    ///
    /// // This is fine
    /// let res = state.slide_and_insert(Slide::new(0, CompassDirection::North));
    /// assert!(res.is_ok());
    ///  
    /// // This is not
    /// let res = state.slide_and_insert(Slide::new(0, CompassDirection::South));
    /// assert!(res.is_err());
    ///
    /// // This would however be fine
    /// let res = state.slide_and_insert(Slide::new(2, CompassDirection::South));
    /// assert!(res.is_ok());
    ///
    /// ```
    pub fn slide_and_insert(&mut self, slide: Slide) -> BoardResult<()> {
        if !slide.is_valid_slide(self.board.grid.len(), self.board.grid[0].len()) {
            Err("Slide move is invalid".to_string())?;
        } else if let Some(prev) = self.previous_slide {
            if prev.direction.opposite() == slide.direction && prev.index == slide.index {
                // Kicking player out code can go here
                Err("Attempted to do a slide action that would undo the previous slide")?;
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
    pub fn move_player(&mut self, destination: Position) -> BoardResult<()> {
        if !self.can_reach_position(destination) {
            Err("Active player cannot reach the requested tile")?;
        } else if self.player_info[0].position == destination {
            Err("Active player is already on that tile")?;
        }
        self.player_info[0].position = destination;
        Ok(())
    }

    /// Returns a Vec of positions reachable by the active player
    pub fn reachable_by_player(&self) -> Vec<Position> {
        self.board
            .reachable(self.player_info[0].position)
            .expect("Positions in `self.player_info` are never out of bounds")
    }

    /// Determines if the currently active `Player` can reach the `Tile` at the given `Position`
    #[must_use]
    pub fn can_reach_position(&self, target: Position) -> bool {
        self.board
            .reachable(self.player_info[0].position)
            .expect("Active player positions are always in bounds")
            .contains(&target)
    }

    /// Checks if the currently active `Player` has landed on its goal tile
    #[must_use]
    pub fn player_reached_goal(&self) -> bool {
        let player_info = &self.player_info[0];
        let gem_at_player = self.board[player_info.position].gems;
        player_info.reached_goal(gem_at_player)
    }

    /// Checks if the currently active `Player` has landed on its home tile
    #[must_use]
    pub fn player_reached_home(&self) -> bool {
        let player_info = &self.player_info[0];
        player_info.reached_home()
    }

    /// Adds a `Player` to the end of the list of currently active players
    pub fn add_player(&mut self, to_add: PlayerInfo) {
        self.player_info.push_back(to_add);
    }

    /// Sets `self.active_player` to be the next player by indexing `self.player_info`
    pub fn next_player(&mut self) {
        if !self.player_info.is_empty() {
            self.player_info.rotate_left(1);
        }
    }

    /// Removes the currently active `Player` from game.
    pub fn remove_player(&mut self) -> BoardResult<PlayerInfo> {
        self.player_info
            .pop_front()
            .ok_or("No Players left".to_string())
    }
}

#[cfg(test)]
mod StateTests {
    use crate::{
        gem::Gem,
        tile::{CompassDirection::*, ConnectorShape::*, PathOrientation::*},
    };

    use super::*;

    #[test]
    fn test_add_player() {
        let mut state = State::default();

        assert!(state.player_info.is_empty());

        state.add_player(PlayerInfo {
            home: (0, 0),
            position: (0, 0),
            goal: (Gem::ruby, Gem::sphalerite).into(),
            color: ColorName::Red.into(),
        });

        assert!(!state.player_info.is_empty());

        assert_eq!(state.player_info.len(), 1);

        state.add_player(PlayerInfo::new(
            (0, 1),
            (1, 0),
            (Gem::blue_cushion, Gem::garnet),
            ColorName::Blue.into(),
        ));

        assert_eq!(state.player_info.len(), 2);
    }

    #[test]
    fn test_remove_player() {
        let mut state = State::default();
        state.add_player(PlayerInfo::new(
            (0, 0),
            (0, 0),
            (Gem::ruby, Gem::sphalerite),
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
        assert_eq!(state.active_player, 0);
        state.next_player();
        assert_eq!(state.active_player, 0);

        state.add_player(PlayerInfo::new(
            (0, 0),
            (0, 0),
            (Gem::ruby, Gem::diamond),
            ColorName::Red.into(),
        ));
        assert_eq!(state.active_player, 0);
        state.next_player();
        assert_eq!(state.active_player, 0);

        state.add_player(PlayerInfo::new(
            (0, 0),
            (0, 0),
            (Gem::ruby, Gem::magnesite),
            ColorName::Green.into(),
        ));
        assert_eq!(state.active_player, 0);
        state.next_player();
        assert_eq!(state.active_player, 1);
        state.next_player();
        assert_eq!(state.active_player, 0);

        state.add_player(PlayerInfo::new(
            (0, 0),
            (0, 0),
            (Gem::ruby, Gem::black_onyx),
            ColorName::Yellow.into(),
        ));
        assert_eq!(state.active_player, 0);
        state.next_player();
        assert_eq!(state.active_player, 1);
        state.next_player();
        assert_eq!(state.active_player, 2);
        state.next_player();
        assert_eq!(state.active_player, 0);
    }

    #[test]
    fn test_slide_and_insert() {
        let mut state = State::default();

        let res = state.slide_and_insert(Slide::new(0, North));
        assert!(res.is_ok());

        // Sliding without inserting will not do anything
        let res = state.slide_and_insert(Slide::new(0, South));
        assert!(res.is_err());
    }

    #[test]
    fn test_slide_no_undo() {
        let mut state = State::default();

        let res = state.slide_and_insert(Slide::new(0, North));
        assert!(res.is_ok());

        let res = state.slide_and_insert(Slide::new(0, South));
        assert!(res.is_err());

        // Doing it twice should not matter
        let res = state.slide_and_insert(Slide::new(0, South));
        assert!(res.is_err());

        // Doing it in another index is fine
        let res = state.slide_and_insert(Slide::new(2, South));
        assert!(res.is_ok());
    }

    #[test]
    fn test_slide_players() {
        let mut state = State::default();
        state.player_info.push(PlayerInfo::new(
            (0, 0),
            (0, 0),
            (Gem::ruby, Gem::carnelian),
            ColorName::Red.into(),
        ));
        state.player_info.push(PlayerInfo::new(
            (0, 0),
            (1, 2),
            (Gem::amethyst, Gem::raw_citrine),
            ColorName::Yellow.into(),
        ));
        assert_eq!(state.player_info[0].position, (0, 0));
        assert_eq!(state.player_info[1].position, (1, 2));

        // Only player 1 is in the sliding column so it should move
        state.slide_players(&Slide::new(0, South));

        assert_eq!(state.player_info[0].position, (0, 1));
        assert_eq!(state.player_info[1].position, (1, 2));

        // Only player 2 is in the sliding row so it should move
        state.slide_players(&Slide::new(2, East));

        assert_eq!(state.player_info[0].position, (0, 1));
        assert_eq!(state.player_info[1].position, (2, 2));

        // Only player 1 is in the sliding column so it should move
        // but it should also wrap
        state.slide_players(&Slide::new(0, North));
        state.slide_players(&Slide::new(0, North));

        assert_eq!(state.player_info[0].position, (0, 6));
        assert_eq!(state.player_info[1].position, (2, 2));

        // Only player 2 is in the sliding row so it should move
        // but it should also wrap
        state.slide_players(&Slide::new(2, West));
        state.slide_players(&Slide::new(2, West));
        state.slide_players(&Slide::new(2, West));

        assert_eq!(state.player_info[0].position, (0, 6));
        assert_eq!(state.player_info[1].position, (6, 2));
    }

    #[test]
    fn test_rotate_spare() {
        let mut state = State::default();

        assert_eq!(state.board.extra.connector, Crossroads);
        state.rotate_spare(1);
        assert_eq!(state.board.extra.connector, Crossroads);

        let res = state.slide_and_insert(Slide::new(0, North));
        assert!(res.is_ok());

        assert_eq!(state.board.extra.connector, Path(Horizontal));
        state.rotate_spare(1);
        assert_eq!(state.board.extra.connector, Path(Vertical));

        let res = state.slide_and_insert(Slide::new(0, North));
        assert!(res.is_ok());

        assert_eq!(state.board.extra.connector, Fork(East));
        state.rotate_spare(1);
        assert_eq!(state.board.extra.connector, Fork(South));
        state.rotate_spare(3);
        assert_eq!(state.board.extra.connector, Fork(East));
        state.rotate_spare(8);
        assert_eq!(state.board.extra.connector, Fork(East));
    }

    #[test]
    fn test_can_reach_position() {
        let mut state = State::default();
        state.player_info.push(PlayerInfo {
            home: (1, 1),
            position: (1, 1),
            goal: (Gem::ametrine, Gem::purple_cabochon).into(),
            color: ColorName::Yellow.into(),
        });
        state.player_info.push(PlayerInfo {
            home: (3, 1),
            position: (1, 3),
            goal: (Gem::diamond, Gem::raw_beryl).into(),
            color: ColorName::Red.into(),
        });
        state.active_player = 0;

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

        let res = state.slide_and_insert(Slide::new(0, North));
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

        let res = state.slide_and_insert(Slide::new(2, South));
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
        state.player_info.push(PlayerInfo {
            home: (1, 1),
            position: (1, 1),
            goal: (Gem::ametrine, Gem::purple_cabochon).into(),
            color: ColorName::Green.into(),
        });
        state.player_info.push(PlayerInfo {
            home: (3, 1),
            position: (1, 3),
            goal: (Gem::diamond, Gem::raw_beryl).into(),
            color: ColorName::Red.into(),
        });
        state.player_info.push(PlayerInfo {
            home: (5, 1),
            position: (3, 6),
            goal: (Gem::hackmanite, Gem::iolite_emerald_cut).into(),
            color: ColorName::Purple.into(),
        });
        state.active_player = 0;
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

        assert!(state.slide_and_insert(Slide::new(0, West)).is_ok());

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
    fn test_move_player() {
        let mut state = State::default();
        state.player_info.push(PlayerInfo {
            home: (1, 1),
            position: (1, 1),
            goal: (Gem::ametrine, Gem::peridot).into(),
            color: ColorName::Yellow.into(),
        });
        state.player_info.push(PlayerInfo {
            home: (3, 1),
            position: (3, 1),
            goal: (Gem::diamond, Gem::clinohumite).into(),
            color: ColorName::Red.into(),
        });
        state.player_info.push(PlayerInfo {
            home: (5, 1),
            position: (0, 4),
            goal: (Gem::zircon, Gem::gray_agate).into(),
            color: ColorName::Blue.into(),
        });
        state.active_player = 0;

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
        assert_eq!(state.player_info[state.active_player].position, (2, 1));
        // try to move the player to the right
        // should error and not update the player's position
        assert!(state.move_player((4, 1)).is_err());
        assert_eq!(state.player_info[state.active_player].position, (2, 1));
        // try to move the player to its current position
        // should error and not update the player's position
        assert!(state.move_player((2, 1)).is_err());
        assert_eq!(state.player_info[state.active_player].position, (2, 1));
        // set active player to Red player
        // Red player can go right to (4, 1)
        state.next_player();
        assert!(state.move_player((4, 1)).is_ok());
        assert_eq!(state.player_info[state.active_player].position, (4, 1));
        // try and go left to where Yellow player is, should error
        assert!(state.move_player((2, 1)).is_err());
        assert_eq!(state.player_info[state.active_player].position, (4, 1));
        // set active player to the Blue player
        state.next_player();
        // tests for moving multiple tiles at a time
        assert!(state.move_player((1, 2)).is_ok());
        assert_eq!(state.player_info[state.active_player].position, (1, 2));
        assert!(state.move_player((1, 3)).is_ok());
        assert_eq!(state.player_info[state.active_player].position, (1, 3));
    }

    #[test]
    fn test_player_reached_home() {
        // home tile is not on the same connected component as active player
        let mut state = State::default();
        state.player_info.push(PlayerInfo {
            home: (1, 1),
            position: (2, 3),
            goal: (Gem::beryl, Gem::chrysolite).into(),
            color: ColorName::Blue.into(),
        });
        state.active_player = 0;
        assert!(!state.player_reached_home());

        // player is on the same connected component, but not on their home tile
        let mut state = State::default();
        state.player_info.push(PlayerInfo {
            home: (1, 1),
            position: (0, 1),
            goal: (Gem::kunzite_oval, Gem::pink_round).into(),
            color: ColorName::Red.into(),
        });
        state.active_player = 0;
        assert!(!state.player_reached_home());

        // active player is not on a home tile, but another player is
        let mut state = State::default();
        state.player_info.push(PlayerInfo {
            home: (1, 1),
            position: (2, 3),
            goal: (Gem::beryl, Gem::prasiolite).into(),
            color: ColorName::Green.into(),
        });
        state.player_info.push(PlayerInfo {
            home: (3, 1),
            position: (3, 1),
            goal: (Gem::diamond, Gem::red_diamond).into(),
            color: ColorName::Blue.into(),
        });
        state.active_player = 0;
        assert!(!state.player_reached_home());
        state.active_player = 1;
        assert!(state.player_reached_home());
    }

    #[test]
    fn test_player_reached_goal() {
        // Current Implementation of the Default board has Garnets and Amethysts in every Tile
        let mut state = State::default();
        state.player_info.push(PlayerInfo {
            home: (1, 1),
            position: (2, 3),
            goal: (Gem::beryl, Gem::moss_agate).into(),
            color: ColorName::Red.into(),
        });
        state.active_player = 0;
        assert!(!state.player_reached_goal());

        let mut state = State::default();
        state.player_info.push(PlayerInfo {
            home: (1, 1),
            position: (2, 3),
            goal: state.board[(2, 3)].gems,
            color: ColorName::Green.into(),
        });
        state.active_player = 0;
        assert!(state.player_reached_goal());
    }
}
