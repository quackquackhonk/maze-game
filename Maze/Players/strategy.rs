#![allow(dead_code)]
use std::cmp::Ordering;

use common::board::Board;
use common::tile::CompassDirection;
use common::{board::Slide, grid::Position, BOARD_SIZE};

/// This type represents the data a player recieves from the Referee about the Game State
#[derive(Debug, Clone)]
pub struct PlayerBoardState {
    board: Board<BOARD_SIZE>,
    player_positions: Vec<Position>,
}

/// This trait represents getting a valid move from a given board state
pub trait Strategy {
    /// This returns a valid move given the game state
    fn get_move(
        &self,
        board_state: PlayerBoardState,
        start: Position,
        goal_tile: Position,
    ) -> PlayerAction;
}

/// This type represents a possible player action  
/// `None` -> A pass  
/// `Some(PlayerMove)` -> A move  
pub type PlayerAction = Option<PlayerMove>;

/// This type represents all the data needed to execute a move
///
/// # Warning
/// This type does not self-validate because it has no knowledge of the board it will be played on.
#[derive(Debug, Clone, Copy)]
pub struct PlayerMove {
    pub slide: Slide<BOARD_SIZE>,
    pub rotations: usize,
    pub destination: Position,
}

#[derive(Debug)]
/// Implements a strategy that after failing to find a move directly to the goal tile, checks
/// every other board position as a location to move. The order in which it checks every location
/// depends on the `NativeStrategy` type.
pub enum NaiveStrategy {
    /// This variant sorts the posssible alternative goals in order of smallest to largest
    /// euclidian distance. It breaks any ties by picking the first one in row-column order.
    Euclid,
    /// This variant sorts the posssible alternative goals in order of row-column order.
    Reimann,
}

fn row_col_order(p1: &Position, p2: &Position) -> Ordering {
    if p1.1 < p2.1 {
        Ordering::Less
    } else if p1.1 > p2.1 {
        Ordering::Greater
    } else if p1.0 < p2.0 {
        Ordering::Less
    } else if p1.0 > p2.0 {
        Ordering::Greater
    } else {
        Ordering::Equal
    }
}

fn euclidian_distance(p1: &Position, p2: &Position) -> f32 {
    f32::sqrt((p1.0 as f32 - p2.0 as f32).powi(2) + (p1.1 as f32 - p2.1 as f32).powi(2))
}

impl NaiveStrategy {
    /// This function creates a list of possible goals and orders them according to the strategy
    /// and returns a player action with the move if it found one or a pass if it couldn't
    fn find_move_to_reach_alt_goal(
        &self,
        board_state: &PlayerBoardState,
        start: Position,
        goal_tile: Position,
    ) -> PlayerAction {
        self.get_alt_goals(goal_tile)
            .into_iter()
            .find_map(|goal| self.find_move_to_reach(board_state, start, goal))
    }

    fn get_alt_goals(&self, goal_tile: Position) -> Vec<Position> {
        //! alternative_goal_order is a Comparator<Position> function.
        #[allow(clippy::type_complexity)]
        let alternative_goal_order: Box<dyn Fn(&Position, &Position) -> Ordering> = match self {
            Self::Euclid => Box::new(|p1: &Position, p2: &Position| -> Ordering {
                let euclid1 = euclidian_distance(p1, &goal_tile);
                let euclid2 = euclidian_distance(p2, &goal_tile);
                if euclid1 < euclid2 {
                    Ordering::Less
                } else if euclid1 > euclid2 {
                    Ordering::Greater
                } else {
                    row_col_order(p1, p2)
                }
            }),
            Self::Reimann => Box::new(row_col_order),
        };

        let mut possible_goals: Vec<Position> = (0..BOARD_SIZE).zip(0..BOARD_SIZE).collect();
        possible_goals.sort_by(alternative_goal_order);
        possible_goals
    }

    fn reachable_after_move(
        board_state: &PlayerBoardState,
        PlayerMove {
            slide,
            rotations,
            destination,
        }: PlayerMove,
        start: Position,
    ) -> bool {
        let mut board_state = board_state.clone();
        (0..rotations).for_each(|_| board_state.board.rotate_spare());
        board_state.board.slide_and_insert(slide);
        board_state
            .board
            .reachable(start)
            .expect("Start must be in bounds")
            .contains(&destination)
    }

    fn find_move_to_reach(
        &self,
        board_state: &PlayerBoardState,
        start: Position,
        destination: Position,
    ) -> PlayerAction {
        for row in 0..(BOARD_SIZE / 2) {
            for direction in [CompassDirection::West, CompassDirection::East] {
                for rotations in 0..4 {
                    let slide = Slide::new(row, direction)
                        .expect("The range 0 to BOARD_SIZE/2 is always in bounds");
                    let player_move = PlayerMove {
                        slide,
                        rotations,
                        destination,
                    };
                    if NaiveStrategy::reachable_after_move(board_state, player_move, start) {
                        return Some(player_move);
                    }
                }
            }
        }
        for col in 0..(BOARD_SIZE / 2) {
            for direction in [CompassDirection::North, CompassDirection::South] {
                for rotations in 0..4 {
                    let slide = Slide::new(col, direction)
                        .expect("The range 0 to BOARD_SIZE/2 is always in bounds");
                    let player_move = PlayerMove {
                        slide,
                        rotations,
                        destination,
                    };
                    if NaiveStrategy::reachable_after_move(board_state, player_move, start) {
                        return Some(player_move);
                    }
                }
            }
        }
        None
    }
}

impl Strategy for NaiveStrategy {
    fn get_move(
        &self,
        board_state: PlayerBoardState,
        start: Position,
        goal_tile: Position,
    ) -> PlayerAction {
        self.find_move_to_reach(&board_state, start, goal_tile)
            .or_else(|| self.find_move_to_reach_alt_goal(&board_state, start, goal_tile))
    }
}
