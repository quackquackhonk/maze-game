#![allow(dead_code)]
use std::cmp::Ordering;
use std::iter::repeat;

use common::board::Board;
use common::tile::CompassDirection;
use common::Color;
use common::{board::Slide, grid::Position};

/// This type represents the data a player recieves from the Referee about the Game State
#[derive(Debug, Clone)]
pub struct PlayerBoardState<const COLS: usize, const ROWS: usize> {
    pub board: Board<COLS, ROWS>,
    pub players: Vec<PubPlayerInfo>,
    pub last: Option<Slide<COLS, ROWS>>,
}

#[derive(Debug, Clone)]
pub struct PubPlayerInfo {
    pub current: Position,
    pub home: Position,
    pub color: Color,
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PlayerMove {
    pub slide: Slide,
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
            .filter(|pos| *pos != start)
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

        let mut possible_goals: Vec<Position> = (0..ROWS)
            .flat_map(|row| (0..COLS).zip(repeat(row)))
            .collect();
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
        let start = slide.move_position(
            start,
            board_state.board.grid[0].len(),
            board_state.board.grid.len(),
        );
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
        for row in 0..=(BOARD_SIZE / 2) {
            for direction in [CompassDirection::West, CompassDirection::East] {
                for rotations in 0..4 {
                    let slide = Slide::new(row * 2, direction)
                        .expect("The range 0 to ROWS/2 is always in bounds");
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
        for col in 0..=(COLS / 2) {
            for direction in [CompassDirection::North, CompassDirection::South] {
                for rotations in 0..4 {
                    let slide = Slide::new(col * 2, direction)
                        .expect("The range 0 to COLS/2 is always in bounds");
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
    use common::{ColorName, BOARD_SIZE};
    use CompassDirection::*;

    #[test]
    fn test_get_move_euclid() {
        let board_state: PlayerBoardState = PlayerBoardState {
            board: Board::default(),
            players: vec![
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
            ],
            last: None,
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

        // what will Euclid do to go from (1, 1) -> (1, 3)
        let euclid_move = euclid.get_move(board_state, (1, 1), (1, 3));
        assert!(euclid_move.is_some());
        let euclid_move = euclid_move.unwrap();
        // slides row 2 east, inserts crossroads, goes to (1, 3)
        assert_eq!(
            euclid_move,
            PlayerMove {
                slide: Slide::new(2, East).unwrap(),
                rotations: 0,
                destination: (1, 3),
            }
        );

        // what will Euclid do to go from (0, 0) to (2, 3)?
        let board_state = PlayerBoardState {
            board: Board::default(),
            players: vec![
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
            ],
            last: None,
        };
        let euclid_move = euclid.get_move(board_state, (0, 0), (2, 3));
        assert!(euclid_move.is_some());
        let euclid_move = euclid_move.unwrap();
        // slides the top row east, moves to (2, 2)
        assert_eq!(
            euclid_move,
            PlayerMove {
                slide: Slide::new(0, East).unwrap(),
                rotations: 0,
                destination: (2, 2),
            }
        );

        // what will Euclid do to go from (6, 4) to (2, 0)?
        let board_state = PlayerBoardState {
            board: Board::default(),
            players: vec![
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
            ],
            last: None,
        };
        let euclid_move = euclid.get_move(board_state, (6, 4), (2, 0));
        assert!(euclid_move.is_some());
        let euclid_move = euclid_move.unwrap();
        // slides row 4 east to wrap around to (0, 4) then move to (1, 2)
        assert_eq!(
            euclid_move,
            PlayerMove {
                slide: Slide::new(4, East).unwrap(),
                rotations: 0,
                destination: (1, 2),
            }
        );

        let board_state = PlayerBoardState {
            board: Board::default(),
            players: vec![
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
            ],
            last: None,
        };
        let mut any_passes = (0..BOARD_SIZE)
            .flat_map(|row| (0..BOARD_SIZE).zip(repeat(row)))
            .map(|dest| euclid.get_move(board_state.clone(), (0, 0), dest))
            .filter(|m| m.is_none());
        assert!(any_passes.next().is_none());
    }

    #[test]
    fn test_get_move_reimann() {
        let board_state: PlayerBoardState = PlayerBoardState {
            board: Board::default(),
            players: vec![
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
            ],
            last: None,
        };
        let reimann = NaiveStrategy::Reimann;
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

        // what will Reimann do to go from (1, 1) -> (1, 3)
        let reimann_move = reimann.get_move(board_state, (1, 1), (1, 3));
        assert!(reimann_move.is_some());
        let reimann_move = reimann_move.unwrap();
        // slides row 2 east, inserts crossroads, goes to (1, 3)
        assert_eq!(
            reimann_move,
            PlayerMove {
                slide: Slide::new(2, East).unwrap(),
                rotations: 0,
                destination: (1, 3),
            }
        );

        // what will Reimann do to go from (0, 0) to (2, 3)?
        let board_state = PlayerBoardState {
            board: Board::default(),
            players: vec![
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
            ],
            last: None,
        };
        let reimann_move = reimann.get_move(board_state, (0, 0), (2, 3));
        assert!(reimann_move.is_some());
        let reimann_move = reimann_move.unwrap();
        // slides the top row east, moves to (1, 0)
        assert_eq!(
            reimann_move,
            PlayerMove {
                slide: Slide::new(0, East).unwrap(),
                rotations: 0,
                destination: (1, 0),
            }
        );

        // what will Reimann do to go from (6, 4) to (2, 0)?
        let board_state = PlayerBoardState {
            board: Board::default(),
            players: vec![
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
            ],
            last: None,
        };
        let reimann_move = reimann.get_move(board_state, (6, 4), (2, 0));
        assert!(reimann_move.is_some());
        let reimann_move = reimann_move.unwrap();
        // slides row 4 east to wrap around to (0, 4) then move to (0, 2)
        assert_eq!(
            reimann_move,
            PlayerMove {
                slide: Slide::new(4, East).unwrap(),
                rotations: 0,
                destination: (0, 2),
            }
        );
    }

    #[test]
    fn test_find_move_to_reach_alt_goal() {
        let board_state: PlayerBoardState = PlayerBoardState {
            board: Board::default(),
            players: vec![
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
            ],
            last: None,
        };
        let euclid = NaiveStrategy::Euclid;
        let reimann = NaiveStrategy::Reimann;
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

        // if Euclid is on (0, 2) and its goal is (0, 0), it will slide the leftmost column North
        // and then move to (0, 1)
        let euc_move = euclid.find_move_to_reach_alt_goal(&board_state, (0, 2), (0, 0));
        assert_eq!(
            euc_move,
            Some(PlayerMove {
                slide: Slide::new(0, North).unwrap(),
                rotations: 0,
                destination: (0, 1)
            })
        );
        // With the same conditions, reimann is going to make the same move
        let rei_move = reimann.find_move_to_reach_alt_goal(&board_state, (0, 2), (0, 0));
        assert_eq!(
            rei_move,
            Some(PlayerMove {
                slide: Slide::new(0, North).unwrap(),
                rotations: 0,
                destination: (0, 1)
            })
        );
        // what does Euclid do if on (3, 3) and its goal is (3, 2)?
        // Euclid will Slide the 2nd row West, and then move up to (3, 2) to avoid staying in place
        let euc_move = euclid.find_move_to_reach_alt_goal(&board_state, (3, 3), (2, 3));
        assert_eq!(
            euc_move,
            Some(PlayerMove {
                slide: Slide::new(2, West).unwrap(),
                rotations: 0,
                destination: (3, 2)
            })
        );
        // Reimann will make the same slide but will move all the way up to (3, 0)
        let rei_move = reimann.find_move_to_reach_alt_goal(&board_state, (3, 3), (2, 3));
        assert_eq!(
            rei_move,
            Some(PlayerMove {
                slide: Slide::new(2, West).unwrap(),
                rotations: 0,
                destination: (3, 0)
            })
        );

        // What if you start on (6, 6) and your goal is (0, 5)
        // Euclid will slide the bottom row east and move to (1,5)
        let euc_move = euclid.find_move_to_reach_alt_goal(&board_state, (6, 6), (0, 5));
        assert_eq!(
            euc_move,
            Some(PlayerMove {
                slide: Slide::new(6, East).unwrap(),
                rotations: 0,
                destination: (1, 5)
            })
        );
        // Reimann will slide the last column down and move to (6, 1)
        let rei_move = reimann.find_move_to_reach_alt_goal(&board_state, (6, 6), (0, 5));
        assert_eq!(
            rei_move,
            Some(PlayerMove {
                slide: Slide::new(6, South).unwrap(),
                rotations: 0,
                destination: (6, 0)
            })
        );
    }

    #[test]
    fn test_euclidian_distance() {
        let p_0_0 = (0, 0);
        let p_0_3 = (0, 3);
        let p_3_3 = (3, 3);
        let p_6_3 = (6, 3);
        let p_6_6 = (6, 6);
        assert_eq!(euclidian_distance(&p_0_3, &p_0_3), 0.0);
        assert_eq!(euclidian_distance(&p_0_3, &p_3_3), 3.0);
        assert_eq!(euclidian_distance(&p_0_3, &p_6_3), 6.0);
        assert_eq!(euclidian_distance(&p_0_0, &p_6_6), f32::sqrt(72.0));
    }

    #[test]
    fn test_get_alt_goals_reimann() {
        let reimann_alt_goals = NaiveStrategy::Reimann.get_alt_goals((1, 1));
        assert_eq!(reimann_alt_goals.len(), BOARD_SIZE.pow(2));
        assert_eq!(reimann_alt_goals[0], (0, 0));
        assert_eq!(reimann_alt_goals[1], (1, 0));
        assert_eq!(reimann_alt_goals[2], (2, 0));
        assert_eq!(reimann_alt_goals[BOARD_SIZE.pow(2) - 2], (5, 6));
        assert_eq!(reimann_alt_goals[BOARD_SIZE.pow(2) - 1], (6, 6));
    }

    #[test]
    fn test_get_alt_goals_euclid() {
        let euclid_alt_goals = NaiveStrategy::Euclid.get_alt_goals((1, 1));
        assert_eq!(euclid_alt_goals.len(), BOARD_SIZE.pow(2));
        assert_eq!(euclid_alt_goals[0], (1, 1));
        assert_eq!(euclid_alt_goals[1], (1, 0));
        assert_eq!(euclid_alt_goals[2], (0, 1));
        assert_eq!(euclid_alt_goals[3], (2, 1));
        assert_eq!(euclid_alt_goals[4], (1, 2));
        assert_eq!(euclid_alt_goals[BOARD_SIZE.pow(2) - 2], (5, 6));
        assert_eq!(euclid_alt_goals[BOARD_SIZE.pow(2) - 1], (6, 6));
    }

    #[test]
    fn test_find_move_to_reach() {
        let board_state: PlayerBoardState = PlayerBoardState {
            board: Board::default(),
            players: vec![
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
            ],
            last: None,
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
        let euclid = NaiveStrategy::Euclid;
        let reimann = NaiveStrategy::Reimann;
        let start = board_state.players[0].current;
        let destination = (0, 1);
        assert_eq!(
            euclid.find_move_to_reach(&board_state, start, destination),
            Some(PlayerMove {
                slide: Slide::new(0, West).unwrap(),
                rotations: 0,
                destination: (0, 1),
            })
        );
        assert_eq!(
            reimann.find_move_to_reach(&board_state, start, destination),
            Some(PlayerMove {
                slide: Slide::new(0, West).unwrap(),
                rotations: 0,
                destination: (0, 1),
            })
        );

        // no move will take you from (4, 1) -> (2, 3)
        let destination = (2, 3);
        assert_eq!(
            euclid.find_move_to_reach(&board_state, start, destination),
            None
        );
        assert_eq!(
            reimann.find_move_to_reach(&board_state, start, destination),
            None
        );

        let board_state = PlayerBoardState {
            board: Board::default(),
            players: vec![
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
            ],
            last: None,
        };
        // you can go from (6, 1) -> (1, 1) by wrapping around the board
        let start = board_state.players[0].current;
        let destination = (1, 1);
        assert_eq!(
            euclid.find_move_to_reach(&board_state, start, destination),
            Some(PlayerMove {
                slide: Slide::new(0, East).unwrap(),
                rotations: 0,
                destination: (1, 1)
            })
        );
        assert_eq!(
            reimann.find_move_to_reach(&board_state, start, destination),
            Some(PlayerMove {
                slide: Slide::new(0, East).unwrap(),
                rotations: 0,
                destination: (1, 1)
            })
        )
    }

    #[test]
    fn test_reachable_after_move() {
        let board_state: PlayerBoardState = PlayerBoardState {
            board: Board::default(),
            players: vec![
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
            ],
            last: None,
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
        assert_eq!(board_state.board.reachable((0, 0)).unwrap(), vec![(0, 0)]);
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
            &board_state,
            player_move,
            (0, 0)
        ));

        // slide the bottom row left
        let player_move = PlayerMove {
            slide: Slide::new(6, West).unwrap(),
            rotations: 0,
            destination: (1, 5),
        };
        // starting at (2, 6) you can go to (1, 5)
        assert!(board_state
            .board
            .reachable((2, 6))
            .unwrap()
            .contains(&(1, 5)));
        // If you start at (2, 6) can you go to (1, 5) after making move? no
        assert!(!NaiveStrategy::reachable_after_move(
            &board_state,
            player_move,
            (2, 6)
        ));
    }
}
