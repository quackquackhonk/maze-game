use board::Board;
use board::BoardResult;
use board::Slide;
use gem::Gem;
use grid::Position;

/// Contains all the types needed for the Board State and mutating the `Board`
pub mod board;
/// Contains the enum including all the possible Gems
pub mod gem;
/// Contains types for the `Grid` type and its `Position` type for indexing
pub mod grid;
/// Contains the Tile type for use in the `Board`
pub mod tile;

#[non_exhaustive]
#[derive(Debug, PartialEq, Eq)]
pub enum Color {
    Red,
    Yellow,
    Green,
    Blue,
}

/// Represents a Player and the `Position` of their home and themselves. Also holds their goal `Gem`.
#[derive(Debug, PartialEq, Eq)]
pub struct PlayerInfo {
    home: Position,
    pub(crate) position: Position,
    goal: Gem,
    // Invariant: Every Player should have their own color
    color: Color,
}

impl PlayerInfo {
    /// Constructs a new `Player` from its fields.
    pub fn new(home: Position, position: Position, goal: Gem, color: Color) -> Self {
        Self {
            home,
            position,
            goal,
            color,
        }
    }

    /// Is the given `Gem` this `Player`'s goal?
    fn reached_goal(&self, gem: Gem) -> bool {
        self.goal == gem
    }

    /// Has this `Player` reached their home?
    fn reached_home(&self) -> bool {
        self.home == self.position
    }
}

/// Represents the State of a single Maze Game.
#[derive(Default)]
pub struct State {
    board: Board<BOARD_SIZE>,
    player_info: Vec<PlayerInfo>,
    active_player: usize,
    previous_slide: Option<Slide<BOARD_SIZE>>,
}

const BOARD_SIZE: usize = 7;

impl State {
    /// Rotates the spare `Tile` in the `board` by a given number of 90 degree turns
    ///
    /// Does nothing if we do not currently have a spare tile
    pub fn rotate_spare(&mut self, num_turns: i32) {
        // The modulo operator saves us from doing extraneous turns
        (0..num_turns % 4).for_each(|_| self.board.rotate_spare());
    }

    fn slide_players(&mut self, &slide: &Slide<BOARD_SIZE>) {
        #[allow(clippy::enum_glob_use)]
        use tile::CompassDirection::*;
        match slide {
            Slide {
                index: column,
                direction: North,
            } => self
                .player_info
                .iter_mut()
                .filter(|player| player.position.0 == column)
                .for_each(|player| {
                    if player.position.1 == 0 {
                        player.position.1 = BOARD_SIZE - 1;
                    } else {
                        player.position.1 -= 1;
                    }
                }),
            Slide {
                index: column,
                direction: South,
            } => self
                .player_info
                .iter_mut()
                .filter(|player| player.position.0 == column)
                .for_each(|player| {
                    if player.position.1 == BOARD_SIZE - 1 {
                        player.position.1 = 0;
                    } else {
                        player.position.1 += 1;
                    }
                }),
            Slide {
                index: row,
                direction: East,
            } => self
                .player_info
                .iter_mut()
                .filter(|player| player.position.1 == row)
                .for_each(|player| {
                    if player.position.0 == BOARD_SIZE - 1 {
                        player.position.0 = 0;
                    } else {
                        player.position.0 += 1;
                    }
                }),
            Slide {
                index: row,
                direction: West,
            } => self
                .player_info
                .iter_mut()
                .filter(|player| player.position.1 == row)
                .for_each(|player| {
                    if player.position.0 == 0 {
                        player.position.0 = BOARD_SIZE - 1;
                    } else {
                        player.position.0 -= 1;
                    }
                }),
        };
    }

    /// Performs a slide and insert action
    pub fn slide_and_insert(&mut self, slide: Slide<7>) -> BoardResult<()> {
        if let Some(prev) = self.previous_slide {
            if prev.direction.opposite() == slide.direction && prev.index == slide.index {
                // Kicking player out code can go here
                Err("Attempted to do a slide action that would undo the previous slide")?;
            }
        }
        self.board.slide_and_insert(slide);
        self.slide_players(&slide);
        self.previous_slide = Some(slide);
        Ok(())
    }

