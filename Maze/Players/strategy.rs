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
/// depends on the `NaiveStrategy` type.
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

    /// Returns a `Vec<Position>` containing alternative goals to try and reach
    /// sorted by how desireable they are according to their algorithm.
    /// - `NaiveStrategy::Euclid` sorts alt goals by ascending `euclidian_distance` to the
    /// `goal_tile`
    /// - `NaiveStrategy::Reimann` sorts alt goals in row-column order.
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

    /// After sliding the row specified by `slide` and inserting the spare tile after rotating it
    /// `rotations` times, can the player go from `start` to `destination`
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
        let start = slide.move_position(start);
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

#[cfg(test)]
mod StrategyTests {
    use super::*;
    use CompassDirection::*;

    #[test]
    fn test_get_move_euclid() {
        let pbs = PlayerBoardState {
            board: Board::default(),
            player_positions: vec![(1, 1), (2, 2)],
        };
        let euclid = NaiveStrategy::Euclid;
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

        // can Euclid go from (1, 1) to (1, 3)?
        let euclid_move = euclid.get_move(pbs, (1, 1), (1, 3));
        assert!(euclid_move.is_some());
        let euclid_move = euclid_move.unwrap();
        // slides row 2 east, inserts crossroads, goes to (1, 3)
        assert_eq!(euclid_move.destination, (1, 3));
        assert_eq!(euclid_move.rotations, 0);
        assert_eq!(euclid_move.slide, Slide::new(1, East).unwrap());
    }

    #[test]
    fn test_reachable_after_move() {
        let pbs = PlayerBoardState {
            board: Board::default(),
            player_positions: vec![(0, 0), (2, 2)],
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
        assert_eq!(pbs.board.reachable((0, 0)).unwrap(), vec![(0, 0)]);
        // slides the top row right, moves player to (1, 1)
        let player_move = PlayerMove {
            slide: Slide::new(0, East).unwrap(),
            rotations: 0,
            destination: (2, 2),
        };
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

        // can the player go from (0, 0) to (2, 2) after making the move?
        assert!(NaiveStrategy::reachable_after_move(
            &pbs,
            player_move,
            (0, 0)
        ));

        // slide the bottom row left
        let player_move = PlayerMove {
            slide: Slide::new(3, West).unwrap(),
            rotations: 0,
            destination: (1, 5),
        };
        // starting at (2, 6) you can go to (1, 5)
        assert!(pbs.board.reachable((2, 6)).unwrap().contains(&(1, 5)));
        // If you start at (2, 6) can you go to (1, 5) after making move? no
        assert!(!NaiveStrategy::reachable_after_move(
            &pbs,
            player_move,
            (2, 6)
        ));
    }
}
