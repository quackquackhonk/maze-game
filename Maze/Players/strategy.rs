use std::cmp::Ordering;

use common::board::Board;
use common::{board::Slide, grid::Position, BOARD_SIZE};

pub struct PlayerBoardState {
    board: Board<BOARD_SIZE>,
    player_positions: Vec<Position>,
}

pub trait Strategy {
    fn get_move(&self, board_state: PlayerBoardState) -> PlayerMove;
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

enum NaiveStrategy {
    Euclid,
    Reimann,
}

impl NaiveStrategy {
    fn find_destination(&self, board_state: &mut PlayerBoardState) -> Option<Position> {
        let find_match = match self {
            Self::Euclid => |p1: &Position, p2: &Position| -> Ordering {
                todo!();
            },
            Self::Reimann => |p1: &Position, p2: &Position| -> Ordering {
                todo!();
            },
        };
        todo!();
    }

    fn find_move_to_reach(&self, destination: Position) -> PlayerMove {
        todo!();
    }
}

impl Strategy for NaiveStrategy {
    fn get_move(&self, mut board_state: PlayerBoardState) -> PlayerMove {
        match self.find_destination(&mut board_state) {
            Some(pos) => self.find_move_to_reach(pos),
            None => PlayerMove::Pass,
        }
    }
}