    /// Determines if the currently active `Player` can reach the `Tile` at the given `Position`
    #[must_use]
    pub fn can_reach_position(&self, target: Position) -> bool {
        self.board
            .reachable(self.player_info[self.active_player].position)
            .expect("Active player positions are always in bounds")
            .contains(&target)
    }

    /// Checks if the currently active `Player` has landed on its goal tile
    #[must_use]
    pub fn player_reached_goal(&self) -> bool {
        let player_info = &self.player_info[self.active_player];
        let gem_at_player = self.board[player_info.position].gems;
        player_info.reached_goal(gem_at_player.0) || player_info.reached_goal(gem_at_player.1)
    }

    /// Checks if the currently active `Player` has landed on its home tile
    #[must_use]
    pub fn player_reached_home(&self) -> bool {
        let player_info = &self.player_info[self.active_player];
        player_info.reached_home()
    }

    /// Adds a `Player` to the end of the list of currently active players
    pub fn add_player(&mut self, to_add: PlayerInfo) {
        self.player_info.push(to_add);
    }

    /// Sets `self.active_player` to be the next player by indexing `self.player_info`
    pub fn next_player(&mut self) {
        if !self.player_info.is_empty() {
            self.active_player = (self.active_player + 1) % self.player_info.len();
        }
    }

    /// Removes the currently active `Player` from game.
    pub fn remove_player(&mut self) {
        if !self.player_info.is_empty() {
            self.player_info.remove(self.active_player);
        }
    }
}

