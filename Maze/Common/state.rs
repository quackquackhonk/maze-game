use std::collections::HashMap;

use board::Board;
use board::Slide;
use gem::Gem;
use grid::Position;
use tile::Tile;

/// Contains all the types needed for the Board State and mutating the `Board`
pub mod board;
/// Contains the enum including all the possible Gems
pub mod gem;
/// Contains types for the `Grid` type and its `Position` type for indexing
pub mod grid;
/// Contains the Tile type for use in the `Board`
pub mod tile;

/// Represents a Player and the `Position` of their home and themselves. Also holds their goal `Gem`.
struct Player {
    home: Position,
    pub position: Position,
    goal: Gem,
}

impl Player {
    /// Constructs a new `Player` from its fields.
    fn new(home: Position, position: Position, goal: Gem) -> Self {
        Self {
            home,
            position,
            goal,
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

/// Represents the State of a single MazeGame.
pub struct State {
    board: Board<BOARD_SIZE>,
    player_info: HashMap<i32, Player>,
    spare: Option<Tile>,
}

const BOARD_SIZE: usize = 7;

impl State {
    pub fn new() -> Self {
        State {
            board: Board::default(),
            player_info: HashMap::new(),
            spare: None,
        }
    }

    /// Rotates the spare `Tile` in the `board` by a given number of 90 degree turns
    ///
    /// Does nothing if we do not currently have a spare tile
    pub fn rotate_spare(&mut self, num_turns: i32) {
        if let Some(spare) = &mut self.spare {
            (0..num_turns).for_each(|_| spare.rotate());
        }
    }

    fn slide_players(&mut self, slide: &Slide<BOARD_SIZE>) {
        use tile::CompassDirection::*;
        match *slide {
            Slide {
                index: column,
                direction: North,
            } => self
                .player_info
                .values_mut()
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
                .values_mut()
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
                .values_mut()
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
                .values_mut()
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

    /// Performs a slide action
    pub fn slide(&mut self, slide: Slide<7>) {
        self.spare = self.board.slide(slide).ok();
        self.slide_players(&slide);
    }

    /// Inserts the tile that was slid off
    ///
    /// Does nothing if there is not a spare tile
    pub fn insert(&mut self) {
        if let Some(spare) = self.spare.take() {
            self.board.insert(spare);
        }
    }

    /// Determines if the currently active `Player` can reach the `Tile` at the given `Position`
    pub fn can_reach_position(&self, active_player: i32, target: Position) -> bool {
        self.board
            .reachable(self.player_info[&active_player].position)
            .unwrap()
            .contains(&target)
    }

    /// Checks if the currently active `Player` has landed on its goal tile
    pub fn player_reached_goal(&self, active_player: i32) -> bool {
        let player_info = &self.player_info[&active_player];
        let gem_at_player = self.board[player_info.position].as_ref().unwrap().gems;
        player_info.reached_goal(gem_at_player.0) && player_info.reached_goal(gem_at_player.1)
    }

    pub fn player_reached_home(&self, active_player: i32) -> bool {
        let player_info = &self.player_info[&active_player];
        player_info.reached_home()
    }

    /// Removes the currently active `Player` from game.
    pub fn remove_player(&mut self, to_remove: i32) {
        self.player_info.remove(&to_remove);
    }
}
