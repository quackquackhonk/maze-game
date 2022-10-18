#![allow(dead_code)]
use std::cmp::Ordering;

use common::board::Board;
use common::tile::CompassDirection;
use common::{board::Slide, grid::Position, BOARD_SIZE};

#[derive(Clone)]
pub struct PlayerBoardState {
    board: Board<BOARD_SIZE>,
    player_positions: Vec<Position>,
}

pub trait Strategy {
    fn get_move(
        &self,
        board_state: PlayerBoardState,
        start: Position,
        goal_tile: Position,
    ) -> PlayerMove;
}

#[allow(dead_code)]
pub enum PlayerMove {
    Pass,
    Move {
        slide: Slide<BOARD_SIZE>,
        rotations: usize,
        destination: Position,
    },
}

pub enum NaiveStrategy {
    Euclid,
    Reimann,
}

impl NaiveStrategy {
    fn find_move_to_reach_alt_goal(
        &self,
        board_state: &mut PlayerBoardState,
        start: Position,
    ) -> PlayerMove {
        let find_match = match self {
            Self::Euclid => |p1: &Position, p2: &Position| -> Ordering {
                todo!();
            },
            Self::Reimann => |p1: &Position, p2: &Position| -> Ordering {
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
            },
        };

        let mut possible_goals: Vec<Position> = (0..BOARD_SIZE).zip(0..BOARD_SIZE).collect();
        possible_goals.sort_by(find_match);
        for alt_goal in possible_goals {
            match self.find_move_to_reach(board_state, start, alt_goal) {
                PlayerMove::Pass => {}
                _move => return _move,
            }
        }
        PlayerMove::Pass
    }

    fn try_move(
        board_state: &mut PlayerBoardState,
        slide: Slide<BOARD_SIZE>,
        rotations: usize,
        start: Position,
        destination: Position,
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
        board_state: &mut PlayerBoardState,
        start: Position,
        destination: Position,
    ) -> PlayerMove {
        for row in 0..(BOARD_SIZE / 2) {
            for direction in [CompassDirection::West, CompassDirection::East] {
                for rotations in 0..4 {
                    let slide = Slide::new(row, direction)
                        .expect("The range 0 to BOARD_SIZE/2 is always in bounds");
                    let reachable =
                        NaiveStrategy::try_move(board_state, slide, rotations, start, destination);
                    if reachable {
                        return PlayerMove::Move {
                            slide,
                            rotations,
                            destination,
                        };
                    }
                }
            }
        }
        for col in 0..(BOARD_SIZE / 2) {
            for direction in [CompassDirection::North, CompassDirection::South] {
                for rotations in 0..4 {
                    let slide = Slide::new(col, direction)
                        .expect("The range 0 to BOARD_SIZE/2 is always in bounds");
                    let reachable =
                        NaiveStrategy::try_move(board_state, slide, rotations, start, destination);
                    if reachable {
                        return PlayerMove::Move {
                            slide,
                            rotations,
                            destination,
                        };
                    }
                }
            }
        }
        PlayerMove::Pass
    }
}

impl Strategy for NaiveStrategy {
    fn get_move(
        &self,
        mut board_state: PlayerBoardState,
        start: Position,
        goal_tile: Position,
    ) -> PlayerMove {
        match self.find_move_to_reach(&mut board_state, start, goal_tile) {
            PlayerMove::Pass => self.find_move_to_reach_alt_goal(&mut board_state, start),
            _move => _move,
        }
    }
}
