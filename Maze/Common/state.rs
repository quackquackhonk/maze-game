use crate::board::Board;
use crate::gem::Gem;
use crate::grid::Position;
use crate::tile::Tile;
use std::collections::HashMap;

use self::board::Slide;

pub mod board;
pub mod gem;
mod grid;
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
struct State {
    board: Board<BOARD_SIZE>,
    player_info: HashMap<i32, Player>,
    spare: Option<Tile>,
}

const BOARD_SIZE: usize = 7;

impl State {
    fn new() -> Self {
        State {
            board: Board::default(),
            player_info: HashMap::new(),
            spare: None,
        }
    }

    ///Rotates the spare `Tile` in the `board` by a given number of 90 degree turns
    pub fn rotate_spare(&mut self, num_turns: i32) {
        if let Some(spare) = &mut self.spare {
            (0..num_turns).for_each(|_| spare.rotate());
        }
    }

    /// makes a move
    pub fn make_move(&mut self, slide: Slide<7>) {
        let new_spare = self.board.slide(slide).ok();
        use tile::CompassDirection::*;
        self.board.insert(self.spare.take().unwrap());
        match slide {
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
        self.spare = new_spare;
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

    /// Removes the currently active `Player` from game.
    pub fn remove_player(&mut self, to_remove: i32) {
        self.player_info.remove(&to_remove);
    }
}