#[cfg(test)]
mod tests {
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
            goal: Gem::ruby,
            color: Color::Red,
        });

        assert!(!state.player_info.is_empty());

        assert_eq!(state.player_info.len(), 1);

        state.add_player(PlayerInfo::new(
            (0, 1),
            (1, 0),
            Gem::blue_cushion,
            Color::Blue,
        ));

        assert_eq!(state.player_info.len(), 2);
    }

    #[test]
    fn test_remove_player() {
        let mut state = State::default();
        state.add_player(PlayerInfo::new((0, 0), (0, 0), Gem::ruby, Color::Green));

        assert_eq!(state.player_info.len(), 1);
        // Should not panic because the player exists in the HashMap
        state.remove_player();

        assert_eq!(state.player_info.len(), 0);
        // Should not panic because `remove_player` ignores if players are actually in the game
        state.remove_player();
        assert_eq!(state.player_info.len(), 0);
    }

    #[test]
    fn test_next_player() {
        let mut state = State::default();
        assert_eq!(state.active_player, 0);
        state.next_player();
        assert_eq!(state.active_player, 0);

        state.add_player(PlayerInfo::new((0, 0), (0, 0), Gem::ruby, Color::Red));
        assert_eq!(state.active_player, 0);
        state.next_player();
        assert_eq!(state.active_player, 0);

        state.add_player(PlayerInfo::new((0, 0), (0, 0), Gem::ruby, Color::Green));
        assert_eq!(state.active_player, 0);
        state.next_player();
        assert_eq!(state.active_player, 1);
        state.next_player();
        assert_eq!(state.active_player, 0);

        state.add_player(PlayerInfo::new((0, 0), (0, 0), Gem::ruby, Color::Yellow));
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

        let res = state.slide_and_insert(Slide::new(0, North).unwrap());
        assert!(res.is_ok());

        // Sliding without inserting will not do anything
        let res = state.slide_and_insert(Slide::new(0, South).unwrap());
        assert!(res.is_err());
    }

    #[test]
    fn test_slide_no_undo() {
        let mut state = State::default();

        let res = state.slide_and_insert(Slide::new(0, North).unwrap());
        assert!(res.is_ok());

        let res = state.slide_and_insert(Slide::new(0, South).unwrap());
        assert!(res.is_err());

        // Doing it twice should not matter
        let res = state.slide_and_insert(Slide::new(0, South).unwrap());
        assert!(res.is_err());

        // Doing it in another index is fine
        let res = state.slide_and_insert(Slide::new(1, South).unwrap());
        assert!(res.is_ok());
    }

    #[test]
    fn test_slide_players() {
        let mut state = State::default();
        state
            .player_info
            .push(PlayerInfo::new((0, 0), (0, 0), Gem::ruby, Color::Red));
        state.player_info.push(PlayerInfo::new(
            (0, 0),
            (1, 2),
            Gem::amethyst,
            Color::Yellow,
        ));
        assert_eq!(state.player_info[0].position, (0, 0));
        assert_eq!(state.player_info[1].position, (1, 2));

        // Only player 1 is in the sliding column so it should move
        state.slide_players(&Slide::new(0, South).unwrap());

        assert_eq!(state.player_info[0].position, (0, 1));
        assert_eq!(state.player_info[1].position, (1, 2));

        // Only player 2 is in the sliding row so it should move
        state.slide_players(&Slide::new(1, East).unwrap());

        assert_eq!(state.player_info[0].position, (0, 1));
        assert_eq!(state.player_info[1].position, (2, 2));

        // Only player 1 is in the sliding column so it should move
        // but it should also wrap
        state.slide_players(&Slide::new(0, North).unwrap());
        state.slide_players(&Slide::new(0, North).unwrap());

        assert_eq!(state.player_info[0].position, (0, 6));
        assert_eq!(state.player_info[1].position, (2, 2));

        // Only player 2 is in the sliding row so it should move
        // but it should also wrap
        state.slide_players(&Slide::new(1, West).unwrap());
        state.slide_players(&Slide::new(1, West).unwrap());
        state.slide_players(&Slide::new(1, West).unwrap());

        assert_eq!(state.player_info[0].position, (0, 6));
        assert_eq!(state.player_info[1].position, (6, 2));
    }

    #[test]
    fn test_rotate_spare() {
        let mut state = State::default();

        assert_eq!(state.board.extra.connector, Crossroads);
        state.rotate_spare(1);
        assert_eq!(state.board.extra.connector, Crossroads);

        let res = state.slide_and_insert(Slide::new(0, North).unwrap());
        assert!(res.is_ok());

        assert_eq!(state.board.extra.connector, Path(Horizontal));
        state.rotate_spare(1);
        assert_eq!(state.board.extra.connector, Path(Vertical));

        let res = state.slide_and_insert(Slide::new(0, North).unwrap());
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
            goal: Gem::ametrine,
            color: Color::Yellow,
        });
        state.player_info.push(PlayerInfo {
            home: (3, 1),
            position: (1, 3),
            goal: Gem::diamond,
            color: Color::Red,
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

        let res = state.slide_and_insert(Slide::new(0, North).unwrap());
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

        let res = state.slide_and_insert(Slide::new(1, South).unwrap());
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
    fn test_player_reached_home() {
        // home tile is not on the same connected component as active player
        let mut state = State::default();
        state.player_info.push(PlayerInfo {
            home: (1, 1),
            position: (2, 3),
            goal: Gem::beryl,
            color: Color::Blue,
        });
        state.active_player = 0;
        assert!(!state.player_reached_home());

        // player is on the same connected component, but not on their home tile
        let mut state = State::default();
        state.player_info.push(PlayerInfo {
            home: (1, 1),
            position: (0, 1),
            goal: Gem::kunzite_oval,
            color: Color::Red,
        });
        state.active_player = 0;
        assert!(!state.player_reached_home());

        // active player is not on a home tile, but another player is
        let mut state = State::default();
        state.player_info.push(PlayerInfo {
            home: (1, 1),
            position: (2, 3),
            goal: Gem::beryl,
            color: Color::Green,
        });
        state.player_info.push(PlayerInfo {
            home: (3, 1),
            position: (3, 1),
            goal: Gem::diamond,
            color: Color::Blue,
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
            goal: Gem::beryl,
            color: Color::Red,
        });
        state.active_player = 0;
        assert!(!state.player_reached_goal());

        let mut state = State::default();
        state.player_info.push(PlayerInfo {
            home: (1, 1),
            position: (2, 3),
            goal: state.board[(2, 3)].gems.0,
            color: Color::Green,
        });
        state.active_player = 0;
        assert!(state.player_reached_goal());
    }
}
