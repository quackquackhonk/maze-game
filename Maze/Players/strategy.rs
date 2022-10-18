use common::{board::Slide, grid::Position, BOARD_SIZE};

pub trait Strategy {
    fn get_slide(&self) -> Slide<BOARD_SIZE>;
    fn get_rotations(&self) -> usize;
    fn get_avatar_destination(&self) -> Position;
    fn get_pass(&self) -> bool;

    fn get_move(&self) -> PlayerMove {
        if !self.get_pass() {
            PlayerMove::Move {
                slide: self.get_slide(),
                rotations: self.get_rotations(),
                destination: self.get_avatar_destination(),
            }
        } else {
            PlayerMove::Pass
        }
    }
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
